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
// kind field should be used as a message::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct InitRequest
{
    id: u32,
    category: u8,
    kind: u8,
    version: u32,
}

// --------------------
// auth
// --------------------

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AuthData
{
    authfile_id: u32,
    username: String,
    fsname: String,
}

// Setup client authentication file.
//
// 3 arguments:
// 1. file id of the auth file
// 2. user name
// 3. service name
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AuthRequest
{
    id: u32,
    category: u8,
    kind: u8,
    args: AuthData,
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
// kind field should be used as a message::v1::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FlushRequest
{
    id: u32,
    category: u8,
    kind: u8,
    message_id: u32,
}

// --------------------
// attach
// --------------------

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AttachData
{
    rootdir_id: u32,
    authfile_id: u32,
    username: String,
    fsname: String,
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
// kind field should be used as a message::v1::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AttachRequest
{
    id: u32,
    category: u8,
    kind: u8,
    args: AttachData,
}

// --------------------
// walk
// --------------------

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WalkData
{
    file_id: u32,
    newfile_id: u32,
    path: Vec<String>,
}

// Walk a directory hierarchy
//
// 3 arguments:
// 1. existing file id
// 2. new file id of the walk result
// 3. list of path element strings to walk through
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WalkRequest
{
    id: u32,
    category: u8,
    kind: u8,
    args: WalkData,
}

// --------------------
// open
// --------------------

// The mode field, when used, should be operated on as
// message::v1::util::OpenMode
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct OpenData
{
    file_id: u32,
    mode: u8,
}

// Prepare an existing file id for I/O
//
// 2 arguments:
// 1. existing file id
// 2. mode ie type of I/O
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct OpenRequest
{
    id: u32,
    category: u8,
    kind: u8,
    args: OpenData,
}

// --------------------
// create
// --------------------

// The mode field, when used, should be operated on as
// message::v1::util::OpenMode
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateData
{
    file_id: u32,
    filename: String,
    mode: u8,
}

// Create a file and open it for I/O
//
// 3 arguments:
// 1. existing file id
// 2. name of the new file
// 3. mode ie type of I/O
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateRequest
{
    id: u32,
    category: u8,
    kind: u8,
    args: CreateData,
}

// --------------------
// read
// --------------------

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ReadData
{
    file_id: u32,
    offset: u64,
    count: u32,
}

// Request for a number of bytes from a file
//
// 3 arguments:
// 1. existing file id
// 2. starting offset
// 3. number of bytes to return
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::v1::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ReadRequest
{
    id: u32,
    category: u8,
    kind: u8,
    args: ReadData,
}

// --------------------
// write
// --------------------

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WriteData
{
    file_id: u32,
    offset: u64,
    count: u32,
    data: Vec<u8>,
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
// kind field should be used as a message::v1::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WriteRequest
{
    id: u32,
    category: u8,
    kind: u8,
    args: WriteData,
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
// kind field should be used as a message::v1::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ClunkRequest
{
    id: u32,
    category: u8,
    kind: u8,
    file_id: u32,
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
// kind field should be used as a message::v1::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct RemoveRequest
{
    id: u32,
    category: u8,
    kind: u8,
    file_id: u32,
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
// kind field should be used as a message::v1::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct StatRequest
{
    id: u32,
    category: u8,
    kind: u8,
    file_id: u32,
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
    pub mtime: u32,

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
// kind field should be used as a message::v1::RequestCode type
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WStatRequest
{
    id: u32,
    category: u8,
    kind: u8,
    args: WStatData,
}

// ===========================================================================
//
// ===========================================================================
