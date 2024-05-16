use tiff::tags::Type;

// Base types of the TIFF format.
pub type BYTE      = u8;
pub type SHORT     = u16;
pub type LONG      = u32;
pub type ASCII     = String;
pub type RATIONAL  = (u32, u32);
pub type SBYTE     = i8;
pub type SSHORT    = i16;
pub type SLONG     = i32;
pub type SRATIONAL = (i32, i32);
pub type FLOAT     = f32;
pub type DOUBLE    = f64;

// Different values individual components can take.
enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug)]
    pub enum TIFFByteOrder {
        LittleEndian = 0x4949,
        BigEndian    = 0x4d4d,
    }
}

/// Helper function that returns the size of a certain tag.
pub fn tag_size(t: &Type) -> u32 {
    match *t {
        Type::BYTE => 1,
        Type::ASCII => 1,
        Type::SHORT => 2,
        Type::LONG => 4,
        Type::RATIONAL => 8,
        Type::SBYTE => 1,
        Type::UNDEFINED => 1,
        Type::SSHORT => 2,
        Type::SLONG => 2,
        Type::SRATIONAL => 8,
        Type::FLOAT => 4,
        Type::DOUBLE => 8,
        Type::IFD => 4,
        Type::LONG8 => 8,
        Type::SLONG8 => 8,
        Type::IFD8 => 8,
        _ => unimplemented!(),
    }
}

/// All the possible values of tags.
#[derive(Debug)]
pub enum TagValue {
    ByteValue(BYTE),
    AsciiValue(ASCII),
    ShortValue(SHORT),
    LongValue(LONG),
    RationalValue(RATIONAL),
    SignedByteValue(SBYTE),
    SignedShortValue(SSHORT),
    SignedLongValue(SLONG),
    SignedRationalValue(SRATIONAL),
    FloatValue(FLOAT),
    DoubleValue(DOUBLE),
}

/// The photometric interpretation of the GeoTIFF.
#[repr(u16)]
#[derive(Debug)]
pub enum PhotometricInterpretation {
    WhiteIsZero = 0,
    BlackIsZero = 1,
}

/// The compression chosen for this TIFF.
#[repr(u16)]
#[derive(Debug)]
pub enum Compression {
    None     = 1,
    Huffman  = 2,
    LZW      = 5,
    OJPEG    = 6,
    JPEG     = 7,
    PackBits = 32773,
}

/// The resolution unit of this TIFF.
#[repr(u16)]
#[derive(Debug)]
pub enum ResolutionUnit {
    None       = 1,
    Inch       = 2,
    Centimetre = 3,
}

/// The sample format of this TIFF.
#[repr(u16)]
#[derive(Debug)]
pub enum SampleFormat {
    UnsignedInteger             = 1,
    TwosComplementSignedInteger = 2,
    IEEEFloatingPoint           = 3,
    Undefined                   = 4,
}

/// The image type of this TIFF.
#[derive(Debug)]
pub enum ImageType {
    Bilevel,
    Grayscale,
    PaletteColour,
    RGB,
    YCbCr,
}

/// The image orientation of this TIFF.
#[repr(u16)]
#[derive(Debug)]
pub enum ImageOrientation {
    TopLeft     = 1,	// row 0 top, col 0 lhs
    TopRight    = 2,	// row 0 top, col 0 rhs
    BottomRight = 3,	// row 0 bottom, col 0 rhs
    BottomLeft  = 4,	// row 0 bottom, col 0 lhs
    LeftTop     = 5,	// row 0 lhs, col 0 top
    RightTop    = 6, 	// row 0 rhs, col 0 top
    RightBottom = 7,	// row 0 rhs, col 0 bottom
    LeftBottom  = 8,	// row 0 lhs, col 0 bottom
}


// Baseline Tags
enum_from_primitive! {
    #[repr(u16)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum TIFFTag {

        // Baseline Tags
        ArtistTag                    = 0x013b,
        BitsPerSampleTag             = 0x0102,
        CellLengthTag                = 0x0109,
        CellWidthTag                 = 0x0108,
        ColorMapTag                  = 0x0140,
        CompressionTag               = 0x0103,
        CopyrightTag                 = 0x8298,
        DateTimeTag                  = 0x0132,
        ExtraSamplesTag              = 0x0152,
        FillOrderTag                 = 0x010a,
        FreeByteCountsTag            = 0x0121,
        FreeOffsetsTag               = 0x0120,
        GrayResponseCurveTag         = 0x0123,
        GrayResponseUnitTag          = 0x0122,
        HostComputerTag              = 0x013c,
        ImageDescriptionTag          = 0x010e,
        ImageLengthTag               = 0x0101,
        ImageWidthTag                = 0x0100,
        MakeTag                      = 0x010f,
        MaxSampleValueTag            = 0x0119,
        MinSampleValueTag            = 0x0118,
        ModelTag                     = 0x0110,
        NewSubfileTypeTag            = 0x00fe,
        OrientationTag               = 0x0112,
        PhotometricInterpretationTag = 0x0106,
        PlanarConfigurationTag       = 0x011c,
        PredictorTag                 = 0x013d,
        ResolutionUnitTag            = 0x0128,
        RowsPerStripTag              = 0x0116,
        SampleFormatTag              = 0x0153,
        SamplesPerPixelTag           = 0x0115,
        SoftwareTag                  = 0x0131,
        StripByteCountsTag           = 0x0117,
        StripOffsetsTag              = 0x0111,
        SubfileTypeTag               = 0x00ff,
        ThresholdingTag              = 0x0107,
        XResolutionTag               = 0x011a,
        YResolutionTag               = 0x011b,

        // Section 20: Colorimetry
        WhitePointTag                = 0x013e,
        PrimaryChromaticities        = 0x013f,
        TransferFunction             = 0x012d,
        TransferRange                = 0x0156,
        ReferenceBlackWhite          = 0x0214,

        // Section 21: YCbCr Images
        YCbCrCoefficients            = 0x0211,
        YCbCrSubsampling             = 0x0212,
        YCbCrPositioning             = 0x0213,

        // TIFF/EP Tags
        SubIFDsTag                   = 0x014a,
        JPEGTablesTag                = 0x015b,
        CFARepeatPatternDimTag       = 0x828d,
        BatteryLevelTag              = 0x828f,
        ModelPixelScaleTag           = 0x830e,
        IPTCTag                      = 0x83BB,
        ModelTiepointTag             = 0x8482,
        ModelTransformationTag       = 0x85D8,
        InterColorProfileTag         = 0x8773,
        GeoKeyDirectoryTag           = 0x87AF,
        GeoDoubleParamsTag           = 0x87B0,
        GeoAsciiParamsTag            = 0x87B1,
        InterlaceTag                 = 0x8829,
        TimeZoneOffsetTag            = 0x882a,
        SelfTimerModeTag             = 0x882b,
        NoiseTag                     = 0x920d,
        ImageNumberTag               = 0x9211,
        SecurityClassificationTag    = 0x9212,
        ImageHistoryTag              = 0x9213,
        EPStandardIdTag              = 0x9216,

        // Extension TIFF Tags
        // See http://www.awaresystems.be/imaging/tiff/tifftags/extension.html
        XMPTag                       = 0x02bc,

        // Private Tags
        PhotoshopTag                 = 0x8649,
        EXIFTag                      = 0x8769,

        GDALMETADATA                 = 0xA480,
        GDALNODATA                   = 0xA481,
    }
}

// Default Values
static PHOTOMETRIC_INTERPRETATION_SHORT_DEFAULT: SHORT = 1;
static PHOTOMETRIC_INTERPRETATION_LONG_DEFAULT: LONG = 1;
