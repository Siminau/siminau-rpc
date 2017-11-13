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
// Helpers
// ===========================================================================


fn invalid_string(s: &str) -> bool
{
    if s.is_empty() {
        true
    } else {
        s.chars().any(|c| c.is_whitespace() || c.is_control())
    }
}


// ===========================================================================
// Tests
// ===========================================================================


mod auth {

    // Third party imports

    use quickcheck::TestResult;
    use rmpv::Value;

    // Local imports

    use core::request::RpcRequest;
    use error::RpcErrorKind;
    use message::v1::{request, RequestCode};

    // Helpers
    use super::invalid_string;

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
            // the result is an InvalidRequestArgs error and
            // the error msg is for the user name value
            // --------------------
            let val = match result {
                Ok(_) => false,
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequestArgs(ref m) => {
                            let msg = format!("username is either empty, \
                                               contains whitespace, or \
                                               contains control \
                                               characters: {}",
                                               &user[..]);
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
            // the result is an InvalidRequestArgs error and
            // the error msg is for the fs name value
            // --------------------
            let val = match result {
                Ok(_) => true,
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequestArgs(ref m) => {
                            let msg = format!("filesystem name is either \
                                               empty, contains \
                                               whitespace, or contains \
                                               control characters: {}",
                                               &fs[..]);
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
            // the result is an InvalidRequestArgs error and
            // the error msg is for the username value
            // --------------------
            let val = match result {
                Ok(_) => false,
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequestArgs(ref m) => {
                            let msg = format!("username is either empty, \
                                               contains whitespace, or \
                                               contains control \
                                               characters: {}",
                                              &user[..]);
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

mod flush {
    // Third party imports

    use quickcheck::TestResult;

    // Local imports

    use core::request::RpcRequest;
    use error::RpcErrorKind;
    use message::v1::{request, RequestCode};

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
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequestArgs(ref msg) => {
                            &msg[..] == &format!("invalid argument ({}): prev msg \
                                                  id matches current msg id",
                                                 old_msgid)
                        }
                        _ => false,
                    }
                }
                Ok(_) => false,
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

    use quickcheck::TestResult;
    use rmpv::Value;

    // Local imports

    use core::request::RpcRequest;
    use error::RpcErrorKind;
    use message::v1::{request, RequestCode};

    // Helpers
    use super::invalid_string;

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
                Ok(_) => false,
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequestArgs(ref msg) => {
                            msg == &format!("invalid rootdir_id value ({}): \
                                             rootdir_id and authfile_id must \
                                             have different id numbers",
                                            rootdir_id)
                        }
                        _ => false,
                    }
                }
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
            // the result is an InvalidRequestArgs error and
            // the error msg is for the user name value
            // --------------------
            let val = match result {
                Ok(_) => false,
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequestArgs(ref m) => {
                            let msg = format!("username is either empty, \
                                               contains whitespace, or \
                                               contains control \
                                               characters: {}",
                                              &user[..]);
                            m == &msg
                        }
                        _ => false,
                    }
                }
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
            // the result is an InvalidRequestArgs error and
            // the error msg is for the fs name value
            // --------------------
            let val = match result {
                Ok(_) => true,
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequestArgs(ref m) => {
                            let msg = format!("filesystem name is \
                                               either empty, contains \
                                               whitespace, or \
                                               contains control \
                                               characters: {}",
                                              &fs[..]);
                            m == &msg
                        }
                        _ => false,
                    }
                }
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
            // the result is an InvalidRequestArgs error and
            // the error msg is for the username value
            // --------------------
            let val = match result {
                Ok(_) => false,
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequestArgs(ref m) => {
                            let msg = format!("username is either \
                                               empty, contains \
                                               whitespace, or \
                                               contains control \
                                               characters: {}",
                                              &user[..]);
                            m == &msg
                        }
                        _ => false,
                    }
                }
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

    use quickcheck::TestResult;
    use rmpv::Value;

    // Local imports

    use core::request::RpcRequest;
    use error::RpcErrorKind;
    use message::v1::{request, RequestCode};

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
                Ok(_) => false,
                Err(e) => {
                    match e.kind() {
                        &RpcErrorKind::InvalidRequestArgs(ref msg) => {
                            msg == &format!("invalid newfile_id value \
                                             ({}): newfile_id has the \
                                             same value as file_id",
                                            newfile_id)
                        }
                        _ => false,
                    }
                }
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


// ===========================================================================
//
// ===========================================================================
