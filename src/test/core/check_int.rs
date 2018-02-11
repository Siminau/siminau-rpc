// src/test/core/check_int.rs
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

use quickcheck::TestResult;

// Local imports

use core::{check_int, CheckIntError};

// ===========================================================================
// Tests
// ===========================================================================

quickcheck! {
    // val == None always returns an err with given marker
    fn none_val_argument(xs: u64) -> bool {
        let errmsg = "Expected u8 but got None";
        match check_int(None, xs, "u8".to_owned()) {
            Err(e @ CheckIntError::MissingValue { .. }) => {
                let msg = e.to_string();
                &msg[..] == errmsg
            }
            _ => false
        }
    }

    // val > max value returns an err with given marker
    fn val_argument_gt_max_value(val: u64, max_value: u64) -> TestResult {
        if val <= max_value {
            return TestResult::discard()
        }

        let errmsg = format!("Expected value <= {} but got value {}",
                             max_value, val);
        let result = check_int(Some(val), max_value, val.to_string());
        let val = match result {
            Err(e @ CheckIntError::ValueTooBig { .. }) => {
                let msg = e.to_string();
                msg == errmsg
            }
            _ => false,
        };
        TestResult::from_bool(val)
    }

    // val <= max returns value
    fn val_argument_le_max_value(val: u64, max_value: u64) -> TestResult {
        if val > max_value {
            return TestResult::discard()
        }

        let result = check_int(Some(val), max_value, val.to_string());
        if let Ok(v) = result {
            TestResult::from_bool(v == val)
        } else {
            TestResult::from_bool(false)
        }
    }
}

// ===========================================================================
//
// ===========================================================================
