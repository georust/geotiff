use std::any::type_name;
use std::io::{Read, Seek};

use num_traits::FromPrimitive;
use tiff::decoder::{Decoder, DecodingResult};
use tiff::tags::Tag;
use tiff::TiffResult;

use crate::image_data::*;

mod image_data;

macro_rules! unwrap_primitive_type {
    ($result: expr, $actual: ty, $expected: ty) => {
        $result
            .ok_or_else(|| {
                format!(
                    "Cannot represent {} as {}",
                    type_name::<$actual>(),
                    type_name::<$expected>()
                )
            })
            .unwrap()
    };
}

/// The basic GeoTIFF struct. This includes any metadata as well as the actual image data.
///
/// The image data has a size of raster_width * raster_height * num_samples
#[derive(Debug)]
pub struct GeoTiff {
    pub raster_width: usize,
    pub raster_height: usize,
    pub num_samples: usize,
    image_data: ImageData,
}

impl GeoTiff {
    pub fn read<R: Read + Seek>(reader: R) -> TiffResult<Self> {
        let mut decoder = Decoder::new(reader)?;

        let (raster_width, raster_height) = decoder
            .dimensions()
            .map(|(width, height)| (width as usize, height as usize))?;
        let num_samples = match decoder.find_tag(Tag::SamplesPerPixel)? {
            None => 1,
            Some(value) => value.into_u16()? as usize,
        };
        let image_data = match decoder.read_image()? {
            DecodingResult::U8(data) => ImageData::U8(data),
            DecodingResult::U16(data) => ImageData::U16(data),
            DecodingResult::U32(data) => ImageData::U32(data),
            DecodingResult::U64(data) => ImageData::U64(data),
            DecodingResult::F32(data) => ImageData::F32(data),
            DecodingResult::F64(data) => ImageData::F64(data),
            DecodingResult::I8(data) => ImageData::I8(data),
            DecodingResult::I16(data) => ImageData::I16(data),
            DecodingResult::I32(data) => ImageData::I32(data),
            DecodingResult::I64(data) => ImageData::I64(data),
        };

        Ok(Self {
            raster_width,
            raster_height,
            num_samples,
            image_data,
        })
    }

    pub fn get_value_at<T: FromPrimitive + 'static>(&self, x: usize, y: usize, sample: usize) -> T {
        let GeoTiff {
            raster_width,
            num_samples,
            image_data,
            ..
        } = self;

        if &sample >= num_samples {
            panic!(
                "sample out of bounds: the number of samples is {} but the sample is {}",
                num_samples, sample
            )
        }

        let index = (y * raster_width + x) * num_samples + sample;
        match image_data {
            ImageData::U8(data) => unwrap_primitive_type!(T::from_u8(data[index]), u8, T),
            ImageData::U16(data) => unwrap_primitive_type!(T::from_u16(data[index]), u16, T),
            ImageData::U32(data) => unwrap_primitive_type!(T::from_u32(data[index]), u32, T),
            ImageData::U64(data) => unwrap_primitive_type!(T::from_u64(data[index]), u64, T),
            ImageData::F32(data) => unwrap_primitive_type!(T::from_f32(data[index]), f32, T),
            ImageData::F64(data) => unwrap_primitive_type!(T::from_f64(data[index]), f64, T),
            ImageData::I8(data) => unwrap_primitive_type!(T::from_i8(data[index]), i8, T),
            ImageData::I16(data) => unwrap_primitive_type!(T::from_i16(data[index]), i16, T),
            ImageData::I32(data) => unwrap_primitive_type!(T::from_i32(data[index]), i32, T),
            ImageData::I64(data) => unwrap_primitive_type!(T::from_i64(data[index]), i64, T),
        }
    }
}
