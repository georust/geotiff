use tiff::decoder::ifd::Value;
use tiff::tags::{Tag, Type};

use crate::lowlevel::*;

/// The basic TIFF struct. This includes the header (specifying byte order and IFD offsets) as
/// well as all the image file directories (IFDs) plus image data.
///
/// The image data has a size of width * length * bytes_per_sample.
#[derive(Debug)]
pub struct TIFF {
    pub ifds: Vec<IFD>,
    // This is width * length * bytes_per_sample.
    pub image_data: Vec<Vec<Vec<usize>>>,
}

/// The header of a TIFF file. This comes first in any TIFF file and contains the byte order
/// as well as the offset to the IFD table.
#[derive(Debug)]
pub struct TIFFHeader {
    pub byte_order: TIFFByteOrder,
    pub ifd_offset: LONG,
}

/// An image file directory (IFD) within this TIFF. It contains the number of individual IFD entries
/// as well as a Vec with all the entries.
#[derive(Debug)]
pub struct IFD {
    pub count:   u16,
    pub entries: Vec<IFDEntry>,
}

/// A single entry within an image file directory (IDF). It consists of a tag, a type, and several
/// tag values.
#[derive(Debug)]
pub struct IFDEntry {
    pub tag: Tag,
    pub tpe: Type,
    pub count: LONG,
    pub value_offset: LONG,
    pub value: Vec<Value>,
}

/// Implementations for the IFD struct.
impl IFD {
    pub fn get_image_length() -> usize {
        3
    }

    pub fn get_image_width() -> usize {
        3
    }

    pub fn get_bytes_per_sample() -> usize {
        3
    }
}
