//! A [GeoTIFF](https://www.ogc.org/standard/geotiff) library for Rust
use std::any::type_name;
use std::io::{Read, Seek};

use num_traits::FromPrimitive;
use tiff::decoder::{Decoder, DecodingResult};
use tiff::tags::Tag;
use tiff::TiffResult;

pub use crate::geo_key_directory::*;

use crate::coordinate_transform::*;
use crate::decoder_ext::*;
use crate::raster_data::*;

mod coordinate_transform;
mod decoder_ext;
mod geo_key_directory;
mod raster_data;

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

/// The basic GeoTIFF struct. This includes any metadata as well as the actual raster data.
///
/// The raster data has a size of raster_width * raster_height * num_samples
#[derive(Debug)]
pub struct GeoTiff {
    pub geo_key_directory: GeoKeyDirectory,
    pub raster_width: usize,
    pub raster_height: usize,
    pub num_samples: usize,
    coordinate_transform: Option<CoordinateTransform>,
    raster_data: RasterData,
}

impl GeoTiff {
    pub fn read<R: Read + Seek>(reader: R) -> TiffResult<Self> {
        let mut decoder = Decoder::new(reader)?;

        let geo_key_directory = decoder.geo_key_directory()?;
        let coordinate_transform = decoder.coordinate_transform()?;

        let (raster_width, raster_height) = decoder
            .dimensions()
            .map(|(width, height)| (width as usize, height as usize))?;
        let num_samples = match decoder.find_tag(Tag::SamplesPerPixel)? {
            None => 1,
            Some(value) => value.into_u16()? as usize,
        };

        let raster_data = match decoder.read_image()? {
            DecodingResult::U8(data) => RasterData::U8(data),
            DecodingResult::U16(data) => RasterData::U16(data),
            DecodingResult::U32(data) => RasterData::U32(data),
            DecodingResult::U64(data) => RasterData::U64(data),
            DecodingResult::F32(data) => RasterData::F32(data),
            DecodingResult::F64(data) => RasterData::F64(data),
            DecodingResult::I8(data) => RasterData::I8(data),
            DecodingResult::I16(data) => RasterData::I16(data),
            DecodingResult::I32(data) => RasterData::I32(data),
            DecodingResult::I64(data) => RasterData::I64(data),
        };

        Ok(Self {
            geo_key_directory,
            raster_width,
            raster_height,
            num_samples,
            coordinate_transform,
            raster_data,
        })
    }

    pub fn get_value_at<T: FromPrimitive + 'static>(&self, x: usize, y: usize, sample: usize) -> T {
        let GeoTiff {
            raster_width,
            num_samples,
            raster_data,
            ..
        } = self;

        if &sample >= num_samples {
            panic!(
                "sample out of bounds: the number of samples is {} but the sample is {}",
                num_samples, sample
            )
        }

        let index = (y * raster_width + x) * num_samples + sample;
        match raster_data {
            RasterData::U8(data) => unwrap_primitive_type!(T::from_u8(data[index]), u8, T),
            RasterData::U16(data) => unwrap_primitive_type!(T::from_u16(data[index]), u16, T),
            RasterData::U32(data) => unwrap_primitive_type!(T::from_u32(data[index]), u32, T),
            RasterData::U64(data) => unwrap_primitive_type!(T::from_u64(data[index]), u64, T),
            RasterData::F32(data) => unwrap_primitive_type!(T::from_f32(data[index]), f32, T),
            RasterData::F64(data) => unwrap_primitive_type!(T::from_f64(data[index]), f64, T),
            RasterData::I8(data) => unwrap_primitive_type!(T::from_i8(data[index]), i8, T),
            RasterData::I16(data) => unwrap_primitive_type!(T::from_i16(data[index]), i16, T),
            RasterData::I32(data) => unwrap_primitive_type!(T::from_i32(data[index]), i32, T),
            RasterData::I64(data) => unwrap_primitive_type!(T::from_i64(data[index]), i64, T),
        }
    }
}
