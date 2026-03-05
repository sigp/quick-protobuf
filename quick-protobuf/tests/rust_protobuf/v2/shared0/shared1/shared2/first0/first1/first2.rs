// Automatically generated rust module for 'test_name_resolution_p0.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result, PackedFixed, PackedFixedIntoIter, PackedFixedRefIter};
use quick_protobuf::sizeofs::*;
use super::super::super::super::super::super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct TestPackagePrefixedFieldNames {
    pub b: Option<shared0::shared1::shared2::second0::second1::second2::B>,
}

impl<'a> MessageRead<'a> for TestPackagePrefixedFieldNames {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.b = Some(r.read_message::<shared0::shared1::shared2::second0::second1::second2::B>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestPackagePrefixedFieldNames {
    fn get_size(&self) -> usize {
        0
        + self.b.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.b.as_ref().map_or(Ok(()), |m| w.write_with_tag(10, |w| w.write_message(m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct TestNonUniquePackagePrefixedFieldNames {
    pub b: Option<shared0::shared1::shared2::second0::second1::second2::B>,
}

impl<'a> MessageRead<'a> for TestNonUniquePackagePrefixedFieldNames {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.b = Some(r.read_message::<shared0::shared1::shared2::second0::second1::second2::B>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestNonUniquePackagePrefixedFieldNames {
    fn get_size(&self) -> usize {
        0
        + self.b.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.b.as_ref().map_or(Ok(()), |m| w.write_with_tag(10, |w| w.write_message(m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct TestAbsoluteFieldNames {
    pub b: Option<shared0::shared1::shared2::second0::second1::second2::B>,
}

impl<'a> MessageRead<'a> for TestAbsoluteFieldNames {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.b = Some(r.read_message::<shared0::shared1::shared2::second0::second1::second2::B>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestAbsoluteFieldNames {
    fn get_size(&self) -> usize {
        0
        + self.b.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.b.as_ref().map_or(Ok(()), |m| w.write_with_tag(10, |w| w.write_message(m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ShouldUseInternalB {
    pub b: Option<shared0::shared1::shared2::first0::first1::first2::mod_ShouldUseInternalB::B>,
}

impl<'a> MessageRead<'a> for ShouldUseInternalB {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.b = Some(r.read_message::<shared0::shared1::shared2::first0::first1::first2::mod_ShouldUseInternalB::B>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ShouldUseInternalB {
    fn get_size(&self) -> usize {
        0
        + self.b.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.b.as_ref().map_or(Ok(()), |m| w.write_with_tag(10, |w| w.write_message(m)))?;
        Ok(())
    }
}

pub mod mod_ShouldUseInternalB {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct B { }

impl<'a> MessageRead<'a> for B {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for B { }

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ShouldUseSamePackageB {
    pub b: Option<shared0::shared1::shared2::first0::first1::first2::B>,
}

impl<'a> MessageRead<'a> for ShouldUseSamePackageB {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.b = Some(r.read_message::<shared0::shared1::shared2::first0::first1::first2::B>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ShouldUseSamePackageB {
    fn get_size(&self) -> usize {
        0
        + self.b.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.b.as_ref().map_or(Ok(()), |m| w.write_with_tag(10, |w| w.write_message(m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct B { }

impl<'a> MessageRead<'a> for B {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for B { }

