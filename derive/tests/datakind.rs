// tests/datakind.rs
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
//
// ===========================================================================

#[test]
#[allow(dead_code)]
fn derive_with_lifetime()
{
    #[derive(IsDataKind)]
    enum DataKind<'data>
    {
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        String(&'data str),
        List(Vec<DataKind<'data>>),
        ByteList(&'data [u8]),
    }
}

#[test]
#[allow(dead_code)]
fn derive_without_lifetime()
{
    #[derive(IsDataKind)]
    enum Hello
    {
        U(u8),
        Wot(u16),
        Mate(u32),
        ExclamationPoint(u64),
    }
}

// ===========================================================================
//
// ===========================================================================
