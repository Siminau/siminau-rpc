// src/test/core/notify.rs
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
use core::notify::NotificationMessage;


// ===========================================================================
// Helpers
// ===========================================================================


#[derive(Debug, PartialEq, Clone, CodeConvert)]
enum TestCode {
    One,
    Two,
    Three,
}


type Notice = NotificationMessage<TestCode>;


// ===========================================================================
// Tests
// ===========================================================================


mod new {
    // Stdlib imports

    // Third-party imports

    use quickcheck::TestResult;
    use rmpv::Value;

    // // Local imports

    use core::{CodeConvert, MessageType, RpcMessage};

    // Helpers
    use super::{Notice, TestCode};

    quickcheck! {
        fn messagetype_always_notify(code: u8, args: Vec<u8>) -> TestResult {
            if code > 2 {
                return TestResult::discard()
            }

            let msgtype = Value::from(MessageType::Notification.to_number());
            let array: Vec<Value> = args
                .iter()
                .map(|v| Value::from(v.clone()))
                .collect();
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
}


mod from {
    // Stdlib imports

    // Third-party imports

    use failure::Fail;
    use quickcheck::TestResult;
    use rmpv::{Utf8String, Value};

    // // Local imports

    use core::{value_type, CodeConvert, FromMessage, Message, MessageType};
    use core::notify::{NoticeCodeError, ToNoticeError};

    // Helpers
    use super::{Notice, TestCode};

    #[test]
    fn invalid_arraylen() {
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
        let msg = Message::from_msg(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // NotificationMessage::from_msg is called with the message
        let result = Notice::from_msg(msg);

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
    fn invalid_messagetype() {
        // --------------------
        // GIVEN
        // --------------------
        // Message with MessageType::Request

        // Create message
        let msgtype = Value::from(MessageType::Request.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgcode, msgval]);
        let msg = Message::from_msg(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // NotificationMessage::from_msg is called with the message
        let result = Notice::from_msg(msg);

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
            _ => assert!(false),
        }
    }

    #[test]
    fn message_code_invalid_type() {
        // --------------------
        // GIVEN
        // --------------------
        // Message with a string for message code

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::String(Utf8String::from("hello"));
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgcode, msgval]);
        let msg = Message::from_msg(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // NotificationMessage::from_msg is called with the message
        let result = Notice::from_msg(msg);

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
        fn message_code_invalid_value(msgcode: u64) -> TestResult {
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
            let msg = Message::from_msg(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // NotificationMessage::from_msg is called with the message
            let result = Notice::from_msg(msg);

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

        fn message_code_invalid_code(code: u8) -> TestResult {

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
            let msg = Message::from_msg(val).unwrap();

            // --------------------
            // WHEN
            // --------------------
            // NotificationMessage::from_msg is called with the message
            let result = Notice::from_msg(msg);

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
    fn message_args_invalid_type() {
        // --------------------
        // GIVEN
        // --------------------
        // Message with an integer for message args

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let msgval = Value::from(42);

        let val = Value::Array(vec![msgtype, msgcode, msgval.clone()]);
        let msg = Message::from_msg(val).unwrap();

        // --------------------
        // WHEN
        // --------------------
        // NotificationMessage::from_msg is called with the message
        let result = Notice::from_msg(msg);

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
}


mod rpcmessage {
    // Stdlib imports

    // Third-party imports

    use rmpv::Value;

    // // Local imports

    use core::{CodeConvert, FromMessage, Message, MessageType, RpcMessage};

    // Helpers
    use super::{Notice, TestCode};

    #[test]
    fn as_vec() {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgcode, msgval]);
        let msg = Message::from_msg(val).unwrap();
        let expected = msg.clone();
        let notice = Notice::from_msg(msg).unwrap();

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
    fn as_value() {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgcode, msgval]);
        let msg = Message::from_msg(val).unwrap();
        let expected = msg.clone();
        let notice = Notice::from_msg(msg).unwrap();

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

}


mod rpcnotice {
    // Stdlib imports

    // Third-party imports

    use rmpv::Value;

    // // Local imports

    use core::{CodeConvert, FromMessage, Message, MessageType, RpcMessage};
    use core::notify::RpcNotice;

    // Helpers
    use super::{Notice, TestCode};

    #[test]
    fn message_code() {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let msgval = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgcode, msgval]);
        let msg = Message::from_msg(val).unwrap();
        let expected = msg.clone();
        let notice = Notice::from_msg(msg).unwrap();

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
    fn message_args() {
        // --------------------
        // GIVEN
        // --------------------
        // A request message

        // Create message
        let msgtype = Value::from(MessageType::Notification.to_number());
        let msgcode = Value::from(TestCode::One.to_number());
        let msgargs = Value::Array(vec![Value::from(42)]);

        let val = Value::Array(vec![msgtype, msgcode, msgargs.clone()]);
        let msg = Message::from_msg(val).unwrap();
        let notice = Notice::from_msg(msg).unwrap();

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


mod convert_bytes {
    // Stdlib imports

    // Third-party imports
    use bytes::BytesMut;
    use rmpv::Value;

    // Local imports

    use core::{AsBytes, FromBytes, FromBytesError, RpcMessage};

    // Helpers

    use test::core::decode;
    use super::{Notice, TestCode};

    #[test]
    fn serialize() {
        // --------------------
        // GIVEN
        // a valid NotificationMessage
        // --------------------
        let msgargs = vec![Value::from(9001)];
        let msg = Notice::new(TestCode::Three, msgargs);

        // --------------------
        // WHEN
        // NotificationMessage::as_bytes() is called
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
        // a valid NotificationMessage and
        // the message is serialized into msgpack
        // --------------------
        let msgargs = vec![Value::from(9001)];
        let msg = Notice::new(TestCode::One, msgargs);
        let expected = msg.clone();
        let mut msgpack = msg.as_bytes().try_mut().unwrap();

        // --------------------
        // WHEN
        // NotificationMessage::from_bytes() is called with the msgpack bytes
        // --------------------
        let result = Notice::from_bytes(&mut msgpack);

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
        // NotificationMessage::from_bytes() is called with the empty buffer
        // --------------------
        let result = Notice::from_bytes(&mut buf);

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
        // a valid NotificationMessage and
        // the message is serialized into msgpack bytes
        // and some bytes are discarded
        // --------------------
        let msgargs = vec![Value::from(9001)];
        let msg = Notice::new(TestCode::Two, msgargs);
        let mut msgpack = msg.as_bytes().try_mut().unwrap();

        // Make sure we have bytes
        assert!(!msgpack.is_empty());

        // Discard some bytes to make message bytes incomplete
        let size = msgpack.len() - 2;
        msgpack.truncate(size);

        // --------------------
        // WHEN
        // NotificationMessage::from_bytes() is called with the buffer
        // --------------------
        let result = Notice::from_bytes(&mut msgpack);

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
        // a valid NotificationMessage and
        // the message is serialized into msgpack bytes
        // and half of the bytes are discarded
        // --------------------
        let msgargs = vec![Value::from(9001)];
        let msg = Notice::new(TestCode::Three, msgargs);
        let mut msgpack = msg.as_bytes().try_mut().unwrap();

        // Make sure we have bytes
        assert!(!msgpack.is_empty());

        // Discard half of the bytes; this should cause an invalid marker error
        // for this specific request message
        let size = msgpack.len() / 2;
        msgpack.truncate(size);

        // --------------------
        // WHEN
        // NotificationMessage::from_bytes() is called with the buffer
        // --------------------
        let result = Notice::from_bytes(&mut msgpack);

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
