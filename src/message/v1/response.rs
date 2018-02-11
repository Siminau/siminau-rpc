// src/message/v1/response.rs
// Copyright (C) 2018 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

// Third-party imports

// Local imports

// ===========================================================================
//
// ===========================================================================

// --------------------
// General
// --------------------

// This is essentially message::v1::FileID
#[derive(Debug, Deserialize, Serialize)]
pub struct FileID
{
    pub kind: u8,
    pub version: u32,
    pub path: u64,
}

// --------------------
// init
// --------------------

// Response to client session request if the Version request did not
// generate an error.
//
// Single argument:
// 1. Protocol version number that will be used
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct InitResponse
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
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse
{
    pub id: u32,
    pub error_msg: String,
}

// --------------------
// auth
// --------------------

// Auth init succeeded
//
// Single argument:
// 1. Unique server identifier for the auth file
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse
{
    pub id: u32,
    pub server_id: FileID,
}

// --------------------
// flush
// --------------------
// Flush request succeeded
//
// No arguments
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct FlushResponse
{
    pub id: u32,
}

// --------------------
// attach
// --------------------

// Attach request succeeded
//
// Single argument:
// 1. Unique server identifier for the root directory
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct AttachResponse
{
    pub id: u32,
    pub rootdir_id: FileID,
}

// --------------------
// walk
// --------------------
// Walk request succeded
//
// Single argument:
// 1. List of unique server identifiers for each path element specified in
//    the request
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct WalkResponse
{
    pub id: u32,
    pub path_id: Vec<FileID>,
}

// --------------------
// open
// --------------------

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenCreateData
{
    pub file_id: FileID,
    pub max_size: u32,
}

// Open request succeeded
//
// 2 arguments:
// 1. Unique server identifier for the opened file
// 2. Maximum number of bytes guaranteed to be read from or written to the
//    file without a separate message. May be 0 which means no limit.
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenResponse
{
    pub id: u32,
    pub args: OpenCreateData,
}

// --------------------
// create
// --------------------

// Create request succeeded
//
// 2 arguments:
// 1. Unique server identifier for the created file
// 2. Maximum number of bytes guaranteed to be read from or written to the
//    file without a separate message. May be 0 which means no limit.
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateResponse
{
    pub id: u32,
    pub args: OpenCreateData,
}

// --------------------
// read
// --------------------

#[derive(Debug, Deserialize, Serialize)]
pub struct ReadData
{
    countpub: u32,
    pub data: Vec<u8>,
}

// Read request succeeded
//
// 2 arguments:
// 1. Number of bytes read from the file
// 2. List of bytes read from the file
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct ReadResponse
{
    pub id: u32,
    pub args: ReadData,
}

// --------------------
// write
// --------------------

// Write request succeeded
//
// Single argument:
// 1. Number of bytes written to the file
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct WriteResponse
{
    pub id: u32,
    pub count: u32,
}

// --------------------
// clunk
// --------------------

// Clunk request succeeded
//
// No arguments
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct ClunkResponse
{
    pub id: u32,
}

// --------------------
// remove
// --------------------

// Remove request succeeded
//
// No arguments
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveResponse
{
    pub id: u32,
}

// --------------------
// stat
// --------------------

#[derive(Debug, Deserialize, Serialize)]
pub struct StatData
{
    // Total byte count of all fields except for size
    pub size: u16,

    pub fileid: FileID,

    // File attributes and permissions
    // The high 8 bits are a copy of FileKind, and the other 24 bits are for
    // permissions
    pub mode: u32,

    // last access time
    // date field
    // this should be a rfc3339 compliant string
    pub atime: String,

    // last modified time
    // date field
    // this should be a rfc3339 compliant string
    pub mtime: String,

    // length of file in bytes
    pub length: u64,

    // File name
    pub name: String,

    // Owner name
    pub uid: String,

    // Group name
    pub gid: String,

    // name of the user who last modified the file
    pub muid: String,
}

// Stat request succeeded
//
// Single argument:
// 1. map of file attributes
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct StatResponse
{
    pub id: u32,
    pub stat: StatData,
}

// --------------------
// wstat
// --------------------

// Write stat request succeeded
//
// No arguments
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseKind type
#[derive(Debug, Deserialize, Serialize)]
pub struct WStatResponse
{
    pub id: u32,
}

// ===========================================================================
//
// ===========================================================================
