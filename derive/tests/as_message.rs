// tests/as_message.rs
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
fn derive_as_message()
{
    #[derive(AsMessage)]
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
