use std::array;
use std::rc::Rc;

use delaunator::{Point, Triangulation};
use geo_index::rtree::sort::STRSort;
use geo_index::rtree::{OwnedRTree, RTreeBuilder, RTreeIndex};
use geo_types::Coord;
use tiff::TiffResult;

use crate::coordinate_transform::CoordinateTransform;

impl CoordinateTransform {
    pub(super) fn from_tie_points(tie_points: &[f64]) -> TiffResult<CoordinateTransform> {
        let capacity = tie_points.iter().len() / 6;
        let mut raster_points = Vec::with_capacity(capacity);
        let mut model_points = Vec::with_capacity(capacity);

        for chunk in tie_points.chunks(6) {
            raster_points.push(Point {
                x: chunk[0],
                y: chunk[1],
            });
            model_points.push(Point {
                x: chunk[3],
                y: chunk[4],
            });
        }

        let triangulation = delaunator::triangulate(&raster_points);
        let raster_mesh = Rc::new(Self::build_faces(raster_points, &triangulation));
        let model_mesh = Rc::new(Self::build_faces(model_points, &triangulation));
        let raster_index = Self::build_index(&raster_mesh);
        let model_index = Self::build_index(&model_mesh);

        Ok(Self::TiePoints {
            raster_mesh: raster_mesh.clone(),
            raster_index,
            model_mesh: model_mesh.clone(),
            model_index,
        })
    }

    fn build_faces(points: Vec<Point>, triangulation: &Triangulation) -> Vec<Face> {
        let Triangulation {
            triangles, hull, ..
        } = triangulation;

        let len = hull.len();
        let mut angle_bisectors = vec![None; points.len()];
        for i in 0..len {
            let pi = hull[i];
            let ci = hull[(i + 1) % len];
            let ni = hull[(i + 2) % len];

            let prev = &points[pi];
            let curr = &points[ci];
            let next = &points[ni];

            let prev_curr = Coord {
                x: curr.x - prev.x,
                y: curr.y - prev.y,
            }
            .normalize();
            let next_curr = Coord {
                x: curr.x - next.x,
                y: curr.y - next.y,
            }
            .normalize();
            let direction = Coord {
                x: prev_curr.x + next_curr.x,
                y: prev_curr.y + next_curr.y,
            }
            .normalize();

            angle_bisectors[ci] = Some(direction);
        }
        triangles
            .chunks(3)
            .map(|chunk| {
                let i1 = chunk[0];
                let i2 = chunk[1];
                let i3 = chunk[2];

                let b12 = hull.as_slice().contains_sequence(&chunk[0..2]);
                let b23 = hull.as_slice().contains_sequence(&chunk[1..3]);
                let b31 = hull.as_slice().contains_sequence(&[i3, i1]);

                let c1 = Coord {
                    x: points[i1].x,
                    y: points[i1].y,
                };
                let c2 = Coord {
                    x: points[i2].x,
                    y: points[i2].y,
                };
                let c3 = Coord {
                    x: points[i3].x,
                    y: points[i3].y,
                };

                let boundary = if b12 {
                    if b23 {
                        if b31 {
                            // Open
                            None
                        } else {
                            // Closed at edge 3-1
                            Some(Boundary::Open {
                                coords: vec![c3, c1],
                                from_direction: angle_bisectors[i3].unwrap(),
                                to_direction: angle_bisectors[i1].unwrap(),
                            })
                        }
                    } else if b31 {
                        // Closed at edge 2-3
                        Some(Boundary::Open {
                            coords: vec![c2, c3],
                            from_direction: angle_bisectors[i2].unwrap(),
                            to_direction: angle_bisectors[i3].unwrap(),
                        })
                    } else {
                        // Closed at edges 2-3 and 3-1
                        Some(Boundary::Open {
                            coords: vec![c2, c3, c1],
                            from_direction: angle_bisectors[i2].unwrap(),
                            to_direction: angle_bisectors[i1].unwrap(),
                        })
                    }
                } else if b23 {
                    if b31 {
                        // Closed at edge 1-2
                        Some(Boundary::Open {
                            coords: vec![c1, c2],
                            from_direction: angle_bisectors[i1].unwrap(),
                            to_direction: angle_bisectors[i2].unwrap(),
                        })
                    } else {
                        // Closed at edges 1-2 and 3-1
                        Some(Boundary::Open {
                            coords: vec![c3, c1, c2],
                            from_direction: angle_bisectors[i3].unwrap(),
                            to_direction: angle_bisectors[i2].unwrap(),
                        })
                    }
                } else if b31 {
                    // Closed at edges 1-2 and 2-3
                    Some(Boundary::Open {
                        coords: vec![c1, c2, c3],
                        from_direction: angle_bisectors[i1].unwrap(),
                        to_direction: angle_bisectors[i3].unwrap(),
                    })
                } else {
                    // Closed
                    Some(Boundary::Closed {
                        coords: vec![c1, c2, c3, c1],
                    })
                };

                Face {
                    boundary,
                    support_points: array::from_fn(|i| {
                        let point = &points[chunk[i]];
                        Coord {
                            x: point.x,
                            y: point.y,
                        }
                    }),
                }
            })
            .collect()
    }

    fn build_index(mesh: &[Face]) -> OwnedRTree<f64> {
        let mut builder = RTreeBuilder::new(mesh.len());
        for face in mesh.iter() {
            let (min_x, min_y, max_x, max_y) = face.compute_envelope();
            builder.add(min_x, min_y, max_x, max_y);
        }
        builder.finish::<STRSort>()
    }

    pub(super) fn transform_by_tie_points(
        source_index: &OwnedRTree<f64>,
        source_mesh: &Rc<Vec<Face>>,
        target_mesh: &Rc<Vec<Face>>,
        coord: &Coord,
    ) -> Coord {
        let index = source_index
            .search(coord.x, coord.y, coord.x, coord.y)
            .into_iter()
            .find(|face_index| source_mesh[*face_index].contains(coord))
            .unwrap();
        let uv = source_mesh[index].locate(coord);
        target_mesh[index].interpolate(uv)
    }
}

#[derive(Debug)]
pub struct Face {
    boundary: Option<Boundary>,
    support_points: [Coord; 3],
}

impl Face {
    fn contains(&self, coord: &Coord) -> bool {
        let Some(boundary) = &self.boundary else {
            return true;
        };

        let check = |c1: &Coord, c2: &Coord| -> bool {
            ((c2.x - c1.x) * (coord.y - c1.y) - (c2.y - c1.y) * (coord.x - c1.x)).is_sign_positive()
        };

        match boundary {
            Boundary::Open {
                coords,
                from_direction,
                to_direction,
            } => {
                check(&(coords[0] + *from_direction), &coords[1])
                    && check(&coords[1], &(coords[1] + *to_direction))
                    && coords.windows(2).all(|w| check(&w[0], &w[1]))
            }
            Boundary::Closed { coords } => {
                let len = self.support_points.len();
                (0..=len).all(|i| check(&coords[i], &coords[(i + 1) % len]))
            }
        }
    }

    fn locate(&self, coord: &Coord) -> [f64; 2] {
        let [a, b, c] = &self.support_points;
        let d = c.x * (a.y - b.y) - b.x * (a.y - c.y) + a.x * (b.y - c.y);
        [
            -(coord.x * (a.y - c.y) - c.x * (a.y - coord.y) + a.x * (c.y - coord.y)) / d,
            (coord.x * (a.y - b.y) - b.x * (a.y - coord.y) + a.x * (b.y - coord.y)) / d,
        ]
    }

    fn interpolate(&self, params: [f64; 2]) -> Coord {
        let [u, v] = params;
        let [a, b, c] = &self.support_points;
        Coord {
            x: -u * a.x - v * a.x + a.x + u * b.x + v * c.x,
            y: -u * a.y - v * a.y + a.y + u * b.y + v * c.y,
        }
    }

    fn compute_envelope(&self) -> (f64, f64, f64, f64) {
        let Some(boundary) = &self.boundary else {
            return (f64::MIN, f64::MIN, f64::MAX, f64::MAX);
        };

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        let mut update_envelope = |coords: &[Coord]| {
            for c in coords {
                min_x = min_x.min(c.x);
                min_y = min_y.min(c.y);
                max_x = max_x.max(c.x);
                max_y = max_y.max(c.y);
            }
        };

        match boundary {
            Boundary::Open {
                coords,
                from_direction,
                to_direction,
            } => {
                update_envelope(coords);

                for direction in [from_direction, to_direction] {
                    // Compute right-hand side normal vector
                    let nx = direction.y;
                    let ny = -direction.x;

                    if nx.is_sign_positive() {
                        max_x = f64::MAX;
                    } else {
                        min_x = f64::MIN;
                    }

                    if ny.is_sign_positive() {
                        max_y = f64::MAX;
                    } else {
                        min_y = f64::MIN;
                    }
                }
            }
            Boundary::Closed { coords } => update_envelope(coords),
        }

        (min_x, min_y, max_x, max_y)
    }
}

#[derive(Debug)]
enum Boundary {
    Open {
        coords: Vec<Coord>,
        from_direction: Coord,
        to_direction: Coord,
    },
    Closed {
        coords: Vec<Coord>,
    },
}

pub(crate) trait CoordExt {
    fn normalize(self) -> Self;
}

impl CoordExt for Coord {
    fn normalize(mut self) -> Self {
        let len = (self.x.powi(2) + self.y.powi(2)).sqrt();
        self.x /= len;
        self.y /= len;
        self
    }
}

trait HullExt<T>
where
    T: PartialEq<T>,
{
    fn contains_sequence(&self, seq: &[T]) -> bool;
}

impl<T> HullExt<T> for &[T]
where
    T: PartialEq<T>,
{
    fn contains_sequence(&self, seq: &[T]) -> bool {
        let len = self.len();
        for i in 0..len {
            let (a, b) = seq.split_at(seq.len().min(len - i));
            if self[i..].starts_with(a) && self.starts_with(b) {
                return true;
            }
        }
        false
    }
}
