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

use util::is_printable;

// Parent-module imports
use super::{OpenMode, Request, RequestCode};


// ===========================================================================
// Helper
// ===========================================================================


#[derive(Debug, Fail)]
pub enum CheckNameError
{
    #[fail(display = "{} is either empty, or contains control characters", _0)]
    WSPrintable(String),

    #[fail(display = "{} is either empty, contains whitespace, or contains \
                      control characters",
           _0)]
    WSNotPrintable(String),
}


fn check_name(
    var: &str, name: &str, ws_printable: bool
) -> Result<(), CheckNameError>
{
    // Name must not be empty and must not have any control characters
    if !is_printable(name, ws_printable) {
        let err = if ws_printable {
            CheckNameError::WSPrintable(var.to_owned())
        } else {
            CheckNameError::WSNotPrintable(var.to_owned())
        };
        return Err(err);
    }

    Ok(())
}


// ===========================================================================
// Request builder errors
// ===========================================================================


#[derive(Debug, Fail)]
pub enum BuildAttachError
{
    #[fail(display = "Name error: {}", _0)] NameError(#[cause] CheckNameError),

    #[fail(display = "Invalid rootdir_id value ({}): rootdir_id matches \
                      authfile_id",
           _0)]
    MatchingID(u32),
}


#[derive(Debug, Fail)]
pub enum BuildRequestError
{
    #[fail(display = "Unable to build auth request message")]
    Auth(#[cause] CheckNameError),

    #[fail(display = "Unable to build flush request message: prev msg id \
                      ({}) matches current msg id",
           _0)]
    Flush(u32),

    #[fail(display = "Unable to build attach request message")]
    Attach(#[cause] BuildAttachError),

    #[fail(display = "Unable to build walk request message: newfile_id ({}) \
                      has the same value as file_id",
           _0)]
    Walk(u32),

    #[fail(display = "Unable to build create request message")]
    Create(#[cause] CheckNameError),
}


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

    // Setup client authentication file.
    //
    // 3 arguments:
    // 1. file id of the auth file
    // 2. user name
    // 3. service name
    pub fn auth(
        self, authfile_id: u32, username: &str, fsname: &str
    ) -> Result<Request, BuildRequestError>
    {
        check_name("username", username, false)
            .map_err(|e| BuildRequestError::Auth(e))?;
        check_name("filesystem name", fsname, false)
            .map_err(|e| BuildRequestError::Auth(e))?;

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
    pub fn flush(self, prev_msgid: u32) -> Result<Request, BuildRequestError>
    {
        if prev_msgid == self.id {
            return Err(BuildRequestError::Flush(prev_msgid));
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
    ) -> Result<Request, BuildRequestError>
    {
        if rootdir_id == authfile_id {
            let err = BuildAttachError::MatchingID(rootdir_id);
            return Err(BuildRequestError::Attach(err));
        }

        check_name("username", username, false).map_err(|e| {
            BuildRequestError::Attach(BuildAttachError::NameError(e))
        })?;
        check_name("filesystem name", fsname, false).map_err(|e| {
            BuildRequestError::Attach(BuildAttachError::NameError(e))
        })?;

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
    ) -> Result<Request, BuildRequestError>
    {
        // file_id cannot be the same value as newfile_id
        if file_id == newfile_id {
            return Err(BuildRequestError::Walk(newfile_id));
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

    // Create a file and open it for I/O
    //
    // 3 arguments:
    // 1. existing file id
    // 2. name of the new file
    // 3. mode ie type of I/O
    pub fn create(
        self, file_id: u32, filename: &str, mode: OpenMode
    ) -> Result<Request, BuildRequestError>
    {
        check_name("filename", filename, false)
            .map_err(|e| BuildRequestError::Create(e))?;

        // Construct msg args
        let msgargs = vec![
            Value::from(file_id),
            Value::from(filename),
            Value::from(mode.bits()),
        ];

        // Create request message
        let ret = Request::new(self.id, RequestCode::Create, msgargs);
        Ok(ret)
    }
}


pub fn request(msgid: u32) -> RequestBuilder
{
    RequestBuilder::new(msgid)
}


// ===========================================================================
//
// ===========================================================================
