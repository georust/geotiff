use std::collections::{HashSet};
use enum_primitive::FromPrimitive;
use lowlevel::*;

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
    pub tag:          TIFFTag,
    pub tpe:          TagType,
    pub count:        LONG,
    pub value_offset: LONG,
    pub value:        Vec<TagValue>,
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

/// Decodes an u16 value into a TIFFTag.
pub fn decode_tag(value: u16) -> Option<TIFFTag> {
    TIFFTag::from_u16(value)
}

/// Decodes an u16 value into a TagType.
pub fn decode_tag_type(tpe: u16) -> Option<TagType> {
    TagType::from_u16(tpe)
}

/// Validation functions to make sure all the required tags are existing for a certain GeoTiff
/// image type (e.g., grayscale or RGB image).
pub fn validate_required_tags_for(typ: &ImageType) -> Option<HashSet<TIFFTag>> {
    let required_grayscale_tags: HashSet<TIFFTag> = [
        TIFFTag::ImageWidthTag,
        TIFFTag::ImageLengthTag,
        TIFFTag::BitsPerSampleTag,
        TIFFTag::CompressionTag,
        TIFFTag::PhotometricInterpretationTag,
        TIFFTag::StripOffsetsTag,
        TIFFTag::RowsPerStripTag,
        TIFFTag::StripByteCountsTag,
        TIFFTag::XResolutionTag,
        TIFFTag::YResolutionTag,
        TIFFTag::ResolutionUnitTag].iter().cloned().collect();

    let required_rgb_image_tags: HashSet<TIFFTag> = [
        TIFFTag::ImageWidthTag,
        TIFFTag::ImageLengthTag,
        TIFFTag::BitsPerSampleTag,
        TIFFTag::CompressionTag,
        TIFFTag::PhotometricInterpretationTag,
        TIFFTag::StripOffsetsTag,
        TIFFTag::SamplesPerPixelTag,
        TIFFTag::RowsPerStripTag,
        TIFFTag::StripByteCountsTag,
        TIFFTag::XResolutionTag,
        TIFFTag::YResolutionTag,
        TIFFTag::ResolutionUnitTag,
    ].iter().cloned().collect();

    match *typ {
        ImageType::Bilevel => None,
        ImageType::Grayscale => None,
        ImageType::PaletteColour => None,
        ImageType::RGB => Some(required_rgb_image_tags.difference(&required_grayscale_tags).cloned().collect()),
        ImageType::YCbCr => None,
    }
}
