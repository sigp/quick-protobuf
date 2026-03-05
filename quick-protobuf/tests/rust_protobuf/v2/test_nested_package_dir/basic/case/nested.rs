// Automatically generated rust module for 'test_nested_basic_case_p0.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result, PackedFixed, PackedFixedIntoIter, PackedFixedRefIter};
use quick_protobuf::sizeofs::*;
use super::super::super::super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Enum {
    Variant_1 = 0,
}

impl Default for Enum {
    fn default() -> Self {
        Enum::Variant_1
    }
}

impl From<i32> for Enum {
    fn from(i: i32) -> Self {
        match i {
            0 => Enum::Variant_1,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Enum {
    fn from(s: &'a str) -> Self {
        match s {
            "Variant_1" => Enum::Variant_1,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Outer {
}

impl<'a> MessageRead<'a> for Outer {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Outer {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Outer {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Inner {
    pub enum_1: Option<test_nested_package_dir::basic::case::nested::Enum>,
}

impl<'a> MessageRead<'a> for Inner {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(0) => msg.enum_1 = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Inner {
    fn get_size(&self) -> usize {
        0
        + self.enum_1.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.enum_1.as_ref().map_or(Ok(()), |&m| w.write_with_tag(0, |w| w.write_enum(*&m as i32)))?;
        Ok(())
    }
}

}

