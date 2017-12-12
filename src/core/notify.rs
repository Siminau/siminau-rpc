// src/core/notify.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

//! This module defines the Notification RPC message type.
//!
//! A Notification RPC message is used by either server or client to send a
//! notification.  server. Based on the generic [`Message`] type, the
//! Notification message type is essentially an array containing 3 items. These
//! 3 items are:
//!
//! 1. Message Type - This will always be the Notification message type. While
//!    represented as the enum variant `MessageType::Notification`, the value
//!    stored in the array is actually a u8 integer.
//!
//! 2. Message code - This is an unsigned integer representing the specific
//!    notification being made.
//!
//! 3. Message arguments - this is an array of values used to provide
//!    information needed to be included with notice specified by the message
//!    code.
//!
//! # Example
//!
//! To create a new Notification object, you can create one from an existing
//! [`Message`] instance. This is used, for example, when the message is
//! deserialized by the server into a general [`Message`] object, identified as
//! a Notification message, and it is required to perform Notification specific
//! operations on the message.
//!
//! Alternatively, a `NotificationMessage` object can be created manually via
//! the `NotificationMessage::new()` method
//!
//! ```text
//!
//! extern crate rmpv;
//!
//! // `macro_use` needed to use CodeConvert custom derive
//! #[macro_use] extern crate siminau_rpc;
//!
//! use rmpv::Value;
//!
//! // Message and notify types
//! // Note: CodeValueError is needed for CodeConvert custom derive
//! use siminau_rpc::core::{CodeConvert, CodeValueError, Message, MessageType,
//!                         RpcMessage};
//! use siminau_rpc::core::notify::{NotificationMessage, RpcNotice};
//!
//! // Define Error codes
//! #[derive(Debug, Clone, PartialEq, CodeConvert)]
//! enum NotifyCode {
//!     Yep,
//!     UhHuh,
//! }
//!
//! # fn main() {
//! // Create an alias for NotificationMessage
//! type Notice = NotificationMessage<NotifyCode>;
//!
//! // Build Message
//! let msgtype = Value::from(MessageType::Notification.to_number());
//! let msgcode = Value::from(Notify::Yep.to_number());
//! let msgargs = Value::Array(vec![Value::from(9001)]);
//! let msgval = Value::Array(vec![msgtype, msgcode, msgargs]);
//! let msg = Message::from_value(msgval).unwrap();
//!
//! // Turn the message into a Notice type
//! let nmsg = Notice::from(msg).unwrap();
//! assert_eq!(nmsg.message_type(), MessageType::Notification);
//! assert_eq!(nmsg.message_code(), NotifyCode::Yep);
//! assert_eq!(nmsg.message_args(), &vec![Value::from(9001)]);
//!
//! // Create a brand new response from scratch
//! let new_nmsg = Notice::new(NotifyCode::UhHuh, vec![Value::from(42)]);
//! assert_eq!(new_nmsg.message_type(), MessageType::Response);
//! assert_eq!(new_nmsg.message_code(), NotifyCode::UhHuh);
//! assert_eq!(new_nmsg.message_args(), &vec![Value::from(42)]);
//! # }
//!
//! ```
//!
// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

use std::marker::PhantomData;

// Third-party imports

use rmpv::Value;

// Local imports

use core::{check_int, value_type, CheckIntError, CodeConvert, FromMessage,
           Message, MessageType, RpcMessage, RpcMessageType, ToMessageError};


// ===========================================================================
// NotificationMessage errors
// ===========================================================================


#[derive(Debug, Fail)]
#[fail(display = "Expected notification message type value {}, got {}",
       expected_type, msgtype)]
pub struct NoticeTypeError
{
    expected_type: u8,
    msgtype: u8,
}


#[derive(Debug, Fail)]
pub enum NoticeCodeError
{
    #[fail(display = "Invalid notification code value")]
    InvalidValue(#[cause] CheckIntError),

    #[fail(display = "Cannot cast {} into notification code value", _0)]
    ToNumber(u64),

    #[fail(display = "Cannot convert method value {} into notification code",
           _0)]
    ToCode(u64),
}


#[derive(Debug, Fail)]
#[fail(display = "Expected array for notification arguments but got {}",
       value_type)]
pub struct NoticeArgsError
{
    value_type: String,
}


#[derive(Debug, Fail)]
pub enum ToNoticeError
{
    #[fail(display = "Expected array length of 3, got {}", _0)]
    ArrayLength(usize),

    #[fail(display = "Invalid notification message type")]
    InvalidType(#[cause] NoticeTypeError),

    #[fail(display = "Invalid notification message code")]
    InvalidCode(#[cause] NoticeCodeError),

    #[fail(display = "Invalid notification message arguments")]
    InvalidArgs(#[cause] NoticeArgsError),

    #[fail(display = "Unable to convert message")]
    MessageError(#[cause] ToMessageError),
}


impl From<ToMessageError> for ToNoticeError
{
    fn from(e: ToMessageError) -> ToNoticeError
    {
        ToNoticeError::MessageError(e)
    }
}


// ===========================================================================
// NotificationMessage
// ===========================================================================


/// Trait providing Notification message specific getter methods.
///
/// # Example
///
/// ```rust
/// extern crate rmpv;
/// extern crate siminau_rpc;
///
/// use rmpv::Value;
/// use siminau_rpc::core::{MessageType, RpcMessage};
/// use siminau_rpc::core::notify::{NotificationMessage, RpcNotice};
///
/// # fn main() {
/// // Create Notice alias
/// type Notice = NotificationMessage<MessageType>;
///
/// // Re-use MessageType as message code
/// let req = Notice::new(MessageType::Request,
///                       vec![Value::from(42)]);
///
/// // Check all getter methods
/// assert_eq!(req.message_type(), MessageType::Notification);
/// assert_eq!(req.message_code(), MessageType::Request);
/// assert_eq!(req.message_args(), &vec![Value::from(42)]);
/// # }
/// ```
pub trait RpcNotice<C>: RpcMessage
where
    C: CodeConvert<C>,
{
    fn message_code(&self) -> C
    {
        let msgcode = &self.as_vec()[1];
        let msgcode = msgcode.as_u64().unwrap();
        let msgcode = C::cast_number(msgcode).unwrap();
        C::from_number(msgcode).unwrap()
    }

    fn message_args(&self) -> &Vec<Value>
    {
        let msgargs = &self.as_vec()[2];
        msgargs.as_array().unwrap()
    }
}


/// A representation of the Notification RPC message type.
#[derive(Debug, Clone, PartialEq)]
pub struct NotificationMessage<C>
{
    msg: Message,
    msgtype: PhantomData<C>,
}


impl<C> RpcMessage for NotificationMessage<C>
where
    C: CodeConvert<C>,
{
    type Err = ToNoticeError;

    fn as_vec(&self) -> &Vec<Value>
    {
        self.msg.as_vec()
    }

    fn as_value(&self) -> &Value
    {
        self.msg.as_value()
    }
}


impl<C> RpcMessageType for NotificationMessage<C>
where
    C: CodeConvert<C>,
{
    fn as_message(&self) -> &Message
    {
        &self.msg
    }
}


impl<C> RpcNotice<C> for NotificationMessage<C>
where
    C: CodeConvert<C>,
{
}


impl<C> NotificationMessage<C>
where
    C: CodeConvert<C>,
{
    /// Create a brand new NotificationMessage object.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate rmpv;
    /// extern crate siminau_rpc;
    ///
    /// use rmpv::Value;
    /// use siminau_rpc::core::{MessageType, RpcMessage};
    /// use siminau_rpc::core::notify::{NotificationMessage, RpcNotice};
    ///
    /// # fn main() {
    /// // Create Notice alias
    /// type Notice = NotificationMessage<MessageType>;
    ///
    /// // Re-use MessageType as message code
    /// let req = Notice::new(MessageType::Notification,
    ///                       vec![Value::from(42)]);
    /// # }
    /// ```
    pub fn new(notifycode: C, args: Vec<Value>) -> Self
    {
        let msgtype = Value::from(MessageType::Notification as u8);
        let notifycode = Value::from(notifycode.to_u64());
        let msgargs = Value::from(args);
        let msgval = Value::from(vec![msgtype, notifycode, msgargs]);

        match Message::from_msg(msgval) {
            Ok(msg) => Self {
                msg: msg,
                msgtype: PhantomData,
            },
            Err(_) => unreachable!(),
        }
    }

    // Checks that the message type parameter of a Notification message is
    // valid.
    //
    // This is a private method used by the public from_msg() method
    fn check_message_type(msgtype: &Value) -> Result<(), NoticeTypeError>
    {
        let msgtype = msgtype.as_u64().unwrap() as u8;
        let expected_msgtype = MessageType::Notification.to_number();
        if msgtype != expected_msgtype {
            let err = NoticeTypeError {
                expected_type: expected_msgtype,
                msgtype: msgtype,
            };
            Err(err)
        } else {
            Ok(())
        }
    }

    // Checks that the message code parameter of a Notification message is
    // valid.
    //
    // This is a private method used by the public from_msg() method
    fn check_message_code(msgcode: &Value) -> Result<(), NoticeCodeError>
    {
        let msgcode =
            check_int(msgcode.as_u64(), C::max_number(), "a value".to_string())
                .map_err(|e| NoticeCodeError::InvalidValue(e))?;

        // Convert msgcode into a number that can be accepted by the
        // CodeConvert type
        let msgcode_u64 = msgcode as u64;
        let val = match C::cast_number(msgcode_u64) {
            Some(v) => v,
            None => {
                let err = NoticeCodeError::ToNumber(msgcode_u64);
                return Err(err);
            }
        };

        // Try to convert msgcode into a CodeConvert type
        C::from_number(val).map_err(|_| NoticeCodeError::ToCode(msgcode_u64))?;
        Ok(())
    }

    // Check that the message arguments parameter of a Notification message is
    // valid.
    //
    // This is a private method used by the public from_msg() method
    fn check_message_args(msgargs: &Value) -> Result<(), NoticeArgsError>
    {
        match msgargs.as_array() {
            Some(_) => Ok(()),
            None => {
                let err = NoticeArgsError {
                    value_type: value_type(&msgargs),
                };
                Err(err)
            }
        }
    }
}


impl<C> FromMessage<Message> for NotificationMessage<C>
where
    C: CodeConvert<C>
{
    type Err = ToNoticeError;

    /// Create a NotificationMessage from a Message
    ///
    /// # Errors
    ///
    /// An error is returned if any of the following are true:
    ///
    /// 1. The message is an array with a len != 3
    /// 2. The message's type parameter is not MessageType::Notification
    /// 3. The message's code parameter cannot be converted into the
    ///    notification's expected code type
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate rmpv;
    /// extern crate siminau_rpc;
    ///
    /// use rmpv::Value;
    /// use siminau_rpc::core::{CodeConvert, FromMessage, Message, MessageType,
    ///                         RpcMessage};
    /// use siminau_rpc::core::notify::{NotificationMessage, RpcNotice};
    ///
    /// # fn main() {
    /// // Create an alias for NotificationMessage, re-using `MessageType` as the
    /// // message code.
    /// type Notice = NotificationMessage<MessageType>;
    ///
    /// // Build Message
    /// let msgtype = Value::from(MessageType::Notification.to_number());
    /// let msgcode = Value::from(MessageType::Request.to_number());
    /// let msgargs = Value::Array(vec![Value::from(9001)]);
    /// let msgval = Value::Array(vec![msgtype, msgcode, msgargs]);
    /// let msg = Message::from_msg(msgval).unwrap();
    ///
    /// // Turn the message into a Notice type
    /// let req = Notice::from_msg(msg).unwrap();
    /// # }
    /// ```
    fn from_msg(msg: Message) -> Result<Self, Self::Err>
    {
        // Notifications is always represented as an array of 4 values
        {
            // Requests is always represented as an array of 3 values
            let array = msg.as_vec();
            let arraylen = array.len();
            if arraylen != 3 {
                let err = ToNoticeError::ArrayLength(arraylen);
                return Err(err);
            }

            // Run all check functions and return the first error generated
            Self::check_message_type(&array[0])
                .map_err(|e| ToNoticeError::InvalidType(e))?;

            Self::check_message_code(&array[1])
                .map_err(|e| ToNoticeError::InvalidCode(e))?;

            Self::check_message_args(&array[2])
                .map_err(|e| ToNoticeError::InvalidArgs(e))?;
        }

        Ok(Self {
            msg: msg,
            msgtype: PhantomData,
        })
    }
}


// Also implements Into<Message> for NotificationMessage
impl<C> From<NotificationMessage<C>> for Message
{
    fn from(req: NotificationMessage<C>) -> Message
    {
        req.msg
    }
}


// Also implements Into<Value> for NotificationMessage
impl<C> From<NotificationMessage<C>> for Value
{
    fn from(req: NotificationMessage<C>) -> Value
    {
        req.msg.into()
    }
}


// ===========================================================================
//
// ===========================================================================
