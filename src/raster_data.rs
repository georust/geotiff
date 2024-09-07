use std::fmt;
use std::fmt::{Debug, Formatter};

pub(super) enum RasterData {
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

impl Debug for RasterData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "RasterData {{ type: {}, len: {} }}",
            match self {
                RasterData::U8(_) => "u8",
                RasterData::U16(_) => "u16",
                RasterData::U32(_) => "u32",
                RasterData::U64(_) => "u64",
                RasterData::F32(_) => "f32",
                RasterData::F64(_) => "f64",
                RasterData::I8(_) => "i8",
                RasterData::I16(_) => "i16",
                RasterData::I32(_) => "i32",
                RasterData::I64(_) => "i64",
            },
            self.len()
        ))
    }
}

impl RasterData {
    fn len(&self) -> usize {
        match self {
            RasterData::U8(data) => data.len(),
            RasterData::U16(data) => data.len(),
            RasterData::U32(data) => data.len(),
            RasterData::U64(data) => data.len(),
            RasterData::F32(data) => data.len(),
            RasterData::F64(data) => data.len(),
            RasterData::I8(data) => data.len(),
            RasterData::I16(data) => data.len(),
            RasterData::I32(data) => data.len(),
            RasterData::I64(data) => data.len(),
        }
    }
}
