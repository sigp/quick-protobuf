// Automatically generated rust module for 'test_default_values_dont_use_cow_pb.proto' file

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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct TestDefaultValuesDontUseCowOptional {
    pub string_field: Option<String>,
    pub bytes_field: Option<Vec<u8>>,
}

impl TestDefaultValuesDontUseCowOptional {
    pub const DEFAULT_string_field: &str = "abc\n22";
    pub const DEFAULT_bytes_field: &[u8] = b"cde\n33";
}


impl TestDefaultValuesDontUseCowOptional {
    pub fn get_string_field(&self) -> &str {
        &self.string_field.as_ref().map(|s| s.as_str()).unwrap_or(Self::DEFAULT_string_field)
    }
    pub fn get_bytes_field(&self) -> &[u8] {
        &self.bytes_field.as_ref().map(|s| s.as_slice()).unwrap_or(Self::DEFAULT_bytes_field)
    }
}


impl<'a> MessageRead<'a> for TestDefaultValuesDontUseCowOptional {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(114) => msg.string_field = Some(r.read_string(bytes)?.to_owned()),
                Ok(122) => msg.bytes_field = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestDefaultValuesDontUseCowOptional {
    fn get_size(&self) -> usize {
        0
        + self.string_field.as_ref().map_or(0, |m| 1 + sizeof_len(m.len()))
        + self.bytes_field.as_ref().map_or(0, |m| 1 + sizeof_len(m.len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.string_field.as_ref().map_or(Ok(()), |m| w.write_with_tag(114, |w| w.write_string(&m)))?;
        self.bytes_field.as_ref().map_or(Ok(()), |m| w.write_with_tag(122, |w| w.write_bytes(&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, PartialEq, Clone)]
pub struct TestDefaultValuesDontUseCowRequired {
    pub string_field: String,
    pub bytes_field: Vec<u8>,
}

impl Default for TestDefaultValuesDontUseCowRequired {
    fn default() -> Self {
        Self {
            string_field: "abc\n22".to_string(),
            bytes_field: b"cde\n33".to_vec(),
        }
    }
}

impl<'a> MessageRead<'a> for TestDefaultValuesDontUseCowRequired {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(114) => msg.string_field = r.read_string(bytes)?.to_owned(),
                Ok(122) => msg.bytes_field = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestDefaultValuesDontUseCowRequired {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len(self.string_field.len())
        + 1 + sizeof_len(self.bytes_field.len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(114, |w| w.write_string(&self.string_field))?;
        w.write_with_tag(122, |w| w.write_bytes(&self.bytes_field))?;
        Ok(())
    }
}

