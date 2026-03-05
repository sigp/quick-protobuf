// Automatically generated rust module for 'test_deprecated_lifetime_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result, PackedFixed, PackedFixedIntoIter, PackedFixedRefIter};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ThisShouldNotHaveALifetimeParameter {
}

impl<'a> MessageRead<'a> for ThisShouldNotHaveALifetimeParameter {
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

impl MessageWrite for ThisShouldNotHaveALifetimeParameter {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ThisShouldntEither {
    pub ContainsDeprecated: mod_ThisShouldntEither::OneOfContainsDeprecated,
}

impl<'a> MessageRead<'a> for ThisShouldntEither {
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

impl MessageWrite for ThisShouldntEither {
    fn get_size(&self) -> usize {
        0
        + match &self.ContainsDeprecated {
            mod_ThisShouldntEither::OneOfContainsDeprecated::None => 0,
        }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match &self.ContainsDeprecated {
            mod_ThisShouldntEither::OneOfContainsDeprecated::None => {},
        }
        Ok(())
    }
}

pub mod mod_ThisShouldntEither {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfContainsDeprecated {
    None,
}

impl Default for OneOfContainsDeprecated {
    fn default() -> Self {
        OneOfContainsDeprecated::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct NorShouldThis {
}

impl<'a> MessageRead<'a> for NorShouldThis {
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

impl MessageWrite for NorShouldThis {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct MessageWithLifetime<'a> {
    pub s: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for MessageWithLifetime<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.s = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MessageWithLifetime<'a> {
    fn get_size(&self) -> usize {
        0
        + self.s.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.s.as_ref().map_or(Ok(()), |m| w.write_with_tag(10, |w| w.write_string(&m)))?;
        Ok(())
    }
}

