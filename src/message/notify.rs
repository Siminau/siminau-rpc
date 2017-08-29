// src/message/notify.rs
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
//! // Error types must be in scope in order for CodeConvert to work
//! use siminau_rpc::error::{RpcErrorKind, RpcResult};
//!
//! // Message and notify types
//! use siminau_rpc::message::{CodeConvert, Message, MessageType,
//!                            RpcMessage};
//! use siminau_rpc::message::notify::{NotificationMessage, RpcNotice};
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

use error::{RpcErrorKind, RpcResult, RpcResultExt};
use message::{CodeConvert, Message, MessageType, RpcMessage, RpcMessageType,
              check_int, value_type};


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
/// use siminau_rpc::message::{MessageType, RpcMessage};
/// use siminau_rpc::message::notify::{NotificationMessage, RpcNotice};
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
pub struct NotificationMessage<C> {
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
    /// use siminau_rpc::message::{MessageType, RpcMessage};
    /// use siminau_rpc::message::notify::{NotificationMessage, RpcNotice};
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
    /// use siminau_rpc::message::{CodeConvert, Message, MessageType,
    ///                            RpcMessage};
    /// use siminau_rpc::message::notify::{NotificationMessage, RpcNotice};
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
    pub fn from(msg: Message) -> RpcResult<Self>
    {
        // Notifications is always represented as an array of 4 values
        {
            // Requests is always represented as an array of 3 values
            let array = msg.as_vec();
            let arraylen = array.len();
            if arraylen != 3 {
                let errmsg =
                    format!("expected array length of 3, got {}", arraylen);
                bail!(RpcErrorKind::InvalidArrayLength(errmsg))
            }

            // Run all check functions and return the first error generated
            let funcvec: Vec<fn(&Value) -> RpcResult<()>>;
            funcvec = vec![
                Self::check_message_type,
                Self::check_message_code,
                Self::check_message_args,
            ];

            for (i, func) in funcvec.iter().enumerate() {
                func(&array[i]).chain_err(
                    || RpcErrorKind::InvalidNotification,
                )?;
            }
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
    fn check_message_type(msgtype: &Value) -> RpcResult<()>
    {
        let msgtype = msgtype.as_u64().unwrap() as u8;
        let expected_msgtype = MessageType::Notification.to_number();
        if msgtype != expected_msgtype {
            let errmsg = format!(
                "expected {} for message type (ie \
                 MessageType::Notification), got {}",
                expected_msgtype,
                msgtype
            );
            bail!(RpcErrorKind::InvalidMessageType(errmsg))
        }
        Ok(())
    }

    // Checks that the message code parameter of a Notification message is
    // valid.
    //
    // This is a private method used by the public from() method
    fn check_message_code(msgcode: &Value) -> RpcResult<()>
    {
        let msgcode =
            check_int(msgcode.as_u64(), C::max_number(), "a value".to_string())
                .chain_err(|| {
                    RpcErrorKind::InvalidNotificationCode(
                        String::from("invalid value for notification code"),
                    )
                })?;

        // Convert msgcode into a number that can be accepted by the
        // CodeConvert type
        let val = match C::cast_number(msgcode as u64) {
            Some(v) => v,
            None => {
                let errmsg = format!(
                    "Cannot cast {} into notification code value",
                    msgcode
                );
                bail!(RpcErrorKind::InvalidNotificationCode(errmsg))
            }
        };

        // Try to convert msgcode into a CodeConvert type
        C::from_number(val).chain_err(|| {
            let errmsg =
                format!("Cannot convert {} into notification code", msgcode);
            RpcErrorKind::InvalidNotificationCode(errmsg)
        })?;
        Ok(())
    }

    // Check that the message arguments parameter of a Notification message is
    // valid.
    //
    // This is a private method used by the public from() method
    fn check_message_args(msgargs: &Value) -> RpcResult<()>
    {
        match msgargs.as_array() {
            Some(_) => Ok(()),
            None => {
                let errmsg = format!(
                    "expected array for notification arguments but \
                     got {}",
                    value_type(&msgargs)
                );
                Err(RpcErrorKind::InvalidNotificationArgs(errmsg).into())
            }
        }
    }
}


// Also implements Into<Message> for NotificationMessage
impl<C> From<NotificationMessage<C>> for Message {
    fn from(req: NotificationMessage<C>) -> Message
    {
        req.msg
    }
}


// Also implements Into<Value> for NotificationMessage
impl<C> From<NotificationMessage<C>> for Value {
    fn from(req: NotificationMessage<C>) -> Value
    {
        req.msg.into()
    }
}


// ===========================================================================
// Tests
// ===========================================================================


#[cfg(test)]
mod tests {
    // --------------------
    // Imports
    // --------------------
    // Stdlib imports

    // Third-party imports

    use quickcheck::TestResult;
    use rmpv::{Utf8String, Value};

    // // Local imports

    use error::{RpcErrorKind, RpcResult};
    use message::{CodeConvert, Message, MessageType, RpcMessage, value_type};
    use message::notify::{NotificationMessage, RpcNotice};

    // --------------------
    // Helpers
    // --------------------
    #[derive(Debug, PartialEq, Clone, CodeConvert)]
    enum TestCode {
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
        match result {
            Err(e) => {
                let errmsg = "expected array length of 3, got 4".to_string();
                let expected =
                    format!("Invalid message array length: {}", errmsg);
                let res = match e.kind() {
                    &RpcErrorKind::InvalidArrayLength(ref v) => v == &errmsg,
                    _ => false,
                };
                assert!(res);
                assert_eq!(e.to_string(), expected);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn notificationmessage_from_invalid_messagetype()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with MessageType::Request

        // Create message
        let expected = MessageType::Request.to_number();
        let msgtype = Value::from(expected);
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
            Err(e) => {
                match e.kind() {
                    &RpcErrorKind::InvalidNotification => {
                        // Get cause
                        let all_err: Vec<_> = e.iter().collect();
                        assert_eq!(all_err.len(), 2);
                        let cause = all_err[1];

                        // Compare cause message
                        let errmsg = format!(
                            "expected {} for message type (ie \
                             MessageType::Notification), got {}",
                            MessageType::Notification.to_number(),
                            expected
                        );
                        let expected =
                            format!("Invalid message type: {}", errmsg);
                        assert_eq!(cause.to_string(), expected);
                    }
                    _ => assert!(false),
                }
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
            Err(e) => {
                match e.kind() {
                    &RpcErrorKind::InvalidNotification => {
                        // Get cause
                        let all_err: Vec<_> = e.iter().collect();
                        assert_eq!(all_err.len(), 3);
                        let next_err = all_err[1];
                        let cause = all_err[2];

                        // Compare next err message ie error generated by
                        // check_message_code
                        assert_eq!(
                            next_err.to_string(),
                            format!(
                                "Invalid notification code: {}",
                                "invalid value for notification code"
                            )
                        );

                        // Compare cause message
                        let errmsg = "expected a value but got None";
                        let expected = format!("Invalid type: {}", errmsg);
                        assert_eq!(cause.to_string(), expected);
                    }
                    _ => assert!(false),
                }
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
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidNotification => {
                            // Get cause
                            let all_err: Vec<_> = e.iter().collect();
                            let numerror = all_err.len() == 3;
                            let next_err = all_err[1];
                            let cause = all_err[2];

                            // Compare next err message ie error generated by
                            // check_message_method
                            let next_errmsg = "Invalid notification code: \
                                               invalid value for notification code";
                            let next_errmsg =
                                next_err.to_string() == next_errmsg;

                            // Compare cause message
                            let errmsg = format!("expected value <= {} but got value {}",
                                                 TestCode::max_number().to_string(),
                                                 msgcode.to_string());
                            let expected = format!("Invalid type: {}", errmsg);
                            let cause_errmsg = cause.to_string() == expected;
                            numerror && next_errmsg && cause_errmsg
                        }
                        _ => false
                    }
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
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidNotification => {
                            // Get cause
                            let all_err: Vec<_> = e.iter().collect();
                            let numerror = all_err.len() == 3;
                            let next_err = all_err[1];
                            let cause = all_err[2];

                            // Compare next err message ie error generated by
                            // check_message_method
                            let next_errmsg = "Invalid notification code: \
                                               invalid value for notification code";
                            let next_errmsg =
                                next_err.to_string() == next_errmsg;

                            // Compare cause message
                            let errmsg = format!("expected value <= {} but got value {}",
                                                 TestCode::max_number().to_string(),
                                                 msgcode.to_string());
                            let expected = format!("Invalid type: {}", errmsg);
                            let cause_errmsg = cause.to_string() == expected;
                            numerror && next_errmsg && cause_errmsg
                        }
                        _ => false
                    }
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
            Err(e) => {
                match e.kind() {
                    &RpcErrorKind::InvalidNotification => {
                        // Get cause
                        let all_err: Vec<_> = e.iter().collect();
                        assert_eq!(all_err.len(), 2);
                        let cause = all_err[1];

                        // Compare cause message
                        let errmsg = format!(
                            "expected array for notification \
                             arguments but got {}",
                            value_type(&msgval)
                        );
                        let expected = format!(
                            "Invalid notification arguments: {}",
                            errmsg
                        );
                        assert_eq!(cause.to_string(), expected);
                    }
                    _ => assert!(false),
                }
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
