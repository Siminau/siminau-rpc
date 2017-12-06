// src/test/core/response.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports


// Local imports

use core::{CodeConvert, CodeValueError};
use core::response::ResponseMessage;


// ===========================================================================
// Helpers
// ===========================================================================


#[derive(Debug, PartialEq, Clone, CodeConvert)]
enum TestError {
    One,
    Two,
    Three,
}


type Response = ResponseMessage<TestError>;


// ===========================================================================
// Tests
// ===========================================================================


mod new {
    // Stdlib imports

    // Third-party imports

    use quickcheck::TestResult;
    use rmpv::Value;

    // Local imports

    use core::{CodeConvert, MessageType, RpcMessage};

    // Helpers
    use super::{Response, TestError};

    quickcheck! {
        fn messagetype_always_response(msgid: u32, err: u8) -> TestResult {
            if err > 2 {
                return TestResult::discard()
            }

            let msgtype = Value::from(MessageType::Response.to_number());

            // Build expected
            let a = vec![msgtype, Value::from(msgid), Value::from(err),
                         Value::from(42)];
            let expected = Value::Array(a);

            // Compare ResponseMessage object to expected
            let req = Response::new(
                msgid,
                TestError::from_number(err).unwrap(),
                Value::from(42)
            );
            TestResult::from_bool(req.as_value() == &expected)
        }
    }
}


mod from {
    // Stdlib imports

    // Third-party imports

    use failure::Fail;
    use quickcheck::TestResult;
    use rmpv::{Utf8String, Value};

    // Local imports

    use core::{CodeConvert, Message, MessageType};
    use core::response::{ResponseCodeError, ToResponseError};

    // Helpers
    use super::{Response, TestError};

    #[test]
    fn invalid_arraylen() {
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
    fn invalid_messagetype() {
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
    fn message_id_invalid_type() {
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
        fn message_id_invalid_value(msgid: u64) -> TestResult {
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

    #[test]
    fn error_code_invalid_type() {
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
        fn error_code_invalid_value(msgcode: u64) -> TestResult {
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

        fn error_code_invalid_code(code: u8) -> TestResult {

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
}


mod rpcmessage {
    // Stdlib imports

    // Third-party imports

    use rmpv::Value;

    // Local imports

    use core::{CodeConvert, Message, MessageType, RpcMessage};

    // Helpers
    use super::{Response, TestError};

    #[test]
    fn as_vec() {
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
    fn as_value() {
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
}


mod rpcresponse {
    // Stdlib imports

    // Third-party imports

    use rmpv::Value;

    // Local imports

    use core::{CodeConvert, Message, MessageType, RpcMessage};
    use core::response::RpcResponse;

    // Helpers
    use super::{Response, TestError};

    #[test]
    fn message_id() {
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
    fn result() {
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

mod convert_bytes {
    // Stdlib imports

    // Third-party imports
    use bytes::BytesMut;
    use rmpv::Value;

    // Local imports

    use core::{AsBytes, FromBytes, FromBytesError, RpcMessage};

    // Helpers

    use test::core::decode;
    use super::{Response, TestError};

    #[test]
    fn serialize() {
        // --------------------
        // GIVEN
        // a valid ResponseMessage
        // --------------------
        let msg = Response::new(42, TestError::Two, Value::from(9001));

        // --------------------
        // WHEN
        // ResponseMessage::as_bytes() is called
        // --------------------
        let result = msg.as_bytes();

        // --------------------
        // THEN
        // a valid Bytes object is returned
        // --------------------
        let mut buf = result.try_mut().unwrap();
        let expected = decode(&mut buf).unwrap();
        assert_eq!(&expected, msg.as_value());
    }

    #[test]
    fn deserialize() {
        // --------------------
        // GIVEN
        // an empty BytesMut buffer and
        // a valid ResponseMessage and
        // the message is serialized into msgpack
        // --------------------
        let msgargs = Value::from(9001);
        let msg = Response::new(42, TestError::One, msgargs);
        let expected = msg.clone();
        let mut msgpack = msg.as_bytes().try_mut().unwrap();

        // --------------------
        // WHEN
        // ResponseMessage::from_bytes() is called with the msgpack bytes
        // --------------------
        let result = Response::from_bytes(&mut msgpack);

        // --------------------
        // THEN
        // the a message object is returned and
        // the msg is equal to the original message
        // --------------------
        match result {
            Ok(Some(msg)) => assert_eq!(msg, expected),
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_nobytes() {
        // --------------------
        // GIVEN
        // an empty BytesMut buffer and
        // --------------------
        let mut buf = BytesMut::new();

        // --------------------
        // WHEN
        // ResponseMessage::from_bytes() is called with the empty buffer
        // --------------------
        let result = Response::from_bytes(&mut buf);

        // --------------------
        // THEN
        // None is returned
        // --------------------
        let val = match result {
            Ok(None) => true,
            _ => false,
        };

        assert!(val);
    }

    #[test]
    fn deserialize_incomplete_message() {
        // --------------------
        // GIVEN
        // an empty BytesMut buffer and
        // a valid ResponseMessage and
        // the message is serialized into msgpack bytes
        // and some bytes are discarded
        // --------------------
        let msgargs = Value::from(9001);
        let msg = Response::new(42, TestError::One, msgargs);
        let mut msgpack = msg.as_bytes().try_mut().unwrap();

        // Make sure we have bytes
        assert!(!msgpack.is_empty());

        // Discard some bytes to make message bytes incomplete
        let size = msgpack.len() - 2;
        msgpack.truncate(size);

        // --------------------
        // WHEN
        // ResponseMessage::from_bytes() is called with the buffer
        // --------------------
        let result = Response::from_bytes(&mut msgpack);

        // --------------------
        // THEN
        // None is returned
        // --------------------
        let val = match result {
            Ok(None) => true,
            _ => false,
        };

        assert!(val);
    }

    #[test]
    fn deserialize_invalid_message() {
        // --------------------
        // GIVEN
        // an empty BytesMut buffer and
        // a valid ResponseMessage and
        // the message is serialized into msgpack bytes
        // and half of the bytes are discarded
        // --------------------
        let msgargs = Value::from(9001);
        let msg = Response::new(42, TestError::Three, msgargs);
        let mut msgpack = msg.as_bytes().try_mut().unwrap();

        // Make sure we have bytes
        assert!(!msgpack.is_empty());

        // Discard half of the bytes; this should cause an invalid marker error
        // for this specific response message
        let size = msgpack.len() / 2;
        msgpack.truncate(size);

        // --------------------
        // WHEN
        // ResponseMessage::from_bytes() is called with the buffer
        // --------------------
        let result = Response::from_bytes(&mut msgpack);

        // --------------------
        // THEN
        // None is returned
        // --------------------
        let val = match result {
            Err(FromBytesError::InvalidMarkerRead(_)) => true,
            _ => false,
        };

        assert!(val);
    }
}

// ===========================================================================
//
// ===========================================================================
