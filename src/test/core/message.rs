// src/test/core/message.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

use rmpv::Value;

// Local imports

use core::{value_type, FromMessage, Message, ToMessageError};

// Helpers
use super::TestEnum;

// ===========================================================================
// Tests
// ===========================================================================

// If a non-Value::Array is stored then will always return an error
#[test]
fn non_array_always_err()
{
    let v = Value::from(42);
    let errmsg = format!("expected array but got {}", value_type(&v));
    let ret = match Message::from_msg(v) {
        Err(e @ ToMessageError::NotArray(_)) => errmsg == e.to_string(),
        _ => false,
    };
    assert!(ret)
}

// ===========================================================================
// Modules
// ===========================================================================

mod from
{

    // Stdlib imports

    // Third-party imports

    use failure::Fail;
    use quickcheck::TestResult;
    use rmpv::Value;

    // Local imports

    use core::{CodeConvert, FromMessage, Message, MessageType, RpcMessage,
               ToMessageError};

    quickcheck! {
        fn invalid_array_length(val: Vec<u8>) -> TestResult {
            let arraylen = val.len();
            if arraylen == 3 || arraylen == 4 {
                return TestResult::discard()
            }

            // GIVEN
            // an array with length either < 3 or > 4
            let valvec: Vec<Value> = val.iter()
                .map(|v| Value::from(v.clone())).collect();
            let array = Value::from(valvec);

            // WHEN
            // creating a message using from method
            let result = Message::from_msg(array);

            // THEN
            // an appropriate error is returned
            let errmsg = format!("expected array length of either 3 or 4, got {}",
                                 arraylen);
            let val = match result {
                Err(e @ ToMessageError::ArrayLength(_)) => {
                    errmsg == e.to_string()
                },
                _ => false
            };
            TestResult::from_bool(val)
        }

        fn invalid_messagetype_number(code: u64) -> TestResult {
            let maxval = MessageType::max_number() as u64;
            if code <= maxval {
                return TestResult::discard()
            }

            // GIVEN
            // array with invalid code number (ie code number is >
            // u8::max_value()
            let array: Vec<Value> = vec![code, 42, 42].iter()
                .map(|v| Value::from(v.clone())).collect();

            // WHEN
            // creating a message via Message::from_msg()
            let cause_errmsg = format!("Expected value <= 2 but got value {}", code);
            let result = Message::from_msg(Value::from(array));

            // THEN
            // MessageError::InvalidType error is returned
            let val = match result {
                Err(e @ ToMessageError::InvalidType(_)) => {

                    // Check error
                    let ret = e.to_string() == "Invalid message type";

                    // Get cause error
                    let cause = e.cause().unwrap();

                    // No further causes
                    let ret = ret && cause.cause().is_none();

                    // Check cause message
                    let expected = cause.to_string() == cause_errmsg;

                    // Return result of checks
                    ret && expected
                }
                _ => false
            };
            TestResult::from_bool(val)
        }
    }

    // A valid value is an array with a length of 3 or 4 and the first item in
    // the array is u8 that is < 3
    #[test]
    fn message_from_valid_value()
    {
        let valvec: Vec<Value> = vec![1, 42, 42]
            .iter()
            .map(|v| Value::from(v.clone()))
            .collect();
        let array = Value::from(valvec);
        let expected = array.clone();

        let ret = match Message::from_msg(array) {
            Ok(m) => m.as_value() == &expected,
            _ => false,
        };
        assert!(ret)
    }
}

mod message_type
{
    // std lib imports

    // Third-party imports

    use quickcheck::TestResult;
    use rmpv::Value;

    // Local imports

    use core::{CodeConvert, FromMessage, Message, MessageType, RpcMessage};

    // Helpers
    fn mkmessage(msgtype: u8) -> Message
    {
        let msgtype = Value::from(msgtype);
        let msgid = Value::from(0);
        let msgcode = Value::from(0);
        let msgargs = Value::Nil;
        let val = Value::from(vec![msgtype, msgid, msgcode, msgargs]);
        Message::from_msg(val).unwrap()
    }

    quickcheck! {
        // Known code number returns MessageType variant
        fn good_code_number(varnum: u8) -> TestResult {
            if varnum >= 3 {
                return TestResult::discard()
            }
            let expected = MessageType::from_number(varnum).unwrap();
            let msg = mkmessage(varnum);
            TestResult::from_bool(msg.message_type() == expected)
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

    use core::{AsBytes, FromBytes, FromBytesError, FromMessage, Message,
               RpcMessage};
    use core::request::RequestMessage;

    // Helpers

    use super::TestEnum;
    use test::core::decode;

    type Request = RequestMessage<TestEnum>;

    #[test]
    fn serialize()
    {
        // --------------------
        // GIVEN
        // a valid Message
        //
        // A valid message is a Value that is an array with a length of 3 or 4
        // and the first item in the array is u8 that is < 3
        // --------------------
        let valvec: Vec<Value> = vec![1, 42, 42]
            .iter()
            .map(|v| Value::from(v.clone()))
            .collect();
        let array = Value::from(valvec);
        let msg = Message::from_msg(array).unwrap();

        // --------------------
        // WHEN
        // Message::as_bytes() is called
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
        // a valid Message and
        // the message is serialized into msgpack
        //
        // A valid message is a Value that is an array with a length of 3 or 4
        // and the first item in the array is u8 that is < 3
        // --------------------
        let valvec: Vec<Value> = vec![1, 42, 42]
            .iter()
            .map(|v| Value::from(v.clone()))
            .collect();
        let array = Value::from(valvec);
        let msg = Message::from_msg(array).unwrap();
        let expected = msg.clone();
        let mut msgpack = msg.as_bytes().try_mut().unwrap();

        // --------------------
        // WHEN
        // Message::from_bytes() is called with the msgpack bytes
        // --------------------
        let result = Message::from_bytes(&mut msgpack);

        // --------------------
        // THEN
        // the a message object is returned and
        // the msg is equal to the original message
        // --------------------
        match result {
            Ok(Some(msg)) => assert_eq!(msg, expected),
            _ => assert!(false),
        }
        // let mut buf = result.try_mut().unwrap();
        // let expected = decode(&mut buf).unwrap();
        // assert_eq!(&expected, msg.as_value());
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
        // Message::from_bytes() is called with the empty buffer
        // --------------------
        let result = Message::from_bytes(&mut buf);

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
        // a valid RequestMessage converted into a Message and
        // the message is serialized into msgpack bytes
        // and some bytes are discarded
        // --------------------
        let msgargs = vec![Value::from(9001)];
        let req = Request::new(42, TestEnum::One, msgargs);
        let msg: Message = req.into();
        let mut msgpack = msg.as_bytes().try_mut().unwrap();

        // Make sure we have bytes
        assert!(!msgpack.is_empty());

        // Discard some bytes to make message bytes incomplete
        let size = msgpack.len() - 2;
        msgpack.truncate(size);

        // --------------------
        // WHEN
        // Message::from_bytes() is called with the buffer
        // --------------------
        let result = Message::from_bytes(&mut msgpack);

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
        // a valid RequestMessage converted into a Message and
        // the message is serialized into msgpack bytes
        // and half of the bytes are discarded
        // --------------------
        let msgargs = vec![Value::from(9001)];
        let req = Request::new(42, TestEnum::One, msgargs);
        let msg: Message = req.into();
        let mut msgpack = msg.as_bytes().try_mut().unwrap();

        // Make sure we have bytes
        assert!(!msgpack.is_empty());

        // Discard half of the bytes; this should cause an invalid marker error
        // for this specific request message
        let size = msgpack.len() / 2;
        msgpack.truncate(size);

        // --------------------
        // WHEN
        // Message::from_bytes() is called with the buffer
        // --------------------
        let result = Message::from_bytes(&mut msgpack);

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

// TODO: evaluate if need RpcMessage::value_type_name
// mod value_type_name
// {
//     // std lib imports

//     // Third-party imports

//     use quickcheck::TestResult;
//     use rmpv::{Integer, Utf8String, Value};

//     // Local imports

//     use core::{Message, RpcMessage};

//     // Helpers

//     quickcheck! {

//         // Return value is never the empty string
//         fn return_nonempty_string(i: usize) -> TestResult {
//             let values = vec![
//                 Value::Nil,
//                 Value::Boolean(true),
//                 Value::Integer(Integer::from(42)),
//                 Value::F32(42.0),
//                 Value::F64(42.0),
//                 Value::String(Utf8String::from("hello")),
//                 Value::Binary(vec![0, 0]),
//                 Value::Array(vec![Value::from(42)]),
//                 Value::Map(vec![(Value::from(42), Value::from("ANSWER"))]),
//                 Value::Ext(-42, vec![0, 1, 2]),
//             ];

//             if i > values.len() - 1 {
//                 return TestResult::discard()
//             }

//             let choice = &values[i];
//             let ret = Message::value_type_name(choice);
//             TestResult::from_bool(ret.len() > 0)
//         }

//         // Return value is expected name of the Value variant
//         fn return_expected_string(i: usize) -> TestResult {
//             let values = vec![
//                 (Value::Nil, "nil"),
//                 (Value::Boolean(true), "bool"),
//                 (Value::Integer(Integer::from(42)), "int"),
//                 (Value::F32(42.0), "float32"),
//                 (Value::F64(42.0), "float64"),
//                 (Value::String(Utf8String::from("hello")), "str"),
//                 (Value::Binary(vec![0, 0]), "bytearray"),
//                 (Value::Array(vec![Value::from(42)]), "array"),
//                 (Value::Map(vec![(Value::from(42), Value::from("ANSWER"))]), "map"),
//                 (Value::Ext(-42, vec![0, 1, 2]), "ext"),
//             ];

//             if i > values.len() - 1 {
//                 return TestResult::discard()
//             }

//             let choice = &values[i];
//             let ret = Message::value_type_name(&choice.0);
//             TestResult::from_bool(ret == choice.1)
//         }
//     }
// }

// ===========================================================================
//
// ===========================================================================
