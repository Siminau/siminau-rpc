// src/test/core/request/rpcmessage.rs
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
use core::request::RequestMessage;

// Helpers
use super::TestEnum;

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn as_vec()
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
fn as_value()
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
    // RequestMessage::as_value() method is called
    let result = req.as_value();

    // --------------------
    // THEN
    // --------------------
    // The contained value is as expected
    let expected = expected.as_value();
    assert_eq!(result, expected)
}

// ===========================================================================
//
// ===========================================================================
