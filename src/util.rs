// src/util.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

// Local imports


// ===========================================================================
// Miscellaneous utility functions
// ===========================================================================


// Return true if given string is not empty and is printable
pub fn is_printable(s: &str, ws_printable: bool) -> bool
{
    if s.is_empty() {
        return false;
    }

    let default = false;
    let printable = s.chars().all(|c| {
        let ret = if ws_printable {
            true
        } else {
            !c.is_whitespace()
        };

        ret && !c.is_control()
    });
    default || printable
}


// ===========================================================================
//
// ===========================================================================
