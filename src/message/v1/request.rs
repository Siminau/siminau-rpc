// src/message/v1/request.rs
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

// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct InitRequest
{
    pub id: u32,
    pub version: u32,
}

// --------------------
// auth
// --------------------

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AuthData
{
    pub authfile_id: u32,
    pub username: String,
    pub fsname: String,
}

// Setup client authentication file.
//
// 3 arguments:
// 1. file id of the auth file
// 2. user name
// 3. service name
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AuthRequest
{
    pub id: u32,
    pub args: AuthData,
}

// --------------------
// flush
// --------------------

// Request to abort a previous request if it hasn't been processed yet.
//
// Single argument:
// 1. message id of the previous request
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FlushRequest
{
    pub id: u32,
    pub message_id: u32,
}

// --------------------
// attach
// --------------------

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AttachData
{
    pub rootdir_id: u32,
    pub authfile_id: u32,
    pub username: String,
    pub fsname: String,
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
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AttachRequest
{
    pub id: u32,
    pub args: AttachData,
}

// --------------------
// walk
// --------------------

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WalkData
{
    pub file_id: u32,
    pub newfile_id: u32,
    pub path: Vec<String>,
}

// Walk a directory hierarchy
//
// 3 arguments:
// 1. existing file id
// 2. new file id of the walk result
// 3. list of path element strings to walk through
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WalkRequest
{
    pub id: u32,
    pub args: WalkData,
}

// --------------------
// open
// --------------------

// The mode field, when used, should be operated on as
// message::v1::util::OpenMode
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct OpenData
{
    pub file_id: u32,
    pub mode: u8,
}

// Prepare an existing file id for I/O
//
// 2 arguments:
// 1. existing file id
// 2. mode ie type of I/O
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct OpenRequest
{
    pub id: u32,
    pub args: OpenData,
}

// --------------------
// create
// --------------------

// The mode field, when used, should be operated on as
// message::v1::util::OpenMode
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateData
{
    pub file_id: u32,
    pub filename: String,
    pub mode: u8,
}

// Create a file and open it for I/O
//
// 3 arguments:
// 1. existing file id
// 2. name of the new file
// 3. mode ie type of I/O
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateRequest
{
    pub id: u32,
    pub args: CreateData,
}

// --------------------
// read
// --------------------

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ReadData
{
    pub file_id: u32,
    pub offset: u64,
    pub count: u32,
}

// Request for a number of bytes from a file
//
// 3 arguments:
// 1. existing file id
// 2. starting offset
// 3. number of bytes to return
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ReadRequest
{
    pub id: u32,
    pub args: ReadData,
}

// --------------------
// write
// --------------------

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WriteData
{
    pub file_id: u32,
    pub offset: u64,
    pub count: u32,
    pub data: Vec<u8>,
}

// Request that a number of bytes be recorded to a file
//
// 4 arguments:
// 1. existing file id
// 2. starting offset
// 3. number of bytes to write
// 4. list of bytes
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WriteRequest
{
    pub id: u32,
    pub args: WriteData,
}

// --------------------
// clunk
// --------------------

// Forget a file id
//
// Single argument:
// 1. existing file id
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ClunkRequest
{
    pub id: u32,
    pub category: u8,
    pub kind: u8,
    pub file_id: u32,
}

// --------------------
// remove
// --------------------

// Remove a file from the server
//
// Single argument:
// 1. existing file id
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct RemoveRequest
{
    pub id: u32,
    pub file_id: u32,
}

// --------------------
// stat
// --------------------

// Retrieve file attributes
//
// Single argument:
// 1. existing file id
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct StatRequest
{
    pub id: u32,
    pub file_id: u32,
}

// --------------------
// wstat
// --------------------

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WStatData
{
    // File attributes and permissions
    // The high 8 bits are a copy of FileKind, and the other 24 bits are for
    // permissions
    pub mode: u32,

    // last modified time
    // date field
    // this should be a rfc3339 compliant string
    pub mtime: String,

    // length of file in bytes
    pub length: u64,

    // File name
    pub name: String,

    // Group name
    pub gid: String,
}

// Change file attributes
//
// 2 arguments:
// 1. existing file id
// 2. map of new file attributes to save to the file
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestKind type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WStatRequest
{
    pub id: u32,
    pub args: WStatData,
}

// ===========================================================================
//
// ===========================================================================
