// tests/from_version_message.rs
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
fn derive_from_version_message()
{
    mod root
    {
        pub mod message
        {
            #[derive(FromMessage)]
            pub enum Hello
            {
                World(helloworld::Hello),
            }

            pub mod helloworld
            {
                use super::super::message;

                #[derive(FromMessage, FromVersionMessage)]
                pub enum Hello
                {
                    U8(u8),
                    U16(u16),
                }
            }
        }
    }
}

// ===========================================================================
//
// ===========================================================================
