// src/message/mod.rs
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

use core::{CodeConvert, CodeValueError};
use core::notify::NotificationMessage;
use core::request::{RequestMessage, RpcRequest};
use core::response::ResponseMessage;


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
pub enum RequestCode
{
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
pub enum ResponseCode
{
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
pub enum NotifyCode
{
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
// Request builder
// ===========================================================================


pub struct RequestBuilder
{
    id: u32,
}


impl RequestBuilder
{
    pub fn new(msgid: u32) -> RequestBuilder
    {
        RequestBuilder { id: msgid }
    }

    pub fn version(self, version_number: u32) -> Request
    {
        let ver = Value::from(version_number);
        Request::new(self.id, RequestCode::Version, vec![ver])
    }
}


pub fn request(msgid: u32) -> RequestBuilder
{
    RequestBuilder::new(msgid)
}


// ===========================================================================
// Response builder
// ===========================================================================


pub struct ResponseBuilder<'request>
{
    request: &'request Request,
}


impl<'request> ResponseBuilder<'request>
{
    pub fn new(request: &'request Request) -> ResponseBuilder
    {
        ResponseBuilder { request: request }
    }

    pub fn error(self, errmsg: &str) -> Response
    {
        let errmsg = Value::from(errmsg);
        let msgid = self.request.message_id();
        Response::new(msgid, ResponseCode::Error, errmsg)
    }

    pub fn version(self, num: u32) -> Response
    {
        let req = self.request;
        match req.message_method() {
            // If add any more variants to RequestCode, pls uncomment below
            // _ => return BuildResponseError)
            RequestCode::Version => {}
        }

        let num = Value::from(num);
        let msgid = req.message_id();
        Response::new(msgid, ResponseCode::Version, num)
    }
}


pub fn response(request: &Request) -> ResponseBuilder
{
    ResponseBuilder::new(request)
}


// ===========================================================================
// Info builder
// ===========================================================================


pub struct InfoBuilder;


impl InfoBuilder
{
    pub fn new() -> InfoBuilder
    {
        InfoBuilder
    }

    pub fn done(self) -> Info
    {
        Info::new(NotifyCode::Done, vec![])
    }
}


pub fn info() -> InfoBuilder
{
    InfoBuilder::new()
}



// ===========================================================================
//
// ===========================================================================
