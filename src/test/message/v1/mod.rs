// src/test/message/v1/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================


mod requestbuilder;
mod responsebuilder;
mod util;


// ===========================================================================
// Helpers
// ===========================================================================


fn invalid_string(s: &str) -> bool
{
    if s.is_empty() {
        true
    } else {
        s.chars().any(|c| c.is_whitespace() || c.is_control())
    }
}


// ===========================================================================
//
// ===========================================================================
