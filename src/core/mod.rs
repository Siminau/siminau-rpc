// src/core/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

//! This module defines the base type of all RPC messages
//!
//! The [`Message`] type is the core underlying type that wraps around the
//! [`rmpv::Value`] type. It ensures that the given [`rmpv::Value`] object
//! conforms with an expected minimum of the RPC spec.
//!
//! The intended use is for a buffer of bytes to be deserialized into a
//! [`rmpv::Value`] value (eg using [`rmp-serde`]). This value would then be
//! used to create a [`Message`] value.
//!
//! # Types and Traits
//!
//! This module provides 2 types and 3 traits as the building blocks of all RPC
//! messages. The types provided are:
//!
//! * MessageType
//! * Message
//!
//! And the traits provided are:
//!
//! * AsBytes
//! * CodeConvert
//! * FromBytes
//! * FromMessage
//! * RpcMessage
//! * RpcMessageType
//!
//! While each type and trait is discussed in more detail in their definition,
//! the following summarizes the purpose of each type and trait.
//!
//! ## MessageType
//!
//! This is an enum that defines all possible message types. Due to sticking
//! somewhat closely to the official [`msgpack-rpc`] spec, there are only 3
//! types of messages that can be defined:
//!
//! * Request
//! * Response
//! * Notification
//!
//! ## Message
//!
//! The core base type of all RPC messages.
//!
//! ## AsBytes
//!
//! This trait provides an interface to convert a message into msgpack bytes.
//!
//! ## FromBytes
//!
//! This trait provides an interface to convert a msgpack bytes into a message.
//!
//! ## FromMessage
//!
//! This trait provides an interface to convert messages into a specific rpc
//! message.
//!
//! ## CodeConvert
//!
//! This trait provides a common interface for converting between a number and
//! a type.
//!
//! ## RpcMessage
//!
//! This trait provides a interface common to all messages.
//!
//! ## RpcMessageType
//!
//! This trait provides an interface to access the type's inner Message object.
//!
//! # Validation
//!
//! When the [`Message`] type is being instantiated, it checks for the following:
//!
//! * The [`rmpv::Value`] type being wrapped is an array
//! * The array is not empty
//! * The array's first item is an integer that can be mapped to the
//!   [`MessageType`] enum
//!
//! [`Message`]: struct.Message.html
//! [`rmpv::Value`]: https://docs.rs/rmpv/0.4.0/rmpv/enum.Value.html
//! [`MessageType`]: enum.MessageType.html
//! [`rmp-serde`]: https://docs.rs/rmp-serde
//! [`msgpack-rpc`]: https://github.com/msgpack-rpc/msgpack-rpc/blob/master/spec.md
//!
//! # Example
//!
//! ```rust
//! extern crate rmp_serde as rmps;
//! extern crate rmpv;
//! extern crate siminau_rpc;
//!
//! use rmpv::Value;
//! use siminau_rpc::core::{CodeConvert, FromMessage, Message, MessageType,
//!                         RpcMessage};
//!
//! # fn main() {
//! // Build expected value
//! let msgtype = Value::from(MessageType::Request.to_number());
//! let msgid = Value::from(42);
//! let msgcode = Value::from(0);
//! let msgargs = Value::Array(vec![Value::from(42)]);
//! let expected = Value::Array(vec![msgtype, msgid, msgcode, msgargs]);
//!
//! // Given a buffer of bytes
//! let buf: Vec<u8> = vec![148, 0, 42, 0, 145, 42];
//!
//! // Deserializing it will give the expected value
//! let val: Value = rmps::from_slice(&buf[..]).unwrap();
//! assert_eq!(val, expected);
//!
//! // Turn the value into a Message type
//! let msg = Message::from_msg(val).unwrap();
//!
//! // Grab a reference to the internal value and check against expected
//! assert_eq!(msg.as_value(), &expected);
//!
//! // Check internal array items against expected
//! let expected_array = expected.as_array().unwrap();
//! let val_array = msg.as_vec();
//! for i in 0..expected_array.len() {
//!     assert_eq!(val_array[i], expected_array[i]);
//! }
//! # }
//! ```
//!


// ===========================================================================
// Modules
// ===========================================================================


pub mod request;
pub mod response;
pub mod notify;


// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

use std::clone::Clone;
use std::io;

// Third-party imports

use bytes::{Bytes, BytesMut};
use failure::Fail;
use rmps::{decode, Deserializer, Serializer};
use rmpv::Value;
use serde::{Deserialize, Serialize};

// Local imports


// ===========================================================================
// Helpers
// ===========================================================================


// Return the name of a Value variant
pub fn value_type(arg: &Value) -> String
{
    let ret = match *arg {
        Value::Nil => "nil",
        Value::Boolean(_) => "bool",
        Value::Integer(_) => "int",
        Value::F32(_) => "float32",
        Value::F64(_) => "float64",
        Value::String(_) => "str",
        Value::Binary(_) => "bytearray",
        Value::Array(_) => "array",
        Value::Map(_) => "map",
        Value::Ext(_, _) => "ext",
    };
    String::from(ret)
}


#[derive(Debug, Fail)]
pub enum CheckIntError
{
    #[fail(display = "Expected {} but got None", expected)]
    MissingValue
    {
        expected: String
    },

    #[fail(display = "Expected value <= {} but got value {}", max_value, value)]
    ValueTooBig
    {
        max_value: u64, value: String
    },
}


/// Check if an unsigned integer value can be cast as a given integer type.
///
/// # Errors
///
/// If the value is None, then the CheckIntError::MissingValue error is
/// returned.
///
/// If the value cannot fit into the type specified by `expected`, then the
/// CheckIntError::ValueTooBig error is returned.
pub fn check_int(
    val: Option<u64>, max_value: u64, expected: String
) -> Result<u64, CheckIntError>
{
    match val {
        None => Err(CheckIntError::MissingValue { expected: expected }),
        Some(v) if v > max_value => {
            let e = CheckIntError::ValueTooBig {
                max_value: max_value,
                value: v.to_string(),
            };
            Err(e)
        }
        Some(v) => Ok(v),
    }
}


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
// MessageType
// ===========================================================================


/// Enum defining different types of messages
#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum MessageType
{
    /// A message initiating a request.
    Request,

    /// A message sent in response to a request.
    Response,

    /// A message notifying of some additional information.
    Notification,
}


// ===========================================================================
// Message traits
// ===========================================================================


pub trait FromMessage<M>
{
    type Err: Fail + From<ToMessageError>;

    /// Create a new message from a Message object
    fn from_msg(M) -> Result<Self, Self::Err>
    where
        Self: Sized;
}


// Default implementation of FromMessage<Value> for specific message types
impl<M> FromMessage<Value> for M
    where M: RpcMessage + RpcMessageType + FromMessage<Message>
{
    type Err = <M as FromMessage<Message>>::Err;

    /// Create a new message from a [`rmpv::Value`] object
    fn from_msg(v: Value) -> Result<Self, Self::Err>
    where
        Self: Sized,
    {
        let msg = Message::from_msg(v).map_err(|e| Self::Err::from(e))?;

        Self::from_msg(msg)
    }
}


/// Define methods common to all RPC messages
pub trait RpcMessage
{
    type Err: Fail + From<ToMessageError>;

    /// View the message as a vector of [`rmpv::Value`] objects.
    fn as_vec(&self) -> &Vec<Value>;

    /// Return a reference to the internally owned [`rmpv::Value`] object.
    fn as_value(&self) -> &Value;

    /// Return the message's type.
    fn message_type(&self) -> MessageType
    {
        let msgtype: u8 = match self.as_vec()[0].as_u64() {
            Some(v) => v as u8,
            None => unreachable!(),
        };
        MessageType::from_number(msgtype)
            .expect(&format!("bad msgtype? {}", msgtype))
    }
}


/// Define methods common to all RPC message types.
pub trait RpcMessageType
{
    /// Return a reference to the inner message.
    fn as_message(&self) -> &Message;
}


pub trait AsBytes<V>
    where V: AsRef<[u8]>,
{
    fn as_bytes(&self) -> V;
}


impl<T> AsBytes<Bytes> for T
    where T: RpcMessage,
{
    // TODO: should there be an unwrap here?
    fn as_bytes(&self) -> Bytes {
        let mut tmpbuf = Vec::new();
        let msg = self.as_value();
        msg.serialize(&mut Serializer::new(&mut tmpbuf)).unwrap();
        let mut buf = Bytes::with_capacity(tmpbuf.len());
        buf.extend_from_slice(&tmpbuf[..]);
        buf
    }
}


#[derive(Debug, Fail)]
pub enum FromBytesError<E>
    where E: Fail
{
    #[fail(display = "MsgPack error: invalid marker")]
    InvalidMarkerRead(#[cause] io::Error),

    #[fail(display = "MsgPack error: invalid data")]
    InvalidDataRead(#[cause] io::Error),

    #[fail(display = "MsgPack error: type mismatch")]
    TypeMismatch(#[cause] decode::Error),

    #[fail(display = "MsgPack error: value out of range")]
    OutOfRange,

    #[fail(display = "MsgPack error: length mismatch {}", _0)]
    LengthMismatch(u32),

    #[fail(display = "MsgPack error: {}", _0)]
    Uncategorized(String),

    #[fail(display = "MsgPack syntax error: {}", _0)]
    Syntax(String),

    #[fail(display = "MsgPack utf-8 error: invalid byte starts at {}", _0)]
    Utf8Error(usize),

    #[fail(display = "MsgPack error: depth limit exceeded")]
    DepthLimitExceeded,

    #[fail(display = "Invalid message")]
    InvalidMessage(#[cause] E),
}


impl<E> From<decode::Error> for FromBytesError<E>
    where E: Fail
{
    fn from(e: decode::Error) -> FromBytesError<E> {
        match e {
            decode::Error::InvalidMarkerRead(err) => FromBytesError::InvalidMarkerRead(err),
            decode::Error::InvalidDataRead(err) => FromBytesError::InvalidDataRead(err),
            err @ decode::Error::TypeMismatch(_) => FromBytesError::TypeMismatch(err),
            decode::Error::OutOfRange => FromBytesError::OutOfRange,
            decode::Error::LengthMismatch(v) => FromBytesError::LengthMismatch(v),
            decode::Error::Uncategorized(v) => FromBytesError::Uncategorized(v),
            decode::Error::Syntax(v) => FromBytesError::Syntax(v),
            decode::Error::Utf8Error(utferr) => {
                let invalid_byte = utferr.valid_up_to();
                FromBytesError::Utf8Error(invalid_byte)
            },
            decode::Error::DepthLimitExceeded => FromBytesError::DepthLimitExceeded,
        }
    }
}


pub trait FromBytes<T, E>
    where
        T: RpcMessage,
        E: Fail + From<ToMessageError>,
{
    fn from_bytes(&mut BytesMut) -> Result<Option<T>, FromBytesError<E>>;
}


impl<T, E> FromBytes<T, E> for T
    where T: RpcMessage<Err = E> + FromMessage<Value, Err = E>,
          E: Fail + From<ToMessageError>,
{
    fn from_bytes(buf: &mut BytesMut) -> Result<Option<T>, FromBytesError<E>> {
        let result;
        let curpos: usize;

        // If no data has been given yet, ask for data to be sent
        if buf.is_empty() {
            return Ok(None);
        }

        // Attempt to deserialize the current buffer
        {
            let cursor = io::Cursor::new(&buf[..]);
            let mut de = Deserializer::new(cursor);
            result = Value::deserialize(&mut de);
            curpos = de.position() as usize;
        }

        // Discard read bytes
        buf.split_to(curpos);

        match result {
            Ok(v) => {
                let msg = T::from_msg(v)
                    .map_err(|e| FromBytesError::InvalidMessage(e))?;
                Ok(Some(msg))
            }
            Err(e) => {
                // If no more data due to eof, ask for more to be sent
                if let decode::Error::InvalidDataRead(ref err) = e {
                    if let io::ErrorKind::UnexpectedEof = err.kind() {
                        return Ok(None);
                    }
                }

                Err(e.into())
            }
        }
    }
}


// ===========================================================================
// Message
// ===========================================================================


// Message errors
#[derive(Debug, Fail)]
pub enum ToMessageError
{
    #[fail(display = "expected array length of either 3 or 4, got {}", _0)]
    ArrayLength(usize),

    #[fail(display = "Invalid message type")]
    InvalidType(#[cause] CheckIntError),

    #[fail(display = "expected array but got {}", _0)] NotArray(String),
}


/// The [`Message`] type is the core underlying type of all RPC messages
///
/// [`Message`] wraps around the [`rmpv::Value`] type. It ensures that the
/// given [`rmpv::Value`] object conforms with the expected RPC spec.
///
/// [`Message`]: message/struct.Message.html
/// [`rmpv::Value`]: https://docs.rs/rmpv/0.4.0/rmpv/enum.Value.html
#[derive(Debug, PartialEq)]
pub struct Message
{
    msg: Value,
}


impl FromMessage<Value> for Message {
    type Err = ToMessageError;

    // TODO: improve call to check_int since it's possible the array's first
    // element is not an integer
    /// Converts an [`rmpv::Value`].
    ///
    /// # Errors
    ///
    /// An error is returned if any of the following are true:
    ///
    /// 1. The value is not an array
    /// 2. The length of the array is less than 3 or greater than 4
    /// 3. The array's first item is not a u8
    /// 4. The array's first item is a value greater than the maximum value
    ///    stored in the MessageType enum
    fn from_msg(val: Value) -> Result<Self, Self::Err>
    {
        if let Some(array) = val.as_array() {
            let arraylen = array.len();
            if arraylen < 3 || arraylen > 4 {
                return Err(ToMessageError::ArrayLength(arraylen));
            }

            // Check msg type
            check_int(
                array[0].as_u64(),
                MessageType::max_number() as u64,
                array[0].as_u64().unwrap().to_string(),
            ).map_err(|e| ToMessageError::InvalidType(e))?;
        } else {
            return Err(ToMessageError::NotArray(value_type(&val)));
        }

        // Return Message object
        Ok(Self { msg: val })
    }

}


impl FromMessage<Message> for Message {
    type Err = ToMessageError;

    fn from_msg(msg: Message) -> Result<Self, Self::Err>
    {
        let ret = Message { msg: msg.msg };
        Ok(ret)
    }
}


impl RpcMessage for Message
{
    type Err = ToMessageError;

    fn as_vec(&self) -> &Vec<Value>
    {
        self.msg.as_array().unwrap()
    }

    fn as_value(&self) -> &Value
    {
        &self.msg
    }
}


// Clone impl
impl Clone for Message
{
    fn clone(&self) -> Self
    {
        Self {
            msg: self.msg.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self)
    {
        self.msg = source.as_value().clone();
    }
}


impl From<Message> for Value
{
    fn from(msg: Message) -> Value
    {
        msg.msg
    }
}


// ===========================================================================
// Tests
// ===========================================================================


// These unit tests require access to the private msg field of the Message
// struct. The bulk of tests can be found in the test::core module.
#[cfg(test)]
mod tests
{
    // std lib imports

    // use std::error::Error;

    // Third-party imports

    use rmpv::Value;

    // Local imports

    use super::{Message, RpcMessage};

    // --------------------
    // Decode tests
    // --------------------

    // #[test]
    // fn test_temp() {
    //     let buf = [0x93, 0xa4, 0x4a, 0x6f, 0x68, 0x6e, 0xa5, 0x53, 0x6d, 0x69, 0x74, 0x68, 0x2a];
    //     let expected = Value::Array(vec![Value::from("John"),
    //                                      Value::from("Smith"),
    //                                      Value::from(42)]);
    //     assert_eq!(expected, rmps::from_slice(&buf[..]).unwrap());
    // }

    // --------------------
    // Message
    // --------------------

    // Message::message
    #[test]
    fn message_message_value()
    {
        let v = Value::from(vec![Value::from(42)]);
        let expected = v.clone();
        let m = Message { msg: v };

        let msg_val = m.as_vec();
        assert_eq!(msg_val, expected.as_array().unwrap());
    }

    // Should only panic if manually creating a Message object using a non
    // Vec<Value> instead of using the from function
    #[test]
    #[should_panic]
    fn message_as_vec_panic()
    {
        let v = Value::from(Value::from(42));
        let m = Message { msg: v };
        m.as_vec();
    }

    // Message::raw_message
    #[test]
    fn message_as_value()
    {
        let v = Value::from(42);
        let expected = v.clone();
        let msg = Message { msg: v };
        assert_eq!(msg.as_value(), &expected);
    }
}


// ===========================================================================
//
// ===========================================================================
