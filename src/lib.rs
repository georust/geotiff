//! A [GeoTIFF](https://www.ogc.org/standard/geotiff) library for Rust
use std::any::type_name;
use std::io::{Read, Seek};

use geo_types::{Coord, Rect};
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

macro_rules! unwrap_type_cast {
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

/// The raster value type that can be read from a GeoTIFF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RasterDataType {
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    CInt16,
    CInt32,
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
    /// Reads a GeoTIFF from the given source.
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
            DecodingResult::F16(data) => todo!(),
            DecodingResult::CInt16(data) => RasterData::Cint16(data),
            DecodingResult::CInt32(data) => RasterData::Cint32(data),
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

    /// Returns the sample type of the raster data.
    pub fn sample_type(&self) -> RasterDataType {
        match &self.raster_data {
            RasterData::U8(_) => RasterDataType::U8,
            RasterData::U16(_) => RasterDataType::U16,
            RasterData::U32(_) => RasterDataType::U32,
            RasterData::U64(_) => RasterDataType::U64,
            RasterData::F32(_) => RasterDataType::F32,
            RasterData::F64(_) => RasterDataType::F64,
            RasterData::I8(_) => RasterDataType::I8,
            RasterData::I16(_) => RasterDataType::I16,
            RasterData::I32(_) => RasterDataType::I32,
            RasterData::I64(_) => RasterDataType::I64,
            RasterData::Cint16(_) => RasterDataType::CInt16,
            RasterData::Cint32(_) => RasterDataType::CInt32,
        }
    }

    /// Returns the extent of the image in model space.
    pub fn model_extent(&self) -> Rect {
        let offset = self.raster_offset();
        let lower = Coord {
            x: offset,
            y: offset,
        };
        let upper = Coord {
            x: self.raster_width as f64 + offset,
            y: self.raster_height as f64 + offset,
        };

        if let Some(coordinate_transform) = &self.coordinate_transform {
            Rect::new(
                coordinate_transform.transform_to_model(&lower),
                coordinate_transform.transform_to_model(&upper),
            )
        } else {
            Rect::new(lower, upper)
        }
    }

    /// Returns the value at the given location for the specified sample.
    /// The coordinates are in model space.
    pub fn get_value_at(&self, coord: &Coord, sample: usize) -> Option<RasterValue> {
        let index = self.compute_index(coord, sample)?;
        let value = self.raster_data.get_value(index);
        Some(value)
    }

    /// Returns the value at the given pixel coordinates for the specified sample.
    /// The coordinates are in pixel space (0-based).
    pub fn get_value_at_pixel(&self, x: usize, y: usize, sample: usize) -> Option<RasterValue> {
        let GeoTiff {
            raster_width,
            raster_height,
            num_samples,
            ..
        } = self;

        // Check bounds
        if x >= *raster_width || y >= *raster_height {
            return None;
        }

        // Check sample bounds
        if sample >= *num_samples {
            panic!(
                "sample out of bounds: the number of samples is {} but the sample is {}",
                num_samples, sample
            );
        }

        // Compute index directly from pixel coordinates
        let index = (y * raster_width + x) * num_samples + sample;

        // Get the value from the appropriate data array
        let value = self.raster_data.get_value(index);

        Some(value)
    }

    fn compute_index(&self, coord: &Coord, sample: usize) -> Option<usize> {
        let GeoTiff {
            raster_width,
            raster_height,
            num_samples,
            coordinate_transform,
            ..
        } = self;

        if &sample >= num_samples {
            panic!(
                "sample out of bounds: the number of samples is {} but the sample is {}",
                num_samples, sample
            )
        }

        let mut coord = match coordinate_transform {
            None => *coord,
            Some(transform) => transform.transform_to_raster(coord),
        };

        // See https://docs.ogc.org/is/19-008r4/19-008r4.html#_raster_space for reference
        let raster_offset = self.raster_offset();
        coord.x -= raster_offset;
        coord.y -= raster_offset;

        if coord.x < 0.0
            || coord.x >= *raster_width as f64
            || coord.y < 0.0
            || coord.y >= *raster_height as f64
        {
            return None;
        }

        Some((coord.y as usize * raster_width + coord.x as usize) * num_samples + sample)
    }

    fn raster_offset(&self) -> f64 {
        match self.geo_key_directory.raster_type {
            Some(RasterType::RasterPixelIsPoint) => -0.5,
            _ => 0.0,
        }
    }
}
