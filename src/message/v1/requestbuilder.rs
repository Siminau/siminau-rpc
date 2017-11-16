// src/message/v1/requestbuilder.rs
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

use error::{RpcErrorKind, RpcResult};
use util::is_printable;

// Parent-module imports
use super::{OpenMode, Request, RequestCode};


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

    // TODO: allow restricting length of path vec
    //
    // Walk a directory hierarchy
    //
    // 3 arguments:
    // 1. existing file id
    // 2. new file id of the walk result
    // 3. list of path element strings to walk through
    pub fn walk(
        self, file_id: u32, newfile_id: u32, path: Vec<&str>
    ) -> RpcResult<Request>
    {
        // file_id cannot be the same value as newfile_id
        if file_id == newfile_id {
            let errmsg = format!(
                "invalid newfile_id value ({}): newfile_id \
                 has the same value as file_id",
                newfile_id
            );
            bail!(RpcErrorKind::InvalidRequestArgs(errmsg));
        }

        // Convert Vec<&str> into Vec<Value>
        let pathargs: Vec<Value> =
            path.iter().map(|i| Value::from(*i)).collect();

        // Construct msg args
        let msgargs = vec![
            Value::from(file_id),
            Value::from(newfile_id),
            Value::Array(pathargs),
        ];

        // Create request message
        let ret = Request::new(self.id, RequestCode::Walk, msgargs);
        Ok(ret)
    }

    // Prepare an existing file id for I/O
    //
    // 2 arguments:
    // 1. existing file id
    // 2. mode ie type of I/O
    pub fn open(self, file_id: u32, mode: OpenMode) -> Request
    {
        // Construct msg args
        let msgargs = vec![Value::from(file_id), Value::from(mode.bits())];

        // Create request message
        Request::new(self.id, RequestCode::Open, msgargs)
    }
}


pub fn request(msgid: u32) -> RequestBuilder
{
    RequestBuilder::new(msgid)
}


// ===========================================================================
//
// ===========================================================================
