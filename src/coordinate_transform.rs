#[cfg(feature = "tie-points")]
use std::rc::Rc;

#[cfg(feature = "tie-points")]
use geo_index::rtree::OwnedRTree;
use geo_types::Coord;
use tiff::{TiffError, TiffFormatError, TiffResult};

#[cfg(feature = "tie-points")]
use crate::coordinate_transform::tie_points::Face;

mod affine_transform;
mod tie_point_and_pixel_scale;
#[cfg(feature = "tie-points")]
mod tie_points;

const MODEL_TIE_POINT_TAG: &str = "ModelTiePointTag";
const MODEL_PIXEL_SCALE_TAG: &str = "ModelPixelScaleTag";
const MODEL_TRANSFORMATION_TAG: &str = "ModelTransformationTag";

/// Defines the transformation between raster space and model space.
///
/// Ref: https://docs.ogc.org/is/19-008r4/19-008r4.html#_raster_to_model_coordinate_transformation_requirements
#[derive(Debug)]
pub enum CoordinateTransform {
    AffineTransform {
        transform: [f64; 6],
        inverse_transform: [f64; 6],
    },
    TiePointAndPixelScale {
        raster_point: Coord,
        model_point: Coord,
        pixel_scale: Coord,
    },
    #[cfg(feature = "tie-points")]
    TiePoints {
        raster_mesh: Rc<Vec<Face>>,
        raster_index: OwnedRTree<f64>,
        model_mesh: Rc<Vec<Face>>,
        model_index: OwnedRTree<f64>,
    },
}

impl CoordinateTransform {
    pub(super) fn from_tag_data(
        pixel_scale_data: Option<Vec<f64>>,
        model_tie_points_data: Option<Vec<f64>>,
        model_transformation_data: Option<Vec<f64>>,
    ) -> TiffResult<Self> {
        let pixel_scale = pixel_scale_data
            .map(|data| {
                <[f64; 3]>::try_from(data).map_err(|_| {
                    TiffError::FormatError(TiffFormatError::Format(format!(
                        "Number values in {MODEL_PIXEL_SCALE_TAG} must be equal to 3"
                    )))
                })
            })
            .transpose()?;
        let tie_points = model_tie_points_data
            .map(|data| {
                let len = data.len();
                if len == 0 {
                    return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                        "Number of values in {MODEL_TIE_POINT_TAG} must be greater than 0"
                    ))));
                }

                if len % 6 != 0 {
                    return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                        "Number of values in {MODEL_TIE_POINT_TAG} must be divisible by 6"
                    ))));
                }

                Ok(data)
            })
            .transpose()?;
        let transformation_matrix = model_transformation_data
            .map(|data| {
                <[f64; 16]>::try_from(data).map_err(|_| {
                    TiffError::FormatError(TiffFormatError::Format(format!(
                        "Number of values in {MODEL_TRANSFORMATION_TAG} must be equal to 16"
                    )))
                })
            })
            .transpose()?;

        if let Some(transformation_matrix) = transformation_matrix {
            if pixel_scale.is_some() {
                return Err(TiffError::FormatError(TiffFormatError::Format(
                    format!("{MODEL_PIXEL_SCALE_TAG} must not be specified when {MODEL_TRANSFORMATION_TAG} is present"),
                )));
            }
            if tie_points.is_some() {
                return Err(TiffError::FormatError(TiffFormatError::Format(
                    format!("{MODEL_TIE_POINT_TAG} must not be specified when {MODEL_TRANSFORMATION_TAG} is present"),
                )));
            }

            Self::from_transformation_matrix(transformation_matrix)
        } else {
            let Some(tie_points) = tie_points else {
                return Err(TiffError::FormatError(TiffFormatError::Format(
                    format!("{MODEL_TIE_POINT_TAG} must be present when {MODEL_TRANSFORMATION_TAG} is missing"),
                )));
            };

            if tie_points.len() == 6 {
                let Some(pixel_scale) = pixel_scale else {
                    return Err(TiffError::FormatError(TiffFormatError::Format(
                        format!("{MODEL_PIXEL_SCALE_TAG} must be specified when {MODEL_TIE_POINT_TAG} contains 6 values"),
                    )));
                };

                Self::from_tie_point_and_pixel_scale(&tie_points, &pixel_scale)
            } else {
                #[cfg(feature = "tie-points")]
                {
                    Self::from_tie_points(&tie_points)
                }
                #[cfg(not(feature = "tie-points"))]
                {
                    Err(TiffError::FormatError(TiffFormatError::Format(
                        "Transformation by tie points is not supported".into(),
                    )))
                }
            }
        }
    }

    pub fn transform_to_model(&self, coord: &Coord) -> Coord {
        match self {
            CoordinateTransform::AffineTransform { transform, .. } => {
                Self::transform_by_affine_transform(transform, coord)
            }
            CoordinateTransform::TiePointAndPixelScale {
                raster_point,
                model_point,
                pixel_scale,
            } => Self::transform_to_model_by_tie_point_and_pixel_scale(
                raster_point,
                model_point,
                pixel_scale,
                coord,
            ),
            #[cfg(feature = "tie-points")]
            CoordinateTransform::TiePoints {
                raster_index,
                raster_mesh,
                model_mesh,
                ..
            } => Self::transform_by_tie_points(raster_index, raster_mesh, model_mesh, coord),
        }
    }

    pub(super) fn transform_to_raster(&self, coord: &Coord) -> Coord {
        match self {
            CoordinateTransform::AffineTransform {
                inverse_transform, ..
            } => Self::transform_by_affine_transform(inverse_transform, coord),
            CoordinateTransform::TiePointAndPixelScale {
                raster_point,
                model_point,
                pixel_scale,
            } => Self::transform_to_raster_by_tie_point_and_pixel_scale(
                raster_point,
                model_point,
                pixel_scale,
                coord,
            ),
            #[cfg(feature = "tie-points")]
            CoordinateTransform::TiePoints {
                model_index,
                model_mesh,
                raster_mesh,
                ..
            } => Self::transform_by_tie_points(model_index, model_mesh, raster_mesh, coord),
        }
    }
}
