// src/test/core/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

mod check_int;
mod message;
mod messagetype;
mod notify;
mod request;
mod response;
mod rpcmessage;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::io;

// Third-party imports
use bytes::BytesMut;
use rmps::{decode, Deserializer};
use rmpv::Value;
use serde::Deserialize;

// Local imports
use core::{CodeConvert, CodeValueError};

// ===========================================================================
// Helpers
// ===========================================================================

#[derive(Debug, PartialEq, Clone, CodeConvert)]
enum TestEnum
{
    One,
    Two,
    Three,
}

// Helper
fn decode(buf: &mut BytesMut) -> Result<Value, decode::Error>
{
    let result;
    let curpos: usize;

    // Should never happen that no data has been given yet
    if buf.len() == 0 {
        unreachable!();
    }

    // Attempt to deserialize the current buffer
    {
        let cursor = io::Cursor::new(&buf[..]);
        let mut de = Deserializer::new(cursor);
        result = Value::deserialize(&mut de);
        curpos = de.position() as usize;
    }

    // Discard read bytes
    buf.split_to(curpos);

    // Return result
    result
}

// ===========================================================================
//
// ===========================================================================
