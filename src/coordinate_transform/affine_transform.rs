use geo_types::Coord;
use tiff::{TiffError, TiffFormatError, TiffResult};

use crate::coordinate_transform::CoordinateTransform;

impl CoordinateTransform {
    pub fn from_transformation_matrix(transformation_matrix: [f64; 16]) -> TiffResult<Self> {
        let transform = [
            transformation_matrix[0],
            transformation_matrix[1],
            transformation_matrix[3],
            transformation_matrix[4],
            transformation_matrix[5],
            transformation_matrix[7],
        ];

        let det = transform[0] * transform[4] - transform[1] * transform[3];
        if det.abs() < 0.000000000000001 {
            return Err(TiffError::FormatError(TiffFormatError::Format(
                "Provided transformation matrix is not invertible".into(),
            )));
        }

        let inverse_transform = [
            transform[4] / det,
            -transform[1] / det,
            (transform[1] * transform[5] - transform[2] * transform[4]) / det,
            -transform[3] / det,
            transform[0] / det,
            (-transform[0] * transform[5] + transform[2] * transform[3]) / det,
        ];

        Ok(CoordinateTransform::AffineTransform {
            transform,
            inverse_transform,
        })
    }

    pub(super) fn transform_by_affine_transform(transform: &[f64; 6], coord: &Coord) -> Coord {
        Coord {
            x: coord.x * transform[0] + coord.y * transform[1] + transform[2],
            y: coord.x * transform[3] + coord.y * transform[4] + transform[5],
        }
    }
}
