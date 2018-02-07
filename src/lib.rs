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
#[macro_use]
extern crate bitflags;
extern crate bytes;
extern crate chrono;

extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

#[cfg(test)]
#[macro_use]
extern crate proptest;

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

pub mod core;
pub mod future;
pub mod message;
pub mod util;

#[cfg(test)]
mod test;

// ===========================================================================
// Exports
// ===========================================================================

// Enums

pub use self::core::MessageType;

// Types

pub use self::core::Message;
// pub use self::core::notify::NotificationMessage;

pub use self::core::request::RequestMessage;
// pub use self::core::response::ResponseMessage;

// Traits

pub use self::core::{CodeConvert, RpcMessage, RpcMessageType};
// pub use self::core::notify::RpcNotice;

pub use self::core::request::RpcRequest;
// pub use self::core::response::RpcResponse;

// Derive
#[doc(hidden)]
pub use siminau_rpc_derive::*;

// ===========================================================================
//
// ===========================================================================
