// src/message/request.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

//! This module defines the Request RPC message type.
//!
//! A Request RPC message is used by a client to send an initial request to a
//! server. Based on the generic [`Message`] type, the Request message type is
//! essentially an array containing 4 items. These 4 items are:
//!
//! 1. Message Type - This will always be the Request message type. While
//!    represented as the enum variant `MessageType::Request`, the value stored
//!    in the array is actually a u8 integer.
//!
//! 2. Message ID - This is a u32 integer that is unique for the
//!    session/connection. If the message id is re-used, then the server is
//!    expected to respond with an error.
//!
//! 3. Message method - This is an integer representing the specific request
//!    being made. This is analogous to the method parameter in the
//!    [`msgpack-rpc`] spec.
//!
//! 4. Message arguments - this is an array of values used to provide
//!    information needed by the server to fulfill the request specified by the
//!    message method.
//!
//! # Example
//!
//! To create a new Request object, you can create one from an existing
//! [`Message`] instance. This is used, for example, when the message is
//! deserialized by the server into a general [`Message`] object, identified as
//! a Request message, and it is required to perform Request specific
//! operations on the message.
//!
//! Alternatively, a `RequestMessage` object can be created manually via the
//! `RequestMessage::new()` method
//!
//! ```rust
//! extern crate rmpv;
//! extern crate siminau_rpc;
//!
//! // This proc macro is needed in order to use the CodeConvert custom derive
//! #[macro_use]
//! extern crate siminau_rpc_derive;
//!
//! use rmpv::Value;
//!
//! // Error types must be in scope in order for CodeConvert to work
//! use siminau_rpc::error::{RpcErrorKind, RpcResult};
//!
//! // Message and request types
//! use siminau_rpc::message::{CodeConvert, Message, MessageType,
//!                            RpcMessage};
//! use siminau_rpc::message::request::{RequestMessage, RpcRequest};
//!
//! // Define Request methods
//! #[derive(Debug, Clone, PartialEq, CodeConvert)]
//! enum Func {
//!     Question,
//!     Answer,
//! }
//!
//! # fn main() {
//! // Create an alias for RequestMessage
//! type Request = RequestMessage<Func>;
//!
//! // Build Message
//! let msgtype = Value::from(MessageType::Request.to_number());
//! let msgid = Value::from(42);
//! let msgmeth = Value::from(Func::Question.to_number());
//! let msgargs = Value::Array(vec![Value::from(42)]);
//! let msgval = Value::Array(vec![msgtype, msgid, msgmeth, msgargs]);
//! let msg = Message::from(msgval).unwrap();
//!
//! // Turn the message into a Request type
//! let req = Request::from(msg).unwrap();
//! assert_eq!(req.message_type(), MessageType::Request);
//! assert_eq!(req.message_id(), 42);
//! assert_eq!(req.message_method(), Func::Question);
//! assert_eq!(req.message_args(), &vec![Value::from(42)]);
//!
//! // Create a brand new request from scratch
//! let new_req = Request::new(42, Func::Answer, vec![Value::from(9000)]);
//! assert_eq!(new_req.message_type(), MessageType::Request);
//! assert_eq!(new_req.message_id(), 42);
//! assert_eq!(new_req.message_method(), Func::Answer);
//! assert_eq!(new_req.message_args(), &vec![Value::from(9000)]);
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
// RequestMessage
// ===========================================================================


/// Trait providing Request message specific getter methods.
///
/// # Example
///
/// ```rust
/// extern crate rmpv;
/// extern crate siminau_rpc;
///
/// use rmpv::Value;
/// use siminau_rpc::message::{MessageType, RpcMessage};
/// use siminau_rpc::message::request::{RequestMessage, RpcRequest};
///
/// # fn main() {
/// // Create Request alias
/// type Request = RequestMessage<MessageType>;
///
/// // Re-use MessageType as message code
/// let req = Request::new(42, MessageType::Notification,
///                        vec![Value::from(42)]);
///
/// // Check all getter methods
/// assert_eq!(req.message_type(), MessageType::Request);
/// assert_eq!(req.message_id(), 42);
/// assert_eq!(req.message_method(), MessageType::Notification);
/// assert_eq!(req.message_args(), &vec![Value::from(42)]);
/// # }
/// ```
pub trait RpcRequest<C>: RpcMessage
where
    C: CodeConvert<C>,
{
    /// Return the message's ID value.
    fn message_id(&self) -> u32
    {
        let msgid = &self.as_vec()[1];
        msgid.as_u64().unwrap() as u32
    }

    /// Return the message's code/method value.
    fn message_method(&self) -> C
    {
        let msgmeth = &self.as_vec()[2];
        let msgmeth = msgmeth.as_u64().unwrap();
        let msgmeth = C::cast_number(msgmeth).unwrap();
        C::from_number(msgmeth).unwrap()
    }

    /// Return the message's arguments.
    fn message_args(&self) -> &Vec<Value>
    {
        let msgargs = &self.as_vec()[3];
        msgargs.as_array().unwrap()
    }
}


/// A representation of the Request RPC message type.
pub struct RequestMessage<C> {
    msg: Message,
    codetype: PhantomData<C>,
}


impl<C> RpcMessage for RequestMessage<C>
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


impl<C> RpcMessageType for RequestMessage<C>
where
    C: CodeConvert<C>,
{
    fn as_message(&self) -> &Message
    {
        &self.msg
    }
}


impl<C> RpcRequest<C> for RequestMessage<C>
where
    C: CodeConvert<C>,
{
}


impl<C> RequestMessage<C>
where
    C: CodeConvert<C>,
{
    /// Create a brand new RequestMessage object.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate rmpv;
    /// extern crate siminau_rpc;
    ///
    /// use rmpv::Value;
    /// use siminau_rpc::message::{MessageType, RpcMessage};
    /// use siminau_rpc::message::request::{RequestMessage, RpcRequest};
    ///
    /// # fn main() {
    /// // Create Request alias
    /// type Request = RequestMessage<MessageType>;
    ///
    /// // Re-use MessageType as message code
    /// let req = Request::new(42, MessageType::Notification,
    ///                        vec![Value::from(42)]);
    /// # }
    /// ```
    pub fn new(msgid: u32, msgmeth: C, args: Vec<Value>) -> Self
    {
        let msgtype = Value::from(MessageType::Request as u8);
        let msgid = Value::from(msgid);
        let msgmeth = Value::from(msgmeth.to_u64());
        let msgargs = Value::from(args);
        let msgval = Value::from(vec![msgtype, msgid, msgmeth, msgargs]);

        match Message::from(msgval) {
            Ok(msg) => Self {
                msg: msg,
                codetype: PhantomData,
            },
            Err(_) => unreachable!(),
        }
    }

    /// Create a RequestMessage from a Message
    ///
    /// # Errors
    ///
    /// An error is returned if any of the following are true:
    ///
    /// 1. The message is an array with a len != 4
    /// 2. The message's type parameter cannot be converted into a
    ///    MessageType variant
    /// 3. The message's id parameter is not a u32
    /// 4. The message's method parameter cannot be converted into the request's
    ///    expected method type
    /// 5. The message's arguments parameter is not an array
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate rmpv;
    /// extern crate siminau_rpc;
    ///
    /// use rmpv::Value;
    /// use siminau_rpc::message::{CodeConvert, Message, MessageType, RpcMessage};
    /// use siminau_rpc::message::request::{RequestMessage, RpcRequest};
    ///
    /// # fn main() {
    /// // Create an alias for RequestMessage, re-using `MessageType` as the
    /// // message code.
    /// type Request = RequestMessage<MessageType>;
    ///
    /// // Build Message
    /// let msgtype = Value::from(MessageType::Request.to_number());
    /// let msgid = Value::from(42);
    /// let msgmeth = Value::from(MessageType::Notification.to_number());
    /// let msgargs = Value::Array(vec![Value::from(9001)]);
    /// let msgval = Value::Array(vec![msgtype, msgid, msgmeth, msgargs]);
    /// let msg = Message::from(msgval).unwrap();
    ///
    /// // Turn the message into a Request type
    /// let req = Request::from(msg).unwrap();
    /// # }
    /// ```
    pub fn from(msg: Message) -> RpcResult<Self>
    {
        {
            // Requests is always represented as an array of 4 values
            let array = msg.as_vec();
            let arraylen = array.len();
            if arraylen != 4 {
                let errmsg =
                    format!("expected array length of 4, got {}", arraylen);
                bail!(RpcErrorKind::InvalidArrayLength(errmsg))
            }

            // Run all check functions and return the first error generated
            let funcvec: Vec<fn(&Value) -> RpcResult<()>>;
            funcvec = vec![
                Self::check_message_type,
                Self::check_message_id,
                Self::check_message_method,
                Self::check_message_args,
            ];

            for (i, func) in funcvec.iter().enumerate() {
                func(&array[i]).chain_err(|| RpcErrorKind::InvalidRequest)?;
            }
        }
        Ok(Self {
            msg: msg,
            codetype: PhantomData,
        })
    }

    // Checks that the message type parameter of a Request message is valid
    //
    // This is a private method used by the public from() method
    fn check_message_type(msgtype: &Value) -> RpcResult<()>
    {
        let msgtype = msgtype.as_u64().unwrap() as u8;
        let expected_msgtype = MessageType::Request.to_number();
        if msgtype != expected_msgtype {
            let errmsg = format!(
                "expected {} for message type (ie \
                 MessageType::Request), got {}",
                expected_msgtype,
                msgtype
            );
            bail!(RpcErrorKind::InvalidMessageType(errmsg))
        }
        Ok(())
    }

    // Checks that the message id parameter of a Request message is valid
    //
    // This is a private method used by the public from() method
    fn check_message_id(msgid: &Value) -> RpcResult<()>
    {
        check_int(msgid.as_u64(), u32::max_value() as u64, "u32".to_string())
            .chain_err(|| RpcErrorKind::InvalidIDType)?;
        Ok(())
    }

    // Checks that the message method parameter of a Request message is valid
    //
    // This is a private method used by the public from() method
    fn check_message_method(msgmeth: &Value) -> RpcResult<()>
    {
        let msgmeth =
            check_int(msgmeth.as_u64(), C::max_number(), "a value".to_string())
                .chain_err(|| {
                    RpcErrorKind::InvalidRequestMethod(
                        String::from("invalid value for method"),
                    )
                })?;

        // Convert msgmeth into a number that can be accepted by the CodeConvert
        // type
        let val = match C::cast_number(msgmeth as u64) {
            Some(v) => v,
            None => {
                let errmsg = format!(
                    "Cannot cast {} into request method value",
                    msgmeth
                );
                bail!(RpcErrorKind::InvalidRequestMethod(errmsg))
            }
        };

        // Try to convert msgmeth into a CodeConvert type
        C::from_number(val).chain_err(|| {
            let errmsg =
                format!("Cannot convert {} into request method", msgmeth);
            RpcErrorKind::InvalidRequestMethod(errmsg)
        })?;
        Ok(())
    }

    // Check that the message arguments parameter of a Request message is valid
    //
    // This is a private method used by the public from() method
    fn check_message_args(msgargs: &Value) -> RpcResult<()>
    {
        match msgargs.as_array() {
            Some(_) => Ok(()),
            None => {
                let errmsg = format!(
                    "expected array for request arguments but got {}",
                    value_type(&msgargs)
                );
                Err(RpcErrorKind::InvalidRequestArgs(errmsg).into())
            }
        }
    }
}


// Also implements Into<Message> for RequestMessage
impl<C> From<RequestMessage<C>> for Message {
    fn from(req: RequestMessage<C>) -> Message
    {
        req.msg
    }
}


// Also implements Into<Value> for RequestMessage
impl<C> From<RequestMessage<C>> for Value {
    fn from(req: RequestMessage<C>) -> Value
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

    // Local imports

    use error::{RpcErrorKind, RpcResult};
    use message::{CodeConvert, Message, MessageType, RpcMessage, value_type};
    use message::request::{RequestMessage, RpcRequest};

    // --------------------
    // Helpers
    // --------------------
    #[derive(Debug, PartialEq, Clone, CodeConvert)]
    enum TestEnum {
        One,
        Two,
        Three,
    }

    // --------------------
    // RequestMessage::new
    // --------------------

    quickcheck! {
        fn requestmessage_new_messagetype_always_request(msgid: u32, code: u8, args: Vec<u8>) -> TestResult {
            if code > 2 {
                return TestResult::discard()
            }

            let msgtype = Value::from(MessageType::Request.to_number());
            let array: Vec<Value> = args.iter().map(|v| Value::from(v.clone())).collect();
            let array_copy = array.clone();

            // Build expected
            let msgargs = Value::Array(array);
            let a = vec![msgtype, Value::from(msgid), Value::from(code),
                         msgargs];
            let expected = Value::Array(a);

            // Compare RequestMessage object to expected
            let req = RequestMessage::new(msgid,
                                          TestEnum::from_number(code).unwrap(),
                                          array_copy);
            TestResult::from_bool(req.as_value() == &expected)
        }
    }

    // --------------------
    // RequestMessage::from
    // --------------------

    #[test]
    fn requestmessage_from_invalid_arraylen()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with only 3 arguments

        // Create message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::from(TestEnum::One.to_number());
        let array: Vec<Value> = vec![msgtype, msgid, msgmeth];

        let val = Value::Array(array);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::from is called with the message
        let result: RpcResult<RequestMessage<TestEnum>>;
        result = RequestMessage::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned
        match result {
            Err(e) => {
                let errmsg = "expected array length of 4, got 3".to_string();
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
    fn requestmessage_from_invalid_messagetype()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with MessageType::Notification

        // Create message
        let expected = MessageType::Notification.to_number();
        let msgtype = Value::from(expected);
        let msgid = Value::from(42);
        let msgmeth = Value::from(TestEnum::One.to_number());
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgid, msgmeth, msgval]);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::from is called with the message
        let result: RpcResult<RequestMessage<TestEnum>>;
        result = RequestMessage::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned
        match result {
            Err(e) => {
                match e.kind() {
                    &RpcErrorKind::InvalidRequest => {
                        // Get cause
                        let all_err: Vec<_> = e.iter().collect();
                        assert_eq!(all_err.len(), 2);
                        let cause = all_err[1];

                        // Compare cause message
                        let errmsg = format!(
                            "expected {} for message type (ie \
                             MessageType::Request), got {}",
                            MessageType::Request.to_number(),
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
    fn requestmessage_from_message_id_invalid_type()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with a string for message id

        // Create message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::String(Utf8String::from("hello"));
        let msgmeth = Value::from(TestEnum::One.to_number());
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgid, msgmeth, msgval]);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::from is called with the message
        let result: RpcResult<RequestMessage<TestEnum>>;
        result = RequestMessage::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned for the invalid message id
        match result {
            Err(e) => {
                match e.kind() {
                    &RpcErrorKind::InvalidRequest => {
                        // Get cause
                        let all_err: Vec<_> = e.iter().collect();
                        assert_eq!(all_err.len(), 3);
                        let next_err = all_err[1];
                        let cause = all_err[2];

                        // Compare next err message ie error generated by
                        // check_message_id
                        assert_eq!(
                            next_err.to_string(),
                            "Invalid message id type"
                        );

                        // Compare cause message
                        let errmsg = "expected u32 but got None";
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
        fn requestmessage_from_message_id_invalid_value(msgid: u64) -> TestResult {
            if msgid <= u32::max_value() as u64 {
                return TestResult::discard()
            }

            // --------------------
            // GIVEN
            // --------------------
            // Message with a val > u32::max_value() for message id

            // Create message
            let msgtype = Value::from(MessageType::Request.to_number());
            let msgid = Value::from(msgid);
            let msgmeth = Value::from(TestEnum::One.to_number());
            let msgval = Value::from(42);

            let val = Value::Array(vec![msgtype, msgid.clone(), msgmeth, msgval]);
            let msg = Message::from(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // RequestMessage::from is called with the message
            let result: RpcResult<RequestMessage<TestEnum>>;
            result = RequestMessage::from(msg);

            // --------------------
            // THEN
            // --------------------
            // Error is returned for the invalid message id value
            let res = match result {
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequest => {
                            // Get cause
                            let all_err: Vec<_> = e.iter().collect();
                            let numerror = all_err.len() == 3;
                            // assert!(numerror);
                            let next_err = all_err[1];
                            let cause = all_err[2];

                            // Compare next err message ie error generated by
                            // check_message_id
                            let next_errmsg = next_err.to_string() == "Invalid message id type";
                            // assert!(next_errmsg);

                            // Compare cause message
                            let errmsg = format!("expected value <= {} but got value {}",
                                                 u32::max_value().to_string(),
                                                 msgid.to_string());
                            let expected = format!("Invalid type: {}", errmsg);
                            let cause_errmsg = cause.to_string() == expected;
                            // assert!(cause_errmsg);
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
    fn requestmessage_from_message_method_invalid_type()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with a string for message code

        // Create message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::String(Utf8String::from("hello"));
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgid, msgmeth, msgval]);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::from is called with the message
        let result: RpcResult<RequestMessage<TestEnum>>;
        result = RequestMessage::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned for the invalid message method
        match result {
            Err(e) => {
                match e.kind() {
                    &RpcErrorKind::InvalidRequest => {
                        // Get cause
                        let all_err: Vec<_> = e.iter().collect();
                        assert_eq!(all_err.len(), 3);
                        let next_err = all_err[1];
                        let cause = all_err[2];

                        // Compare next err message ie error generated by
                        // check_message_method
                        assert_eq!(
                            next_err.to_string(),
                            format!(
                                "Invalid request method: {}",
                                "invalid value for method"
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
        fn requesgmessage_from_message_method_invalid_value(msgmeth: u64) -> TestResult {
            if msgmeth <= u8::max_value() as u64 {
                return TestResult::discard()
            }

            // --------------------
            // GIVEN
            // --------------------
            // Message with a msgmeth > u8::max_value() for message code

            // Create message
            let msgtype = Value::from(MessageType::Request.to_number());
            let msgid = Value::from(42);
            let msgmeth = Value::from(msgmeth);
            let msgval = Value::from(42);

            let val = Value::Array(vec![msgtype, msgid, msgmeth.clone(), msgval]);
            let msg = Message::from(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // RequestMessage::from is called with the message
            let result: RpcResult<RequestMessage<TestEnum>>;
            result = RequestMessage::from(msg);

            // --------------------
            // THEN
            // --------------------
            // Error is returned for the invalid message method value
            let res = match result {
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequest => {
                            // Get cause
                            let all_err: Vec<_> = e.iter().collect();
                            let numerror = all_err.len() == 3;
                            let next_err = all_err[1];
                            let cause = all_err[2];

                            // Compare next err message ie error generated by
                            // check_message_method
                            let next_errmsg = "Invalid request method: \
                                               invalid value for method";
                            let next_errmsg =
                                next_err.to_string() == next_errmsg;

                            // Compare cause message
                            let errmsg = format!("expected value <= {} but got value {}",
                                                 u8::max_value().to_string(),
                                                 msgmeth.to_string());
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

        fn from_message_method_invalid_code(code: u8) -> TestResult {

            // --------------------
            // GIVEN
            // --------------------
            // Message with a msgmeth > 2 for message code
            if code <= 2 {
                return TestResult::discard()
            }

            // Create message
            let msgtype = Value::from(MessageType::Request.to_number());
            let msgid = Value::from(42);
            let msgmeth = Value::from(code);
            let msgval = Value::from(42);

            let val = Value::Array(vec![msgtype, msgid, msgmeth.clone(), msgval]);
            let msg = Message::from(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // RequestMessage::from is called with the message
            let result: RpcResult<RequestMessage<TestEnum>>;
            result = RequestMessage::from(msg);

            // --------------------
            // THEN
            // --------------------
            let res = match result {
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequest => {
                            // Get cause
                            let all_err: Vec<_> = e.iter().collect();
                            let numerror = all_err.len() == 3;
                            let next_err = all_err[1];
                            let cause = all_err[2];

                            // Compare next err message ie error generated by
                            // check_message_method
                            let next_errmsg = "Invalid request method: \
                                               invalid value for method";
                            let next_errmsg =
                                next_err.to_string() == next_errmsg;

                            // Compare cause message
                            let errmsg = format!("expected value <= {} but got value {}",
                                                 TestEnum::max_number().to_string(),
                                                 msgmeth.to_string());
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
    fn requestmessage_from_message_args_invalid_type()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with an integer for message args

        // Create message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::from(TestEnum::One.to_number());
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgid, msgmeth, msgval.clone()]);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::from is called with the message
        let result: RpcResult<RequestMessage<TestEnum>>;
        result = RequestMessage::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned for the invalid message args
        match result {
            Err(e) => {
                match e.kind() {
                    &RpcErrorKind::InvalidRequest => {
                        // Get cause
                        let all_err: Vec<_> = e.iter().collect();
                        assert_eq!(all_err.len(), 2);
                        let cause = all_err[1];

                        // Compare cause message
                        let errmsg = format!(
                            "expected array for request arguments \
                             but got {}",
                            value_type(&msgval)
                        );
                        let expected =
                            format!("Invalid request arguments: {}", errmsg);
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
    fn rpcmessage_as_vec()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::from(TestEnum::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgid, msgmeth, msgval]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let req: RequestMessage<TestEnum> = RequestMessage::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::as_vec() method is called
        let result = req.as_vec();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let expected = expected.as_vec();
        assert_eq!(result, expected)
    }

    #[test]
    fn rpcmessage_as_value()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::from(TestEnum::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgid, msgmeth, msgval]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let req: RequestMessage<TestEnum> = RequestMessage::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::as_value() method is called
        let result = req.as_value();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let expected = expected.as_value();
        assert_eq!(result, expected)
    }

    // --------------------
    // RpcRequest methods
    // --------------------

    #[test]
    fn rpcrequest_message_id()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::from(TestEnum::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgid, msgmeth, msgval]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let req: RequestMessage<TestEnum> = RequestMessage::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::message_id() method is called
        let result = req.message_id();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let expected = expected.as_vec()[1].as_u64().unwrap() as u32;
        assert_eq!(result, expected)
    }

    #[test]
    fn rpcrequest_message_method()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::from(TestEnum::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgid, msgmeth, msgval]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let req: RequestMessage<TestEnum> = RequestMessage::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::message_method() method is called
        let result = req.message_method();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let code = expected.as_vec()[2].as_u64().unwrap() as u8;
        let expected = TestEnum::from_number(code).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn rpcrequest_message_args()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::from(TestEnum::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgid, msgmeth, msgval]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let req: RequestMessage<TestEnum> = RequestMessage::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::message_id() method is called
        let result = req.message_args();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let expected = expected.as_vec()[3].as_array().unwrap();
        assert_eq!(result, expected)
    }
}


// ===========================================================================
//
// ===========================================================================
