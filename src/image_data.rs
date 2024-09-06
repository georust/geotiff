use std::fmt;
use std::fmt::{Debug, Formatter};

pub(super) enum ImageData {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    U64(Vec<u64>),
    F32(Vec<f32>),
    F64(Vec<f64>),
    I8(Vec<i8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    I64(Vec<i64>),
}

impl Debug for ImageData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "ImageData {{ type: {}, len: {} }}",
            match self {
                ImageData::U8(_) => "u8",
                ImageData::U16(_) => "u16",
                ImageData::U32(_) => "u32",
                ImageData::U64(_) => "u64",
                ImageData::F32(_) => "f32",
                ImageData::F64(_) => "f64",
                ImageData::I8(_) => "i8",
                ImageData::I16(_) => "i16",
                ImageData::I32(_) => "i32",
                ImageData::I64(_) => "i64",
            },
            self.len()
        ))
    }
}

impl ImageData {
    fn len(&self) -> usize {
        match self {
            ImageData::U8(data) => data.len(),
            ImageData::U16(data) => data.len(),
            ImageData::U32(data) => data.len(),
            ImageData::U64(data) => data.len(),
            ImageData::F32(data) => data.len(),
            ImageData::F64(data) => data.len(),
            ImageData::I8(data) => data.len(),
            ImageData::I16(data) => data.len(),
            ImageData::I32(data) => data.len(),
            ImageData::I64(data) => data.len(),
        }
    }
}
