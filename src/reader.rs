use std::io::{Result, Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;
use std::fs::File;
use num::FromPrimitive;

use byteorder::{ReadBytesExt, ByteOrder, BigEndian, LittleEndian};

use lowlevel::{TIFFByteOrder, TIFFTag, BYTE, SBYTE, SHORT, SSHORT, LONG, SLONG, FLOAT,
               TagType, TagValue, tag_size};
use tiff::{TIFF, IFD, IFDEntry, decode_tag, decode_tag_type};

/// A helper trait to indicate that something needs to be seekable and readable.
pub trait SeekableReader: Seek + Read {}

impl<T: Seek + Read> SeekableReader for T {}

/// The TIFF reader class that encapsulates all functionality related to reading `.tiff` files.
/// In particular, this includes reading the TIFF header, the image file directories (IDF), and
/// the plain data.
pub struct TIFFReader;

impl TIFFReader {
    /// Loads a `.tiff` file, as specified by `filename`.
    pub fn load(&self, filename: &str) -> Result<Box<TIFF>> {
        let filepath = Path::new(filename);
        let mut reader = File::open(&filepath)?;

        self.read(&mut reader)
    }

    /// Reads the `.tiff` file, starting with the byte order.
    pub fn read(&self, reader: &mut SeekableReader) -> Result<Box<TIFF>> {
        match self.read_byte_order(reader)? {
            TIFFByteOrder::LittleEndian => self.read_tiff::<LittleEndian>(reader),
            TIFFByteOrder::BigEndian => self.read_tiff::<BigEndian>(reader),
        }
    }

    /// Helper function to read the byte order, one of `LittleEndian` or `BigEndian`.
    pub fn read_byte_order(&self, reader: &mut SeekableReader) -> Result<TIFFByteOrder> {
        // Bytes 0-1: "II" or "MM"
        // Read and validate ByteOrder
        match TIFFByteOrder::from_u16(reader.read_u16::<LittleEndian>()?) {
            Some(TIFFByteOrder::LittleEndian) => Ok(TIFFByteOrder::LittleEndian),
            Some(TIFFByteOrder::BigEndian) => Ok(TIFFByteOrder::BigEndian),
            None => Err(Error::new(ErrorKind::Other, format!("Invalid byte order in header."))),
        }
    }

    /// Reads the `.tiff` file, given a `ByteOrder`.
    ///
    /// This starts by reading the magic number, the IFD offset, the IFDs themselves, and finally,
    /// the image data.
    fn read_tiff<T: ByteOrder>(&self, reader: &mut SeekableReader) -> Result<Box<TIFF>> {
        self.read_magic::<T>(reader)?;
        let ifd_offset = self.read_ifd_offset::<T>(reader)?;
        let ifd = self.read_IFD::<T>(reader, ifd_offset)?;
        let image_data = self.read_image_data::<T>(reader, &ifd)?;
        Ok(Box::new(TIFF {
            ifds: vec![ifd],
            image_data,
        }))
    }

    /// Reads the magic number, i.e., 42.
    fn read_magic<T: ByteOrder>(&self, reader: &mut SeekableReader) -> Result<()> {
        // Bytes 2-3: 0042
        // Read and validate HeaderMagic
        match reader.read_u16::<T>()? {
            42 => Ok(()),
            _ => Err(Error::new(ErrorKind::Other, "Invalid magic number in header")),
        }
    }

    /// Reads the IFD offset. The first IFD is then read from this position.
    pub fn read_ifd_offset<T: ByteOrder>(&self, reader: &mut SeekableReader) -> Result<u32> {
        // Bytes 4-7: offset
        // Offset from start of file to first IFD
        let ifd_offset_field = reader.read_u32::<T>()?;
        //println!("IFD offset: {:?}", ifd_offset_field);
        Ok(ifd_offset_field)
    }

    /// Reads an IFD.
    ///
    /// This starts by reading the number of entries, and then the tags within each entry.
    #[allow(non_snake_case)]
    fn read_IFD<T: ByteOrder>(&self, reader: &mut SeekableReader, ifd_offset: u32) -> Result<IFD> {
        reader.seek(SeekFrom::Start(ifd_offset as u64))?;
        // 2 byte count of IFD entries
        let entry_count = reader.read_u16::<T>()?;

        //println!("IFD entry count: {}", entry_count);

        let mut ifd = IFD { count: entry_count, entries: Vec::with_capacity(entry_count as usize) };

        for entry_number in 0..entry_count as usize {
            let entry = self.read_tag::<T>(ifd_offset as u64 + 2, entry_number, reader);
            match entry {
                Ok(e) => ifd.entries.push(e),
                Err(err) => println!("Invalid tag at index {}: {}", entry_number, err),
            }
        }

        Ok(ifd)
    }

    /// Reads `n` bytes from a reader into a Vec<u8>.
    fn read_n(&self, reader: &mut SeekableReader, bytes_to_read: u64) -> Vec<u8> {
        let mut buf = Vec::with_capacity(bytes_to_read as usize);
        let mut chunk = reader.take(bytes_to_read);
        let status = chunk.read_to_end(&mut buf);
        match status {
            Ok(n) => assert_eq!(bytes_to_read as usize, n),
            _ => panic!("Didn't read enough"),
        }
        buf
    }

    /// Converts a Vec<u8> into a TagValue, depending on the type of the tag. In the TIFF file
    /// format, each tag type indicates which value it stores (e.g., a byte, ascii, or long value).
    /// This means that the tag values have to be read taking the tag type into consideration.
    fn vec_to_tag_value<Endian: ByteOrder>(&self, vec: Vec<u8>, tpe: &TagType) -> TagValue {
        let len = vec.len();
        match tpe {
            &TagType::ByteTag => TagValue::ByteValue(vec[0]),
            &TagType::ASCIITag => TagValue::AsciiValue(String::from_utf8_lossy(&vec).to_string()),
            &TagType::ShortTag => TagValue::ShortValue(Endian::read_u16(&vec[..])),
            &TagType::LongTag => TagValue::LongValue(Endian::read_u32(&vec[..])),
            &TagType::RationalTag => TagValue::RationalValue((Endian::read_u32(&vec[..(len / 2)]),
                                                              Endian::read_u32(&vec[(len / 2)..]))),
            &TagType::SignedByteTag => TagValue::SignedByteValue(vec[0] as i8),
            &TagType::SignedShortTag => TagValue::SignedShortValue(Endian::read_i16(&vec[..])),
            &TagType::SignedLongTag => TagValue::SignedLongValue(Endian::read_i32(&vec[..])),
            &TagType::SignedRationalTag => TagValue::SignedRationalValue((Endian::read_i32(&vec[..(len / 2)]),
                                                                          Endian::read_i32(&vec[(len / 2)..]))),
            &TagType::FloatTag => TagValue::FloatValue(Endian::read_f32(&vec[..])),
            &TagType::DoubleTag => TagValue::DoubleValue(Endian::read_f64(&vec[..])),
            &TagType::UndefinedTag => TagValue::ByteValue(0),
            _ => panic!("Tag not found!"),
        }
    }

    /// Converts a number of u8 values to a usize value. This doesn't check if usize is at least
    /// u64, so be careful with large values.
    fn vec_to_value<Endian: ByteOrder>(&self, vec: Vec<u8>) -> usize {
        let len = vec.len();
        match len {
            0 => 0 as usize,
            1 => vec[0] as usize,
            2 => Endian::read_u16(&vec[..]) as usize,
            4 => Endian::read_u32(&vec[..]) as usize,
            8 => Endian::read_u64(&vec[..]) as usize,
            _ => panic!("Vector has wrong number of elements!"),
        }
    }

    /// Reads a single tag (given an IFD offset) into an IFDEntry.
    ///
    /// This consists of reading the tag ID, field type, number of values, offset to values. After
    /// decoding the tag and type, the values are retrieved.
    fn read_tag<Endian: ByteOrder>(&self, ifd_offset: u64, entry_number: usize,
                                   reader: &mut SeekableReader) -> Result<IFDEntry> {
        //println!("Reading tag at {}/{}", ifd_offset, entry_number);
        // Seek beginning (as each tag is 12 bytes long).
        reader.seek(SeekFrom::Start(ifd_offset + 12 * entry_number as u64))?;

        // Bytes 0..1: u16 tag ID
        let tag_value = reader.read_u16::<Endian>()?;

        // Bytes 2..3: u16 field Type
        let tpe_value = reader.read_u16::<Endian>()?;

        // Bytes 4..7: u32 number of Values of type
        let count_value = reader.read_u32::<Endian>()?;

        // Bytes 8..11: u32 offset in file to Value
        let value_offset_value = reader.read_u32::<Endian>()?;

        // Decode the tag.
        let tag_msg = format!("Invalid tag {:04X}", tag_value);
        let tag = decode_tag(tag_value).ok_or(Error::new(ErrorKind::InvalidData, tag_msg))?;

        // Decode the type.
        let tpe_msg = format!("Invalid tag type {:04X}", tpe_value);
        let tpe = decode_tag_type(tpe_value).expect(&tpe_msg);
        let value_size = tag_size(&tpe);

        // Let's get the value(s) of this tag.
        let tot_size = count_value * value_size;
        //println!("{:04X} {:04X} {:08X} {:08X} {:?} {:?} {:?} {:?}", tag_value, tpe_value,
                 count_value, value_offset_value, tag, tpe, value_size, tot_size);

        let mut values = Vec::with_capacity(count_value as usize);
        if tot_size <= 4 {
            // Can directly read the value at the value field. For simplicity, we simply reset
            // the reader to the correct position.
            reader.seek(SeekFrom::Start(ifd_offset + 12 * entry_number as u64 + 8))?;
            for _ in 0..count_value as usize {
                let value = self.read_n(reader, value_size as u64);
                values.push(self.vec_to_tag_value::<Endian>(value, &tpe));
            }
        } else {
            // Have to read from the address pointed at by the value field.
            reader.seek(SeekFrom::Start(value_offset_value as u64))?;
            for _ in 0..count_value as usize {
                let value = self.read_n(reader, value_size as u64);
                values.push(self.vec_to_tag_value::<Endian>(value, &tpe));
            }
        }

        // Create IFD entry.
        let ifd_entry = IFDEntry {
            tag,
            tpe,
            count: count_value,
            value_offset: value_offset_value,
            value: values,
        };

        //println!("IFD[{:?}] tag: {:?} type: {:?} count: {} offset: {:08x} value: {:?}",
                 entry_number, ifd_entry.tag, ifd_entry.tpe, ifd_entry.count,
                 ifd_entry.value_offset, ifd_entry.value);

        Ok(ifd_entry)
    }

    /// Reads the image data into a 3D-Vec<u8>.
    ///
    /// As for now, the following assumptions are made:
    /// * No compression is used, i.e., CompressionTag == 1.
    fn read_image_data<Endian: ByteOrder>(&self, reader: &mut SeekableReader,
                                          ifd: &IFD) -> Result<Vec<Vec<Vec<usize>>>> {
        // Image size and depth.
        let image_length = ifd.entries.iter().find(|&e| e.tag == TIFFTag::ImageLengthTag)
            .ok_or(Error::new(ErrorKind::InvalidData, "Image length not found."))?;
        let image_width = ifd.entries.iter().find(|&e| e.tag == TIFFTag::ImageWidthTag)
            .ok_or(Error::new(ErrorKind::InvalidData, "Image width not found."))?;
        let image_depth = ifd.entries.iter().find(|&e| e.tag == TIFFTag::BitsPerSampleTag)
            .ok_or(Error::new(ErrorKind::InvalidData, "Image depth not found."))?;

        // Storage location within the TIFF. First, lets get the number of rows per strip.
        let rows_per_strip = ifd.entries.iter().find(|&e| e.tag == TIFFTag::RowsPerStripTag)
            .ok_or(Error::new(ErrorKind::InvalidData, "Rows per strip not found."))?;
        // For each strip, its offset within the TIFF file.
        let strip_offsets = ifd.entries.iter().find(|&e| e.tag == TIFFTag::StripOffsetsTag)
            .ok_or(Error::new(ErrorKind::InvalidData, "Strip offsets not found."))?;
        let strip_byte_counts = ifd.entries.iter().find(|&e| e.tag == TIFFTag::StripByteCountsTag)
            .ok_or(Error::new(ErrorKind::InvalidData, "Strip byte counts not found."))?;

        // Create the output Vec.
        let image_length = match image_length.value[0] {
            TagValue::ShortValue(v) => v,
            _ => 0 as u16,
        };
        let image_width = match image_width.value[0] {
            TagValue::ShortValue(v) => v,
            _ => 0 as u16,
        };
        let image_depth = match image_depth.value[0] {
            TagValue::ShortValue(v) => v / 8,
            _ => 0 as u16,
        };
        // TODO The img Vec should optimally not be of usize, but of size "image_depth".
        let mut img: Vec<Vec<Vec<usize>>> = Vec::with_capacity(image_length as usize);
        for i in 0..image_length {
            &img.push(Vec::with_capacity(image_width as usize));
            for j in 0..image_width {
                &img[i as usize].push(vec![0; 1]); // TODO To be changed to take into account SamplesPerPixel!
            }
        }

        // Read strip after strip, and copy it into the output Vec.
        let rows_per_strip = match rows_per_strip.value[0] {
            TagValue::ShortValue(v) => v,
            _ => 0 as u16,
        };
        let mut offsets: Vec<u32> = Vec::with_capacity(strip_offsets.value.len());
        for v in &strip_offsets.value {
            match v {
                TagValue::LongValue(v) => offsets.push(*v),
                _ => (),
            };
        }
        let mut byte_counts: Vec<u32> = Vec::with_capacity(strip_byte_counts.value.len());
        for v in &strip_byte_counts.value {
            match v {
                TagValue::LongValue(v) => byte_counts.push(*v),
                _ => (),
            };
        }
        // A bit much boilerplate, but should be okay and fast.
        let mut curr_x = 0;
        let mut curr_y = 0;
        let mut curr_z = 0;
        for (offset, byte_count) in offsets.iter().zip(byte_counts.iter()) {
            reader.seek(SeekFrom::Start(*offset as u64))?;
            for i in 0..(*byte_count / image_depth as u32) {
                let v = self.read_n(reader, image_depth as u64);
                // println!("x {:?} len {:?}", curr_x, img.len());
                // println!("y {:?} wid {:?}", curr_y, img[0].len());
                // println!("z {:?} dep {:?}", curr_z, img[0][0].len());
                img[curr_x][curr_y][curr_z] = self.vec_to_value::<Endian>(v);
                curr_z += 1;
                if curr_z >= img[curr_x][curr_y].len() {
                    curr_z = 0;
                    curr_y += 1;
                }
                if curr_y >= img[curr_x].len() as usize {
                    curr_y = 0;
                    curr_x += 1;
                }
            }
        }

        // Return the output Vec.
        Ok(img)
    }
}
