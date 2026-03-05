// Automatically generated rust module for 'test_nested_p1.proto' file

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OutermostEnum {
    P1_OUTERMOST = 0,
}

impl Default for OutermostEnum {
    fn default() -> Self {
        OutermostEnum::P1_OUTERMOST
    }
}

impl From<i32> for OutermostEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => OutermostEnum::P1_OUTERMOST,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for OutermostEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "P1_OUTERMOST" => OutermostEnum::P1_OUTERMOST,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Level1 {
    pub level1: Option<i32>,
}

impl<'a> MessageRead<'a> for Level1 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.level1 = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Level1 {
    fn get_size(&self) -> usize {
        0
        + self.level1.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.level1.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_int32(*&m)))?;
        Ok(())
    }
}

pub mod mod_Level1 {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Level2 {
    pub level2: Option<i32>,
    pub OM: Option<test_nested_package_dir_alt::different::package::level::same::names::OutermostEnum>,
    pub L1: Option<test_nested_package_dir_alt::different::package::level::same::names::mod_Level1::Level1NativeEnum>,
    pub L2: Option<test_nested_package_dir::this::is::a::nested::package::mod_Level1::mod_Level2::Level2NativeEnum>,
    pub L3: Option<test_nested_package_dir_alt::different::package::level::same::names::mod_Level1::mod_Level2::mod_Level3::Level3NativeEnum>,
    pub L4: Option<test_nested_package_dir_alt::different::package::level::same::names::mod_Level1::mod_Level2::mod_Level3::mod_Level4::Level4NativeEnum>,
    pub L5: Option<test_nested_package_dir_alt::different::package::level::same::names::mod_Level1::mod_Level2::mod_Level3::mod_Level4::mod_Level5::Level5NativeEnum>,
}

impl<'a> MessageRead<'a> for Level2 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.level2 = Some(r.read_int32(bytes)?),
                Ok(16) => msg.OM = Some(r.read_enum(bytes)?),
                Ok(24) => msg.L1 = Some(r.read_enum(bytes)?),
                Ok(32) => msg.L2 = Some(r.read_enum(bytes)?),
                Ok(40) => msg.L3 = Some(r.read_enum(bytes)?),
                Ok(48) => msg.L4 = Some(r.read_enum(bytes)?),
                Ok(56) => msg.L5 = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Level2 {
    fn get_size(&self) -> usize {
        0
        + self.level2.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.OM.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.L1.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.L2.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.L3.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.L4.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.L5.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.level2.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_int32(*&m)))?;
        self.OM.as_ref().map_or(Ok(()), |&m| w.write_with_tag(16, |w| w.write_enum(*&m as i32)))?;
        self.L1.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_enum(*&m as i32)))?;
        self.L2.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_enum(*&m as i32)))?;
        self.L3.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_enum(*&m as i32)))?;
        self.L4.as_ref().map_or(Ok(()), |&m| w.write_with_tag(48, |w| w.write_enum(*&m as i32)))?;
        self.L5.as_ref().map_or(Ok(()), |&m| w.write_with_tag(56, |w| w.write_enum(*&m as i32)))?;
        Ok(())
    }
}

pub mod mod_Level2 {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Level3 {
    pub level3: Option<i32>,
}

impl<'a> MessageRead<'a> for Level3 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.level3 = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Level3 {
    fn get_size(&self) -> usize {
        0
        + self.level3.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.level3.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_int32(*&m)))?;
        Ok(())
    }
}

pub mod mod_Level3 {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Level4 {
    pub level4: Option<i32>,
}

impl<'a> MessageRead<'a> for Level4 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.level4 = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Level4 {
    fn get_size(&self) -> usize {
        0
        + self.level4.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.level4.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_int32(*&m)))?;
        Ok(())
    }
}

pub mod mod_Level4 {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Level5 {
    pub level5: Option<i32>,
}

impl<'a> MessageRead<'a> for Level5 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.level5 = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Level5 {
    fn get_size(&self) -> usize {
        0
        + self.level5.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.level5.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_int32(*&m)))?;
        Ok(())
    }
}

pub mod mod_Level5 {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Level5NativeEnum {
    LEVEL5 = 0,
}

impl Default for Level5NativeEnum {
    fn default() -> Self {
        Level5NativeEnum::LEVEL5
    }
}

impl From<i32> for Level5NativeEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => Level5NativeEnum::LEVEL5,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Level5NativeEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "LEVEL5" => Level5NativeEnum::LEVEL5,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Level4NativeEnum {
    LEVEL4 = 0,
}

impl Default for Level4NativeEnum {
    fn default() -> Self {
        Level4NativeEnum::LEVEL4
    }
}

impl From<i32> for Level4NativeEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => Level4NativeEnum::LEVEL4,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Level4NativeEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "LEVEL4" => Level4NativeEnum::LEVEL4,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Level3NativeEnum {
    LEVEL3 = 0,
}

impl Default for Level3NativeEnum {
    fn default() -> Self {
        Level3NativeEnum::LEVEL3
    }
}

impl From<i32> for Level3NativeEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => Level3NativeEnum::LEVEL3,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Level3NativeEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "LEVEL3" => Level3NativeEnum::LEVEL3,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Level2NativeEnum {
    LEVEL2 = 0,
}

impl Default for Level2NativeEnum {
    fn default() -> Self {
        Level2NativeEnum::LEVEL2
    }
}

impl From<i32> for Level2NativeEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => Level2NativeEnum::LEVEL2,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Level2NativeEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "LEVEL2" => Level2NativeEnum::LEVEL2,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Level1NativeEnum {
    LEVEL1 = 0,
}

impl Default for Level1NativeEnum {
    fn default() -> Self {
        Level1NativeEnum::LEVEL1
    }
}

impl From<i32> for Level1NativeEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => Level1NativeEnum::LEVEL1,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Level1NativeEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "LEVEL1" => Level1NativeEnum::LEVEL1,
            _ => Self::default(),
        }
    }
}

}

