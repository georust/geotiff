extern crate byteorder;
#[macro_use] extern crate enum_primitive;
extern crate num;

use num::FromPrimitive;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::io::{Read, Seek};
use std::collections::{HashMap, HashSet};

pub mod lowlevel;
pub mod tiff;
pub mod reader;

use lowlevel::*;
use tiff::*;

impl TIFF {
    pub fn open(filename: &str) {

    }
}
