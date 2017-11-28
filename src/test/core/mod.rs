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

// Third-party imports

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


// ===========================================================================
//
// ===========================================================================
