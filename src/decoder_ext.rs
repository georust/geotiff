use std::io::{Read, Seek};

use tiff::decoder::Decoder;
use tiff::tags::Tag;
use tiff::TiffResult;

use crate::coordinate_transform::CoordinateTransform;
use crate::geo_key_directory::GeoKeyDirectory;

pub(super) trait DecoderExt {
    fn coordinate_transform(&mut self) -> TiffResult<Option<CoordinateTransform>>;

    fn geo_key_directory(&mut self) -> TiffResult<GeoKeyDirectory>;
}

impl<R: Read + Seek> DecoderExt for Decoder<R> {
    fn coordinate_transform(&mut self) -> TiffResult<Option<CoordinateTransform>> {
        let pixel_scale_data = self
            .find_tag(Tag::ModelPixelScaleTag)?
            .map(|value| value.into_f64_vec())
            .transpose()?;
        let tie_points_data = self
            .find_tag(Tag::ModelTiepointTag)?
            .map(|value| value.into_f64_vec())
            .transpose()?;
        let model_transformation_data = self
            .find_tag(Tag::ModelTransformationTag)?
            .map(|value| value.into_f64_vec())
            .transpose()?;

        if pixel_scale_data.is_none()
            && tie_points_data.is_none()
            && model_transformation_data.is_none()
        {
            return Ok(None);
        }

        Ok(Some(CoordinateTransform::from_tag_data(
            pixel_scale_data,
            tie_points_data,
            model_transformation_data,
        )?))
    }

    fn geo_key_directory(&mut self) -> TiffResult<GeoKeyDirectory> {
        let Some(directory_data) = self
            .find_tag(Tag::GeoKeyDirectoryTag)?
            .map(|v| v.into_u16_vec())
            .transpose()?
        else {
            return Ok(GeoKeyDirectory::default());
        };

        let double_params_data = self
            .find_tag(Tag::GeoDoubleParamsTag)?
            .map(|v| v.into_f64_vec())
            .transpose()?
            .unwrap_or_else(|| Vec::with_capacity(0));

        let ascii_params_data = self
            .find_tag(Tag::GeoAsciiParamsTag)?
            .map(|v| v.into_string())
            .transpose()?
            .unwrap_or_else(|| String::with_capacity(0));

        GeoKeyDirectory::from_tag_data(directory_data, double_params_data, ascii_params_data)
    }
}
