// src/message/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

// Local imports

use core::CodeConvert;
use core::notify::NotificationMessage;
use core::request::RequestMessage;
use core::response::ResponseMessage;
use error::{RpcErrorKind, RpcResult};


// ===========================================================================
// Modules
// ===========================================================================


pub mod v1;


// ===========================================================================
// Message codes
// ===========================================================================

// --------------------
// Requests
// --------------------

#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum RequestCode {
    // Initiate client session by requesting an API version
    //
    // Single argument:
    // 1. Protocol version number to use
    Version = 2,
}


// --------------------
// Responses
// --------------------

#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum ResponseCode {
    // Any error that is generated in response to a request.
    //
    // Single argument:
    // 1. error message string
    Error = 1,

    // Response to client session request if the Version request did not
    // generate an error.
    //
    // Single argument:
    // 1. Protocol version number that will be used
    Version = 3,
}


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum NotifyCode {
    // No more requests will be made
    //
    // No arguments
    Done = 0,
}


// ===========================================================================
// New types
// ===========================================================================


pub type Request = RequestMessage<RequestCode>;


pub type Response = ResponseMessage<ResponseCode>;


pub type Info = NotificationMessage<NotifyCode>;


// ===========================================================================
//
// ===========================================================================
