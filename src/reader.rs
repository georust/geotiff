use std::io::{Result, Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;
use std::fs::File;
use num::FromPrimitive;

use byteorder::{ReadBytesExt, ByteOrder, BigEndian, LittleEndian};

use {TIFFByteOrder, IFD, IFDEntry, decode_tag, decode_tag_type, tag_size,
BYTE, SBYTE, SHORT, SSHORT, LONG, SLONG, FLOAT, TagType, TagValue};

pub trait SeekableReader: Seek + Read {}
impl<T: Seek + Read> SeekableReader for T {}

pub struct TIFFReader;

impl TIFFReader {

    pub fn load(&self, filename: &str) -> Result<Box<()>> {
        let filepath = Path::new(filename);
        let mut reader = File::open(&filepath)?;

        self.read(&mut reader)
    }

    pub fn read(&self, reader: &mut SeekableReader) -> Result<Box<()>> {
        match self.read_byte_order(reader)? {
            TIFFByteOrder::LittleEndian => self.read_tiff::<LittleEndian>(reader),
            TIFFByteOrder::BigEndian => self.read_tiff::<BigEndian>(reader),
        }
    }

    pub fn read_byte_order(&self, reader: &mut SeekableReader) -> Result<TIFFByteOrder> {
        // Bytes 0-1: "II" or "MM"
        // Read and validate ByteOrder
        match TIFFByteOrder::from_u16(reader.read_u16::<LittleEndian>()?) {
            Some(TIFFByteOrder::LittleEndian) => Ok(TIFFByteOrder::LittleEndian),
            Some(TIFFByteOrder::BigEndian) => Ok(TIFFByteOrder::BigEndian),
            None => Err(Error::new(ErrorKind::Other, format!("Invalid byte order in header."))),
        }
    }

    fn read_tiff<T: ByteOrder>(&self, reader: &mut SeekableReader) -> Result<Box<()>> {
        self.read_magic::<T>(reader)?;
        let ifd_offset = self.read_ifd_offset::<T>(reader)?;
        let ifd = self.read_IFD::<T>(reader, ifd_offset)?;
        Ok(Box::new(()))
    }

    fn read_magic<T: ByteOrder>(&self, reader: &mut SeekableReader) -> Result<()> {
        // Bytes 2-3: 0042
        // Read and validate HeaderMagic
        match reader.read_u16::<T>()? {
            42 => Ok(()),
            _ => Err(Error::new(ErrorKind::Other, "Invalid magic number in header")),
        }
    }

    pub fn read_ifd_offset<T: ByteOrder>(&self, reader: &mut SeekableReader) -> Result<u32> {
        // Bytes 4-7: offset
        // Offset from start of file to first IFD
        let ifd_offset_field = reader.read_u32::<T>()?;
        println!("IFD offset: {:?}", ifd_offset_field);
        Ok(ifd_offset_field)
    }

    #[allow(non_snake_case)]
    fn read_IFD<T: ByteOrder>(&self, reader: &mut SeekableReader, ifd_offset: u32) -> Result<Box<IFD>> {
        try!(reader.seek(SeekFrom::Start(ifd_offset as u64)));
        // 2 byte count of IFD entries
        let entry_count = try!(reader.read_u16::<T>());

        println!("IFD entry count: {}", entry_count);

        let mut ifd = Box::new(IFD { count: entry_count, entries: Vec::with_capacity(entry_count as usize) });

        for entry_number in 0..entry_count as usize {
            let entry = self.read_tag::<T>(ifd_offset as u64 + 2, entry_number, reader);
            match entry {
                Ok(e) => ifd.entries.push(e),
                Err(err) => println!("Invalid tag at index {}: {}", entry_number, err),
            }
        }

        Ok(ifd)
    }

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

    fn vec_to_value<Endian: ByteOrder>(&self, vec: Vec<u8>, tpe: &TagType) -> TagValue {
        let len = vec.len();
        match tpe {
            &TagType::ByteTag => TagValue::ByteValue(vec[0]),
            &TagType::ASCIITag => TagValue::AsciiValue(String::from_utf8_lossy(&vec).to_string()),
            &TagType::ShortTag => TagValue::ShortValue(Endian::read_u16(&vec[..])),
            &TagType::LongTag => TagValue::LongValue(Endian::read_u32(&vec[..])),
            &TagType::RationalTag => TagValue::RationalValue((Endian::read_u32(&vec[..(len/2)]),
                Endian::read_u32(&vec[(len/2)..]))),
            &TagType::SignedByteTag => TagValue::SignedByteValue(vec[0] as i8),
            &TagType::SignedShortTag => TagValue::SignedShortValue(Endian::read_i16(&vec[..])),
            &TagType::SignedLongTag => TagValue::SignedLongValue(Endian::read_i32(&vec[..])),
            &TagType::SignedRationalTag => TagValue::SignedRationalValue((Endian::read_i32(&vec[..(len/2)]),
                Endian::read_i32(&vec[(len/2)..]))),
            &TagType::FloatTag => TagValue::FloatValue(Endian::read_f32(&vec[..])),
            &TagType::DoubleTag => TagValue::DoubleValue(Endian::read_f64(&vec[..])),
            &TagType::UndefinedTag => TagValue::ByteValue(0),
            _ => panic!("Tag not found"),
        }
    }

    fn read_tag<Endian: ByteOrder>(&self, ifd_offset: u64, entry_number: usize,
        reader: &mut SeekableReader) -> Result<IFDEntry> {
        println!("Reading tag at {}/{}", ifd_offset, entry_number);
        // Seek beginning (as each tag is 12 bytes long).
        try!(reader.seek(SeekFrom::Start(ifd_offset + 12 * entry_number as u64)));

        // Bytes 0..1: u16 tag ID
        let tag_value = try!(reader.read_u16::<Endian>());

        // Bytes 2..3: u16 field Type
        let tpe_value = try!(reader.read_u16::<Endian>());

        // Bytes 4..7: u32 number of Values of type
        let count_value = try!(reader.read_u32::<Endian>());

        // Bytes 8..11: u32 offset in file to Value
        let value_offset_value = try!(reader.read_u32::<Endian>());

        // Decode tag
        let tag_msg = format!("Invalid tag {:04X}", tag_value);
        let tag = decode_tag(tag_value).expect(&tag_msg);

        // Decode type
        let tpe_msg = format!("Invalid tag type {:04X}", tpe_value);
        let tpe = decode_tag_type(tpe_value).expect(&tpe_msg);
        let value_size = tag_size(&tpe);

        // Let's get the value(s) of this tag.
        let tot_size = count_value * value_size;
        println!("{:04X} {:04X} {:08X} {:08X} {:?} {:?} {:?} {:?}", tag_value, tpe_value,
            count_value, value_offset_value, tag, tpe, value_size, tot_size);

        let mut values = Vec::with_capacity(count_value as usize);
        if tot_size <= 4 {
            // Can directly read the value at the value field. For simplicity, we simply reset
            // the reader to the correct position.
            try!(reader.seek(SeekFrom::Start(ifd_offset + 12 * entry_number as u64 + 8)));
            for _ in 0..count_value as usize {
                let value = self.read_n(reader, value_size as u64);
                values.push(self.vec_to_value::<Endian>(value, &tpe));
            }
        } else {
            // Have to read from the address pointed at by the value field.
            try!(reader.seek(SeekFrom::Start(value_offset_value as u64)));
            for _ in 0..count_value as usize {
                let value = self.read_n(reader, value_size as u64);
                values.push(self.vec_to_value::<Endian>(value, &tpe));
            }
        }

        // Create entry
        let ifd_entry = IFDEntry {
            tag: tag,
            tpe: tpe,
            count: count_value,
            value_offset: value_offset_value,
            value: values,
        };

        println!("IFD[{:?}] tag: {:?} type: {:?} count: {} offset: {:08x} value: {:?}",
                 entry_number, ifd_entry.tag, ifd_entry.tpe, ifd_entry.count,
                 ifd_entry.value_offset, ifd_entry.value);

        Ok(ifd_entry)
    }
}
