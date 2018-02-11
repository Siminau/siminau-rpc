// src/test/core/request/requestmessage.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

// Local imports

// Helpers

use super::TestEnum;

// ===========================================================================
// Tests
// ===========================================================================

mod new
{
    // Stdlib imports

    // Third-party imports

    // use failure::Fail;
    use quickcheck::TestResult;
    // use rmpv::{Utf8String, Value};
    use rmpv::Value;

    // Local imports

    use core::{CodeConvert, MessageType, RpcMessage};
    use core::request::RequestMessage;

    // Helpers
    use super::TestEnum;

    quickcheck! {
        fn new_messagetype_always_request(msgid: u32, code: u8, args: Vec<u8>)
            -> TestResult
        {
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
}

mod from
{
    // Stdlib imports

    // Third-party imports

    use failure::Fail;
    use quickcheck::TestResult;
    // use rmpv::{Utf8String, Value};
    use rmpv::{Utf8String, Value};

    // Local imports

    use core::{value_type, CheckIntError, CodeConvert, FromMessage, Message,
               MessageType};
    use core::request::{RequestCodeError, RequestMessage, ToRequestError};

    // Helpers
    use super::TestEnum;

    #[test]
    fn invalid_arraylen()
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
        let msg = Message::from_msg(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::from_msg is called with the message
        let result: Result<RequestMessage<TestEnum>, ToRequestError>;
        result = RequestMessage::from_msg(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned
        match result {
            Err(e @ ToRequestError::ArrayLength(_)) => {
                let expected = "expected array length of 4, got 3".to_string();
                assert_eq!(e.to_string(), expected);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn invalid_messagetype()
    {
        // --------------------
        // GIVEN
        // --------------------
        // Message with MessageType::Notification

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgid = Value::from(42);
        let msgmeth = Value::from(TestEnum::One.to_number());
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgid, msgmeth, msgval]);
        let msg = Message::from_msg(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::from_msg is called with the message
        let result: Result<RequestMessage<TestEnum>, ToRequestError>;
        result = RequestMessage::from_msg(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned
        match result {
            Err(e @ ToRequestError::InvalidType(_)) => {
                // Check top level error
                let expected = "Invalid request message type".to_owned();
                assert_eq!(e.to_string(), expected);

                // Check the cause error
                let expected_cause_msg = format!(
                    "expected request message type value {}, got {}",
                    MessageType::Request.to_number(),
                    MessageType::Notification.to_number()
                );

                let cause = e.cause().unwrap();
                assert_eq!(cause.to_string(), expected_cause_msg);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn message_id_invalid_type()
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
        let msg = Message::from_msg(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::from_msg is called with the message
        let result: Result<RequestMessage<TestEnum>, ToRequestError>;
        result = RequestMessage::from_msg(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned for the invalid message id
        match result {
            Err(e1 @ ToRequestError::InvalidID(_)) => {
                // Check cause error
                match e1 {
                    ToRequestError::InvalidID(
                        CheckIntError::MissingValue { .. },
                    ) => {}
                    _ => assert!(false),
                }

                // Check top msg
                let expected = "Invalid request message id".to_owned();
                assert_eq!(e1.to_string(), expected);

                // Get cause error
                let val = match e1.cause() {
                    Some(e2) => {
                        assert!(e2.cause().is_none());
                        e2.to_string() == "Expected u32 but got None".to_owned()
                    }
                    _ => false,
                };
                assert!(val);
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
            let msgtype = Value::from(MessageType::Request.to_number());
            let reqid = Value::from(msgid);
            let msgmeth = Value::from(TestEnum::One.to_number());
            let msgval = Value::from(42);

            let val = Value::Array(vec![msgtype, reqid, msgmeth, msgval]);
            let msg = Message::from_msg(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // RequestMessage::from_msg is called with the message
            let result: Result<RequestMessage<TestEnum>, ToRequestError>;
            result = RequestMessage::from_msg(msg);

            // --------------------
            // THEN
            // --------------------
            // Error is returned for the invalid message id value
            let res = match result {
                Err(e @ ToRequestError::InvalidID(_)) => {
                    assert_eq!(e.to_string(), "Invalid request message id".to_owned());

                    // Get cause error
                    let expected = format!("Expected value <= {} but got \
                                            value {}",
                                            u32::max_value(),
                                            msgid);
                    e.cause().unwrap().to_string() == expected
                }
                _ => false
            };
            TestResult::from_bool(res)
        }
    }

    #[test]
    fn message_method_invalid_type()
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
        let msg = Message::from_msg(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::from_msg is called with the message
        let result: Result<RequestMessage<TestEnum>, ToRequestError>;
        result = RequestMessage::from_msg(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned for the invalid message method
        match result {
            Err(e @ ToRequestError::InvalidCode(_)) => {
                // Check top level error message
                let expected = "Invalid request message code".to_owned();
                assert_eq!(e.to_string(), expected);

                // Check specific code error
                let code_err = e.cause().unwrap();
                let expected = "Invalid request code value".to_owned();
                assert_eq!(code_err.to_string(), expected);

                // Check cause error
                let cause = code_err.cause().unwrap();
                let expected = "Expected a value but got None".to_string();
                assert_eq!(cause.to_string(), expected);
            }
            _ => assert!(false),
        }
    }

    quickcheck! {
        fn message_method_invalid_value(msgmeth: u64) -> TestResult {
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
            let msg = Message::from_msg(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // RequestMessage::from_msg is called with the message
            let result: Result<RequestMessage<TestEnum>, ToRequestError>;
            result = RequestMessage::from_msg(msg);

            // --------------------
            // THEN
            // --------------------
            // Error is returned for the invalid message method value
            let res = match result {
                Err(e @ ToRequestError::InvalidCode(_)) => {
                    // Confirm type of code error
                    match e {
                        ToRequestError::InvalidCode(
                            RequestCodeError::InvalidValue(_)
                        ) => {}
                       _ => return TestResult::from_bool(false),
                    }

                    // Check top level error message
                    let expected = "Invalid request message code".to_owned();
                    assert_eq!(e.to_string(), expected);

                    // Check specific code error
                    let code_err = e.cause().unwrap();
                    let expected = "Invalid request code value".to_owned();
                    assert_eq!(code_err.to_string(), expected);

                    // Check cause error
                    let cause = code_err.cause().unwrap();
                    let expected = format!("Expected value <= {} but got \
                                            value {}",
                                            u8::max_value(),
                                            msgmeth.to_string());
                    // No more errors
                    assert!(cause.cause().is_none());

                    cause.to_string() == expected
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
            let msg = Message::from_msg(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // RequestMessage::from_msg is called with the message
            let result: Result<RequestMessage<TestEnum>, ToRequestError>;
            result = RequestMessage::from_msg(msg);

            // --------------------
            // THEN
            // --------------------
            let res = match result {
                Err(e @ ToRequestError::InvalidCode(_)) => {
                    // Confirm type of code error
                    match e {
                        ToRequestError::InvalidCode(
                            RequestCodeError::InvalidValue(_)
                        ) => {}
                       _ => return TestResult::from_bool(false),
                    }

                    // Check top level error message
                    let expected = "Invalid request message code".to_owned();
                    assert_eq!(e.to_string(), expected);

                    // Check specific code error
                    let code_err = e.cause().unwrap();
                    let expected = "Invalid request code value".to_owned();
                    assert_eq!(code_err.to_string(), expected);

                    // Check cause error
                    let cause = code_err.cause().unwrap();
                    let expected = format!("Expected value <= {} but got \
                                            value {}",
                                            TestEnum::max_number(),
                                            msgmeth.to_string());
                    // No more errors
                    assert!(cause.cause().is_none());

                    cause.to_string() == expected
                }
                _ => false
            };
            TestResult::from_bool(res)
        }
    }

    #[test]
    fn message_args_invalid_type()
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
        let msg = Message::from_msg(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // RequestMessage::from_msg is called with the message
        let result: Result<RequestMessage<TestEnum>, ToRequestError>;
        result = RequestMessage::from_msg(msg);

        // --------------------
        // THEN
        // --------------------
        // Error is returned for the invalid message args
        match result {
            Err(e @ ToRequestError::InvalidArgs(_)) => {
                // Check top level error
                let expected = "Invalid request message arguments".to_owned();
                assert_eq!(e.to_string(), expected);

                // Check cause error
                let cause = e.cause().unwrap();
                let expected = format!(
                    "Expected array for request arguments but got {}",
                    value_type(&msgval)
                );
                assert_eq!(cause.to_string(), expected);
            }
            _ => assert!(false),
        }
    }
}

mod convert_bytes
{
    // Stdlib imports

    // Third-party imports
    use bytes::BytesMut;
    use rmpv::Value;

    // Local imports

    use core::{AsBytes, FromBytes, FromBytesError, RpcMessage};
    use core::request::RequestMessage;

    // Helpers

    use test::core::{decode, TestEnum};

    type Request = RequestMessage<TestEnum>;

    #[test]
    fn serialize()
    {
        // --------------------
        // GIVEN
        // a valid RequestMessage
        // --------------------
        let msgargs = vec![Value::from(9001)];
        let msg = Request::new(42, TestEnum::One, msgargs);

        // --------------------
        // WHEN
        // RequestMessage::as_bytes() is called
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
    fn deserialize()
    {
        // --------------------
        // GIVEN
        // an empty BytesMut buffer and
        // a valid RequestMessage and
        // the message is serialized into msgpack
        // --------------------
        let msgargs = vec![Value::from(9001)];
        let msg = Request::new(42, TestEnum::One, msgargs);
        let expected = msg.clone();
        let mut msgpack = msg.as_bytes().try_mut().unwrap();

        // --------------------
        // WHEN
        // RequestMessage::from_bytes() is called with the msgpack bytes
        // --------------------
        let result = Request::from_bytes(&mut msgpack);

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
    fn deserialize_nobytes()
    {
        // --------------------
        // GIVEN
        // an empty BytesMut buffer and
        // --------------------
        let mut buf = BytesMut::new();

        // --------------------
        // WHEN
        // RequestMessage::from_bytes() is called with the empty buffer
        // --------------------
        let result = Request::from_bytes(&mut buf);

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
    fn deserialize_incomplete_message()
    {
        // --------------------
        // GIVEN
        // an empty BytesMut buffer and
        // a valid RequestMessage and
        // the message is serialized into msgpack bytes
        // and some bytes are discarded
        // --------------------
        let msgargs = vec![Value::from(9001)];
        let msg = Request::new(42, TestEnum::One, msgargs);
        let mut msgpack = msg.as_bytes().try_mut().unwrap();

        // Make sure we have bytes
        assert!(!msgpack.is_empty());

        // Discard some bytes to make message bytes incomplete
        let size = msgpack.len() - 2;
        msgpack.truncate(size);

        // --------------------
        // WHEN
        // RequestMessage::from_bytes() is called with the buffer
        // --------------------
        let result = Request::from_bytes(&mut msgpack);

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
    fn deserialize_invalid_message()
    {
        // --------------------
        // GIVEN
        // an empty BytesMut buffer and
        // a valid RequestMessage and
        // the message is serialized into msgpack bytes
        // and half of the bytes are discarded
        // --------------------
        let msgargs = vec![Value::from(9001)];
        let msg = Request::new(42, TestEnum::One, msgargs);
        let mut msgpack = msg.as_bytes().try_mut().unwrap();

        // Make sure we have bytes
        assert!(!msgpack.is_empty());

        // Discard half of the bytes; this should cause an invalid marker error
        // for this specific request message
        let size = msgpack.len() / 2;
        msgpack.truncate(size);

        // --------------------
        // WHEN
        // RequestMessage::from_bytes() is called with the buffer
        // --------------------
        let result = Request::from_bytes(&mut msgpack);

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
