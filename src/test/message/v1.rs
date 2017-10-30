// src/test/message/v1.rs
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


mod requestbuilder {

    mod auth {

        // Third party imports

        use quickcheck::TestResult;
        use rmpv::Value;

        // Local imports

        use core::request::RpcRequest;
        use error::RpcErrorKind;
        use message::v1::{RequestCode, request};

        quickcheck! {

            fn auth_request(fileid: u32, user: String, fs: String) -> TestResult
            {
                // Ignore empty strings or strings with whitespace
                let names = vec![&user[..], &fs[..]];
                for n in names {
                    if n.is_empty() || n.chars().any(|c| c.is_whitespace()) {
                        return TestResult::discard();
                    }
                }

                // --------------------
                // GIVEN
                // a u32 file id and
                // a user name string and
                // the user name string is not an empty string and
                // the user name string does not contain whitespace chars and
                // a filesystem name string and
                // the filesystem string is not an empty string and
                // the filesystem string does not contain whitespace chars and
                // a request builder
                // --------------------
                let builder = request(42);

                // --------------------
                // WHEN
                // RequestBuilder::auth() is called w/ fileid, user, and fs
                // names
                // --------------------
                let result = builder.auth(fileid, &user[..], &fs[..]);

                // --------------------
                // THEN
                // the result is a request message and
                // the message's code is RequestCode::Auth and
                // the message has 3 arguments and
                // the message's arguments are the fileid, user, and fs values
                // --------------------
                let val = match result {
                    Ok(msg) => {
                        let args = msg.message_args();
                        let numargs = args.len();
                        let expected = vec![Value::from(fileid),
                                            Value::from(&user[..]),
                                            Value::from(&fs[..])];

                        msg.message_id() == 42 &&
                            msg.message_method() == RequestCode::Auth &&
                            numargs == 3 &&
                            args == &expected
                    }
                    Err(_) => false,
                };

                TestResult::from_bool(val)
            }

            fn bad_username(fileid: u32, user: String, fs: String) -> TestResult
            {
                // Ignore bad fs strings
                if fs.is_empty() || fs.chars().any(|c| c.is_whitespace()) {
                    return TestResult::discard();
                }

                // Ignore valid username strings
                if !user.is_empty() && user.chars().all(|c| !c.is_whitespace()) {
                    return TestResult::discard()
                }

                // --------------------
                // GIVEN
                // a u32 file id and
                // a user name string and
                // the user name string may be an empty string and
                // the user name may contain whitespace characters
                // a filesystem name string and
                // the filesystem string is not an empty string and
                // the filesystem string does not contain whitespace chars and
                // a request builder
                // --------------------
                let builder = request(42);

                // --------------------
                // WHEN
                // RequestBuilder::auth() is called w/ fileid, user, and fs
                // names
                // --------------------
                let result = builder.auth(fileid, &user[..], &fs[..]);

                // --------------------
                // THEN
                // the result is an InvalidRequestArgs error and
                // the error msg is for the user name value
                // --------------------
                let val = match result {
                    Ok(_) => false,
                    Err(e) => {
                        match e.kind() {
                            &RpcErrorKind::InvalidRequestArgs(ref m) => {
                                let msg = if user.is_empty() {
                                    "username is an empty string".to_owned()
                                } else {
                                    format!("username contains whitespace characters: {}",
                                            &user[..])
                                };
                                m == &msg
                            }
                            _ => false,
                        }
                    }
                };

                TestResult::from_bool(val)
            }

            fn bad_fsname(fileid: u32, user: String, fs: String) -> TestResult
            {
                // Ignore bad user strings
                if user.is_empty() || user.chars().any(|c| c.is_whitespace()) {
                    return TestResult::discard();
                }

                // Ignore valid fs strings
                if !fs.is_empty() && fs.chars().all(|c| !c.is_whitespace()) {
                    return TestResult::discard()
                }

                // --------------------
                // GIVEN
                // a u32 file id and
                // a user name string and
                // the user name string is not an empty string and
                // the user name string does not contain whitespace chars and
                // a filesystem name string and
                // the fs name string may be an empty string and
                // the fs name may contain whitespace characters
                // a request builder
                // --------------------
                let builder = request(42);

                // --------------------
                // WHEN
                // RequestBuilder::auth() is called w/ fileid, user, and fs
                // names
                // --------------------
                let result = builder.auth(fileid, &user[..], &fs[..]);

                // --------------------
                // THEN
                // the result is an InvalidRequestArgs error and
                // the error msg is for the fs name value
                // --------------------
                let val = match result {
                    Ok(_) => true,
                    Err(e) => {
                        match e.kind() {
                            &RpcErrorKind::InvalidRequestArgs(ref m) => {
                                let msg = if fs.is_empty() {
                                    "filesystem name is an empty string".to_owned()
                                } else {
                                    format!("filesystem name contains whitespace \
                                             characters: {}",
                                            &fs[..])
                                };
                                m == &msg
                            }
                            _ => false,
                        }
                    }
                };

                TestResult::from_bool(val)
            }

            fn bad_username_fsname(fileid: u32, user: String, fs: String) -> TestResult
            {
                // Ignore valid username and fsname strings
                let names = vec![&user[..], &fs[..]];
                for n in names {
                    if !n.is_empty() && n.chars().all(|c| !c.is_whitespace()) {
                        return TestResult::discard()
                    }
                }

                // --------------------
                // GIVEN
                // a u32 file id and
                // a user name string and
                // the user name string may be an empty string and
                // the user name may contain whitespace characters
                // a filesystem name string and
                // the fs name string may be an empty string and
                // the fs name may contain whitespace characters
                // a request builder
                // --------------------
                let builder = request(42);

                // --------------------
                // WHEN
                // RequestBuilder::auth() is called w/ fileid, user, and fs
                // names
                // --------------------
                let result = builder.auth(fileid, &user[..], &fs[..]);

                // --------------------
                // THEN
                // the result is an InvalidRequestArgs error and
                // the error msg is for the username value
                // --------------------
                let val = match result {
                    Ok(_) => false,
                    Err(e) => {
                        match e.kind() {
                            &RpcErrorKind::InvalidRequestArgs(ref m) => {
                                let msg = if user.is_empty() {
                                    "username is an empty string".to_owned()
                                } else {
                                    format!("username contains whitespace characters: {}",
                                            &user[..])
                                };
                                m == &msg
                            }
                            _ => false,
                        }
                    }
                };

                TestResult::from_bool(val)
            }
        }
    }
}


mod responsebuilder {
    mod auth {
        // Third party imports

        use quickcheck::TestResult;
        use rmpv::Value;

        // Local imports

        use core::request::RpcRequest;
        use core::response::RpcResponse;
        use error::RpcErrorKind;
        use message::v1::{FileID, FileKind, ProtocolResponse, ResponseCode,
                          request, response};

        #[test]
        fn invalid_fileid() {
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
                Err(e) => {
                    if let &RpcErrorKind::Msg(ref m) = e.kind() {
                        &m[..] == "id contains invalid FileKind"
                    } else {
                        false
                    }
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
        }
    }
}


// ===========================================================================
//
// ===========================================================================
