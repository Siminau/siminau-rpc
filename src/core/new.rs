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
use core::{CodeConvert, CodeValueError};

// ===========================================================================
// General message
// ===========================================================================

// trait IsDataKind
// {
//     fn is_u8(&self) -> bool;
//     fn is_u16(&self) -> bool;
//     fn is_u32(&self) -> bool;
//     fn is_u64(&self) -> bool;
//     fn is_string(&self) -> bool;
//     fn is_datetime(&self) -> bool;
//     fn is_list(&self) -> bool;
//     fn is_bytelist(&self) -> bool;
// }

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

pub trait IdentMessage<T>: Message<T>
where
    T: CodeConvert<T>,
{
    /// Return the message's ID value.
    fn id(&self) -> u32;
}

// ===========================================================================
//
// ===========================================================================
