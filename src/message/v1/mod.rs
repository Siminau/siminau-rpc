// src/message/v1/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Modules
// ===========================================================================

pub mod request;
pub mod response;
// mod requestbuilder;
// mod responsebuilder;
mod util;

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

// Local imports

use core::{CodeConvert, CodeValueError};
// use core::request::RequestMessage;
// use core::response::ResponseMessage;

// Re-exports
// pub use self::requestbuilder::{request, BuildRequestError, RequestBuilder};
// pub use self::responsebuilder::{response, BuildResponseError, ResponseBuilder};
pub use self::util::{openmode, FileID, FileKind, OpenFlag, OpenKind, OpenMode,
                     OpenModeError};

// ===========================================================================
// Message codes
// ===========================================================================

#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum RequestKind
{
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
pub enum ResponseKind
{
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

// pub type Request = RequestMessage<RequestCode>;

// pub type Response = ResponseMessage<ResponseCode>;

// ===========================================================================
//
// ===========================================================================
