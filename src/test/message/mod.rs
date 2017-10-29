// src/test/message/mod.rs
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

    mod version {
        // Third party imports

        use quickcheck::TestResult;

        // Local imports

        use core::request::RpcRequest;
        use message::{RequestCode, request};

        quickcheck! {

            fn set_version_number(num: u32) -> TestResult {
                // --------------------
                // GIVEN
                // a u32 version number and
                // a request builder
                // --------------------
                let builder = request(42);

                // --------------------
                // WHEN
                // RequestBuilder::version is called with the u32 number
                // --------------------
                let result = builder.version(num);

                // --------------------
                // THEN
                // a request message is returned and
                // the message's code is RequestCode::Version and
                // the message has 1 argument and
                // the argument is the u32 version number
                // --------------------
                let val = result.message_id() == 42 &&
                    result.message_method() == RequestCode::Version &&
                    result.message_args().len() == 1 &&
                    result.message_args()[0].as_u64().unwrap() == num as u64;
                TestResult::from_bool(val)
            }
        }

    }
}


mod responsebuilder {

    mod error {

        // Third party imports

        use quickcheck::TestResult;

        // Local imports

        use core::request::RpcRequest;
        use core::response::RpcResponse;
        use message::{ResponseCode, request, response};

        quickcheck! {

            fn error_message_result(errmsg: String) -> TestResult {
                // --------------------
                // GIVEN
                // a request and
                // an error message string slice and
                // a response builder created from the request
                // --------------------
                let error_message = &errmsg[..];
                let req = request(42).version(2);
                let builder = response(&req);

                // --------------------
                // WHEN
                // ResponseBuilder::error() is called w/ the error message
                // --------------------
                let result = builder.error(error_message);

                // --------------------
                // THEN
                // the result is a response message and
                // the message has the same message id as the request msg and
                // the message's result is the error message string
                // --------------------
                let val = result.message_id() == req.message_id() &&
                    result.error_code() == ResponseCode::Error &&
                    result.result().as_str().unwrap() == error_message;

                TestResult::from_bool(val)
            }
        }
    }

    mod version {
        // Third party imports

        use quickcheck::TestResult;

        // Local imports

        use core::request::RpcRequest;
        use core::response::RpcResponse;
        use message::{ResponseCode, request, response};

        quickcheck! {

            fn version_number(num: u32) -> TestResult {

                // --------------------
                // GIVEN
                // a u32 version number and
                // a request message with code == RequestCode::Version and
                // a response builder
                // --------------------
                let req = request(42).version(num);
                let builder = response(&req);

                // --------------------
                // WHEN
                // ResponseBuilder::version() is called w/ the version number
                // --------------------
                let result = builder.version(num);

                // --------------------
                // THEN
                // the result is a response message and
                // the response's msg id is the same as the request msg id and
                // the response's code == ResponseCode::Version and
                // the response's result is the version number
                // --------------------
                let val = match result {
                    Ok(msg) => {
                        msg.message_id() == req.message_id() &&
                            msg.error_code() == ResponseCode::Version &&
                            msg.result().as_u64().unwrap() == num as u64
                    }
                    Err(_) => unreachable!()
                };

                TestResult::from_bool(val)
            }
        }
    }
}


mod infobuilder {

    mod done {

        // Local imports

        use core::{MessageType, RpcMessage};
        use core::notify::RpcNotice;
        use message::{NotifyCode, info};

        #[test]
        fn info_msg()
        {
            // --------------------
            // GIVEN
            // an InfoBuilder
            // --------------------
            let builder = info();

            // --------------------
            // WHEN
            // InfoBuilder::done() is called
            // --------------------
            let msg = builder.done();

            // --------------------
            // THEN
            // the result is a notification message and
            // the message has a message code == NotifyCode::Done and
            // the message does not have any arguments
            // --------------------
            assert_eq!(msg.message_type(), MessageType::Notification);
            assert_eq!(msg.message_code(), NotifyCode::Done);
            assert_eq!(msg.message_args().len(), 0);
        }
    }
}


// ===========================================================================
//
// ===========================================================================
