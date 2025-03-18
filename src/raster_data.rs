use std::fmt;
use std::fmt::{Debug, Formatter};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RasterValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl RasterValue {
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            RasterValue::U8(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        match self {
            RasterValue::U16(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            RasterValue::U32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            RasterValue::U64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            RasterValue::F32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            RasterValue::F64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i8(&self) -> Option<i8> {
        match self {
            RasterValue::I8(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i16(&self) -> Option<i16> {
        match self {
            RasterValue::I16(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            RasterValue::I32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            RasterValue::I64(value) => Some(*value),
            _ => None,
        }
    }
}

pub enum RasterData {
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

    pub fn get_value(&self, index: usize) -> RasterValue {
        match self {
            RasterData::U8(data) => RasterValue::U8(data[index]),
            RasterData::U16(data) => RasterValue::U16(data[index]),
            RasterData::U32(data) => RasterValue::U32(data[index]),
            RasterData::U64(data) => RasterValue::U64(data[index]),
            RasterData::F32(data) => RasterValue::F32(data[index]),
            RasterData::F64(data) => RasterValue::F64(data[index]),
            RasterData::I8(data) => RasterValue::I8(data[index]),
            RasterData::I16(data) => RasterValue::I16(data[index]),
            RasterData::I32(data) => RasterValue::I32(data[index]),
            RasterData::I64(data) => RasterValue::I64(data[index]),
        }
    }
}
