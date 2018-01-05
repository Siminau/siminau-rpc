// src/test/message/v1/responsebuilder.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

// Local imports


// ===========================================================================
// Tests
// ===========================================================================


mod auth {
    // Third party imports

    use quickcheck::TestResult;
    // use rmpv::Value;

    // Local imports

    use core::request::RpcRequest;
    use core::response::RpcResponse;
    use message::v1::{request, response, BuildResponseError, FileID, FileKind,
                      ResponseCode};

    #[test]
    fn invalid_fileid()
    {
        // --------------------
        // GIVEN
        // a request message and
        // an invalid file id and
        // a response builder
        // --------------------
        let req = request(42).auth(9001, "hello", "world").unwrap();
        let invalid_filekind = FileKind::DIR | FileKind::AUTH;
        let fileid = FileID::new(invalid_filekind, 0, 0);
        assert!(!fileid.is_valid());

        // --------------------
        // WHEN
        // ResponseBuilder::auth() is called w/ the invalid file id
        // --------------------
        let result = response(&req).auth(fileid);

        // --------------------
        // THEN
        // an error is returned
        // --------------------
        let val = match result {
            Err(e @ BuildResponseError::Auth(_)) => {
                let expected = format!("Unable to build auth response \
                                        message: file id has invalid \
                                        kind {}",
                                       fileid.kind.bits());
                e.to_string() == expected
            }
            _ => false,
        };
        assert!(val);
    }

    quickcheck! {
        fn valid_fileid(filekind: u8, version: u32, path: u64) -> TestResult {
            let invalid: u8 = 0b00000111;

            // Use bitwise AND to check if kind has invalid bits set
            if filekind & invalid != 0 {
                return TestResult::discard();
            }
            let kind = FileKind::from_bits(filekind).unwrap();

            // discard invalid filekind values
            if !kind.is_valid() {
                return TestResult::discard();
            }

            // --------------------
            // GIVEN
            // a request message and
            // a valid FileID and
            // a response builder
            // --------------------
            let req = request(42).auth(9001, "hello", "world").unwrap();
            let fileid = FileID::new(kind, version, path);
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::auth() is called w/ the valid fileid
            // --------------------
            let result = builder.auth(fileid);

            // --------------------
            // THEN
            // the result is a response message and
            // the message's id matches the request message id and
            // the message's code is ResponseCode::Auth and
            // the message's result is an array and
            // the result array has 3 items and
            // the result array's items are filekind (u8), version (u32),
            // and path (u64)
            // --------------------
            let val = match result {
                Ok(msg) => {
                    // Check basic criteria for valid message
                    let resp_fileid = msg.result().as_array().unwrap();
                    let val = msg.message_id() == req.message_id() &&
                        msg.error_code() == ResponseCode::Auth &&
                        resp_fileid.len() == 3;

                    // Construct fileid from the response
                    let bits = resp_fileid[0].as_u64().unwrap() as u8;
                    let filekind = FileKind::from_bits(bits).unwrap();
                    let version = resp_fileid[1].as_u64().unwrap() as u32;
                    let path = resp_fileid[2].as_u64().unwrap();
                    let resp_fileid = FileID::new(filekind, version, path);

                    // Return test result
                    val && resp_fileid.is_valid() && resp_fileid == fileid
                }
                Err(_) => false,
            };

            TestResult::from_bool(val)
        }

        fn non_auth_request(filekind: u8, version: u32, path: u64) -> TestResult {
            let invalid: u8 = 0b00000111;

            // Use bitwise AND to check if kind has invalid bits set
            if filekind & invalid != 0 {
                return TestResult::discard();
            }
            let kind = FileKind::from_bits(filekind).unwrap();

            // discard invalid filekind values
            if !kind.is_valid() {
                return TestResult::discard();
            }

            // --------------------
            // GIVEN
            // a valid FileID and
            // a request w/ a non-Auth code and
            // a response builder
            // --------------------
            let req = request(42).flush(41).unwrap();
            let fileid = FileID::new(kind, version, path);
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::auth() is called
            // --------------------
            let result = builder.auth(fileid);

            // --------------------
            // THEN
            // an error is returned
            // --------------------
            let val = match result {
                Err(e @ BuildResponseError::WrongCode { .. }) => {
                    let expected = "Unable to build response message: \
                                    expected RequestCode::Auth, got \
                                    RequestCode::Flush instead";
                    e.to_string() == expected.to_owned()
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }
    }
}

mod flush {
    // Third party imports

    // use rmpv::Value;

    // Local imports

    use core::request::RpcRequest;
    use core::response::RpcResponse;
    use message::v1::{request, response, BuildResponseError, ResponseCode};

    #[test]
    fn valid_message_id()
    {
        // --------------------
        // GIVEN
        // a flush request and
        // a response builder
        // --------------------
        let req = request(42).flush(41).unwrap();
        let builder = response(&req);

        // --------------------
        // WHEN
        // ResponseBuilder::flush() is called
        // --------------------
        let result = builder.flush();

        // --------------------
        // THEN
        // a response message is returned and
        // the response's id is the same as the request's id
        // --------------------
        let val = match result {
            Ok(msg) => msg.message_id() == req.message_id(),
            Err(_) => false,
        };
        assert!(val);
    }

    #[test]
    fn valid_response_code()
    {
        // --------------------
        // GIVEN
        // a flush request and
        // a response builder
        // --------------------
        let req = request(42).flush(41).unwrap();
        let builder = response(&req);

        // --------------------
        // WHEN
        // ResponseBuilder::flush() is called
        // --------------------
        let result = builder.flush();

        // --------------------
        // THEN
        // a response message is returned and
        // the response's code is ResponseCode::Flush
        // --------------------
        let val = match result {
            Ok(msg) => msg.error_code() == ResponseCode::Flush,
            Err(_) => false,
        };
        assert!(val);
    }

    #[test]
    fn non_flush_request()
    {
        // --------------------
        // GIVEN
        // a request w/ a non-Flush code and
        // a response builder
        // --------------------
        let req = request(42).auth(9001, "hello", "world").unwrap();
        let builder = response(&req);

        // --------------------
        // WHEN
        // ResponseBuilder::flush() is called
        // --------------------
        let result = builder.flush();

        // --------------------
        // THEN
        // an error is returned
        // --------------------
        let val = match result {
            Err(e @ BuildResponseError::WrongCode { .. }) => {
                let expected = "Unable to build response message: expected \
                                RequestCode::Flush, got RequestCode::Auth \
                                instead";
                e.to_string() == expected.to_owned()
            }
            _ => false,
        };

        assert!(val);
    }
}


mod attach {
    // Third party imports

    use quickcheck::TestResult;
    // use rmpv::Value;

    // Local imports

    use core::request::RpcRequest;
    use core::response::RpcResponse;
    use message::v1::{request, response, BuildResponseError, FileID, FileKind,
                      ResponseCode};

    #[test]
    fn invalid_fileid()
    {
        // --------------------
        // GIVEN
        // an attach request message and
        // an invalid file id and
        // a response builder
        // --------------------
        // Create attach request message
        let rootdir_id = 0;
        let authfile_id = 1;
        let username = "hello";
        let fsname = "world";
        let req = request(42)
            .attach(rootdir_id, authfile_id, username, fsname)
            .unwrap();

        // Create invalid fileid
        let invalid_filekind = FileKind::DIR | FileKind::AUTH;
        let fileid = FileID::new(invalid_filekind, 0, 0);
        assert!(!fileid.is_valid());

        // --------------------
        // WHEN
        // ResponseBuilder::attach() is called w/ the invalid file id
        // --------------------
        let result = response(&req).attach(fileid);

        // --------------------
        // THEN
        // an error is returned
        // --------------------
        let val = match result {
            Err(e @ BuildResponseError::Attach(_)) => {
                let expected = format!("Unable to build attach response \
                                        message: rootfile_id has invalid \
                                        kind {}",
                                       fileid.kind.bits());
                e.to_string() == expected
            }
            _ => false,
        };
        assert!(val);
    }

    quickcheck! {
        fn valid_fileid(filekind: u8, version: u32, path: u64) -> TestResult {
            let invalid: u8 = 0b00000111;

            // Use bitwise AND to check if kind has invalid bits set
            if filekind & invalid != 0 {
                return TestResult::discard();
            }
            let kind = FileKind::from_bits(filekind).unwrap();

            // discard invalid filekind values
            if !kind.is_valid() {
                return TestResult::discard();
            }

            // --------------------
            // GIVEN
            // a request message and
            // a valid FileID and
            // a response builder
            // --------------------
            // Create attach request message
            let rootdir_id = 0;
            let authfile_id = 1;
            let username = "hello";
            let fsname = "world";
            let req = request(42).attach(rootdir_id, authfile_id, username,
                                         fsname).unwrap();

            // Create valid fileid
            let fileid = FileID::new(kind, version, path);
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::attach() is called w/ the valid fileid
            // --------------------
            let result = builder.attach(fileid);

            // --------------------
            // THEN
            // the result is a response message and
            // the message's id matches the request message id and
            // the message's code is ResponseCode::Attach and
            // the message's result is an array and
            // the result array has 3 items and
            // the result array's items are filekind (u8), version (u32),
            // and path (u64)
            // --------------------
            let val = match result {
                Ok(msg) => {
                    // Check basic criteria for valid message
                    let resp_fileid = msg.result().as_array().unwrap();
                    let val = msg.message_id() == req.message_id() &&
                        msg.error_code() == ResponseCode::Attach &&
                        resp_fileid.len() == 3;

                    // Construct fileid from the response
                    let bits = resp_fileid[0].as_u64().unwrap() as u8;
                    let filekind = FileKind::from_bits(bits).unwrap();
                    let version = resp_fileid[1].as_u64().unwrap() as u32;
                    let path = resp_fileid[2].as_u64().unwrap();
                    let resp_fileid = FileID::new(filekind, version, path);

                    // Return test result
                    val && resp_fileid.is_valid() && resp_fileid == fileid
                }
                Err(_) => false,
            };

            TestResult::from_bool(val)
        }

        fn non_attach_request(filekind: u8, version: u32, path: u64) -> TestResult {
            let invalid: u8 = 0b00000111;

            // Use bitwise AND to check if kind has invalid bits set
            if filekind & invalid != 0 {
                return TestResult::discard();
            }
            let kind = FileKind::from_bits(filekind).unwrap();

            // discard invalid filekind values
            if !kind.is_valid() {
                return TestResult::discard();
            }

            // --------------------
            // GIVEN
            // a valid FileID and
            // a request w/ a non-Attach code and
            // a response builder
            // --------------------
            let req = request(42).flush(41).unwrap();
            let fileid = FileID::new(kind, version, path);
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::attach() is called
            // --------------------
            let result = builder.attach(fileid);

            // --------------------
            // THEN
            // an error is returned
            // --------------------
            let val = match result {
                Err(e @ BuildResponseError::WrongCode { .. }) => {
                    let expected = "Unable to build response message: expected \
                                    RequestCode::Attach, got \
                                    RequestCode::Flush instead";
                    e.to_string() == expected.to_owned()
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }
    }
}

mod walk {
    // Third party imports

    use quickcheck::TestResult;
    use std::mem;
    // use rmpv::Value;

    // Local imports

    use core::request::RpcRequest;
    use core::response::RpcResponse;
    use message::v1::{request, response, BuildResponseError, FileID, FileKind,
                      ResponseCode};

    quickcheck! {
        fn has_invalid_fileid(path_id: Vec<u8>,
                              version: u32,
                              pathpart: u64) -> TestResult
        {
            let invalid: u8 = 0b00000111;
            let invalid_filekind = FileKind::DIR | FileKind::AUTH;
            let invalid_bits = invalid_filekind.bits();
            let mut path: Vec<FileID> = Vec::with_capacity(path_id.len());
            let mut bad_index: Option<usize> = None;

            for (i, filekind) in path_id.iter().enumerate() {
                // Use bitwise AND to check if kind has any invalid bits set
                if filekind & invalid != 0 {
                    return TestResult::discard();
                }

                // Record first index w/ invalid fileid
                let bad_filekind = filekind & invalid_bits;
                if bad_filekind == invalid_bits && bad_index.is_none() {
                    mem::replace(&mut bad_index, Some(i));
                }

                // Create FileID
                let kind = FileKind::from_bits(*filekind).unwrap();
                let fileid = FileID::new(kind, version, pathpart);
                path.push(fileid);
            }

            // If no bad fileids were pushed into path, discard
            if bad_index.is_none() {
                return TestResult::discard();
            }

            // --------------------
            // GIVEN
            // a walk request message and
            // a vec of invalid file ids and
            // a response builder
            // --------------------
            // Create walk request message
            let file_id = 41;
            let newfile_id = 42;
            let reqpath = vec!["hello", "world"];
            let req = request(42)
                .walk(file_id, newfile_id, reqpath)
                .unwrap();

            // --------------------
            // WHEN
            // ResponseBuilder::walk() is called w/ the vec of invalid file ids
            // --------------------
            let result = response(&req).walk(&path);

            // --------------------
            // THEN
            // an error is returned
            // --------------------
            let val = match result {
                Err(e @ BuildResponseError::Walk { .. }) => {
                    let index = bad_index.unwrap();
                    let bad_kind = path[index].kind.bits();
                    let expected = format!("Unable to build walk response \
                                            message: item {} of path_id has \
                                            invalid kind {}",
                                           index, bad_kind);
                    e.to_string() == expected.to_owned()
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }

        fn has_valid_fileid(path_id: Vec<u8>, version: u32, pathpart: u64)
            -> TestResult
        {
            let invalid: u8 = 0b00000111;
            let mut path: Vec<FileID> = Vec::with_capacity(path_id.len());

            for filekind in path_id.iter() {
                // Use bitwise AND to check if kind has any invalid bits set
                if *filekind & invalid != 0 {
                    return TestResult::discard();
                }

                // Create FileID
                let kind = FileKind::from_bits(*filekind).unwrap();
                if !kind.is_valid() {
                    return TestResult::discard();
                }
                let fileid = FileID::new(kind, version, pathpart);
                path.push(fileid);
            }

            // --------------------
            // GIVEN
            // a walk request message and
            // a vec of valid file ids and
            // a response builder
            // --------------------
            // Create walk request message
            let file_id = 41;
            let newfile_id = 42;
            let reqpath = vec!["hello", "world"];
            let req = request(42)
                .walk(file_id, newfile_id, reqpath)
                .unwrap();

            // --------------------
            // WHEN
            // ResponseBuilder::walk() is called w/ the vec of valid file ids
            // --------------------
            let result = response(&req).walk(&path);

            // --------------------
            // THEN
            // the result is a response message and
            // the message's id matches the request message id and
            // the message's code is ResponseCode::Walk and
            // the message's result is an array and
            // the result array has the same number of items as path_id and
            // each item in the result item is an array of 3 items and
            // the array of 3 items are filekind (u8), version (u32),
            //     and path (u64)
            // --------------------
            let val = match result {
                Ok(msg) => {
                    // Check basic criteria for valid message
                    let resp_fileid = msg.result().as_array().unwrap();
                    let val = msg.message_id() == req.message_id() &&
                        msg.error_code() == ResponseCode::Walk &&
                        resp_fileid.len() == path.len();

                    // Construct fileids from the response
                    let resp_fileid = resp_fileid.iter().map(|i| {
                        let value = i.as_array().unwrap();
                        assert_eq!(value.len(), 3);
                        let bits = value[0].as_u64().unwrap() as u8;
                        let filekind = FileKind::from_bits(bits).unwrap();
                        let version = value[1].as_u64().unwrap() as u32;
                        let path = value[2].as_u64().unwrap();
                        FileID::new(filekind, version, path)
                    }).collect::<Vec<FileID>>();

                    // Return test result
                    let all_valid = resp_fileid.is_empty() ||
                        resp_fileid.iter().all(|v| v.is_valid());
                    val && all_valid && resp_fileid == path
                }
                Err(_) => false,
            };

            TestResult::from_bool(val)
        }

        fn non_walk_request(path_id: Vec<u8>, version: u32, pathpart: u64)
            -> TestResult
        {
            let invalid: u8 = 0b00000111;
            let mut path: Vec<FileID> = Vec::with_capacity(path_id.len());

            for filekind in path_id.iter() {
                // Use bitwise AND to check if kind has any invalid bits set
                if *filekind & invalid != 0 {
                    return TestResult::discard();
                }

                // Create FileID
                let kind = FileKind::from_bits(*filekind).unwrap();
                if !kind.is_valid() {
                    return TestResult::discard();
                }
                let fileid = FileID::new(kind, version, pathpart);
                path.push(fileid);
            }

            // --------------------
            // GIVEN
            // a request message w/ non-walk code and
            // a vec of valid file ids and
            // a response builder
            // --------------------
            // Create walk request message
            let req = request(42).flush(41).unwrap();

            // --------------------
            // WHEN
            // ResponseBuilder::walk() is called w/ the vec of valid file ids
            // --------------------
            let result = response(&req).walk(&path);

            // --------------------
            // THEN
            // an error is returned
            // --------------------
            let val = match result {
                Err(e @ BuildResponseError::WrongCode { .. }) => {
                    let expected = "Unable to build response message: \
                                    expected RequestCode::Walk, got \
                                    RequestCode::Flush instead";
                    e.to_string() == expected.to_owned()
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }
    }
}


mod open {
    // Third party imports

    use quickcheck::TestResult;

    // Local imports

    use core::request::RpcRequest;
    use core::response::RpcResponse;
    use message::v1::{request, response, BuildResponseError, FileID, FileKind,
                      OpenMode, ResponseCode};

    quickcheck! {
        fn has_invalid_fileid(max_size: u32,
                              mode: u8,
                              filekind: u8,
                              version: u32,
                              path: u64) -> TestResult
        {
            let invalid: u8 = 0b00000111;

            // Use bitwise AND to check if kind has any invalid bits set
            // Discard any invalid values
            if filekind & invalid != 0 {
                return TestResult::discard();
            }

            // Create FileID
            let kind = FileKind::from_bits(filekind).unwrap();
            if kind.is_valid() {
                return TestResult::discard();
            }

            let fileid = FileID::new(kind, version, path);

            // --------------------
            // GIVEN
            // an open request message and
            // an invalid file id and
            // a u32 max_size value and
            // a response builder
            // --------------------
            // Create open request message
            let client_file_id = 42;
            let open_mode = match OpenMode::from_bits(mode) {
                // Discard any mode that has invalid bits set
                Err(_) => return TestResult::discard(),

                Ok(m) => m,
            };
            let req = request(42).open(client_file_id, open_mode);
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::open() is called w/ the invalid file id and
            //    max_size
            // --------------------
            let result = builder.open(fileid, max_size);

            // --------------------
            // THEN
            // an error is returned
            // --------------------
            let val = match result {
                Err(e @ BuildResponseError::Open(_)) => {
                    let expected = format!("Unable to build open response \
                                            message: file id has invalid \
                                            kind {}",
                                           fileid.kind.bits());
                    e.to_string() == expected.to_owned()
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }

        fn has_valid_fileid(max_size: u32,
                            mode: u8,
                            filekind: u8,
                            version: u32,
                            path: u64) -> TestResult
        {
            let invalid: u8 = 0b00000111;

            // Use bitwise AND to check if kind has any invalid bits set
            if filekind & invalid != 0 {
                return TestResult::discard();
            }

            // Create FileID
            let kind = FileKind::from_bits(filekind).unwrap();

            // Discard invalid values
            if !kind.is_valid() {
                return TestResult::discard();
            }

            let fileid = FileID::new(kind, version, path);

            // --------------------
            // GIVEN
            // an open request message and
            // a valid file id and
            // a u32 max_size value and
            // a response builder
            // --------------------
            // Create open request message
            let client_file_id = 42;
            let open_mode = match OpenMode::from_bits(mode) {
                // Discard any mode that has invalid bits set
                Err(_) => return TestResult::discard(),

                Ok(m) => m,
            };
            let req = request(42).open(client_file_id, open_mode);
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::open() is called w/ the valid file id and
            //    max_size
            // --------------------
            let result = builder.open(fileid, max_size);

            // --------------------
            // THEN
            // a response message is returned and
            // the msg's code is ResponseCode::Open and
            // the msg's result is an array of 2 values and
            // the result array's first item is an array of 3 values:
            //     1. file id kind (u8)
            //     2. file id version (u32)
            //     3. file id path (u64)
            //
            //     and
            // the result array's second item is a u32 value that's equal to
            //    max_size
            // --------------------
            let val = match result {
                Err(_) => false,
                Ok(msg) => {
                    // Check basic criteria for valid message
                    let result = msg.result().as_array().unwrap();
                    let val = msg.message_id() == req.message_id() &&
                        msg.error_code() == ResponseCode::Open &&
                        result.len() == 2;

                    // Construct fileid from the response
                    let resp_fileid = {
                        let fileid = result[0].as_array().unwrap();
                        assert_eq!(fileid.len(), 3);
                        let bits = fileid[0].as_u64().unwrap() as u8;
                        let filekind = FileKind::from_bits(bits).unwrap();
                        let version = fileid[1].as_u64().unwrap() as u32;
                        let path = fileid[2].as_u64().unwrap();
                        FileID::new(filekind, version, path)
                    };

                    // Get response max size
                    let resp_maxsize = result[1].as_u64().unwrap() as u32;

                    // Return result
                    val && resp_fileid == fileid &&  resp_maxsize == max_size
                }
            };

            TestResult::from_bool(val)
        }
    }
}


mod create {
    // Third party imports

    use quickcheck::TestResult;

    // Local imports

    use core::request::RpcRequest;
    use core::response::RpcResponse;
    use message::v1::{request, response, BuildResponseError, FileID, FileKind,
                      OpenMode, ResponseCode};

    // Helpers
    use test::message::v1::invalid_string;

    quickcheck! {
        fn has_invalid_fileid(filename: String,
                              max_size: u32,
                              mode: u8,
                              filekind: u8,
                              version: u32,
                              path: u64) -> TestResult
        {
            // Discard invalid filenames
            if invalid_string(&filename[..]) {
                return TestResult::discard();
            }

            let invalid: u8 = 0b00000111;

            // Use bitwise AND to check if kind has any invalid bits set
            // Discard any invalid values
            if filekind & invalid != 0 {
                return TestResult::discard();
            }

            // Create FileID
            let kind = FileKind::from_bits(filekind).unwrap();
            if kind.is_valid() {
                return TestResult::discard();
            }

            let fileid = FileID::new(kind, version, path);

            // --------------------
            // GIVEN
            // a valid filename string and
            // a create request message and
            // an invalid file id and
            // a u32 max_size value and
            // a response builder
            // --------------------
            // Create open request message
            let client_file_id = 42;
            let open_mode = match OpenMode::from_bits(mode) {
                // Discard any mode that has invalid bits set
                Err(_) => return TestResult::discard(),

                Ok(m) => m,
            };
            let req = request(42).create(client_file_id, &filename[..], open_mode).unwrap();
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::create() is called w/ the invalid file id and
            //    max_size
            // --------------------
            let result = builder.create(fileid, max_size);

            // --------------------
            // THEN
            // an error is returned
            // --------------------
            let val = match result {
                Err(e @ BuildResponseError::Create(_)) => {
                    let expected = format!("Unable to build create response \
                                            message: file id has invalid \
                                            kind {}",
                                           fileid.kind.bits());
                    e.to_string() == expected.to_owned()
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }

        fn has_valid_fileid(filename: String,
                            max_size: u32,
                            mode: u8,
                            filekind: u8,
                            version: u32,
                            path: u64) -> TestResult
        {
            // Discard invalid filenames
            if invalid_string(&filename[..]) {
                return TestResult::discard();
            }

            let invalid: u8 = 0b00000111;

            // Use bitwise AND to check if kind has any invalid bits set
            if filekind & invalid != 0 {
                return TestResult::discard();
            }

            // Create FileID
            let kind = FileKind::from_bits(filekind).unwrap();

            // Discard invalid values
            if !kind.is_valid() {
                return TestResult::discard();
            }

            let fileid = FileID::new(kind, version, path);

            // --------------------
            // GIVEN
            // an open request message and
            // a valid file id and
            // a u32 max_size value and
            // a response builder
            // --------------------
            // Create open request message
            let client_file_id = 42;
            let open_mode = match OpenMode::from_bits(mode) {
                // Discard any mode that has invalid bits set
                Err(_) => return TestResult::discard(),

                Ok(m) => m,
            };
            let req = request(42).create(client_file_id, &filename[..], open_mode).unwrap();
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::create() is called w/ the valid file id and
            //    max_size
            // --------------------
            let result = builder.create(fileid, max_size);

            // --------------------
            // THEN
            // a response message is returned and
            // the msg's code is ResponseCode::Open and
            // the msg's result is an array of 2 values and
            // the result array's first item is an array of 3 values:
            //     1. file id kind (u8)
            //     2. file id version (u32)
            //     3. file id path (u64)
            //
            //     and
            // the result array's second item is a u32 value that's equal to
            //    max_size
            // --------------------
            let val = match result {
                Err(_) => false,
                Ok(msg) => {
                    // Check basic criteria for valid message
                    let result = msg.result().as_array().unwrap();
                    let val = msg.message_id() == req.message_id() &&
                        msg.error_code() == ResponseCode::Create &&
                        result.len() == 2;

                    // Construct fileid from the response
                    let resp_fileid = {
                        let fileid = result[0].as_array().unwrap();
                        assert_eq!(fileid.len(), 3);
                        let bits = fileid[0].as_u64().unwrap() as u8;
                        let filekind = FileKind::from_bits(bits).unwrap();
                        let version = fileid[1].as_u64().unwrap() as u32;
                        let path = fileid[2].as_u64().unwrap();
                        FileID::new(filekind, version, path)
                    };

                    // Get response max size
                    let resp_maxsize = result[1].as_u64().unwrap() as u32;

                    // Return result
                    val && resp_fileid == fileid &&  resp_maxsize == max_size
                }
            };

            TestResult::from_bool(val)
        }
    }
}


mod read {
    // Third party imports

    use proptest::prelude::*;

    // Local imports

    use core::request::RpcRequest;
    use core::response::RpcResponse;
    use message::v1::{request, response, BuildResponseError, RequestCode,
                      ResponseCode};

    proptest! {
        #[test]
        fn bad_request(
            ref data in prop::collection::vec(prop::num::u8::ANY, 0..1000)
        )
        {
            // --------------------
            // GIVEN
            // a Vec<u8> data and
            // a request with code != RequestCode::Read and
            // a response builder
            // --------------------
            let req = request(42).flush(0).unwrap();
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::read() is called w/ data
            // --------------------
            let result = builder.read(data.len() as u32, data);

            // --------------------
            // THEN
            // an error is returned
            // --------------------
            let val = match result {
                Err(BuildResponseError::WrongCode { value, expected }) => {
                    value == req.message_method() && expected == RequestCode::Read
                }
                _ => false,
            };

            assert!(val);
        }
    }

    // --------------------
    // count_datalen_nomatch
    // --------------------
    prop_compose! {
        fn mk_nomatch_veclen(count: u32)
            (size in (0..1000usize)
                .prop_filter("Cannot use size the same as count".to_owned(),
                             move |v| *v as u64 != count as u64)) -> usize
        {
            size
        }
    }

    prop_compose! {
        fn read_bytes_nomatch(count: u32)
            (size in mk_nomatch_veclen(count))
            (bytes in prop::collection::vec(prop::num::u8::ANY, size..size+1)) -> Vec<u8>
        {
            bytes
        }
    }

    prop_compose! {
        fn read_args_nomatch()
            (count in 0..1000u32)
            (bytes in read_bytes_nomatch(count), count in Just(count))-> (u32, Vec<u8>)
        {
            (count, bytes)
        }
    }

    proptest! {
        // Generate an error if the count arg does not match the length of bytes
        #[test]
        fn count_datalen_nomatch((count, ref bytes) in read_args_nomatch())
        {
            // --------------------
            // GIVEN
            // a u32 count and
            // a Vec<u8> bytes and
            // a response builder and
            // the len of bytes != count
            // --------------------
            let req = request(42).read(42, 0, 42);
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::read() is called w/ count and
            //    bytes
            // --------------------
            let result = builder.read(count, bytes);

            // --------------------
            // THEN
            // a response message is returned and
            // the msg's code is ResponseCode::Read and
            // the msg's result is an array of 2 values and
            // the result array's first item is equal to count and
            // the result array's second item is equal to bytes
            // --------------------

            let val = match result {
                Err(BuildResponseError::Read(c, n)) => {
                    c == count && n == bytes.len() && c as u64 != n as u64
                }
                _ => false,
            };

            prop_assert!(val);
        }
    }

    // --------------------
    // count_datalen_match
    // --------------------
    prop_compose! {
        fn read_bytes_match(count: u32)
            (bytes in
                prop::collection::vec(prop::num::u8::ANY,
                                      count as usize..(count as usize)+1))
            -> Vec<u8>
        {
            bytes
        }
    }

    prop_compose! {
        fn read_args_match()
            (count in 0..1000u32)
            (bytes in read_bytes_match(count), count in Just(count))-> (u32, Vec<u8>)
        {
            (count, bytes)
        }
    }

    proptest! {
        // Generate response message if the count arg matches the length of bytes
        #[test]
        fn count_datalen_match((count, ref bytes) in read_args_match())
        {
            // --------------------
            // GIVEN
            // a u32 count and
            // a Vec<u8> bytes and
            // a response builder and
            // the len of bytes == count
            // --------------------
            let req = request(42).read(42, 0, 42);
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::read() is called w/ count and
            //    bytes
            // --------------------
            let result = builder.read(count, bytes);

            // --------------------
            // THEN
            // a response message is returned and
            // the msg's code is ResponseCode::Read and
            // the msg's result is an array of 2 values and
            // the result array's first item is equal to count and
            // the result array's second item is equal to bytes
            // --------------------

            let val = match result {
                Err(_) => false,
                Ok(msg) => {
                    // Check basic criteria for valid message
                    let result = msg.result().as_array().unwrap();
                    let val = msg.message_id() == req.message_id() &&
                        msg.error_code() == ResponseCode::Read &&
                        result.len() == 2;

                    // Get response count
                    let resp_count = result[0].as_u64().unwrap() as u32;

                    // Get response bytes
                    let resp_bytes: Vec<u8> = result[1].as_slice().unwrap().into();

                    prop_assert_eq!(resp_count as usize, resp_bytes.len());
                    prop_assert_eq!(resp_count, count);
                    prop_assert_eq!(&resp_bytes, bytes);

                    val
                }
            };

            prop_assert!(val);
        }
    }
}


mod write {

    // Third party imports

    use proptest::prelude::*;


    // Local imports

    use core::request::RpcRequest;
    use core::response::RpcResponse;
    use message::v1::{request, response, BuildResponseError, RequestCode,
                      ResponseCode};

    proptest! {

        #[test]
        fn make_response(count in prop::num::u32::ANY)
        {
            // --------------------
            // GIVEN
            // a u32 count and
            // a valid request and
            // a response builder
            // --------------------
            let data = vec![];
            let req = request(42).write(42, 0, 0, &data).unwrap();
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::write() is called w/ count value
            // --------------------
            let result = builder.write(count);

            // --------------------
            // THEN
            // a response message is returned and
            // the msg's code is ResponseCode::Write and
            // the msg's result is a single u32 value and
            // the result is equal to count
            // --------------------
            // Check basic criteria for valid message
            let val = match result {
                Err(_) => false,
                Ok(msg) => {
                    let resp_count = msg.result().as_u64().unwrap() as u32;
                    let val = msg.message_id() == req.message_id() &&
                        msg.error_code() == ResponseCode::Write &&
                        resp_count == count;
                    val
                }
            };

            prop_assert!(val);
        }
    }

    #[test]
    fn bad_request() {
        // --------------------
        // GIVEN
        // a u32 count and
        // a request with code != RequestCode::Write and
        // a response builder
        // --------------------
        let req = request(42).read(42, 0, 42);
        let builder = response(&req);

        // --------------------
        // WHEN
        // ResponseBuilder::write() is called w/ count value
        // --------------------
        let result = builder.write(0);

        // --------------------
        // THEN
        // an error is returned
        // --------------------
        let val = match result {
            Err(BuildResponseError::WrongCode { value, expected }) => {
                value == req.message_method() && expected == RequestCode::Write
            }
            _ => false,
        };

        assert!(val);
    }

}


mod clunk {
    // Third party imports

    use proptest::prelude::*;
    use rmpv::Value;

    // Local imports

    use core::request::RpcRequest;
    use core::response::RpcResponse;
    use message::v1::{request, response, BuildResponseError, RequestCode,
                      ResponseCode};

    #[test]
    fn bad_request() {
        // --------------------
        // GIVEN
        // a u32 count and
        // a request with code != RequestCode::Clunk and
        // a response builder
        // --------------------
        let req = request(42).read(42, 0, 42);
        let builder = response(&req);

        // --------------------
        // WHEN
        // ResponseBuilder::clunk() is called
        // --------------------
        let result = builder.clunk();

        // --------------------
        // THEN
        // an error is returned
        // --------------------
        let val = match result {
            Err(BuildResponseError::WrongCode { value, expected }) => {
                value == req.message_method() && expected == RequestCode::Clunk
            }
            _ => false,
        };

        assert!(val);
    }

    proptest! {

        #[test]
        fn make_response(file_id in prop::num::u32::ANY)
        {
            // --------------------
            // GIVEN
            // a u32 file_id and
            // a valid request and
            // a response builder
            // --------------------
            let req = request(42).clunk(file_id);
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::clunk() is called
            // --------------------
            let result = builder.clunk();

            // --------------------
            // THEN
            // a response message is returned and
            // the msg's code is ResponseCode::Clunk and
            // the msg's result is nil
            // --------------------
            // Check basic criteria for valid message
            let val = match result {
                Ok(msg) => {
                    let val = msg.message_id() == req.message_id() &&
                        msg.error_code() == ResponseCode::Clunk &&
                        msg.result() == &Value::Nil;
                    val
                }
                _ => false
            };
            prop_assert!(val);
        }
    }
}


mod remove {
    // Third party imports

    use proptest::prelude::*;
    use rmpv::Value;

    // Local imports

    use core::request::RpcRequest;
    use core::response::RpcResponse;
    use message::v1::{request, response, BuildResponseError, RequestCode,
                      ResponseCode};

    #[test]
    fn bad_request() {
        // --------------------
        // GIVEN
        // a u32 count and
        // a request with code != RequestCode::Clunk and
        // a response builder
        // --------------------
        let req = request(42).read(42, 0, 42);
        let builder = response(&req);

        // --------------------
        // WHEN
        // ResponseBuilder::remove() is called
        // --------------------
        let result = builder.remove();

        // --------------------
        // THEN
        // an error is returned
        // --------------------
        let val = match result {
            Err(BuildResponseError::WrongCode { value, expected }) => {
                value == req.message_method() && expected == RequestCode::Remove
            }
            _ => false,
        };

        assert!(val);
    }

    proptest! {

        #[test]
        fn make_response(file_id in prop::num::u32::ANY)
        {
            // --------------------
            // GIVEN
            // a u32 file_id and
            // a valid request and
            // a response builder
            // --------------------
            let req = request(42).remove(file_id);
            let builder = response(&req);

            // --------------------
            // WHEN
            // ResponseBuilder::remove() is called
            // --------------------
            let result = builder.remove();

            // --------------------
            // THEN
            // a response message is returned and
            // the msg's code is ResponseCode::Remove and
            // the msg's result is nil
            // --------------------
            // Check basic criteria for valid message
            let val = match result {
                Ok(msg) => {
                    let val = msg.message_id() == req.message_id() &&
                        msg.error_code() == ResponseCode::Remove &&
                        msg.result() == &Value::Nil;
                    val
                }
                _ => false
            };
            prop_assert!(val);
        }
    }
}


// ===========================================================================
//
// ===========================================================================
