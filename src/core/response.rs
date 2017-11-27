// src/core/response.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

//! This module defines the Response RPC message type.
//!
//! A Response RPC message is used by a server to send a response to a client's request.
//! server. Based on the generic [`Message`] type, the Response message type is
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
//! 3. Error code - This is an integer representing any errors due to the
//!    client's request. If there is no error, then this value is `0`.
//!
//! 4. Message result - this is an arbitrary value.
//!
//! # Example
//!
//! To create a new Response object, you can create one from an existing
//! [`Message`] instance. This is used, for example, when the message is
//! deserialized by the server into a general [`Message`] object, identified as
//! a Response message, and it is required to perform Response specific
//! operations on the message.
//!
//! Alternatively, a `ResponseMessage` object can be created manually via the
//! `ResponseMessage::new()` method
//!
//! ```rust
//!
//! extern crate rmpv;
//! extern crate siminau_rpc;
//!
//! #[macro_use]
//! extern crate siminau_rpc_derive;
//!
//! use rmpv::Value;
//!
//! // Message and response types
//! // Note: CodeValueError is needed for CodeConvert custom derive
//! use siminau_rpc::core::{CodeConvert, CodeValueError, Message, MessageType,
//!                         RpcMessage};
//! use siminau_rpc::core::response::{ResponseMessage, RpcResponse};
//!
//! // Define Error codes
//! #[derive(Debug, Clone, PartialEq, CodeConvert)]
//! enum RequestError {
//!     Nope,
//!     NuhUh,
//! }
//!
//! # fn main() {
//! // Create an alias for ResponseMessage
//! type Response = ResponseMessage<RequestError>;
//!
//! // Build Message
//! let msgtype = Value::from(MessageType::Response.to_number());
//! let msgid = Value::from(42);
//! let msgcode = Value::from(RequestError::Nope.to_number());
//! let msgresult = Value::from(9001);
//! let msgval = Value::Array(vec![msgtype, msgid, msgcode, msgresult]);
//! let msg = Message::from(msgval).unwrap();
//!
//! // Turn the message into a Response type
//! let res = Response::from(msg).unwrap();
//! assert_eq!(res.message_type(), MessageType::Response);
//! assert_eq!(res.message_id(), 42);
//! assert_eq!(res.error_code(), RequestError::Nope);
//! assert_eq!(res.result(), &Value::from(9001));
//!
//! // Create a brand new response from scratch
//! let new_res = Response::new(42, RequestError::NuhUh, Value::from(9001));
//! assert_eq!(new_res.message_type(), MessageType::Response);
//! assert_eq!(new_res.message_id(), 42);
//! assert_eq!(new_res.error_code(), RequestError::NuhUh);
//! assert_eq!(new_res.result(), &Value::from(9001));
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

use core::{check_int, CheckIntError, CodeConvert, Message, MessageType,
           RpcMessage, RpcMessageType};


// ===========================================================================
// ResponseMessage errors
// ===========================================================================


#[derive(Debug, Fail)]
#[fail(display = "Expected response message type value {}, got {}",
       expected_type, msgtype)]
pub struct ResponseTypeError
{
    expected_type: u8,
    msgtype: u8,
}


#[derive(Debug, Fail)]
#[fail(display = "Invalid response message id")]
pub struct ResponseIDError
{
    #[cause] err: CheckIntError,
}


#[derive(Debug, Fail)]
pub enum ResponseCodeError
{
    #[fail(display = "Invalid response code value")]
    InvalidValue(#[cause] CheckIntError),

    #[fail(display = "Cannot cast {} into response method value", _0)]
    ToNumber(u64),

    #[fail(display = "Cannot convert method value {} into response code", _0)]
    ToCode(u64),
}


#[derive(Debug, Fail)]
pub enum ToResponseError
{
    #[fail(display = "Expected array length of 4, got {}", _0)]
    ArrayLength(usize),

    #[fail(display = "Invalid response message type")]
    InvalidType(#[cause] ResponseTypeError),

    #[fail(display = "Invalid response message id")]
    InvalidID(#[cause] CheckIntError),

    #[fail(display = "Invalid response message code")]
    InvalidCode(#[cause] ResponseCodeError),
}


// ===========================================================================
// ResponseMessage
// ===========================================================================


/// Trait providing Response message specific getter methods.
///
/// # Example
///
/// ```rust
/// extern crate rmpv;
/// extern crate siminau_rpc;
///
/// use rmpv::Value;
/// use siminau_rpc::core::{MessageType, RpcMessage};
/// use siminau_rpc::core::response::{ResponseMessage, RpcResponse};
///
/// # fn main() {
/// // Create Response alias
/// type Response = ResponseMessage<MessageType>;
///
/// // Re-use MessageType as error code
/// let req = Response::new(42, MessageType::Notification,
///                         Value::from(9001));
///
/// // Check all getter methods
/// assert_eq!(req.message_type(), MessageType::Response);
/// assert_eq!(req.message_id(), 42);
/// assert_eq!(req.error_code(), MessageType::Notification);
/// assert_eq!(req.result(), &Value::from(9001));
/// # }
/// ```
pub trait RpcResponse<C>: RpcMessage
where
    C: CodeConvert<C>,
{
    fn message_id(&self) -> u32
    {
        let msgid = &self.as_vec()[1];
        msgid.as_u64().unwrap() as u32
    }

    fn error_code(&self) -> C
    {
        let errcode = &self.as_vec()[2];
        let errcode = errcode.as_u64().unwrap();
        let errcode = C::cast_number(errcode).unwrap();
        C::from_number(errcode).unwrap()
    }

    fn result(&self) -> &Value
    {
        let msgresult = &self.as_vec()[3];
        msgresult
    }
}


/// A representation of the Response RPC message type.
#[derive(Debug)]
pub struct ResponseMessage<C>
{
    msg: Message,
    msgtype: PhantomData<C>,
}


impl<C> RpcMessage for ResponseMessage<C>
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


impl<C> RpcMessageType for ResponseMessage<C>
where
    C: CodeConvert<C>,
{
    fn as_message(&self) -> &Message
    {
        &self.msg
    }
}


impl<C> RpcResponse<C> for ResponseMessage<C>
where
    C: CodeConvert<C>,
{
}


impl<C> ResponseMessage<C>
where
    C: CodeConvert<C>,
{
    /// Create a brand new ResponseMessage object.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate rmpv;
    /// extern crate siminau_rpc;
    ///
    /// use rmpv::Value;
    /// use siminau_rpc::core::{MessageType, RpcMessage};
    /// use siminau_rpc::core::response::{ResponseMessage, RpcResponse};
    ///
    /// # fn main() {
    /// // Create Response alias
    /// type Response = ResponseMessage<MessageType>;
    ///
    /// // Re-use MessageType as error code
    /// let res = Response::new(42, MessageType::Notification,
    ///                         Value::from(42));
    /// # }
    /// ```
    pub fn new(msgid: u32, errcode: C, result: Value) -> Self
    {
        let msgtype = Value::from(MessageType::Response as u8);
        let msgid = Value::from(msgid);
        let errcode = Value::from(errcode.to_u64());
        let msgval = Value::from(vec![msgtype, msgid, errcode, result]);

        match Message::from(msgval) {
            Ok(msg) => Self {
                msg: msg,
                msgtype: PhantomData,
            },
            Err(_) => unreachable!(),
        }
    }

    /// Create a ResponseMessage from a Message
    ///
    /// # Errors
    ///
    /// An error is returned if any of the following are true:
    ///
    /// 1. The message is an array with a len != 4
    /// 2. The message's type parameter is not MessageType::Response
    /// 3. The message's id parameter is not a u32
    /// 4. The message's error parameter cannot be converted into the request's
    ///    expected error code type
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate rmpv;
    /// extern crate siminau_rpc;
    ///
    /// use rmpv::Value;
    /// use siminau_rpc::core::{CodeConvert, Message, MessageType, RpcMessage};
    /// use siminau_rpc::core::response::{ResponseMessage, RpcResponse};
    ///
    /// # fn main() {
    /// // Create an alias for ResponseMessage, re-using `MessageType` as the
    /// // message code.
    /// type Response = ResponseMessage<MessageType>;
    ///
    /// // Build Message
    /// let msgtype = Value::from(MessageType::Response.to_number());
    /// let msgid = Value::from(42);
    /// let msgcode = Value::from(MessageType::Notification.to_number());
    /// let msgresult = Value::from(9001);
    /// let msgval = Value::Array(vec![msgtype, msgid, msgcode, msgresult]);
    /// let msg = Message::from(msgval).unwrap();
    ///
    /// // Turn the message into a Response type
    /// let res = Response::from(msg).unwrap();
    /// # }
    /// ```
    pub fn from(msg: Message) -> Result<Self, ToResponseError>
    {
        // Response is always represented as an array of 4 values
        {
            // Response is always represented as an array of 4 values
            let array = msg.as_vec();
            let arraylen = array.len();
            if arraylen != 4 {
                return Err(ToResponseError::ArrayLength(arraylen));
            }

            // Run all check functions and return the first error generated
            Self::check_message_type(&array[0])
                .map_err(|e| ToResponseError::InvalidType(e))?;

            Self::check_message_id(&array[1]).map_err(|e| {
                let ResponseIDError { err } = e;
                ToResponseError::InvalidID(err)
            })?;

            Self::check_error_code(&array[2])
                .map_err(|e| ToResponseError::InvalidCode(e))?;
        }
        Ok(Self {
            msg: msg,
            msgtype: PhantomData,
        })
    }

    // Checks that the message type parameter of a Response message is valid
    //
    // This is a private method used by the public from() method
    fn check_message_type(msgtype: &Value) -> Result<(), ResponseTypeError>
    {
        let msgtype = msgtype.as_u64().unwrap() as u8;
        let expected_msgtype = MessageType::Response.to_number();
        if msgtype != expected_msgtype {
            let err = ResponseTypeError {
                expected_type: expected_msgtype,
                msgtype: msgtype,
            };
            return Err(err);
        }
        Ok(())
    }

    // Checks that the message id parameter of a Response message is valid
    //
    // This is a private method used by the public from() method
    fn check_message_id(msgid: &Value) -> Result<(), ResponseIDError>
    {
        check_int(msgid.as_u64(), u32::max_value() as u64, "u32".to_string())
            .map_err(|e| ResponseIDError { err: e })?;
        Ok(())
    }

    // Checks that the error code parameter of a Response message is valid
    //
    // This is a private method used by the public from() method
    fn check_error_code(errcode: &Value) -> Result<(), ResponseCodeError>
    {
        let errcode =
            check_int(errcode.as_u64(), C::max_number(), "a value".to_string())
                .map_err(|e| ResponseCodeError::InvalidValue(e))?;

        // Convert errcode into a number that can be accepted by the
        // CodeConvert type
        let errcode_u64 = errcode as u64;
        let val = match C::cast_number(errcode_u64) {
            Some(v) => v,
            None => {
                return Err(ResponseCodeError::ToNumber(errcode_u64));
            }
        };

        // Try to convert errcode into a CodeConvert type
        C::from_number(val)
            .map_err(|_| ResponseCodeError::ToCode(errcode_u64))?;
        Ok(())
    }
}


// Also implements Into<Message> for ResponseMessage
impl<C> From<ResponseMessage<C>> for Message
{
    fn from(req: ResponseMessage<C>) -> Message
    {
        req.msg
    }
}


// Also implements Into<Value> for ResponseMessage
impl<C> From<ResponseMessage<C>> for Value
{
    fn from(req: ResponseMessage<C>) -> Value
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

    // Local imports

    use core::{CodeConvert, CodeValueError, Message, MessageType, RpcMessage};
    use core::response::{ResponseCodeError, ResponseMessage, RpcResponse,
                         ToResponseError};

    // --------------------
    // Helpers
    // --------------------
    #[derive(Debug, PartialEq, Clone, CodeConvert)]
    enum TestError
    {
        One,
        Two,
        Three,
    }

    type Response = ResponseMessage<TestError>;

    // --------------------
    // ResponseMessage::new
    // --------------------

    quickcheck! {
        fn responsemessage_new_messagetype_always_response(msgid: u32, err: u8) -> TestResult {
            if err > 2 {
                return TestResult::discard()
            }

            let msgtype = Value::from(MessageType::Response.to_number());

            // Build expected
            let a = vec![msgtype, Value::from(msgid), Value::from(err),
                         Value::from(42)];
            let expected = Value::Array(a);

            // Compare ResponseMessage object to expected
            let req = Response::new(msgid,
                                    TestError::from_number(err).unwrap(),
                                    Value::from(42));
            TestResult::from_bool(req.as_value() == &expected)
        }
    }

    // --------------------
    // ResponseMessage::from
    // --------------------

    #[test]
    fn responsemessage_from_invalid_arraylen()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with only 3 arguments

        // Create message
        let msgtype = Value::from(MessageType::Response.to_number());
        let msgid = Value::from(42);
        let msgcode = Value::from(TestError::One.to_number());
        let array: Vec<Value> = vec![msgtype, msgid, msgcode];

        let val = Value::Array(array);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // ResponseMessage::from is called with the message
        let result = Response::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned
        let val = match result {
            Err(e @ ToResponseError::ArrayLength(_)) => {
                // Check error message
                let expected = "Expected array length of 4, got 3".to_string();
                e.to_string() == expected
            }
            _ => false,
        };
        assert!(val);
    }

    #[test]
    fn responsemessage_from_invalid_messagetype()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with MessageType::Notification

        // Create message
        let expected = MessageType::Notification.to_number();
        let msgtype = Value::from(expected);
        let msgid = Value::from(42);
        let msgcode = Value::from(TestError::One.to_number());
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgid, msgcode, msgval]);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // ResponseMessage::from is called with the message
        let result = Response::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned
        match result {
            Err(e @ ToResponseError::InvalidType(_)) => {
                // Check top-level error message
                let expected = "Invalid response message type".to_owned();
                assert_eq!(e.to_string(), expected);

                // Check cause error
                let cause = e.cause().unwrap();
                let msg = format!(
                    "Expected response message type value {}, got {}",
                    MessageType::Response.to_number(),
                    MessageType::Notification.to_number(),
                );
                assert_eq!(cause.to_string(), msg);

                // Make sure no further errors
                assert!(cause.cause().is_none());
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn responsemessage_from_message_id_invalid_type()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with a string for message id

        // Create message
        let msgtype = Value::from(MessageType::Response.to_number());
        let msgid = Value::String(Utf8String::from("hello"));
        let msgcode = Value::from(TestError::One.to_number());
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgid, msgcode, msgval]);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // ResponseMessage::from is called with the message
        let result = Response::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned for the invalid message id
        match result {
            Err(e @ ToResponseError::InvalidID(_)) => {
                // Check top-level message
                let expected = "Invalid response message id".to_owned();
                assert_eq!(e.to_string(), expected);

                // Check cause error
                let cause = e.cause().unwrap();
                let expected = "Expected u32 but got None".to_owned();
                assert_eq!(cause.to_string(), expected);
            }
            _ => assert!(false),
        }
    }

    quickcheck! {
        fn responsemessage_from_message_id_invalid_value(msgid: u64) -> TestResult {
            if msgid <= u32::max_value() as u64 {
                return TestResult::discard()
            }

            // --------------------
            // GIVEN
            // --------------------
            // Message with a val > u32::max_value() for message id

            // Create message
            let msgtype = Value::from(MessageType::Response.to_number());
            let msgid = Value::from(msgid);
            let msgcode = Value::from(TestError::One.to_number());
            let msgval = Value::from(42);

            let val = Value::Array(vec![msgtype, msgid.clone(), msgcode, msgval]);
            let msg = Message::from(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // ResponseMessage::from is called with the message
            let result = Response::from(msg);

            // --------------------
            // THEN
            // --------------------
            // Error is returned for the invalid message id value
            let res = match result {
                Err(e @ ToResponseError::InvalidID(_)) => {
                    // Check top-level error
                    let expected = "Invalid response message id".to_owned();
                    if expected != e.to_string() {
                        return TestResult::from_bool(false);
                    }

                    // Check cause
                    let cause = e.cause().unwrap();
                    let expected = format!("Expected value <= {} but got value \
                                            {}",
                                           u32::max_value(),
                                           msgid.to_string());
                    cause.to_string() == expected &&
                        cause.cause().is_none()
                }
                _ => false
            };
            TestResult::from_bool(res)
        }
    }

    // TODO: continue here
    #[test]
    fn responsemessage_from_error_code_invalid_type()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with a string for message code

        // Create message
        let msgtype = Value::from(MessageType::Response.to_number());
        let msgid = Value::from(42);
        let msgcode = Value::String(Utf8String::from("hello"));
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgid, msgcode, msgval]);
        let msg = Message::from(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // ResponseMessage::from is called with the message
        let result = Response::from(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned for the invalid message id
        match result {
            Err(e @ ToResponseError::InvalidCode(_)) => {
                // Check top-level message
                let expected = "Invalid response message code".to_owned();
                assert_eq!(e.to_string(), expected);

                // Check code error
                let code_err = e.cause().unwrap();
                let expected = "Invalid response code value".to_owned();
                assert_eq!(code_err.to_string(), expected);

                // Check cause error
                let cause = code_err.cause().unwrap();
                let expected = "Expected a value but got None".to_owned();
                assert_eq!(cause.to_string(), expected);

                // Confirm no more errors
                assert!(cause.cause().is_none());
            }
            _ => assert!(false),
        }
    }

    quickcheck! {
        fn responsemessage_from_error_code_invalid_value(msgcode: u64) -> TestResult {
            if msgcode <= u8::max_value() as u64 {
                return TestResult::discard()
            }

            // --------------------
            // GIVEN
            // --------------------
            // Message with a msgcode > u8::max_value() for message code

            // Create message
            let msgtype = Value::from(MessageType::Response.to_number());
            let msgid = Value::from(42);
            let msgcode = Value::from(msgcode);
            let msgval = Value::from(42);

            let val = Value::Array(vec![msgtype, msgid, msgcode.clone(), msgval]);
            let msg = Message::from(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // ResponseMessage::from is called with the message
            let result = Response::from(msg);

            // --------------------
            // THEN
            // --------------------
            // Error is returned for the invalid error code value
            let res = match result {
                Err(e @ ToResponseError::InvalidCode(_)) => {
                    match e {
                        ToResponseError::InvalidCode(
                            ResponseCodeError::InvalidValue(_)
                        ) => {}
                        _ => unreachable!(),
                    }

                    // Check top-level error
                    let expected = "Invalid response message code".to_owned();
                    assert_eq!(e.to_string(), expected);

                    // Check error msg
                    let err = e.cause().unwrap();
                    let expected = "Invalid response code value".to_owned();
                    assert_eq!(err.to_string(), expected);

                    // Check cause
                    let err = err.cause().unwrap();
                    let expected = format!("Expected value <= {} but got \
                                            value {}", u8::max_value(),
                                            msgcode.to_string());
                    err.to_string() == expected
                }
                _ => false
            };
            TestResult::from_bool(res)
        }

        fn responsemessage_from_error_code_invalid_code(code: u8) -> TestResult {

            // --------------------
            // GIVEN
            // --------------------
            // Message with a msgcode > 2 for message code
            if code <= 2 {
                return TestResult::discard()
            }

            // Create message
            let msgtype = Value::from(MessageType::Response.to_number());
            let msgid = Value::from(42);
            let msgcode = Value::from(code);
            let msgval = Value::from(42);

            let val = Value::Array(vec![msgtype, msgid, msgcode.clone(), msgval]);
            let msg = Message::from(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // ResponseMessage::from is called with the message
            let result = Response::from(msg);

            // --------------------
            // THEN
            // --------------------
            let res = match result {
                Err(e @ ToResponseError::InvalidCode(_)) => {
                    match e {
                        ToResponseError::InvalidCode(
                            ResponseCodeError::InvalidValue(_)
                        ) => {}
                        _ => unreachable!(),
                    }

                    // Check top-level error
                    let expected = "Invalid response message code".to_owned();
                    assert_eq!(e.to_string(), expected);

                    // Check error msg
                    let err = e.cause().unwrap();
                    let expected = "Invalid response code value".to_owned();
                    assert_eq!(err.to_string(), expected);

                    // Check cause
                    let err = err.cause().unwrap();
                    let expected_val = TestError::max_number();
                    let expected = format!("Expected value <= {} but got \
                                            value {}", expected_val,
                                            msgcode.to_string());
                    err.to_string() == expected
                }
                _ => false
            };
            TestResult::from_bool(res)
        }
    }

    // --------------------
    // RpcMessage methods
    // --------------------

    #[test]
    fn responsemessage_rpcmessage_as_vec()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A response message

        // Create message
        let msgtype = Value::from(MessageType::Response.to_number());
        let msgid = Value::from(42);
        let msgcode = Value::from(TestError::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgid, msgcode, msgval]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let res = Response::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // ResponseMessage::as_vec() method is called
        let result = res.as_vec();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let expected = expected.as_vec();
        assert_eq!(result, expected)
    }

    #[test]
    fn responsemessage_rpcmessage_as_value()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A response message

        // Create message
        let msgtype = Value::from(MessageType::Response.to_number());
        let msgid = Value::from(42);
        let msgcode = Value::from(TestError::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgid, msgcode, msgval]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let res = Response::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // ResponseMessage::as_value() method is called
        let result = res.as_value();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let expected = expected.as_value();
        assert_eq!(result, expected)
    }

    // --------------------
    // RpcResponse methods
    // --------------------

    #[test]
    fn rpcresponse_message_id()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A response message

        // Create message
        let msgtype = Value::from(MessageType::Response.to_number());
        let msgid = Value::from(42);
        let msgcode = Value::from(TestError::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgid, msgcode, msgval]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let res = Response::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // ResponseMessage::message_id() method is called
        let result = res.message_id();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let expected = expected.as_vec()[1].as_u64().unwrap() as u32;
        assert_eq!(result, expected)
    }

    #[test]
    fn rpcresponse_error_code()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A response message

        // Create message
        let msgtype = Value::from(MessageType::Response.to_number());
        let msgid = Value::from(42);
        let msgcode = Value::from(TestError::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgid, msgcode, msgval]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let res = Response::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // ResponseMessage::error_code() method is called
        let result = res.error_code();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let code = expected.as_vec()[2].as_u64().unwrap() as u8;
        let expected = TestError::from_number(code).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn rpcresponse_result()
    {
        // --------------------
        // GIVEN
        // --------------------
        // A response message

        // Create message
        let msgtype = Value::from(MessageType::Response.to_number());
        let msgid = Value::from(42);
        let errcode = Value::from(TestError::One.to_number());
        let msgresult = Value::from(42);

        let val = Value::Array(vec![msgtype, msgid, errcode, msgresult]);
        let msg = Message::from(val).unwrap();
        let expected = msg.clone();
        let res = Response::from(msg).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // ResponseMessage::result() method is called
        let result = res.result();

        // --------------------
        // THEN
        // --------------------
        // The contained value is as expected
        let expected = &expected.as_vec()[3];
        assert_eq!(result, expected)
    }
}


// ===========================================================================
//
// ===========================================================================
