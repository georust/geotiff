use geo_types::Coord;
use tiff::TiffResult;

use crate::coordinate_transform::CoordinateTransform;

impl CoordinateTransform {
    pub(super) fn from_tie_point_and_pixel_scale(
        tie_points: &[f64],
        pixel_scale: &[f64],
    ) -> TiffResult<Self> {
        Ok(CoordinateTransform::TiePointAndPixelScale {
            raster_point: Coord {
                x: tie_points[0],
                y: tie_points[1],
            },
            model_point: Coord {
                x: tie_points[3],
                y: tie_points[4],
            },
            pixel_scale: Coord {
                x: pixel_scale[0],
                y: pixel_scale[1],
            },
        })
    }

    pub(super) fn transform_to_model_by_tie_point_and_pixel_scale(
        raster_point: &Coord,
        model_point: &Coord,
        pixel_scale: &Coord,
        coord: &Coord,
    ) -> Coord {
        Coord {
            x: (coord.x - raster_point.x) * pixel_scale.x + model_point.x,
            y: (coord.y - raster_point.y) * -pixel_scale.y + model_point.y,
        }
    }

    pub(super) fn transform_to_raster_by_tie_point_and_pixel_scale(
        raster_point: &Coord,
        model_point: &Coord,
        pixel_scale: &Coord,
        coord: &Coord,
    ) -> Coord {
        Coord {
            x: (coord.x - model_point.x) / pixel_scale.x + raster_point.x,
            y: (coord.y - model_point.y) / -pixel_scale.y + raster_point.y,
        }
    }
}
