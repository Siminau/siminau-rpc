// src/message/v1/util.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports
use std::mem::size_of_val;

// Third-party imports

// Local imports

use core::{CodeConvert, CodeValueError};

// ===========================================================================
// Server File ID
// ===========================================================================


bitflags! {
    pub struct FileKind: u8 {
        const DIR =     0b10000000;
        const APPEND =  0b01000000;
        const EXCL =    0b00100000;
        const AUTH =    0b00010000;
        const TMP =     0b00001000;
        const FILE =    0b00000000;
    }
}


impl FileKind
{
    pub fn is_valid(&self) -> bool
    {
        let invalid = vec![
            FileKind::DIR | FileKind::AUTH,
            FileKind::DIR | FileKind::APPEND,
        ];

        // Return false if any invalid bits are found in filekind
        !invalid.iter().any(|i| self.contains(*i))
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct FileID
{
    pub kind: FileKind,
    pub version: u32,
    pub path: u64,
}


impl FileID
{
    pub fn new(kind: FileKind, version: u32, path: u64) -> FileID
    {
        FileID {
            kind: kind,
            version: version,
            path: path,
        }
    }

    pub fn is_valid(&self) -> bool
    {
        self.kind.is_valid()
    }
}


impl Default for FileID
{
    fn default() -> FileID
    {
        FileID::new(FileKind::FILE, 0, 0)
    }
}


// ===========================================================================
// File open mode
// ===========================================================================


bitflags! {
    pub struct OpenFlag: u8 {
        const OTRUNC =  0b10000000;
        const ORCLOSE = 0b01000000;
        const ONOFLAG = 0b00000000;
    }
}


#[derive(Debug, PartialEq, Copy, Clone, CodeConvert)]
pub enum OpenKind
{
    Read = 0,
    Write = 1,
    ReadWrite = 2,
    Execute = 3,
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct OpenMode
{
    mode: u8,
}


#[derive(Debug, Fail)]
#[fail(display = "Invalid bits set: {:b}", bits)]
pub struct OpenModeError
{
    bits: u8,
}


impl OpenMode
{
    pub const INVALID_BITS: u8 = 0b00111100;
    const OPENKIND_BITS: u8 = 0b00000011;

    pub fn from_bits(bits: u8) -> Result<OpenMode, OpenModeError>
    {
        if OpenMode::INVALID_BITS & bits != 0 {
            return Err(OpenModeError { bits: bits });
        }

        let ret = OpenMode { mode: bits };
        Ok(ret)
    }

    pub fn bits(&self) -> u8
    {
        self.mode
    }

    pub fn flags(&self) -> OpenFlag
    {
        let cur = OpenFlag::all();
        let return_bits = self.mode & cur.bits();
        OpenFlag::from_bits(return_bits).expect("should never panic")
    }

    pub fn kind(&self) -> OpenKind
    {
        let val = OpenMode::OPENKIND_BITS & self.mode;
        OpenKind::from_number(val).expect("should never panic")
    }

    pub fn new_kind(self, kind: OpenKind) -> OpenMode
    {
        let flags = self.mode & OpenFlag::all().bits();
        OpenMode {
            mode: flags | kind.to_number(),
        }
    }

    pub fn replace_flags(&mut self, flags: OpenFlag)
    {
        let kind = OpenMode::OPENKIND_BITS & self.mode;

        // Save flags
        self.mode = flags.bits() | kind;
    }

    pub fn insert_flags(&mut self, other: OpenFlag)
    {
        // Get flags
        let mut flags = self.flags();

        // Insert flags
        flags.insert(other);

        // Save flags
        self.replace_flags(flags);
    }

    pub fn remove_flags(&mut self, other: OpenFlag)
    {
        // Get flags
        let mut flags = self.flags();

        // Remove flags
        flags.remove(other);

        // Save flags
        self.replace_flags(flags);
    }

    pub fn toggle_flags(&mut self, other: OpenFlag)
    {
        // Get flags
        let mut flags = self.flags();

        // Toggle the flag
        flags.toggle(other);

        // Save flags
        self.replace_flags(flags);
    }
}


impl Default for OpenMode
{
    fn default() -> OpenMode
    {
        OpenMode { mode: 0 }
    }
}


pub struct OpenModeBuilder
{
    open_mode: OpenMode,
}


impl Default for OpenModeBuilder
{
    fn default() -> OpenModeBuilder
    {
        OpenModeBuilder {
            open_mode: OpenMode::default(),
        }
    }
}


impl OpenModeBuilder
{
    pub fn kind(mut self, val: OpenKind) -> OpenModeBuilder
    {
        self.open_mode.mode |= val as u8;
        self
    }

    pub fn flags(mut self, val: OpenFlag) -> OpenModeBuilder
    {
        self.open_mode.mode |= val.bits();
        self
    }

    pub fn create(self) -> OpenMode
    {
        self.open_mode
    }
}


pub fn openmode() -> OpenModeBuilder
{
    OpenModeBuilder::default()
}


// ===========================================================================
// Stat
// ===========================================================================


pub trait ReadStat
{
    fn calculate_size(&self) -> u16;
    fn size(&self) -> u16;
    fn fileid(&self) -> FileID;
    fn mode(&self) -> u32;
    fn atime(&self) -> u32;
    fn mtime(&self) -> u32;
    fn length(&self) -> u64;
    fn name(&self) -> &str;
    fn uid(&self) -> &str;
    fn gid(&self) -> &str;
    fn muid(&self) -> &str;
}


#[derive(Debug)]
pub struct StatData
{
    // Total byte count of all fields except for size
    pub size: u16,

    pub fileid: FileID,

    // File attributes and permissions
    // The high 8 bits are a copy of FileKind, and the other 24 bits are for
    // permissions
    pub mode: u32,

    // last access time
    // date field
    pub atime: u32,

    // last modified time
    // date field
    pub mtime: u32,

    // length of file in bytes
    pub length: u64,

    // File name
    pub name: String,

    // Owner name
    pub uid: String,

    // Group name
    pub gid: String,

    // name of the user who last modified the file
    pub muid: String,
}


impl StatData
{
    fn calculate_size(&self) -> u16
    {
        let strsize = vec![self.name, self.uid, self.gid, self.muid]
            .iter()
            .fold(0, |acc, &el| acc + el.len());
        let size = (strsize + size_of_val(&self.fileid)
            + size_of_val(&self.mode)
            + size_of_val(&self.atime)
            + size_of_val(&self.mtime)
            + size_of_val(&self.length)) as u16;
        size
    }
}


#[derive(Debug)]
pub struct FileStat
{
    data: StatData,
}


#[derive(Debug, Fail)]
#[fail(display = "Expected size {}, got size {}", expected_size, size)]
pub struct FromStatError
{
    expected_size: u16,
    size: u16,
}


impl FileStat
{
    fn validate_stat(self) -> Result<Self, FromStatError>
    {
        let size = self.calculate_size();
        if size != self.data.size {
            Err(FromStatError {
                expected_size: size,
                size: self.data.size,
            })
        } else {
            Ok(self)
        }
    }

    pub fn from_stat<T>(stat: T) -> Result<FileStat, FromStatError>
    where
        T: ReadStat,
    {
        let data = StatData {
            size: stat.size(),
            fileid: stat.fileid(),
            mode: stat.mode(),
            atime: stat.atime(),
            mtime: stat.mtime(),
            length: stat.length(),
            name: String::from(stat.name()),
            uid: String::from(stat.uid()),
            gid: String::from(stat.gid()),
            muid: String::from(stat.muid()),
        };
        let ret = FileStat { data };
        ret.validate_stat()
    }
}


impl ReadStat for FileStat
{
    fn calculate_size(&self) -> u16
    {
        self.data.calculate_size()
    }

    fn size(&self) -> u16
    {
        self.data.size
    }

    fn fileid(&self) -> FileID
    {
        self.data.fileid
    }

    fn mode(&self) -> u32
    {
        self.data.mode
    }

    fn atime(&self) -> u32
    {
        self.data.atime
    }

    fn mtime(&self) -> u32
    {
        self.data.mtime
    }

    fn length(&self) -> u64
    {
        self.data.length
    }

    fn name(&self) -> &str
    {
        self.data.name.as_str()
    }

    fn uid(&self) -> &str
    {
        self.data.uid.as_str()
    }

    fn gid(&self) -> &str
    {
        self.data.gid.as_str()
    }

    fn muid(&self) -> &str
    {
        self.data.muid.as_str()
    }
}


// #[derive(Debug)]
// pub struct Stat<'file>
// {
//     // Total byte count of all fields except for size
//     pub size: u16,

//     pub fileid: FileID,

//     // File attributes and permissions
//     // The high 8 bits are a copy of FileKind, and the other 24 bits are for
//     // permissions
//     pub mode: u32,

//     // last access time
//     // date field
//     pub atime: u32,

//     // last modified time
//     // date field
//     pub mtime: u32,

//     // length of file in bytes
//     pub length: u64,

//     // File name
//     pub name: &'file str,

//     // Owner name
//     pub uid: &'file str,

//     // Group name
//     pub gid: &'file str,

//     // name of the user who last modified the file
//     pub muid: &'file str,
// }




// impl FileStat
// {
//     pub fn as_stat(&self) -> Stat
//     {
//         Stat {
//             size: self.size,
//             fileid: self.fileid,
//             mode: self.mode,
//             atime: self.atime,
//             mtime: self.mtime,
//             length: self.length,
//             name: self.name.as_str(),
//             uid: self.uid.as_str(),
//             gid: self.gid.as_str(),
//             muid: self.muid.as_str(),
//         }
//     }
// }


// impl<'file> From<Stat<'file>> for FileStat
// {
//     fn from(s: Stat) -> FileStat
//     {
//         FileStat {
//             size: s.size,
//             fileid: s.fileid,
//             mode: s.mode,
//             atime: s.atime,
//             mtime: s.mtime,
//             length: s.length,
//             name: String::from(s.name),
//             uid: String::from(s.uid),
//             gid: String::from(s.gid),
//             muid: String::from(s.muid),
//         }
//     }
// }


// // Trait used to calculate the size of a Stat or FileStat object
// pub trait CalculateSize
// {
//     fn calculate_size(&self) -> u16;
// }


// impl<'file> CalculateSize for Stat<'file>
// {
//     fn calculate_size(&self) -> u16
//     {
//         let strsize = vec![&self.name, &self.uid, &self.gid, &self.muid]
//             .iter()
//             .fold(0, |acc, &el| acc + el.len());
//         let size = (strsize + size_of_val(&self.fileid)
//             + size_of_val(&self.mode)
//             + size_of_val(&self.atime)
//             + size_of_val(&self.mtime)
//             + size_of_val(&self.length)) as u16;
//         size
//     }
// }


// impl CalculateSize for FileStat
// {
//     fn calculate_size(&self) -> u16
//     {
//         self.as_stat().calculate_size()
//     }
// }



// ===========================================================================
//
// ===========================================================================
