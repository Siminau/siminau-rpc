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
//! extern crate siminau_rpc;
//!
//! #[macro_use]
//! extern crate siminau_rpc_derive;
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
//! let msg = Message::from(msgval).unwrap();
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

use core::{check_int, value_type, CheckIntError, CodeConvert, Message,
           MessageType, RpcMessage, RpcMessageType};


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
#[derive(Debug)]
pub struct NotificationMessage<C>
{
    msg: Message,
    msgtype: PhantomData<C>,
}


impl<C> RpcMessage for NotificationMessage<C>
where
    C: CodeConvert<C>,
{
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

        match Message::from(msgval) {
            Ok(msg) => Self {
                msg: msg,
                msgtype: PhantomData,
            },
            Err(_) => unreachable!(),
        }
    }

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
    /// use siminau_rpc::core::{CodeConvert, Message, MessageType,
    ///                            RpcMessage};
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
    /// let msg = Message::from(msgval).unwrap();
    ///
    /// // Turn the message into a Notice type
    /// let req = Notice::from(msg).unwrap();
    /// # }
    /// ```
    pub fn from(msg: Message) -> Result<Self, ToNoticeError>
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

    // Checks that the message type parameter of a Notification message is
    // valid.
    //
    // This is a private method used by the public from() method
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
    // This is a private method used by the public from() method
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
    // This is a private method used by the public from() method
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
// Tests
// ===========================================================================


#[cfg(test)]
mod tests
{
    // --------------------
    // Imports
    // --------------------
    // Stdlib imports

    // Third-party imports

    use failure::Fail;
    use quickcheck::TestResult;
    use rmpv::{Utf8String, Value};

    // // Local imports

    use core::{value_type, CodeConvert, CodeValueError, Message, MessageType,
               RpcMessage};
    use core::notify::{NoticeCodeError, NotificationMessage, RpcNotice,
                       ToNoticeError};

    // --------------------
    // Helpers
    // --------------------
    #[derive(Debug, PartialEq, Clone, CodeConvert)]
    enum TestCode
    {
        One,
        Two,
        Three,
    }

    type Notice = NotificationMessage<TestCode>;

    // --------------------
    // NotificationMessage::new
    // --------------------

    quickcheck! {
        fn notificationmessage_new_messagetype_always_notify(code: u8, args: Vec<u8>) -> TestResult {
            if code > 2 {
                return TestResult::discard()
            }

            let msgtype = Value::from(MessageType::Notification.to_number());
            let array: Vec<Value> = args.iter().map(|v| Value::from(v.clone())).collect();
            let array_copy = array.clone();

            // Build expected
            let msgargs = Value::Array(array);
            let a = vec![msgtype, Value::from(code), msgargs];
            let expected = Value::Array(a);

            // Compare NotificationMessage object to expected
            let notice = Notice::new(TestCode::from_number(code).unwrap(),
                                     array_copy);
            TestResult::from_bool(notice.as_value() == &expected)
        }
    }

    // --------------------------
    // NotificationMessage::from
    // --------------------------

    #[test]
    fn notificationmessage_from_invalid_arraylen()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with only 4 arguments

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let arg2 = Value::from(42);
        let arg3 = Value::from(42);
        let array: Vec<Value> = vec![msgtype, msgcode, arg2, arg3];

        let val = Value::Array(array);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // NotificationMessage::from is called with the message
        let result = Notice::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned
        let val = match result {
            Err(e @ ToNoticeError::ArrayLength(_)) => {
                let expected = "Expected array length of 3, got 4".to_string();
                e.to_string() == expected
            }
            _ => false,
        };
        assert!(val);
    }

    #[test]
    fn notificationmessage_from_invalid_messagetype()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with MessageType::Request

        // Create message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgcode, msgval]);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // NotificationMessage::from is called with the message
        let result = Notice::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned
        match result {
            Err(e @ ToNoticeError::InvalidType(_)) => {
                // Check top-level err message
                let expected = "Invalid notification message type".to_owned();
                assert_eq!(e.to_string(), expected);

                // Check cause error
                let cause = e.cause().unwrap();
                let expected = format!(
                    "Expected notification message type value {}, got {}",
                    MessageType::Notification.to_number(),
                    MessageType::Request.to_number()
                );
                assert_eq!(cause.to_string(), expected);
                assert!(cause.cause().is_none());
            }
            Err(e2) => {
                println!("ERROR ERROR {:?}", e2);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn notificationmessage_from_message_code_invalid_type()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with a string for message code

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::String(Utf8String::from("hello"));
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgcode, msgval]);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // NotificationMessage::from is called with the message
        let result = Notice::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned for the invalid message id
        match result {
            Err(e @ ToNoticeError::InvalidCode(_)) => {
                // Check code error
                match e {
                    ToNoticeError::InvalidCode(
                        NoticeCodeError::InvalidValue(_),
                    ) => {}
                    _ => unreachable!(),
                }

                // Check top-level error
                let expected = "Invalid notification message code".to_owned();
                assert_eq!(e.to_string(), expected);

                // Check code error
                let code_err = e.cause().unwrap();
                let expected = "Invalid notification code value".to_owned();
                assert_eq!(code_err.to_string(), expected);

                // Check cause error
                let cause = code_err.cause().unwrap();
                let expected = "Expected a value but got None".to_owned();
                assert_eq!(cause.to_string(), expected);

                // Check no more errors
                assert!(cause.cause().is_none());
            }
            _ => assert!(false),
        }
    }

    quickcheck! {
        fn notificationmessage_from_message_code_invalid_value(msgcode: u64) -> TestResult {
            if msgcode <= u8::max_value() as u64 {
                return TestResult::discard()
            }

            // --------------------
            // GIVEN
            // --------------------
            // Message with a msgcode > u8::max_value() for message code

            // Create message
            let msgtype = Value::from(MessageType::Notification.to_number());
            let msgcode = Value::from(msgcode);
            let msgval = Value::from(42);

            let val = Value::Array(vec![msgtype, msgcode.clone(), msgval]);
            let msg = Message::from(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // NotificationMessage::from is called with the message
            let result = Notice::from(msg);

            // --------------------
            // THEN
            // --------------------
            // Error is returned for the invalid notification code value
            let res = match result {
                Err(e @ ToNoticeError::InvalidCode(_)) => {
                    // Check code error
                    match e {
                        ToNoticeError::InvalidCode(
                            NoticeCodeError::InvalidValue(_)
                        ) => {}
                        _ => unreachable!(),
                    }

                    // Check top-level error
                    let expected = "Invalid notification message code"
                        .to_owned();
                    assert_eq!(e.to_string(), expected);

                    // Check code error
                    let code_err = e.cause().unwrap();
                    let expected = "Invalid notificaton code value".to_owned();
                    assert_eq!(code_err.to_string(), expected);

                    // Check cause error
                    let cause = code_err.cause().unwrap();
                    let expected = format!("Expected value <= {} but got \
                                            value {}",
                                           u8::max_value(),
                                           msgcode.to_string());
                    cause.to_string() == expected &&
                        cause.cause().is_none()
                }
                _ => false
            };
            TestResult::from_bool(res)
        }

        fn notificationmessage_from_message_code_invalid_code(code: u8) -> TestResult {

            // --------------------
            // GIVEN
            // --------------------
            // Message with a msgcode > 2 for message code
            if code <= 2 {
                return TestResult::discard()
            }

            // Create message
            let msgtype = Value::from(MessageType::Notification.to_number());
            let msgcode = Value::from(code);
            let msgval = Value::from(42);

            let val = Value::Array(vec![msgtype, msgcode.clone(), msgval]);
            let msg = Message::from(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // NotificationMessage::from is called with the message
            let result = Notice::from(msg);

            // --------------------
            // THEN
            // --------------------
            // Error is returned for the invalid message code value
            let res = match result {
                Err(e @ ToNoticeError::InvalidCode(_)) => {
                    // Check code error
                    match e {
                        ToNoticeError::InvalidCode(
                            NoticeCodeError::InvalidValue(_)
                        ) => {}
                        _ => unreachable!(),
                    }

                    // Check top-level error
                    let expected = "Invalid notification message code"
                        .to_owned();
                    assert_eq!(e.to_string(), expected);

                    // Check code error
                    let code_err = e.cause().unwrap();
                    let expected = "Invalid notification code value".to_owned();
                    assert_eq!(code_err.to_string(), expected);

                    // Check cause error
                    let cause = code_err.cause().unwrap();
                    let expected = format!("Expected value <= {} but got \
                                            value {}",
                                           TestCode::max_number(),
                                           msgcode.to_string());
                    cause.to_string() == expected &&
                        cause.cause().is_none()
                }
                _ => false
            };
            TestResult::from_bool(res)
        }
    }

    #[test]
    fn notificationmessage_from_message_args_invalid_type()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with an integer for message args

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgcode, msgval.clone()]);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // NotificationMessage::from is called with the message
        let result = Notice::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned for the invalid notification args type
        match result {
            Err(e @ ToNoticeError::InvalidArgs(_)) => {
                // Check top-level error
                let expected =
                    "Invalid notification message arguments".to_owned();
                assert_eq!(e.to_string(), expected);

                // Check cause error
                let cause = e.cause().unwrap();
                let expected = format!(
                    "Expected array for notification arguments but got {}",
                    value_type(&msgval)
                );
                assert_eq!(cause.to_string(), expected);

                // Check no more errors
                assert!(cause.cause().is_none());
            }
            _ => assert!(false),
        }
    }

    // --------------------
    // RpcMessage methods
    // --------------------

    #[test]
    fn notificationmessage_rpcmessage_as_vec()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgcode, msgval]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let notice = Notice::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // NotificationMessage::as_vec() method is called
        let result = notice.as_vec();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let expected = expected.as_vec();
        assert_eq!(result, expected)
    }

    #[test]
    fn notificationmessage_rpcmessage_as_value()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgcode, msgval]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let notice = Notice::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // NotificationMessage::as_value() method is called
        let result = notice.as_value();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let expected = expected.as_value();
        assert_eq!(result, expected)
    }

    // --------------------
    // RpcNotice methods
    // --------------------

    #[test]
    fn notificationmessage_rpcnotice_message_code()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgcode, msgval]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let notice = Notice::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // NotificationMessage::message_id() method is called
        let result = notice.message_code();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let code = expected.as_vec()[1].as_u64().unwrap() as u8;
        let expected = TestCode::from_number(code).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn notificationmessage_rpcnotice_message_args()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let msgargs = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgcode, msgargs.clone()]);
        let msg = Message::from(val).unwrap();
        let notice = Notice::from(msg).unwrap();

        let expected = msgargs.as_array().unwrap();

        // --------------------
        // WHEN
        // --------------------
        // NotificationMessage::message_id() method is called
        let result = notice.message_args();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        assert_eq!(result, expected)
    }
}


// ===========================================================================
//
// ===========================================================================
