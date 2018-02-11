// src/message/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports
// use byteorder::{NetworkEndian, WriteBytesExt};
use bytes::{BufMut, Bytes, BytesMut};
use serde::{Serialize, Serializer};

// Local imports
use core::{CodeConvert, CodeValueError};
// use core::notify::NotificationMessage;
// use core::request::{RequestMessage, RpcRequest};
// use core::response::ResponseMessage;

// ===========================================================================
// Modules
// ===========================================================================

pub mod v1;

// ===========================================================================
// Message envelope
// ===========================================================================

#[derive(Debug)]
pub struct Header
{
    pub category: u8,
    pub kind: u8,
}

impl Header
{
    pub fn from_bytes(b: &Bytes) -> Header
    {
        let slice = &b[..2];
        Header {
            category: slice[0],
            kind: slice[1],
        }
    }

    pub fn as_bytes(&self) -> Bytes
    {
        let mut buf = BytesMut::new();
        buf.put_u8(self.category);
        buf.put_u8(self.kind);
        buf.freeze()
    }
}

// Data will be bytes that will need to be deserialized to a
// Request/Response/Notification struct
#[derive(Debug)]
pub struct Envelope<D>
where
    D: Serialize,
{
    pub header: Header,
    pub data: D,
}

impl<D> Envelope<D>
where
    D: Serialize,
{
    pub fn from(header: Header, data: D) -> Envelope<D>
    {
        Envelope { header, data }
    }

    pub fn as_bytes<F, S>(&self, ser: F) -> Bytes
    where
        F: Fn(&mut Vec<u8>) -> S,
        S: Serializer,
    {
        // Serialize data into bytes
        let mut databuf = Vec::new();
        (&self.data).serialize(ser(&mut databuf)).unwrap();

        // Serialize header into bytes
        let header = &self.header;
        let mut buf = BytesMut::new();
        buf.put_u8(header.category);
        buf.put_u8(header.kind);

        // Add data bytes
        buf.extend_from_slice(&databuf[..]);
        buf.freeze()
    }

    pub fn header_bytes<B>(buf: B) -> Bytes
    where
        B: AsRef<[u8]>,
    {
        Bytes::from(&buf.as_ref()[..2])
    }
}

// ===========================================================================
// MessageKind
// ===========================================================================

// --------------------
// Requests
// --------------------

#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum AllRequestKind
{
    // Initiate client session by requesting an API version
    //
    // Single argument:
    // 1. Protocol version number to use
    Version = 2,
}

#[derive(Debug)]
pub enum RequestKind
{
    All(AllRequestKind),
    V1(v1::RequestKind),
}

// --------------------
// Responses
// --------------------

#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum AllResponseKind
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

#[derive(Debug)]
pub enum ResponseKind
{
    All(AllResponseKind),
    V1(v1::ResponseKind),
}

// --------------------
// Notifications
// --------------------

#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum AllNotifyKind
{
    // No more requests will be made
    //
    // No arguments
    Done = 0,
}

#[derive(Debug)]
pub enum NotifyKind
{
    All(AllNotifyKind),
    // V1(v1::NotifyKind),
}

// ===========================================================================
// All version request messages
// ===========================================================================

// --------------------
// version
// --------------------

// Request a specific protocol message version
//
// Single argument:
// 1. protocol message version number
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct VersionRequest
{
    pub id: u32,
    pub version: u32,
}

// ===========================================================================
// All version response messages
// ===========================================================================

// --------------------
// version
// --------------------

// Request a specific protocol message version
//
// Single argument:
// 1. protocol message version number that will be used
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct VersionResponse
{
    pub id: u32,
    pub version: u32,
}

// --------------------
// error
// --------------------

// Any error that is generated in response to a request.
//
// Single argument:
// 1. error message string
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ErrorResponse
{
    pub id: u32,
    pub error_msg: String,
}

// ===========================================================================
// Message enum
// ===========================================================================

#[derive(Debug, FromMessage, AsMessage, Serialize, Deserialize)]
pub enum RequestMessage
{
    VersionRequest(VersionRequest),
    V1(v1::RequestMessage),
}

#[derive(Debug, FromMessage, AsMessage, Serialize, Deserialize)]
pub enum ResponseMessage
{
    VersionResponse(VersionResponse),
    ErrorResponse(ErrorResponse),
    V1(v1::ResponseMessage),
}

// ===========================================================================
// New types
// ===========================================================================

// pub type Request = RequestMessage<RequestCode>;

// pub type Response = ResponseMessage<ResponseCode>;

// pub type Info = NotificationMessage<NotifyCode>;

// ===========================================================================
// Request builder
// ===========================================================================

// pub struct RequestBuilder
// {
//     id: u32,
// }

// impl RequestBuilder
// {
//     pub fn new(msgid: u32) -> RequestBuilder
//     {
//         RequestBuilder { id: msgid }
//     }

//     pub fn version(self, version_number: u32) -> Request
//     {
//         let ver = Value::from(version_number);
//         Request::new(self.id, RequestCode::Version, vec![ver])
//     }
// }

// pub fn request(msgid: u32) -> RequestBuilder
// {
//     RequestBuilder::new(msgid)
// }

// ===========================================================================
// Response builder
// ===========================================================================

// pub struct ResponseBuilder<'request>
// {
//     request: &'request Request,
// }

// impl<'request> ResponseBuilder<'request>
// {
//     pub fn new(request: &'request Request) -> ResponseBuilder
//     {
//         ResponseBuilder { request: request }
//     }

//     pub fn error(self, errmsg: &str) -> Response
//     {
//         let errmsg = Value::from(errmsg);
//         let msgid = self.request.message_id();
//         Response::new(msgid, ResponseCode::Error, errmsg)
//     }

//     pub fn version(self, num: u32) -> Response
//     {
//         let req = self.request;
//         match req.message_method() {
//             // If add any more variants to RequestCode, pls uncomment below
//             // _ => return BuildResponseError)
//             RequestCode::Version => {}
//         }

//         let num = Value::from(num);
//         let msgid = req.message_id();
//         Response::new(msgid, ResponseCode::Version, num)
//     }
// }

// pub fn response(request: &Request) -> ResponseBuilder
// {
//     ResponseBuilder::new(request)
// }

// ===========================================================================
// Info builder
// ===========================================================================

// pub struct InfoBuilder;

// impl InfoBuilder
// {
//     pub fn new() -> InfoBuilder
//     {
//         InfoBuilder
//     }

//     pub fn done(self) -> Info
//     {
//         Info::new(NotifyCode::Done, vec![])
//     }
// }

// pub fn info() -> InfoBuilder
// {
//     InfoBuilder::new()
// }

// ===========================================================================
//
// ===========================================================================
