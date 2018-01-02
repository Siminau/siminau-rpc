// src/test/message/v1/requestbuilder.rs
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

    use failure::Fail;
    use quickcheck::TestResult;
    use rmpv::Value;

    // Local imports

    use core::request::RpcRequest;
    use message::v1::{request, BuildRequestError, RequestCode};

    // Helpers
    use test::message::v1::invalid_string;

    quickcheck! {

        fn auth_request(fileid: u32, user: String, fs: String) -> TestResult
        {
            // Ignore empty strings or strings with whitespace or strings
            // with control characters
            let names = vec![&user[..], &fs[..]];
            for n in names {
                if invalid_string(n) {
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
            if invalid_string(&fs[..]) {
                return TestResult::discard();
            }

            // Ignore valid username strings
            if !invalid_string(&user[..]) {
                return TestResult::discard();
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
            // the result is a BuildRequestError error and
            // the error msg is for the user name value
            // --------------------
            let val = match result {
                Err(e @ BuildRequestError::Auth(_)) => {
                    // Check top-level error
                    let expected = "Unable to build auth request message";
                    let ret = e.to_string() == expected;

                    // Check cause error
                    let cause = e.cause().unwrap();
                    let expected = "username is either empty, contains \
                                    whitespace, or contains control \
                                    characters";
                    ret && cause.to_string() == expected.to_owned()
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }

        fn bad_fsname(fileid: u32, user: String, fs: String) -> TestResult
        {
            // Ignore bad user strings
            if invalid_string(&user[..]) {
                return TestResult::discard();
            }

            // Ignore valid fs strings
            if !invalid_string(&fs[..]) {
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
            // the result is a BuildRequestError error and
            // the error msg is for the fs name value
            // --------------------
            let val = match result {
                Err(e @ BuildRequestError::Auth(_)) => {
                    // Check top-level error
                    let expected = "Unable to build auth request message";
                    let ret = e.to_string() == expected;

                    // Check cause error
                    if ret {
                        let cause = e.cause().unwrap();
                        let expected = "filesystem name is either empty, \
                                        contains whitespace, or contains \
                                        control characters";
                        cause.to_string() == expected.to_owned()
                    } else {
                        false
                    }
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }

        fn bad_username_fsname(fileid: u32, user: String, fs: String) -> TestResult
        {
            // Ignore valid username and fsname strings
            let names = vec![&user[..], &fs[..]];
            for n in names {
                if !invalid_string(n) {
                    return TestResult::discard();
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
            // the result is a BuildRequestError error and
            // the error msg is for the username value
            // --------------------
            let val = match result {
                Err(e @ BuildRequestError::Auth(_)) => {
                    // Check top-level error
                    let expected = "Unable to build auth request message";
                    let ret = e.to_string() == expected;

                    // Check cause error
                    if ret {
                        let cause = e.cause().unwrap();
                        let expected = "username is either empty, \
                                        contains whitespace, or contains \
                                        control characters";
                        cause.to_string() == expected.to_owned()
                    } else {
                        false
                    }
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }
    }
}


mod flush {
    // Third party imports

    // use failure::Fail;
    use quickcheck::TestResult;

    // Local imports

    use core::request::RpcRequest;
    use message::v1::{request, BuildRequestError, RequestCode};

    quickcheck! {
        fn bad_prev_msgid(old_msgid: u32) -> TestResult {
            // --------------------
            // GIVEN
            // a u32 old message id and
            // a request builder created w/ the old message id
            // --------------------
            let builder = request(old_msgid);

            // --------------------
            // WHEN
            // RequestBuilder::flush() is called with the old message id
            // --------------------
            let result = builder.flush(old_msgid);

            // --------------------
            // THEN
            // an error is returned
            // --------------------
            let val = match result {
                Err(e @ BuildRequestError::Flush(_)) => {
                    // Check error msg
                    let expected = format!("Unable to build flush request \
                                            message: prev msg id ({}) matches \
                                            current msg id", old_msgid);
                    e.to_string() == expected
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }

        fn good_prev_msgid(new_msgid: u32, old_msgid: u32) -> TestResult {
            if old_msgid == new_msgid {
                return TestResult::discard();
            }

            // --------------------
            // GIVEN
            // a u32 new message id and
            // a u32 old message id and
            // the new and old message ids are not equal and
            // a request builder created w/ the new message id
            // --------------------
            let builder = request(new_msgid);

            // --------------------
            // WHEN
            // RequestBuilder::flush() is called with the old message id
            // --------------------
            let result = builder.flush(old_msgid);

            // --------------------
            // THEN
            // the result is a request message and
            // the msg's id == new_msgid and
            // the msg's code == RequestCode::Flush and
            // the msg has 1 argument and
            // the msg's single argument == old_msgid
            // --------------------
            let val = match result {
                Ok(msg) => {
                    let msgargs = msg.message_args();
                    let val = msg.message_id() == new_msgid &&
                        msg.message_method() == RequestCode::Flush &&
                        msgargs.len() == 1;

                    let old = msgargs[0].as_u64().unwrap() as u32;
                    val && old == old_msgid
                }
                Err(_) => false,
            };

            TestResult::from_bool(val)
        }
    }
}


mod attach {
    // Third party imports

    use failure::Fail;
    use quickcheck::TestResult;
    use rmpv::Value;

    // Local imports

    use core::request::RpcRequest;
    use message::v1::{request, BuildRequestError, RequestCode};

    // Helpers
    use test::message::v1::invalid_string;

    quickcheck! {

        fn rootdir_equals_authfile_error(rootdir_id: u32) -> TestResult
        {
            // --------------------
            // GIVEN
            // a u32 rootdir id and
            // a u32 authfile id and
            // rootdir id == authfile id and
            // a valid username and
            // a valid fsname and
            // a request builder
            // --------------------
            let authfile_id = rootdir_id;
            let username = "hello";
            let fsname = "world";
            let builder = request(42);

            // --------------------
            // WHEN
            // RequestBuilder::attach() is called
            // --------------------
            let result = builder.attach(rootdir_id, authfile_id, username,
                                        fsname);

            // --------------------
            // THEN
            // the result is an error
            // --------------------
            let val = match result {
                Err(e @ BuildRequestError::Attach(_)) => {
                    // Check top-level error
                    let expected = "Unable to build attach request message";
                    let ret = e.to_string() == expected;

                    // Check cause error
                    if ret {
                        let cause = e.cause().unwrap();
                        let expected = format!("Invalid rootdir_id value \
                                                ({}): rootdir_id matches \
                                                authfile_id", rootdir_id);
                        cause.to_string() == expected.to_owned()
                    } else {
                        false
                    }
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }

        fn bad_username(rootdir_id: u32, authfile_id: u32, user: String,
                        fs: String) -> TestResult
        {
            // Ignore if rootdir_id == authfile_id
            if rootdir_id == authfile_id {
                return TestResult::discard();
            }

            // Ignore bad fs strings
            if invalid_string(&fs[..]) {
                return TestResult::discard();
            }

            // Ignore valid username strings
            if !invalid_string(&user[..]) {
                return TestResult::discard()
            }

            // --------------------
            // GIVEN
            // a u32 rootdir_id and
            // a u32 authfile_id and
            // rootdir_id != authfile_id and
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
            // RequestBuilder::attach() is called
            // --------------------
            let result = builder.attach(rootdir_id, authfile_id, &user[..], &fs[..]);

            // --------------------
            // THEN
            // the result is a BuildRequestError::Attach error and
            // the error msg is for the user name value
            // --------------------
            let val = match result {
                Err(e @ BuildRequestError::Attach(_)) => {
                    // Check top-level error
                    let expected = "Unable to build attach request message";
                    let ret = e.to_string() == expected;

                    // Check cause error
                    if ret {
                        let cause = e.cause().unwrap();
                        let expected = "Name error: username is either empty, \
                                        contains whitespace, or contains \
                                        control characters";
                        cause.to_string() == expected.to_owned()
                    } else {
                        false
                    }
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }

        fn bad_fsname(rootdir_id: u32, authfile_id: u32, user: String,
                      fs: String) -> TestResult
        {
            // Ignore if rootdir_id == authfile_id
            if rootdir_id == authfile_id {
                return TestResult::discard();
            }

            // Ignore bad user strings
            if invalid_string(&user[..]) {
                return TestResult::discard();
            }

            // Ignore valid fs strings
            if !invalid_string(&fs[..]) {
                return TestResult::discard()
            }

            // --------------------
            // GIVEN
            // a u32 rootdir_id and
            // a u32 authfile_id and
            // rootdir_id != authfile_id and
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
            // RequestBuilder::attach() is called
            // --------------------
            let result = builder.attach(rootdir_id, authfile_id, &user[..], &fs[..]);

            // --------------------
            // THEN
            // the result is a BuildRequestError::Attach error and
            // the error msg is for the fs name value
            // --------------------
            let val = match result {
                Err(e @ BuildRequestError::Attach(_)) => {
                    // Check top-level error
                    let expected = "Unable to build attach request message";
                    let ret = e.to_string() == expected;

                    // Check cause error
                    if ret {
                        let cause = e.cause().unwrap();
                        let expected = "Name error: filesystem name is either \
                                        empty, contains whitespace, or \
                                        contains control characters";
                        cause.to_string() == expected.to_owned()
                    } else {
                        false
                    }
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }

        fn bad_username_fsname(rootdir_id: u32, authfile_id: u32, user:
                               String, fs: String) -> TestResult
        {
            // Ignore if rootdir_id == authfile_id
            if rootdir_id == authfile_id {
                return TestResult::discard();
            }

            // Ignore valid username and fsname strings
            let names = vec![&user[..], &fs[..]];
            for n in names {
                if !invalid_string(n) {
                    return TestResult::discard()
                }
            }

            // --------------------
            // GIVEN
            // a u32 rootdir_id and
            // a u32 authfile_id and
            // rootdir_id != authfile_id and
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
            // RequestBuilder::attach() is called
            // --------------------
            let result = builder.attach(rootdir_id, authfile_id, &user[..], &fs[..]);

            // --------------------
            // THEN
            // the result is a BuildRequestError::Attach error and
            // the error msg is for the username value
            // --------------------
            let val = match result {
                Err(e @ BuildRequestError::Attach(_)) => {
                    // Check top-level error
                    let expected = "Unable to build attach request message";
                    let ret = e.to_string() == expected;

                    // Check cause error
                    if ret {
                        let cause = e.cause().unwrap();
                        let expected = "Name error: username is either empty, \
                                        contains whitespace, or contains \
                                        control characters";
                        cause.to_string() == expected.to_owned()
                    } else {
                        false
                    }
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }

        fn create_request_message(rootdir_id: u32, authfile_id: u32, user:
                                  String, fs: String) -> TestResult
        {
            // Ignore if rootdir_id == authfile_id
            if rootdir_id == authfile_id {
                return TestResult::discard();
            }

            // Ignore invalid username and fsname strings
            let names = vec![&user[..], &fs[..]];
            for n in names {
                if invalid_string(n) {
                    return TestResult::discard()
                }
            }

            // --------------------
            // GIVEN
            // a u32 rootdir id and
            // a u32 authfile id and
            // rootdir id != authfile id and
            // a valid username and
            // a valid filesystem name and
            // a request builder
            // --------------------
            let builder = request(42);

            // --------------------
            // WHEN
            // RequestBuilder::attach() is called
            // --------------------
            let result = builder.attach(rootdir_id, authfile_id, &user[..], &fs[..]);

            // --------------------
            // THEN
            // a request message is returned and
            // the msg's code is RequestCode::Attach and
            // the msg has 4 arguments and
            // the msg's arguments, in order, are equal to:
            // - rootdir_id
            // - authfile_id
            // - username
            // - filesystem name
            // --------------------
            let val = match result {
                Ok(msg) => {
                    let expected  = vec![Value::from(rootdir_id),
                                         Value::from(authfile_id),
                                         Value::from(&user[..]),
                                         Value::from(&fs[..])];
                    let msgargs = msg.message_args();
                    let val = msg.message_method() == RequestCode::Attach &&
                        msgargs.len() == 4;
                    val && msgargs == &expected
                }
                Err(_) => false,
            };

            TestResult::from_bool(val)
        }
    }
}


mod walk {
    // Third party imports

    // use failure::Fail;
    use quickcheck::TestResult;
    use rmpv::Value;

    // Local imports

    use core::request::RpcRequest;
    use message::v1::{request, BuildRequestError, RequestCode};

    quickcheck! {

        fn fileid_equals_newfileid_error(file_id: u32) -> TestResult
        {
            // --------------------
            // GIVEN
            // a u32 file id and
            // a u32 newfile id and
            // file id == newfile id and
            // an empty path vector
            // a request builder
            // --------------------
            let newfile_id = file_id;
            let path: Vec<&str> = vec![];
            let builder = request(42);

            // --------------------
            // WHEN
            // RequestBuilder::walk() is called
            // --------------------
            let result = builder.walk(file_id, newfile_id, path);

            // --------------------
            // THEN
            // the result is an error
            // --------------------
            let val = match result {
                Err(e @ BuildRequestError::Walk(_)) => {
                    let expected = format!(
                        "Unable to build walk request message: newfile_id \
                         ({}) has the same value as file_id",
                        newfile_id
                    );
                    e.to_string() == expected
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }

        fn create_request_message(file_id: u32, newfile_id: u32,
                                  path: Vec<String>) -> TestResult
        {
            // Ignore invalid file_id
            if file_id == newfile_id {
                return TestResult::discard();
            }

            // --------------------
            // GIVEN
            // a u32 file id and
            // a u32 newfile id and
            // file id != newfile id and
            // a vec of strings and
            // a request builder
            // --------------------
            // Create path vectors
            let mut expected_path: Vec<Value> = Vec::with_capacity(path.len());
            let mut converted_path: Vec<&str> = Vec::with_capacity(path.len());
            for i in path.iter() {
                let slice = &i[..];
                expected_path.push(Value::from(slice));
                converted_path.push(slice);
            }

            let builder = request(42);

            // --------------------
            // WHEN
            // RequestBuilder::walk() is called
            // --------------------
            let result = builder.walk(file_id, newfile_id, converted_path);

            // --------------------
            // THEN
            // a request message is returned and
            // the msg's code is RequestCode::Walk and
            // the msg has 3 arguments and
            // the msg's arguments, in order, are equal to:
            // - file_id
            // - newfile_id
            // - path
            // --------------------
            let val = match result {
                Ok(msg) => {
                    let expected = vec![Value::from(file_id),
                                        Value::from(newfile_id),
                                        Value::Array(expected_path)];
                    let msgargs = msg.message_args();
                    let val = msg.message_method() == RequestCode::Walk &&
                        msgargs.len() == 3;
                    val && msgargs == &expected
                }
                Err(_) => false,
            };

            TestResult::from_bool(val)
        }
    }
}


mod open {
    // Third party imports

    // use failure::Fail;
    use quickcheck::TestResult;

    // Local imports

    use core::request::RpcRequest;
    use message::v1::{request, OpenMode, RequestCode};

    quickcheck! {

        fn create_request_message(file_id: u32, mode: u8) -> TestResult
        {
            // --------------------
            // GIVEN
            // a u32 file id and
            // an OpenMode object and
            // a RequestBuilder object
            // --------------------
            let open_mode = match OpenMode::from_bits(mode) {
                // Discard any mode that has invalid bits set
                Err(_) => return TestResult::discard(),

                Ok(m) => m,
            };
            let builder = request(42);

            // --------------------
            // WHEN
            // RequestBuilder::open() is called with the u32 file id and the
            //    valid mode
            // --------------------
            let result = builder.open(file_id, open_mode);

            // --------------------
            // THEN
            // a request message is returned and
            // the msg has a code of RequestCode::Open and
            // the msg has 2 arguments and
            // the arguments are:
            //     1. u32 file_id
            //     2. u8 mode
            // and the msg file_id == the given u32 file id and
            // the msg mode == the given u8 mode
            // --------------------
            let args = result.message_args();
            let val = result.message_method() == RequestCode::Open &&
                args.len() == 2;

            let msg_fileid = args[0].as_u64().unwrap() as u32;
            let msg_mode = args[1].as_u64().unwrap() as u8;

            let val = val && msg_fileid == file_id && msg_mode == mode;
            TestResult::from_bool(val)
        }
    }
}


mod create {
    // Third party imports

    use failure::Fail;
    use quickcheck::TestResult;

    // Local imports

    use core::request::RpcRequest;
    use message::v1::{request, BuildRequestError, OpenMode, RequestCode};

    // Helpers
    use test::message::v1::invalid_string;

    quickcheck! {

        fn bad_filename(fileid: u32, filename: String, mode: u8) -> TestResult
        {
            // Ignore valid username strings
            if !invalid_string(&filename[..]) {
                return TestResult::discard();
            }

            // --------------------
            // GIVEN
            // a u32 file id and
            // a filename string and
            // the filename string may be an empty string and
            // the filename may contain whitespace characters and
            // the filename may contain control characters and
            // an OpenMode object and
            // a request builder
            // --------------------
            let open_mode = match OpenMode::from_bits(mode) {
                // Discard any mode that has invalid bits set
                Err(_) => return TestResult::discard(),

                Ok(m) => m,
            };
            let builder = request(42);

            // --------------------
            // WHEN
            // RequestBuilder::create() is called w/ fileid, filename, and mode
            // --------------------
            let result = builder.create(fileid, &filename[..], open_mode);

            // --------------------
            // THEN
            // the result is a BuildRequestError::Create error and
            // the error msg is for the user name value
            // --------------------
            let val = match result {
                Err(e @ BuildRequestError::Create(_)) => {
                    // Check top-level error
                    let expected = "Unable to build create request message";
                    let ret = e.to_string() == expected;

                    // Check cause error
                    if ret {
                        let cause = e.cause().unwrap();
                        let expected = "filename is either empty, \
                                        contains whitespace, or contains \
                                        control characters";
                        cause.to_string() == expected.to_owned()
                    } else {
                        false
                    }
                }
                _ => false,
            };

            TestResult::from_bool(val)
        }

        fn create_request_message(fileid: u32, filename: String, mode: u8) -> TestResult
        {
            // Ignore invalid filename strings
            if invalid_string(&filename[..]) {
                return TestResult::discard();
            }

            // --------------------
            // GIVEN
            // a u32 file id and
            // a valid filename string and
            // an OpenMode object and
            // a RequestBuilder object
            // --------------------
            let open_mode = match OpenMode::from_bits(mode) {
                // Discard any mode that has invalid bits set
                Err(_) => return TestResult::discard(),

                Ok(m) => m,
            };
            let builder = request(42);

            // --------------------
            // WHEN
            // RequestBuilder::create() is called w/ fileid, filename, and mode
            // --------------------
            let result = builder.create(fileid, &filename[..], open_mode);

            // --------------------
            // THEN
            // a request message is returned and
            // the msg has a code of RequestCode::Create and
            // the msg has 3 arguments and
            // the arguments are:
            //     1. u32 file_id
            //     2. &str filename
            //     3. u8 mode
            // and the msg file_id == the given u32 file id and
            // the msg filename == the given String filename and
            // the msg mode == the given u8 mode
            // --------------------
            let val = match result {
                Err(_) => false,
                Ok(msg) => {
                    let args = msg.message_args();
                    let val = msg.message_method() == RequestCode::Create &&
                        args.len() == 3;

                    let msg_fileid = args[0].as_u64().unwrap() as u32;
                    let msg_filename = args[1].as_str().unwrap();
                    let msg_mode = args[2].as_u64().unwrap() as u8;

                    val &&
                        msg_fileid == fileid &&
                        msg_filename == &filename[..] &&
                        msg_mode == mode
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
    use message::v1::{request, RequestCode};

    proptest! {
        #[test]
        fn same_args(file_id in prop::num::u32::ANY,
                     offset in prop::num::u64::ANY,
                     count in prop::num::u32::ANY)
        {
            // --------------------
            // GIVEN
            // a u32 file id and
            // a u64 offset and
            // a 32 count and
            // a request builder
            // --------------------
            let builder = request(42);

            // --------------------
            // WHEN
            // RequestBuilder::read() is called w/ file_id, offset, and count
            // --------------------
            let result = builder.read(file_id, offset, count);

            // --------------------
            // THEN
            // a request message is returned and
            // the msg has a code of RequestCode::Read and
            // the msg has 3 arguments and
            // the msg's args have the same values as
            //    file_id, offset, and count
            // --------------------
            prop_assert_eq!(result.message_method(), RequestCode::Read);

            let args = result.message_args();
            prop_assert_eq!(args.len(), 3);

            let msg_file_id = args[0].as_u64().unwrap() as u32;
            let msg_offset = args[1].as_u64().unwrap();
            let msg_count = args[2].as_u64().unwrap() as u32;

            prop_assert_eq!(msg_file_id, file_id);
            prop_assert_eq!(msg_offset, offset);
            prop_assert_eq!(msg_count, count);
        }
    }

    // quickcheck! {

    //     fn bad_filename(fileid: u32, filename: String, mode: u8) -> TestResult
    //     {
    //         // Ignore valid username strings
    //         if !invalid_string(&filename[..]) {
    //             return TestResult::discard();
    //         }

    //         // --------------------
    //         // GIVEN
    //         // a u32 file id and
    //         // a filename string and
    //         // the filename string may be an empty string and
    //         // the filename may contain whitespace characters and
    //         // the filename may contain control characters and
    //         // an OpenMode object and
    //         // a request builder
    //         // --------------------
    //         let open_mode = match OpenMode::from_bits(mode) {
    //             // Discard any mode that has invalid bits set
    //             Err(_) => return TestResult::discard(),

    //             Ok(m) => m,
    //         };
    //         let builder = request(42);

    //         // --------------------
    //         // WHEN
    //         // RequestBuilder::create() is called w/ fileid, filename, and mode
    //         // --------------------
    //         let result = builder.create(fileid, &filename[..], open_mode);

    //         // --------------------
    //         // THEN
    //         // the result is a BuildRequestError::Create error and
    //         // the error msg is for the user name value
    //         // --------------------
    //         let val = match result {
    //             Err(e @ BuildRequestError::Create(_)) => {
    //                 // Check top-level error
    //                 let expected = "Unable to build create request message";
    //                 let ret = e.to_string() == expected;

    //                 // Check cause error
    //                 if ret {
    //                     let cause = e.cause().unwrap();
    //                     let expected = "filename is either empty, \
    //                                     contains whitespace, or contains \
    //                                     control characters";
    //                     cause.to_string() == expected.to_owned()
    //                 } else {
    //                     false
    //                 }
    //             }
    //             _ => false,
    //         };

    //         TestResult::from_bool(val)
    //     }

    //     fn create_request_message(fileid: u32, filename: String, mode: u8) -> TestResult
    //     {
    //         // Ignore invalid filename strings
    //         if invalid_string(&filename[..]) {
    //             return TestResult::discard();
    //         }

    //         // --------------------
    //         // GIVEN
    //         // a u32 file id and
    //         // a valid filename string and
    //         // an OpenMode object and
    //         // a RequestBuilder object
    //         // --------------------
    //         let open_mode = match OpenMode::from_bits(mode) {
    //             // Discard any mode that has invalid bits set
    //             Err(_) => return TestResult::discard(),

    //             Ok(m) => m,
    //         };
    //         let builder = request(42);

    //         // --------------------
    //         // WHEN
    //         // RequestBuilder::create() is called w/ fileid, filename, and mode
    //         // --------------------
    //         let result = builder.create(fileid, &filename[..], open_mode);

    //         // --------------------
    //         // THEN
    //         // a request message is returned and
    //         // the msg has a code of RequestCode::Create and
    //         // the msg has 3 arguments and
    //         // the arguments are:
    //         //     1. u32 file_id
    //         //     2. &str filename
    //         //     3. u8 mode
    //         // and the msg file_id == the given u32 file id and
    //         // the msg filename == the given String filename and
    //         // the msg mode == the given u8 mode
    //         // --------------------
    //         let val = match result {
    //             Err(_) => false,
    //             Ok(msg) => {
    //                 let args = msg.message_args();
    //                 let val = msg.message_method() == RequestCode::Create &&
    //                     args.len() == 3;

    //                 let msg_fileid = args[0].as_u64().unwrap() as u32;
    //                 let msg_filename = args[1].as_str().unwrap();
    //                 let msg_mode = args[2].as_u64().unwrap() as u8;

    //                 val &&
    //                     msg_fileid == fileid &&
    //                     msg_filename == &filename[..] &&
    //                     msg_mode == mode
    //             }
    //         };

    //         TestResult::from_bool(val)
    //     }
    // }
}


// ===========================================================================
//
// ===========================================================================
