// src/message/v1/util.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

// Local imports

use core::CodeConvert;
use error::{RpcErrorKind, RpcResult};

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


impl FileKind {
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


#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct FileID {
    pub kind: FileKind,
    pub version: u32,
    pub path: u64,
}


impl FileID {
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


impl Default for FileID {
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
pub enum OpenKind {
    Read = 0,
    Write = 1,
    ReadWrite = 2,
    Execute = 3,
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct OpenMode {
    mode: u8,
}


impl OpenMode {
    pub const INVALID_BITS: u8 = 0b00111100;
    const OPENKIND_BITS: u8 = 0b00000011;

    pub fn from_bits(bits: u8) -> RpcResult<OpenMode>
    {
        if OpenMode::INVALID_BITS & bits != 0 {
            let errmsg = format!("Invalid bits set: {:b}", bits);
            bail!(RpcErrorKind::ValueError(errmsg));
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


impl Default for OpenMode {
    fn default() -> OpenMode
    {
        OpenMode { mode: 0 }
    }
}


pub struct OpenModeBuilder {
    open_mode: OpenMode,
}


impl Default for OpenModeBuilder {
    fn default() -> OpenModeBuilder
    {
        OpenModeBuilder {
            open_mode: OpenMode::default(),
        }
    }
}


impl OpenModeBuilder {
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
//
// ===========================================================================
