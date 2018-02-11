// tests/from_message.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Externs
// ===========================================================================

// Stdlib externs

// Third-party externs

// Local externs
#[macro_use]
extern crate siminau_rpc_derive;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

// Local imports

// ===========================================================================
// Tests
// ===========================================================================

#[test]
#[allow(dead_code)]
fn derive_from_message()
{
    #[derive(FromMessage)]
    enum DataKind
    {
        U8(u8),
        U16(u16),
    }
}

// ===========================================================================
//
// ===========================================================================
