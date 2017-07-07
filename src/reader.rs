use std::io::{Result, Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;
use std::fs::File;
use num::FromPrimitive;

use byteorder::{ReadBytesExt, ByteOrder, BigEndian, LittleEndian};

use {TIFFByteOrder, IFD, IFDEntry, decode_tag, decode_tag_type,
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
            let entry = self.read_tag::<T>(entry_number, reader);
            match entry {
                Ok(e) => ifd.entries.push(e),
                Err(err) => println!("Invalid tag at index {}: {}", entry_number, err),
            }
        }

        Ok(ifd)
    }

    fn read_tag<Endian: ByteOrder>(&self, entry_number: usize, reader: &mut SeekableReader) -> Result<IFDEntry> {

        // Bytes 0..1: u16 tag ID
        let tag_value = try!(reader.read_u16::<Endian>());

        // Bytes 2..3: u16 field Type
        let tpe_value = try!(reader.read_u16::<Endian>());

        // Bytes 4..7: u32 number of Values of type
        let count_value = try!(reader.read_u32::<Endian>());

        // Bytes 8..11: u32 offset in file to Value
        let value_offset_value = try!(reader.read_u32::<Endian>());

        // Decode tag
        let tag_msg = format!("Invalid tag {:x}", tag_value);
        let tag = decode_tag(tag_value).expect(&tag_msg);

        // Decode type
        let tpe_msg = format!("Invalid tag type {:x}", tpe_value);
        let tpe = decode_tag_type(tpe_value).expect(&tpe_msg);

        // Create entry
        let mut ifd_entry = IFDEntry {
            tag: tag,
            tpe: tpe,
            count: count_value,
            value_offset: value_offset_value,
            value: None,
        };
/*
        let maybe_tac = type_and_count_for_tag(ifd_entry.tag);

        if maybe_tac.is_none() {
            return Err(Error::new(ErrorKind::Other,
                                  format!("Unknown tag {:?} in IFD", ifd_entry.tag)));
        }

        let (expected_typ, expected_count) = maybe_tac.unwrap();

        println!("IFD[{:?}] tag: {:?} type: {:?} count: {} offset: {:08x}",
                 entry_number, ifd_entry.tag, ifd_entry.typ, ifd_entry.count, ifd_entry.value_offset);

        let valid_short_or_long = expected_typ == TagType::ShortOrLongTag &&
            (ifd_entry.typ == TagType::ShortTag ||
             ifd_entry.typ == TagType::LongTag);

        if  ! valid_short_or_long && ifd_entry.typ != expected_typ {
            println!("    *** ERROR: expected typ: {:?} found: {:?}", expected_typ, ifd_entry.typ);
        }

        if expected_count != 0 && ifd_entry.count != expected_count {
            println!("    *** ERROR: expected count: {:?} found: {:?}", expected_count, ifd_entry.count);
        }

        if ifd_entry.count == 1 {
            ifd_entry.value = match ifd_entry.typ {
                TagType::ByteTag => Some(TagValue::ByteValue(ifd_entry.value_offset as BYTE)),
                TagType::ShortTag => Some(TagValue::ShortValue(ifd_entry.value_offset as SHORT)),
                TagType::LongTag => Some(TagValue::LongValue(ifd_entry.value_offset)),
                TagType::SignedByteTag => Some(TagValue::SignedByteValue(ifd_entry.value_offset as SBYTE)),
                TagType::SignedShortTag => Some(TagValue::SignedShortValue(ifd_entry.value_offset as SSHORT)),
                TagType::SignedLongTag => Some(TagValue::SignedLongValue(ifd_entry.value_offset as SLONG)),
                TagType::FloatTag => Some(TagValue::FloatValue(ifd_entry.value_offset as FLOAT)),
                TagType::ShortOrLongTag => Some(TagValue::LongValue(ifd_entry.value_offset as LONG)), // @todo FIXME
                _ => None
            };
        }

        println!("    {:?}", ifd_entry.value);*/

        Ok(ifd_entry)
    }
}
