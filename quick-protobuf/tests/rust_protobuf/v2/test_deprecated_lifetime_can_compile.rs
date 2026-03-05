// Automatically generated rust module for 'test_deprecated_lifetime_can_compile.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![allow(deprecated)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result, PackedFixed, PackedFixedIntoIter, PackedFixedRefIter};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HasLifetimeParameterIfAddDeprecatedFields<'a> {
    #[deprecated]
    pub dep: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for HasLifetimeParameterIfAddDeprecatedFields<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.dep = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HasLifetimeParameterIfAddDeprecatedFields<'a> {
    fn get_size(&self) -> usize {
        0
        + self.dep.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.dep.as_ref().map_or(Ok(()), |m| w.write_with_tag(10, |w| w.write_string(&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct SameHere<'a> {
    pub ContainsDeprecated: mod_SameHere::OneOfContainsDeprecated<'a>,
}

impl<'a> MessageRead<'a> for SameHere<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ContainsDeprecated = mod_SameHere::OneOfContainsDeprecated::dep(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SameHere<'a> {
    fn get_size(&self) -> usize {
        0
        + match &self.ContainsDeprecated {
            mod_SameHere::OneOfContainsDeprecated::dep(ref m) => 1 + sizeof_len((&m).len()),
            mod_SameHere::OneOfContainsDeprecated::None => 0,
        }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match &self.ContainsDeprecated {
            mod_SameHere::OneOfContainsDeprecated::dep(m) => { w.write_with_tag(10, |w| w.write_string(m))? },
            mod_SameHere::OneOfContainsDeprecated::None => {},
        }
        Ok(())
    }
}

pub mod mod_SameHere {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfContainsDeprecated<'a> {
    #[deprecated]
    dep(Cow<'a, str>),
    None,
}

impl<'a> Default for OneOfContainsDeprecated<'a> {
    fn default() -> Self {
        OneOfContainsDeprecated::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct AndThisToo<'a> {
    #[deprecated]
    pub dep: Option<MessageWithLifetime<'a>>,
}

impl<'a> MessageRead<'a> for AndThisToo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.dep = Some(r.read_message::<MessageWithLifetime>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AndThisToo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.dep.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.dep.as_ref().map_or(Ok(()), |m| w.write_with_tag(10, |w| w.write_message(m)))?;
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

