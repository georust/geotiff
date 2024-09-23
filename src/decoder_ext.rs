use std::io::{Read, Seek};

use tiff::decoder::Decoder;
use tiff::tags::Tag;
use tiff::TiffResult;

use crate::geo_key_directory::GeoKeyDirectory;

pub(super) trait DecoderExt {
    fn geo_key_directory(&mut self) -> TiffResult<GeoKeyDirectory>;
}

impl<R: Read + Seek> DecoderExt for Decoder<R> {
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
