// src/message/v1.rs
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

use core::CodeConvert;
use core::request::{RequestMessage, RpcRequest};
use core::response::{ResponseMessage, RpcResponse};
use error::{RpcErrorKind, RpcResult};
use util::is_printable;

// ===========================================================================
// Server File ID
// ===========================================================================


bitflags! {
    pub struct FileKind: u8 {
        const DIR =     0b10000000;
        const APPEND =  0b01000000;
        const EXCL =    0b00100000;
        const AUTH =    0b00010000;
        const TMP =     0b00001000;
        const FILE =    0b00000000;
    }
}


impl FileKind {
    pub fn is_valid(&self) -> bool
    {
        let invalid = vec![
            FileKind::DIR | FileKind::AUTH,
            FileKind::DIR | FileKind::APPEND,
        ];

        // Return false if any invalid bits are found in filekind
        !invalid.iter().any(|i| self.intersects(*i))
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct FileID {
    pub kind: FileKind,
    pub version: u32,
    pub path: u64,
}


impl FileID {
    pub fn new(kind: FileKind, version: u32, path: u64) -> FileID
    {
        FileID {
            kind: kind,
            version: version,
            path: path,
        }
    }

    pub fn is_valid(&self) -> bool
    {
        self.kind.is_valid()
    }
}


impl Default for FileID {
    fn default() -> FileID
    {
        FileID::new(FileKind::FILE, 0, 0)
    }
}


// ===========================================================================
// Message codes
// ===========================================================================


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum RequestCode {
    // Setup client authentication file.
    //
    // 3 arguments:
    // 1. file id of the auth file
    // 2. user name
    // 3. service name
    Auth = 4,

    // Request to abort a previous request if it hasn't been processed yet.
    //
    // Single argument:
    // 1. message id of the previous request
    Flush = 6,

    // Attach to the root directory of a given service.
    //
    // The auth file id is assumed to have been setup previously via a preceding
    // Auth request.
    //
    // 4 arguments:
    // 1. file id of the root directory
    // 2. file id of the auth file
    // 3. user name
    // 4. service name
    Attach = 8,

    // Walk a directory hierarchy
    //
    // 3 arguments:
    // 1. existing file id
    // 2. new file id of the walk result
    // 3. list of path element strings to walk through
    Walk = 10,

    // Prepare an existing file id for I/O
    //
    // 2 arguments:
    // 1. existing file id
    // 2. mode ie type of I/O
    Open = 12,

    // Create a file and open it for I/O
    //
    // 3 arguments:
    // 1. existing file id
    // 2. name of the new file
    // 3. mode ie type of I/O
    Create = 14,

    // Request for a number of bytes from a file
    //
    // 3 arguments:
    // 1. existing file id
    // 2. starting offset
    // 3. number of bytes to return
    Read = 16,

    // Request that a number of bytes be recorded to a file
    //
    // 4 arguments:
    // 1. existing file id
    // 2. starting offset
    // 3. number of bytes to write
    // 4. list of bytes
    Write = 18,

    // Forget a file id
    //
    // Single argument:
    // 1. existing file id
    Clunk = 20,

    // Remove a file from the server
    //
    // Single argument:
    // 1. existing file id
    Remove = 22,

    // Retrieve file attributes
    //
    // Single argument:
    // 1. existing file id
    Stat = 24,

    // Change file attributes
    //
    // 2 arguments:
    // 1. existing file id
    // 2. map of new file attributes to save to the file
    WStat = 26,
}


// --------------------
// Responses
// --------------------

#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum ResponseCode {
    // Auth init succeeded
    //
    // Single argument:
    // 1. Unique server identifier for the auth file
    Auth = 5,

    // Flush request succeeded
    //
    // No arguments
    Flush = 7,

    // Attach request succeeded
    //
    // Single argument:
    // 1. Unique server identifier for the root directory
    Attach = 9,

    // Walk request succeded
    //
    // Single argument:
    // 1. List of unique server identifiers for each path element specified in
    //    the request
    Walk = 11,

    // Open request succeeded
    //
    // 2 arguments:
    // 1. Unique server identifier for the opened file
    // 2. Maximum number of bytes guaranteed to be read from or written to the
    //    file without a separate message. May be 0 which means no limit.
    Open = 13,

    // Create request succeeded
    //
    // 2 arguments:
    // 1. Unique server identifier for the created file
    // 2. Maximum number of bytes guaranteed to be read from or written to the
    //    file without a separate message. May be 0 which means no limit.
    Create = 15,

    // Read request succeeded
    //
    // 2 arguments:
    // 1. Number of bytes read from the file
    // 2. List of bytes read from the file
    Read = 17,

    // Write request succeeded
    //
    // Single argument:
    // 1. Number of bytes written to the file
    Write = 19,

    // Clunk request succeeded
    //
    // No arguments
    Clunk = 21,

    // Remove request succeeded
    //
    // No arguments
    Remove = 23,

    // Stat request succeeded
    //
    // Single argument:
    // 1. map of file attributes
    Stat = 25,

    // Write stat request succeeded
    //
    // No arguments
    WStat = 27,
}


// ===========================================================================
// New types
// ===========================================================================


pub type Request = RequestMessage<RequestCode>;


pub type Response = ResponseMessage<ResponseCode>;


// ===========================================================================
// Request builder
// ===========================================================================


pub struct RequestBuilder {
    id: u32,
}


fn check_name(var: &str, name: &str, ws_printable: bool) -> RpcResult<()>
{
    // Name must not be empty and must not have any control characters
    if !is_printable(name, ws_printable) {
        let errmsg = if ws_printable {
            format!(
                "{} is either empty, or contains control characters: {}",
                var,
                name
            )
        } else {
            format!(
                "{} is either empty, contains whitespace, or contains control \
                 characters: {}",
                var,
                name
            )
        };
        bail!(RpcErrorKind::InvalidRequestArgs(errmsg));
    }

    Ok(())
}


impl RequestBuilder {
    pub fn new(msgid: u32) -> RequestBuilder
    {
        RequestBuilder { id: msgid }
    }

    // Setup client authentication file.
    //
    // 3 arguments:
    // 1. file id of the auth file
    // 2. user name
    // 3. service name
    pub fn auth(
        self, authfile_id: u32, username: &str, fsname: &str
    ) -> RpcResult<Request>
    {
        check_name("username", username, false)?;
        check_name("filesystem name", fsname, false)?;

        // Create arguments
        let fileid = Value::from(authfile_id);
        let username = Value::from(username);
        let fsname = Value::from(fsname);
        let msgargs = vec![fileid, username, fsname];

        // Create request message
        let ret = Request::new(self.id, RequestCode::Auth, msgargs);
        Ok(ret)
    }

    // Request to abort a previous request if it hasn't been processed yet.
    //
    // Single argument:
    // 1. message id of the previous request
    pub fn flush(self, prev_msgid: u32) -> RpcResult<Request>
    {
        if prev_msgid == self.id {
            let errmsg = format!(
                "invalid argument ({}): prev msg id matches current \
                 msg id",
                prev_msgid
            );
            bail!(RpcErrorKind::InvalidRequestArgs(errmsg));
        }

        // Create argument
        let msgargs = vec![Value::from(prev_msgid)];
        let ret = Request::new(self.id, RequestCode::Flush, msgargs);
        Ok(ret)
    }

    // Attach to the root directory of a given service.
    //
    // The auth file id is assumed to have been setup previously via a preceding
    // Auth request.
    //
    // 4 arguments:
    // 1. file id of the root directory
    // 2. file id of the auth file
    // 3. user name
    // 4. service name
    pub fn attach(
        self, rootdir_id: u32, authfile_id: u32, username: &str, fsname: &str
    ) -> RpcResult<Request>
    {
        if rootdir_id == authfile_id {
            let errmsg = format!(
                "invalid rootdir_id value ({}): rootdir_id and authfile_id \
                 must have different id numbers",
                rootdir_id
            );
            bail!(RpcErrorKind::InvalidRequestArgs(errmsg));
        }

        check_name("username", username, false)?;
        check_name("filesystem name", fsname, false)?;

        // Create request message
        let msgargs = vec![
            Value::from(rootdir_id),
            Value::from(authfile_id),
            Value::from(username),
            Value::from(fsname),
        ];
        let ret = Request::new(self.id, RequestCode::Attach, msgargs);
        Ok(ret)
    }
}


pub fn request(msgid: u32) -> RequestBuilder
{
    RequestBuilder::new(msgid)
}


// ===========================================================================
// Response builder
// ===========================================================================


pub trait ProtocolResponse {
    fn as_fileid(&self) -> Option<FileID>;
}


impl ProtocolResponse for Response {
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


pub struct ResponseBuilder<'request> {
    request: &'request Request,
}


impl<'request> ResponseBuilder<'request> {
    pub fn new(request: &'request Request) -> ResponseBuilder
    {
        ResponseBuilder { request: request }
    }

    // Private helper that validates that a request's method is as expected
    fn check_request_method(&self, expected: RequestCode) -> RpcResult<()>
    {
        let code = self.request.message_method();
        if code != expected {
            let errmsg = format!(
                "expected RequestCode::{:?}, got \
                 RequestCode::{:?} instead",
                expected,
                code
            );
            bail!(RpcErrorKind::InvalidRequestMethod(errmsg));
        }

        Ok(())
    }

    // Auth init succeeded
    //
    // Single argument:
    // 1. Unique server identifier for the auth file
    pub fn auth(self, id: FileID) -> RpcResult<Response>
    {
        // Make sure request message's code is RequestCode::Auth
        self.check_request_method(RequestCode::Auth)?;

        // Make sure given FileID is valid
        if !id.is_valid() {
            bail!("id contains invalid FileKind");
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
    pub fn flush(self) -> RpcResult<Response>
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
    pub fn attach(self, rootdir_id: FileID) -> RpcResult<Response>
    {
        // Make sure request message's code is RequestCode::Attach
        self.check_request_method(RequestCode::Attach)?;

        // Make sure given FileID is valid
        if !rootdir_id.is_valid() {
            let errmsg = "rootdir server id contains invalid FileKind";
            bail!(RpcErrorKind::ValueError(errmsg.to_owned()));
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
