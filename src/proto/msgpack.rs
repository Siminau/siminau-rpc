// src/proto/msgpack.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports
use failure::Fail;
use rmps::decode;

// Local imports
use core::new::{CodeConvert, DataKind, FromBytesError, Message,
                MessageCategory};
use message::v1::request;

// ===========================================================================
// Message conversion trait implementations
// ===========================================================================

impl<E> From<decode::Error> for FromBytesError<E>
where
    E: Fail,
{
    fn from(e: decode::Error) -> FromBytesError<E>
    {
        match e {
            decode::Error::InvalidMarkerRead(err) => {
                FromBytesError::InvalidMarkerRead(err)
            }
            decode::Error::InvalidDataRead(err) => {
                FromBytesError::InvalidDataRead(err)
            }

            // TODO: figure out if possible to include the msgpack error in the
            // variant
            // example:
            // err @ decode::Error::TypeMismatch(_) => {
            //     FromBytesError::TypeMismatch(err)
            // }
            decode::Error::TypeMismatch(_) => FromBytesError::TypeMismatch,
            decode::Error::OutOfRange => FromBytesError::OutOfRange,
            decode::Error::LengthMismatch(v) => {
                FromBytesError::LengthMismatch(v)
            }
            decode::Error::Uncategorized(v) => FromBytesError::Uncategorized(v),
            decode::Error::Syntax(v) => FromBytesError::Syntax(v),
            decode::Error::Utf8Error(utferr) => {
                let invalid_byte = utferr.valid_up_to();
                FromBytesError::Utf8Error(invalid_byte)
            }
            decode::Error::DepthLimitExceeded => {
                FromBytesError::DepthLimitExceeded
            }
        }
    }
}

// ===========================================================================
// InitRequest
// ===========================================================================

pub struct InitRequest
{
    inner: request::InitRequest,
}

impl<T> Message<T> for InitRequest
where
    T: CodeConvert<T>,
{
    fn category(&self) -> MessageCategory
    {
        MessageCategory::from_number(self.inner.category).unwrap()
    }

    /// Return the message's kind
    fn kind(&self) -> T
    {
        unimplemented!();
    }

    /// Return the message's data payload
    fn data(&self) -> DataKind
    {
        unimplemented!();
    }
}

// ===========================================================================
//
// ===========================================================================
