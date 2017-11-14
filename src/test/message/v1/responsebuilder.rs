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
    use error::RpcErrorKind;
    use message::v1::{request, response, FileID, FileKind, ResponseCode};

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
            Err(e) => if let &RpcErrorKind::Msg(ref m) = e.kind() {
                &m[..] == "id contains invalid FileKind"
            } else {
                false
            },
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
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequestMethod(ref s) => {
                            &s[..] == "expected RequestCode::Auth, got \
                                       RequestCode::Flush instead"
                        }
                        _ => true,
                    }
                }
                Ok(_) => false,
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
    use error::RpcErrorKind;
    use message::v1::{request, response, ResponseCode};

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
            Err(e) => match e.kind() {
                &RpcErrorKind::InvalidRequestMethod(ref s) => {
                    &s[..]
                        == "expected RequestCode::Flush, got RequestCode::Auth \
                            instead"
                }
                _ => true,
            },
            Ok(_) => false,
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
    use error::RpcErrorKind;
    use message::v1::{request, response, FileID, FileKind, ResponseCode};

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
            Err(e) => if let &RpcErrorKind::ValueError(ref m) = e.kind() {
                &m[..] == "rootdir server id contains invalid FileKind"
            } else {
                false
            },
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
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequestMethod(ref s) => {
                            &s[..] == "expected RequestCode::Attach, got \
                                       RequestCode::Flush instead"
                        }
                        _ => true,
                    }
                }
                Ok(_) => false,
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
    use error::RpcErrorKind;
    use message::v1::{request, response, FileID, FileKind, ResponseCode};

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
                Err(e) => {
                    if let &RpcErrorKind::ValueError(ref m) = e.kind() {
                        *m == format!("item {} of path_id is an \
                                       invalid FileID",
                                       bad_index.unwrap())
                    } else {
                        false
                    }
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
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequestMethod(ref s) => {
                            &s[..] == "expected RequestCode::Walk, got \
                                       RequestCode::Flush instead"
                        }
                        _ => true,
                    }
                }
                Ok(_) => false,
            };

            TestResult::from_bool(val)
        }
    }
}


// ===========================================================================
//
// ===========================================================================