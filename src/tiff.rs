use std::collections::{HashMap, HashSet};
use enum_primitive::FromPrimitive;
use lowlevel::*;

#[derive(Debug)]
pub struct TIFFHeader {
    pub byte_order: TIFFByteOrder,
    pub ifd_offset: LONG,
}

#[derive(Debug)]
pub struct IFDEntry {
    pub tag:          TIFFTag,
    pub tpe:          TagType,
    pub count:        LONG,
    pub value_offset: LONG,
    pub value:        Option<TagValue>,
}

#[derive(Debug)]
pub struct IFD {
    pub count:   u16,
    pub entries: Vec<IFDEntry>,
}

#[derive(Debug)]
pub struct TIFF {
    header: TIFFHeader,
    ifd: HashMap<TIFFTag, TagValue>,
}

pub fn decode_tag(value: u16) -> Option<TIFFTag> {
    TIFFTag::from_u16(value)
}

pub fn decode_tag_type(tpe: u16) -> Option<TagType> {
    TagType::from_u16(tpe)
}

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
