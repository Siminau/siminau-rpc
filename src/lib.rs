// src/lib.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

//! Types and traits for working with a type of MessagePack RPC
//!
//! The [`Message`] type is the core underlying type that wraps around the
//! [`rmpv::Value`] type. It ensures that the given [`rmpv::Value`] object
//! conforms with the expected RPC spec.
//!
//! [`Message`]: message/struct.Message.html
//! [`rmpv::Value`]: https://docs.rs/rmpv/0.4.0/rmpv/enum.Value.html
//!
//! # MessagePack RPC
//!
//! The [`msgpack-rpc`] spec is mostly followed with a single exception: the
//! method argument of Request and Notification messages is not a string but
//! instead an integer. Since one goal in safesec is to ensure that all public
//! interfaces have strict type and value validation, an integer that could be
//! mapped to a C-style enum made better sense that using an arbitrary string.
//!
//! [`msgpack-rpc`]: https://github.com/msgpack-rpc/msgpack-rpc/blob/master/spec.md

// ===========================================================================
// Externs
// ===========================================================================
#![recursion_limit = "1024"]

// Stdlib externs

// Third-party externs
extern crate bytes;

#[macro_use]
extern crate error_chain;

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate rmp_serde as rmps;
extern crate rmpv;
extern crate serde;

// Local externs

#[macro_use]
extern crate siminau_rpc_derive;


// ===========================================================================
// Modules
// ===========================================================================

// General errors
pub mod error {

    error_chain!{
        types {
            RpcError, RpcErrorKind, RpcResultExt, RpcResult;
        }

        errors {
            // --------------------
            // Message
            // --------------------
            TypeError(t: String) {
                description("invalid type")
                display("Invalid type: {}", t)
            }

            ValueError(v: String) {
                description("invalid value")
                display("Invalid value: {}", v)
            }

            InvalidMessage(m: String) {
                description("invalid message")
                display("Invalid message: {}", m)
            }

            InvalidArrayLength(v: String) {
                description("Invalid message array length")
                display("Invalid message array length: {}", v)
            }

            InvalidMessageType(t: String) {
                description("Invalid message type")
                display("Invalid message type: {}", t)
            }

            // --------------------
            // Request
            // --------------------
            InvalidRequest {
                description("Invalid request message")
                display("Invalid request message")
            }

            InvalidRequestID {
                description("invalid request id")
                display("Invalid request ID")
            }

            InvalidRequestMethod(m: String) {
                description("Invalid request method")
                display("Invalid request method: {}", m)
            }

            InvalidRequestArgs(m: String) {
                description("Invalid request arguments")
                display("Invalid request arguments: {}", m)
            }

            // --------------------
            // Response
            // --------------------
            InvalidResponse {
                description("Invalid response message")
                display("Invalid response message")
            }

            InvalidResponseID {
                description("invalid response id")
                display("Invalid response ID")
            }

            InvalidResponseError(m: String) {
                description("Invalid response error")
                display("Invalid response error: {}", m)
            }

            // --------------------
            // Notification
            // --------------------
            InvalidNotification {
                description("Invalid notification message")
                display("Invalid notification message")
            }

            InvalidNotificationCode(m: String) {
                description("Invalid notification code")
                display("Invalid notification code: {}", m)
            }

            InvalidNotificationArgs(m: String) {
                description("Invalid notification arguments")
                display("Invalid notification arguments: {}", m)
            }

            // --------------------
            // Misc
            // --------------------
            InvalidData {
                description("invalid data")
                display("Invalid data")
            }

            UnexpectedMessage {
                description("unexpected message")
                display("Unexpected message")
            }

        }
    }
}


pub mod codec;
pub mod message;
pub mod server;


// ===========================================================================
// Exports
// ===========================================================================


// Enums

pub use self::message::MessageType;

// Types

pub use self::message::Message;
// pub use self::message::notify::NotificationMessage;

pub use self::message::request::RequestMessage;
// pub use self::message::response::ResponseMessage;

// Traits

pub use self::message::{CodeConvert, RpcMessage, RpcMessageType};
// pub use self::message::notify::RpcNotice;

pub use self::message::request::RpcRequest;
// pub use self::message::response::RpcResponse;


// ===========================================================================
//
// ===========================================================================
