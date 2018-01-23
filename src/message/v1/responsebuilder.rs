// src/message/v1/responsebuilder.rs
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

use core::request::RpcRequest;
use core::response::RpcResponse;

// Parent-module imports
use super::{FileID, FileKind, Request, RequestCode, Response, ResponseCode,
            Stat};

// ===========================================================================
// Errors
// ===========================================================================

// if code != expected {
//     let errmsg = format!(
//         "expected RequestCode::{:?}, got \
//          RequestCode::{:?} instead",
//         expected,
//         code
//     );

#[derive(Debug, Fail)]
pub enum BuildResponseError
{
    #[fail(display = "Unable to build response message: expected \
                      RequestCode::{:?}, got RequestCode::{:?} instead",
           expected, value)]
    WrongCode
    {
        value: RequestCode,
        expected: RequestCode,
    },

    #[fail(display = "Unable to build auth response message: file id has \
                      invalid kind {}",
           _0)]
    Auth(u8),

    #[fail(display = "Unable to build attach response message: rootfile_id \
                      has invalid kind {}",
           _0)]
    Attach(u8),

    #[fail(display = "Unable to build walk response message: item {} \
                      of path_id has invalid kind {}",
           index, kind)]
    Walk
    {
        index: usize, kind: u8
    },

    #[fail(display = "Unable to build open response message: file \
                      id has invalid kind {}",
           _0)]
    Open(u8),

    #[fail(display = "Unable to build create response message: file \
                      id has invalid kind {}",
           _0)]
    Create(u8),

    #[fail(display = "Unable to build create response message: bytes read \
                      ({}) does not match read count ({})",
           _0, _1)]
    Read(u32, usize),
}


impl BuildResponseError
{
    fn from_opencreate(tag: &OpenOrCreate, val: u8) -> BuildResponseError
    {
        match tag {
            &OpenOrCreate::Open => BuildResponseError::Open(val),
            &OpenOrCreate::Create => BuildResponseError::Create(val),
        }
    }
}


// ===========================================================================
// Response builder
// ===========================================================================


pub trait ProtocolResponse
{
    fn as_fileid(&self) -> Option<FileID>;
}


impl ProtocolResponse for Response
{
    fn as_fileid(&self) -> Option<FileID>
    {
        // The response must have a code of ResponseCode::Auth
        match self.error_code() {
            ResponseCode::Auth => {}
            _ => return None,
        }

        // The result must be an array containing 3 items
        let result = match self.result().as_array() {
            Some(val) if val.len() == 3 => val,
            _ => return None,
        };

        // Convert bits into FileKind
        let kind = match result[0].as_u64() {
            Some(v) if v <= u8::max_value() as u64 => {
                match FileKind::from_bits(v as u8) {
                    Some(kind) => kind,
                    None => return None,
                }
            }
            _ => return None,
        };

        // Ensure version is a u32
        let version = match result[1].as_u64() {
            Some(v) if v <= u32::max_value() as u64 => v as u32,
            _ => return None,
        };

        // Ensure path is a u64
        let path = match result[2].as_u64() {
            Some(v) => v,
            None => return None,
        };

        // Create a FileID
        Some(FileID::new(kind, version, path))
    }
}


enum OpenOrCreate
{
    Open,
    Create,
}


impl OpenOrCreate
{
    fn req_resp_code(&self) -> (RequestCode, ResponseCode)
    {
        match *self {
            OpenOrCreate::Open => (RequestCode::Open, ResponseCode::Open),
            OpenOrCreate::Create => (RequestCode::Create, ResponseCode::Create),
        }
    }
}


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

    // Private helper that validates that a request's method is as expected
    fn check_request_method(
        &self, expected: RequestCode
    ) -> Result<(), BuildResponseError>
    {
        let code = self.request.message_method();
        if code != expected {
            let err = BuildResponseError::WrongCode {
                value: code,
                expected: expected,
            };
            Err(err)
        } else {
            Ok(())
        }
    }

    // Auth init succeeded
    //
    // Single argument:
    // 1. Unique server identifier for the auth file
    pub fn auth(self, id: FileID) -> Result<Response, BuildResponseError>
    {
        // Make sure request message's code is RequestCode::Auth
        self.check_request_method(RequestCode::Auth)?;

        // Make sure given FileID is valid
        if !id.is_valid() {
            return Err(BuildResponseError::Auth(id.kind.bits()));
        }

        // Create file id response
        let fileid = vec![
            Value::from(id.kind.bits()),
            Value::from(id.version),
            Value::from(id.path),
        ];

        // Create response message
        let msgid = self.request.message_id();
        let ret =
            Response::new(msgid, ResponseCode::Auth, Value::Array(fileid));
        Ok(ret)
    }

    // Flush request succeeded
    //
    // No arguments
    pub fn flush(self) -> Result<Response, BuildResponseError>
    {
        // Make sure request message's code is RequestCode::Flush
        self.check_request_method(RequestCode::Flush)?;

        // Create response message
        let msgid = self.request.message_id();
        let ret = Response::new(msgid, ResponseCode::Flush, Value::Nil);
        Ok(ret)
    }

    // Attach request succeeded
    //
    // Single argument:
    // 1. Unique server identifier for the root directory
    pub fn attach(
        self, rootdir_id: FileID
    ) -> Result<Response, BuildResponseError>
    {
        // Make sure request message's code is RequestCode::Attach
        self.check_request_method(RequestCode::Attach)?;

        // Make sure given FileID is valid
        if !rootdir_id.is_valid() {
            return Err(BuildResponseError::Attach(rootdir_id.kind.bits()));
        }

        // Create file id response
        let fileid = vec![
            Value::from(rootdir_id.kind.bits()),
            Value::from(rootdir_id.version),
            Value::from(rootdir_id.path),
        ];

        // Create response message
        let msgid = self.request.message_id();
        let ret =
            Response::new(msgid, ResponseCode::Attach, Value::Array(fileid));
        Ok(ret)
    }

    // Walk request succeded
    //
    // Single argument:
    // 1. List of unique server identifiers for each path element specified in
    //    the request
    pub fn walk(
        self, path_id: &Vec<FileID>
    ) -> Result<Response, BuildResponseError>
    {
        // Make sure request message's code is RequestCode::Walk
        self.check_request_method(RequestCode::Walk)?;

        // Setup result vec
        let mut result: Vec<Value> = Vec::with_capacity(path_id.len());

        // Make sure all FileID objects in path_id are valid
        // and convert to values for message
        for (n, fid) in path_id.iter().enumerate() {
            if !fid.is_valid() {
                return Err(BuildResponseError::Walk {
                    index: n,
                    kind: fid.kind.bits(),
                });
            }

            // Create file id response
            let fileid = vec![
                Value::from(fid.kind.bits()),
                Value::from(fid.version),
                Value::from(fid.path),
            ];

            // Store file id in result vec
            result.push(Value::Array(fileid));
        }

        // Create response message
        let msgid = self.request.message_id();
        let ret =
            Response::new(msgid, ResponseCode::Walk, Value::Array(result));
        Ok(ret)
    }

    // Open or create request succeeded
    //
    // 2 arguments:
    // 1. Unique server identifier for the opened file
    // 2. Maximum number of bytes guaranteed to be read from or written to the
    //    file without a separate message. May be 0 which means no limit.
    fn open_or_create(
        self, tag: OpenOrCreate, file_id: FileID, max_size: u32
    ) -> Result<Response, BuildResponseError>
    {
        // Make sure request message's code matches tag
        let (req_tag, resp_tag) = tag.req_resp_code();
        self.check_request_method(req_tag)?;

        if !file_id.is_valid() {
            let val = file_id.kind.bits();
            let err = BuildResponseError::from_opencreate(&tag, val);
            return Err(err);
        }

        // Create file id response
        let fileid = vec![
            Value::from(file_id.kind.bits()),
            Value::from(file_id.version),
            Value::from(file_id.path),
        ];

        let result = vec![Value::Array(fileid), Value::from(max_size)];

        // Create response message
        let msgid = self.request.message_id();
        let ret = Response::new(msgid, resp_tag, Value::Array(result));
        Ok(ret)
    }

    // Open request succeeded
    //
    // 2 arguments:
    // 1. Unique server identifier for the opened file
    // 2. Maximum number of bytes guaranteed to be read from or written to the
    //    file without a separate message. May be 0 which means no limit.
    pub fn open(
        self, file_id: FileID, max_size: u32
    ) -> Result<Response, BuildResponseError>
    {
        self.open_or_create(OpenOrCreate::Open, file_id, max_size)
    }

    // Create request succeeded
    //
    // 2 arguments:
    // 1. Unique server identifier for the created file
    // 2. Maximum number of bytes guaranteed to be read from or written to the
    //    file without a separate message. May be 0 which means no limit.
    pub fn create(
        self, file_id: FileID, max_size: u32
    ) -> Result<Response, BuildResponseError>
    {
        self.open_or_create(OpenOrCreate::Create, file_id, max_size)
    }

    // Read request succeeded
    //
    // 2 arguments:
    // 1. Number of bytes read from the file
    // 2. List of bytes read from the file
    pub fn read<D>(
        self, count: u32, data: &D
    ) -> Result<Response, BuildResponseError>
    where
        D: AsRef<[u8]>,
    {
        // Make sure request message's code is RequestCode::Read
        self.check_request_method(RequestCode::Read)?;

        let bytes = data.as_ref();
        let numbytes = bytes.len();

        // The number of bytes read must match the value of count
        if count as u64 != numbytes as u64 {
            let err = BuildResponseError::Read(count, numbytes);
            return Err(err);
        }

        // Create args
        let msgargs = vec![Value::from(count), Value::Binary(bytes.into())];

        // Create message
        let msgid = self.request.message_id();
        let resp =
            Response::new(msgid, ResponseCode::Read, Value::Array(msgargs));
        Ok(resp)
    }

    // Write request succeeded
    //
    // Single argument:
    // 1. Number of bytes written to the file
    pub fn write(self, count: u32) -> Result<Response, BuildResponseError>
    {
        // Make sure request message's code is RequestCode::Write
        self.check_request_method(RequestCode::Write)?;

        // Create message
        let msgid = self.request.message_id();
        let resp =
            Response::new(msgid, ResponseCode::Write, Value::from(count));
        Ok(resp)
    }

    // Clunk request succeeded
    //
    // No arguments
    pub fn clunk(self) -> Result<Response, BuildResponseError>
    {
        // Make sure request message's code is RequestCode::Clunk
        self.check_request_method(RequestCode::Clunk)?;

        // Create message
        let msgid = self.request.message_id();
        let resp = Response::new(msgid, ResponseCode::Clunk, Value::Nil);
        Ok(resp)
    }

    // Remove request succeeded
    //
    // No arguments
    pub fn remove(self) -> Result<Response, BuildResponseError>
    {
        // Make sure request message's code is RequestCode::Remove
        self.check_request_method(RequestCode::Remove)?;

        // Create message
        let msgid = self.request.message_id();
        let resp = Response::new(msgid, ResponseCode::Remove, Value::Nil);
        Ok(resp)
    }

    // Stat request succeeded
    //
    // Single argument:
    // 1. list of values matching Stat struct members
    pub fn stat(self, stat: &Stat) -> Result<Response, BuildResponseError>
    {
        // Make sure request message's code is RequestCode::Stat
        self.check_request_method(RequestCode::Stat)?;

        // Create message
        let msgid = self.request.message_id();

        // Create fileid
        let stat_fileid = &stat.fileid;
        let fileid = vec![
            Value::from(stat_fileid.kind.bits()),
            Value::from(stat_fileid.version),
            Value::from(stat_fileid.path),
        ];

        // Create result
        let result = vec![
            Value::from(stat.size),
            Value::Array(fileid),
            Value::from(stat.mode),
            Value::from(stat.atime),
            Value::from(stat.mtime),
            Value::from(stat.length),
            Value::from(stat.name),
            Value::from(stat.uid),
            Value::from(stat.gid),
            Value::from(stat.muid),
        ];

        let resp =
            Response::new(msgid, ResponseCode::Stat, Value::Array(result));
        Ok(resp)
    }

    // pub fn version(self, num: u32) -> RpcResult<Response>
    // {
    //     let req = self.request;
    //     match req.message_method() {
    //         RequestCode::Version => {}

    //         // If add any more variants to RequestCode, pls uncomment below
    //         // _ => bail!(RpcErrorKind::InvalidRequest)
    //     }

    //     let num = Value::from(num);
    //     let msgid = req.message_id();
    //     let ret = Response::new(msgid, ResponseCode::Version, num);
    //     Ok(ret)
    // }
}


pub fn response(request: &Request) -> ResponseBuilder
{
    ResponseBuilder::new(request)
}


// ===========================================================================
//
// ===========================================================================
