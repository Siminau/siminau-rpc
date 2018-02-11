// src/test/core/request/rpcrequest.rs
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
use core::{CodeConvert, FromMessage, Message, MessageType, RpcMessage};
use core::request::{RequestMessage, RpcRequest};

// Helpers
use super::TestEnum;

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn message_id()
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
    let msg = Message::from_msg(val).unwrap();
    let expected = msg.clone();
    let req: RequestMessage<TestEnum> = RequestMessage::from_msg(msg).unwrap();

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
fn message_method()
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
    let msg = Message::from_msg(val).unwrap();
    let expected = msg.clone();
    let req: RequestMessage<TestEnum> = RequestMessage::from_msg(msg).unwrap();

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
fn message_args()
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
    let msg = Message::from_msg(val).unwrap();
    let expected = msg.clone();
    let req: RequestMessage<TestEnum> = RequestMessage::from_msg(msg).unwrap();

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

// ===========================================================================
//
// ===========================================================================
