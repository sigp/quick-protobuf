// Automatically generated rust module for 'test_enum_alias_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result, PackedFixed, PackedFixedIntoIter, PackedFixedRefIter};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnumWithAlias {
    UNKNOWN = 0,
    A = 10,
    B = 20,
    A_AGAIN = 10,
}

impl Default for EnumWithAlias {
    fn default() -> Self {
        EnumWithAlias::UNKNOWN
    }
}

impl From<i32> for EnumWithAlias {
    fn from(i: i32) -> Self {
        match i {
            0 => EnumWithAlias::UNKNOWN,
            10 => EnumWithAlias::A,
            20 => EnumWithAlias::B,
            10 => EnumWithAlias::A_AGAIN,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for EnumWithAlias {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => EnumWithAlias::UNKNOWN,
            "A" => EnumWithAlias::A,
            "B" => EnumWithAlias::B,
            "A_AGAIN" => EnumWithAlias::A_AGAIN,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct TestEnumWithAlias {
    pub en: Option<test_enum_alias::EnumWithAlias>,
}

impl<'a> MessageRead<'a> for TestEnumWithAlias {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.en = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestEnumWithAlias {
    fn get_size(&self) -> usize {
        0
        + self.en.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.en.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        Ok(())
    }
}

