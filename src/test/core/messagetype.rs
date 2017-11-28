// src/test/core/messagetype.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Externs
// ===========================================================================


// Stdlib externs

// Third-party externs

// Local externs


// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

// Third-party imports

// Local imports

// ===========================================================================
// Tests
// ===========================================================================


mod from_number
{
    // std lib imports

    // Third-party imports

    use quickcheck::TestResult;

    // Local imports

    use core::{CodeConvert, CodeValueError, MessageType};


    quickcheck! {

        // Returned Ok value when casted as u8 is equal to u8 input value
        fn variant_u8_matches_number(xs: u8) -> TestResult {
            match MessageType::from_number(xs) {
                Err(_) => TestResult::discard(),
                Ok(code) => {
                    TestResult::from_bool(code as u8 == xs)
                }
            }
        }

        // Returns error if input value is >= the number of variants
        fn invalid_number(xs: u8) -> TestResult {
            if xs < 3 {
                return TestResult::discard()
            }
            let val = match MessageType::from_number(xs) {
                Err(e @ CodeValueError { .. }) => {
                    let errmsg = format!("Unknown code value: {}", xs);
                    e.to_string() == errmsg
                }
                Ok(_) => false,
            };
            TestResult::from_bool(val)
        }
    }
}


mod to_number
{
    // std lib imports

    // Third-party imports

    use quickcheck::TestResult;

    // Local imports

    use core::{CodeConvert, MessageType};

    quickcheck! {
        // Always returns an integer < 3
        fn lt_3(xs: u8) -> TestResult {
            if xs > 2 {
                return TestResult::discard()
            }
            let val = MessageType::from_number(xs).unwrap();
            TestResult::from_bool(val.to_number() < 3)
        }

        // Return value converted back to MessageType == original MessageType
        // value
        fn convert_to_messagetype(xs: u8) -> TestResult {
            if xs > 2 {
                return TestResult::discard()
            }
            let val = MessageType::from_number(xs).unwrap();
            let after = MessageType::from_number(val.to_number()).unwrap();
            TestResult::from_bool(val == after)
        }
    }
}


// ===========================================================================
//
// ===========================================================================
