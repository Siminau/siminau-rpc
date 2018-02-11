// src/core/new.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports
use chrono::{DateTime, Utc};
// use failure::Fail;

// Local imports

// ===========================================================================
// CodeConvert
// ===========================================================================

#[derive(Fail, Debug)]
#[fail(display = "Unknown code value: {}", code)]
pub struct CodeValueError
{
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
/// 3. The enum's values are continuous without any gaps ie 0, 1, 2 are valid
///    values but 0, 2, 4 is not
///
/// [`CodeConvert`]: trait.CodeConvert.html
pub trait CodeConvert<T>: Clone + PartialEq
{
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
// General message
// ===========================================================================

#[derive(Debug, PartialEq, Clone, IsDataKind)]
pub enum DataKind<'data>
{
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    String(&'data str),
    DateTime(&'data DateTime<Utc>),
    List(Vec<DataKind<'data>>),
    ByteList(&'data [u8]),
}

/// Enum defining different categories of messages
#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum MessageCategory
{
    /// A message initiating a request.
    Request,

    /// A message sent in response to a request.
    Response,

    /// A message notifying of some additional information.
    Notification,
}

pub trait Message<T>
where
    T: CodeConvert<T>,
{
    /// Return the message's category.
    fn category(&self) -> MessageCategory;

    /// Return the message's kind
    fn kind(&self) -> T;

    /// Return the message's data payload
    fn data(&self) -> DataKind;
}

pub trait IdMessage<T>: Message<T>
where
    T: CodeConvert<T>,
{
    /// Return the message's ID value.
    fn id(&self) -> u32;
}

// ===========================================================================
//
// ===========================================================================
