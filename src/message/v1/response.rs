// /home/smokybobo/src/me/siminau/siminau-rpc/src/message/v1/response.rs
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

// Write stat request succeeded
//
// No arguments

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
// kind field should be used as a message::ResponseCode type
#[derive(Debug, Deserialize, Serialize)]
pub struct InitResponse
{
    id: u32,
    category: u8,
    kind: u8,
    version: u32,
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
// kind field should be used as a message::ResponseCode type
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse
{
    id: u32,
    category: u8,
    kind: u8,
    error_msg: String,
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
// kind field should be used as a message::ResponseCode type
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse
{
    id: u32,
    category: u8,
    kind: u8,
    server_id: FileID,
}

// --------------------
// flush
// --------------------
// Flush request succeeded
//
// No arguments
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseCode type
#[derive(Debug, Deserialize, Serialize)]
pub struct FlushResponse
{
    id: u32,
    category: u8,
    kind: u8,
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
// kind field should be used as a message::ResponseCode type
#[derive(Debug, Deserialize, Serialize)]
pub struct AttachResponse
{
    id: u32,
    category: u8,
    kind: u8,
    rootdir_id: FileID,
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
// kind field should be used as a message::ResponseCode type
#[derive(Debug, Deserialize, Serialize)]
pub struct WalkResponse
{
    id: u32,
    category: u8,
    kind: u8,
    path_id: Vec<FileID>,
}

// --------------------
// open
// --------------------

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenCreateData
{
    file_id: FileID,
    max_size: u32,
}

// Open request succeeded
//
// 2 arguments:
// 1. Unique server identifier for the opened file
// 2. Maximum number of bytes guaranteed to be read from or written to the
//    file without a separate message. May be 0 which means no limit.
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseCode type
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenResponse
{
    id: u32,
    category: u8,
    kind: u8,
    args: OpenCreateData,
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
// kind field should be used as a message::ResponseCode type
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateResponse
{
    id: u32,
    category: u8,
    kind: u8,
    args: OpenCreateData,
}

// --------------------
// read
// --------------------

pub struct ReadData
{
    count: u32,
    data: Vec<u8>,
}

// Read request succeeded
//
// 2 arguments:
// 1. Number of bytes read from the file
// 2. List of bytes read from the file
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseCode type
#[derive(Debug, Deserialize, Serialize)]
pub struct ReadResponse
{
    id: u32,
    category: u8,
    kind: u8,
    args: ReadData,
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
// kind field should be used as a message::ResponseCode type
#[derive(Debug, Deserialize, Serialize)]
pub struct WriteResponse
{
    id: u32,
    category: u8,
    kind: u8,
    count: u32,
}

// --------------------
// clunk
// --------------------

// Clunk request succeeded
//
// No arguments
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseCode type
#[derive(Debug, Deserialize, Serialize)]
pub struct ClunkResponse
{
    id: u32,
    category: u8,
    kind: u8,
}

// --------------------
// remove
// --------------------

// Remove request succeeded
//
// No arguments
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseCode type
#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveResponse
{
    id: u32,
    category: u8,
    kind: u8,
}

// --------------------
// stat
// --------------------

// Stat request succeeded
//
// Single argument:
// 1. map of file attributes
//
// category field should be used as a core::new::MessageCategory type
// kind field should be used as a message::ResponseCode type

// ===========================================================================
//
// ===========================================================================
