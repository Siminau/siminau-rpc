// src/core/new.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports
use std::io;

// Third-party imports
use bytes::BytesMut;
use chrono::{DateTime, Utc};
use failure::Fail;

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
// Message conversion traits
// ===========================================================================

pub trait AsBytes<V, T>: Message<T>
where
    V: AsRef<[u8]>,
    T: CodeConvert<T>,
{
    fn as_bytes(&self) -> V;
}

#[derive(Debug, Fail)]
pub enum FromBytesError<E>
where
    E: Fail,
{
    #[fail(display = "decode error: invalid marker")]
    InvalidMarkerRead(#[cause] io::Error),

    #[fail(display = "decode error: invalid data")]
    InvalidDataRead(#[cause] io::Error),

    #[fail(display = "decode error: type mismatch")]
    TypeMismatch,

    #[fail(display = "decode error: value out of range")]
    OutOfRange,

    #[fail(display = "decode error: length mismatch {}", _0)]
    LengthMismatch(u32),

    #[fail(display = "decode error: {}", _0)]
    Uncategorized(String),

    #[fail(display = "decode syntax error: {}", _0)]
    Syntax(String),

    #[fail(display = "decode utf-8 error: invalid byte starts at {}", _0)]
    Utf8Error(usize),

    #[fail(display = "decode error: depth limit exceeded")]
    DepthLimitExceeded,

    #[fail(display = "Invalid message")]
    InvalidMessage(#[cause] E),
}

pub trait FromBytes<M, T, E>: Message<T>
where
    M: Message<T>,
    T: CodeConvert<T>,
    E: Fail,
{
    fn from_bytes(&mut BytesMut) -> Result<Option<M>, FromBytesError<E>>;
}

// ===========================================================================
// Trait implementations
// ===========================================================================

// TODO: should this have unit tests?
impl<E> From<FromBytesError<E>> for io::Error
where
    E: Fail,
{
    fn from(e: FromBytesError<E>) -> io::Error
    {
        let (kind, errmsg) = match e {
            FromBytesError::InvalidMarkerRead(ioerr) => return ioerr,
            FromBytesError::InvalidDataRead(ioerr) => return ioerr,

            err @ FromBytesError::Uncategorized(_)
            | err @ FromBytesError::DepthLimitExceeded => {
                (io::ErrorKind::Other, err.to_string())
            }

            err => (io::ErrorKind::InvalidData, err.to_string()),
        };
        io::Error::new(kind, errmsg)
    }
}

// ===========================================================================
//
// ===========================================================================
