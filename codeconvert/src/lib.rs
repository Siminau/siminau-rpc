// src/lib.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

//! Trait for converting integers to and from a C-style enum
//!
//! # Types and Traits
//!
//! This module provides 1 trait and 1 error type. The trait is:
//!
//! * CodeConvert
//!
//! And the error type provided is:
//!
//! * CodeValueError
//!
//! While each trait and error is discussed in more detail in their definition,
//! the following summarizes the purpose of each trait and error type.
//!
//! ## CodeConvert
//!
//! This trait provides a common interface for converting between an integer and
//! a type.
//!
//! ## CodeValueError
//!
//! The error returned by some of CodeConvert's methods. As of this writing,
//! this error is only used when attempting to convert an integer into an
//! object that implements CodeConvert.
//!
//! # Example
//!
//! ```rust
//! extern crate codeconvert
//!
//! use codeconvert::{CodeConvert, CodeValueError};
//!
//! # fn main() {
//!
//! // Create an enum
//! enum Code {
//!     One,
//!     Two,
//! }
//!
//! // Implement CodeConvert
//! impl CodeConvert<Code> for Code {
//!     type int_type = u8;
//!
//!     fn from_number(num: Self::int_type) -> Result<Code, CodeValueError> {
//!         Self::from_u64(num as u64)
//!     }
//!
//!     fn from_u64(num: u64) -> Result<Code, CodeValueError> {
//!         let ret = match num {
//!             0 => Code::One,
//!             1 => Code::Two,
//!             _ => {
//!                 return Err(CodeValueError { code: num });
//!             }
//!         };
//!         Ok(ret)
//!     }
//!
//!     fn to_number(&self) -> Self::int_type {
//!         self.clone() as Self::int_type
//!     }
//!
//!     fn to_u64(&self) -> u64 {
//!         self.clone() as u64
//!     }
//!
//!     fn max_number() -> u64 {
//!         Code::Two as u64
//!     }
//!
//!     fn cast_number(n: u64) -> Option<Self::int_type> {
//!         let maxval = Self::int_type::max_value() as u64;
//!         if n <= maxval {
//!             Some(n as Self::int_type)
//!         } else {
//!             None
//!         }
//!     }
//! }
//!
//! # }
//! ```
#![recursion_limit = "1024"]

// ===========================================================================
// Externs
// ===========================================================================


// Stdlib externs

// Third-party externs
#[macro_use] extern crate failure_derive;
extern crate failure;

// Local externs


// ===========================================================================
// CodeConvert
// ===========================================================================


#[derive(Debug, Fail)]
#[fail(display = "Unknown code value: {}", code)]
pub struct CodeValueError {
    pub code: u64,
}


/// Allows converting between a number and a type.
///
/// The type implementing [`CodeConvert`] will usually be an enum that defines
/// different codes.
///
/// # Assumptions
///
/// This trait assumes the following of the implementing enum:
///
/// 1. The enum is a C-style enum
/// 2. The enum's values are unsigned integers
///
/// [`CodeConvert`]: trait.CodeConvert.html
pub trait CodeConvert<T>: Clone + PartialEq {
    type int_type;

    /// Convert a number to type T.
    fn from_number(num: Self::int_type) -> Result<T, CodeValueError>;

    /// Convert a u64 to type T.
    fn from_u64(num: u64) -> Result<T, CodeValueError>;

    /// Convert type T to a number.
    fn to_number(&self) -> Self::int_type;

    /// Convert type T to a u64.
    fn to_u64(&self) -> u64;

    /// Return the maximum number value
    fn max_number() -> u64;

    /// Cast a u64 number into acceptable int type
    fn cast_number(n: u64) -> Option<Self::int_type>;
}


// ===========================================================================
//
// ===========================================================================
