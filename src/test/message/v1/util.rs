// src/test/message/v1/util.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports
use std::mem::size_of_val;

// Third-party imports
use proptest::prelude::*;

// Local imports
use message::v1::{FileID, FileKind, OwnedStat, Stat};

// ===========================================================================
// Helpers
// ===========================================================================

// --------------------
// Helpers
// --------------------

fn is_invalid_filekind_u8(v: u8) -> bool
{
    v & 0b00000111 != 0 && FileKind::from_bits(v).is_some()
}

fn is_valid_filekind_u8(v: u8) -> bool
{
    v & 0b00000111 == 0 && FileKind::from_bits(v).is_some()
}

// --------------------
// Invalid FileID
// --------------------

prop_compose! {
    fn invalid_u8()
        (val in (prop::num::u8::ANY)
             .prop_filter("Values must be invalid FileKind u8".to_owned(),
                          |v| is_invalid_filekind_u8(*v))) -> u8
    {
        val
    }
}

prop_compose! {
    fn invalid_filekind()
        (val in invalid_u8()
             .prop_filter("Values must be an invalid FileKind".to_owned(),
                          |v| !FileKind::from_bits(*v).unwrap().is_valid()))
        -> FileKind
    {
        FileKind::from_bits(val).unwrap()
    }
}

prop_compose! {
    fn invalid_fileid()
        (kind in invalid_filekind(),
         version in prop::num::u32::ANY,
         path in prop::num::u64::ANY)
        -> FileID
    {
        FileID {
            kind,
            version,
            path
        }
    }
}

// --------------------
// Valid FileID
// --------------------
prop_compose! {
    fn valid_u8()
        (val in (prop::num::u8::ANY)
             .prop_filter("Values must be valid FileKind u8".to_owned(),
                          |v| is_valid_filekind_u8(*v))) -> u8
    {
        val
    }
}

prop_compose! {
    fn valid_filekind()
        (val in valid_u8()
             .prop_filter("Values must be a valid FileKind".to_owned(),
                          |v| FileKind::from_bits(*v).unwrap().is_valid()))
        -> FileKind
    {
        FileKind::from_bits(val).unwrap()
    }
}

prop_compose! {
    fn valid_fileid()
        (kind in valid_filekind(),
         version in prop::num::u32::ANY,
         path in prop::num::u64::ANY)
        -> FileID
    {
        FileID {
            kind,
            version,
            path
        }
    }
}

// --------------------
// File names
// --------------------

// A valid filename is either a single foreslash (ie '/') character, or one or
// more of any non-control, non-foreslash character
prop_compose! {
    fn valid_filename()
        (n in r#"[\PC[^/]]+|/"#)
        -> String
    {
        n
    }
}

prop_compose! {
    fn invalid_filename()
        (n in r#"[\pC[/]]*"#)
        -> String
    {
        n
    }
}

// --------------------
// User and group names
// --------------------

// A valid username is one or more of any unicode letter, unicode decimal digit,
// or underscore character
prop_compose! {
    fn valid_authname()
        (n in r#"[\pL\p{Nd}_]+"#)
        -> String
    {
        n
    }
}

prop_compose! {
    fn invalid_authname()
        (n in r#"[\PL\P{Nd}]*"#)
        -> String
    {
        n
    }
}

// --------------------
// Stat
// --------------------

prop_compose! {
    fn valid_stat()
        (fileid in valid_fileid(),
         mode in prop::num::u32::ANY,
         atime in prop::num::u32::ANY,
         mtime in prop::num::u32::ANY,
         length in prop::num::u64::ANY,
         ref name in valid_filename(),
         ref uid in valid_authname(),
         ref gid in valid_authname(),
         ref muid in valid_authname())
        -> OwnedStat
    {
        let filename = name.clone();
        let fileuid = uid.clone();
        let filegid = gid.clone();
        let filemuid = muid.clone();
        let strsize = vec![&filename, &fileuid, &filegid, &filemuid]
            .iter()
            .fold(0, |acc, &el| acc + el.len());
        let size = (strsize + size_of_val(&fileid) + size_of_val(&mode) +
                    size_of_val(&atime) + size_of_val(&mtime) +
                    size_of_val(&length)) as u16;
        OwnedStat {
            size,
            fileid,
            mode,
            atime,
            mtime,
            length,
            name: filename,
            uid: fileuid,
            gid: filegid,
            muid: filemuid
        }
    }
}


pub struct StatBuilder;


impl StatBuilder
{
    pub fn valid_fileid() -> BoxedStrategy<FileID>
    {
        valid_fileid()
    }

    pub fn valid_filename() -> BoxedStrategy<String>
    {
        valid_filename()
    }

    pub fn valid_authname() -> BoxedStrategy<String>
    {
        valid_authname()
    }

    pub fn valid_stat() -> BoxedStrategy<OwnedStat>
    {
        valid_stat()
    }
}


// ===========================================================================
// Tests
// ===========================================================================


mod openmode
{

    mod default
    {
        // Stdlib imports

        // Third-party imports

        // Local imports

        use message::v1::OpenMode;

        // Default value for mode is 0
        #[test]
        fn mode_is_zero()
        {
            // --------------------
            // GIVEN
            // the OpenMode type
            // --------------------

            // --------------------
            // WHEN
            // OpenMode::default() is called
            // --------------------
            let obj = OpenMode::default();

            // --------------------
            // THEN
            // OpenMode.mode is 0u8
            // --------------------
            assert_eq!(obj.bits(), 0u8);
        }
    }

    mod from_bits
    {
        // Stdlib imports

        // Third-party imports
        use quickcheck::TestResult;

        // Local imports

        use message::v1::{OpenMode, OpenModeError};

        quickcheck! {

            fn valid_bits(bits: u8) -> TestResult
            {
                if bits & OpenMode::INVALID_BITS != 0 {
                    return TestResult::discard();
                }

                // --------------------
                // GIVEN
                // valid u8 bits
                // --------------------

                // --------------------
                // WHEN
                // OpenMode::from_bits() is called with the valid u8 bits
                // --------------------
                let result = OpenMode::from_bits(bits);

                // --------------------
                // THEN
                // a valid OpenMode object is returned
                // --------------------
                let val = match result {
                    Ok(mode) => mode.bits() == bits,
                    Err(_) => false,
                };
                TestResult::from_bool(val)
            }

            fn invalid_bits(bits: u8) -> TestResult
            {
                // Discard all valid bits
                if bits & OpenMode::INVALID_BITS == 0 {
                    return TestResult::discard();
                }

                // --------------------
                // GIVEN
                // invalid u8 bits
                // --------------------

                // --------------------
                // WHEN
                // OpenMode::from_bits() is called with the valid u8 bits
                // --------------------
                let result = OpenMode::from_bits(bits);

                // --------------------
                // THEN
                // an error is returned
                // --------------------
                let val = match result {
                    Err(e @ OpenModeError { .. }) => {
                        let expected = format!("Invalid bits set: {:b}", bits);
                        e.to_string() == expected
                    }
                    _ => false,
                };
                TestResult::from_bool(val)
            }
        }
    }

    mod flags
    {
        // Stdlib imports

        // Third-party imports
        use quickcheck::TestResult;

        // Local imports

        use core::CodeConvert;
        use message::v1::{openmode, OpenFlag, OpenKind};

        quickcheck! {

            fn get_flags(mode: u8) -> TestResult {
                let valid_flags = OpenFlag::all().bits();
                let valid_kind = 0b00000011;

                let (flags, kind) = {
                    let flags = mode & valid_flags;
                    let kind = mode & valid_kind;

                    // Discard invalid modes
                    if flags == 0 || kind == 0 {
                        return TestResult::discard();
                    }

                    // Return 2-tuple (flags, kind)
                    (
                        OpenFlag::from_bits(mode & valid_flags).unwrap(),
                        OpenKind::from_number(mode & valid_kind).unwrap()
                    )
                };

                // --------------------
                // GIVEN
                // a valid OpenFlag bitfield and
                // a valid OpenKind enum variant and
                // a mode containing the given OpenFlag and OpenKind
                // --------------------
                let open_mode = openmode()
                    .flags(flags)
                    .kind(kind)
                    .create();

                // --------------------
                // WHEN
                // OpenMode::flags() is called
                // --------------------
                let result = open_mode.flags();

                // --------------------
                // THEN
                // an OpenFlag equal to the given flags is returned
                // --------------------
                TestResult::from_bool(result == flags)
            }
        }
    }

    mod kind
    {
        // Stdlib imports

        // Third-party imports
        use quickcheck::TestResult;

        // Local imports

        use core::CodeConvert;
        use message::v1::{openmode, OpenFlag, OpenKind};

        quickcheck! {

            fn get_kind(mode: u8) -> TestResult {
                let valid_flags = OpenFlag::all().bits();
                let valid_kind = 0b00000011;

                let (flags, kind) = {
                    let flags = mode & valid_flags;
                    let kind = mode & valid_kind;

                    // Discard invalid modes
                    if flags == 0 || kind == 0 {
                        return TestResult::discard();
                    }

                    // Return 2-tuple (flags, kind)
                    (
                        OpenFlag::from_bits(mode & valid_flags).unwrap(),
                        OpenKind::from_number(mode & valid_kind).unwrap()
                    )
                };

                // --------------------
                // GIVEN
                // a valid OpenFlag bitfield and
                // a valid OpenKind enum variant and
                // a mode containing the given OpenFlag and OpenKind
                // --------------------
                let open_mode = openmode()
                    .flags(flags)
                    .kind(kind)
                    .create();

                // --------------------
                // WHEN
                // OpenMode::kind() is called
                // --------------------
                let result = open_mode.kind();

                // --------------------
                // THEN
                // an OpenKind equal to the given kind is returned
                // --------------------
                TestResult::from_bool(result == kind)
            }
        }
    }

    mod new_kind
    {
        // Stdlib imports

        // Third-party imports

        // Local imports

        use message::v1::{openmode, OpenFlag, OpenKind};

        #[test]
        fn new_kind_keep_flags()
        {
            // --------------------
            // GIVEN
            // an OpenMode object and
            // the object has non-zero flags and
            // the object has a non-zero kind
            // --------------------
            let obj = openmode()
                .kind(OpenKind::Execute)
                .flags(OpenFlag::ORCLOSE)
                .create();

            // --------------------
            // WHEN
            // OpenMode::new_kind() is called w/ a different OpenKind object
            // --------------------
            let result = obj.new_kind(OpenKind::ReadWrite);

            // --------------------
            // THEN
            // a new OpenMode object is returned and
            // the new object's kind is equal to the new OpenKind object and
            // the new object's flags is equal to the old object's flags
            // --------------------
            assert_eq!(result.kind(), OpenKind::ReadWrite);
            assert_eq!(result.flags(), OpenFlag::ORCLOSE);
        }
    }

    mod replace_flags
    {
        // Stdlib imports

        // Third-party imports

        // Local imports

        use message::v1::{openmode, OpenFlag, OpenKind};

        #[test]
        fn update_flags()
        {
            // --------------------
            // GIVEN
            // an OpenMode object and
            // the object has a non-zero flags and
            // the object has a non-zero kind and
            // --------------------
            let mut obj = openmode()
                .kind(OpenKind::Execute)
                .flags(OpenFlag::ORCLOSE)
                .create();

            // --------------------
            // WHEN
            // OpenMode::replace_flags() is called with a different OpenFlag
            //    object
            // --------------------
            obj.replace_flags(OpenFlag::OTRUNC);

            // --------------------
            // THEN
            // the OpenMode object's flags are the new flags
            // --------------------
            assert_eq!(obj.kind(), OpenKind::Execute);
            assert_eq!(obj.flags(), OpenFlag::OTRUNC);
        }
    }

    mod insert_flags
    {
        // Stdlib imports

        // Third-party imports

        // Local imports

        use message::v1::{openmode, OpenFlag, OpenKind};

        #[test]
        fn add_flag()
        {
            let expected = OpenFlag::ORCLOSE | OpenFlag::OTRUNC;

            // --------------------
            // GIVEN
            // an OpenMode object and
            // the object has a non-zero flags and
            // the object has a non-zero kind and
            // --------------------
            let mut obj = openmode()
                .kind(OpenKind::Execute)
                .flags(OpenFlag::ORCLOSE)
                .create();

            // --------------------
            // WHEN
            // OpenMode::insert_flags() is called with a different OpenFlag
            //    object
            // --------------------
            obj.insert_flags(OpenFlag::OTRUNC);

            // --------------------
            // THEN
            // the OpenMode object's flags are the original flags + the new flag
            // --------------------
            assert_eq!(obj.kind(), OpenKind::Execute);
            assert_eq!(obj.flags(), expected);
        }
    }

    mod remove_flags
    {
        // Stdlib imports

        // Third-party imports

        // Local imports

        use message::v1::{openmode, OpenFlag, OpenKind};

        #[test]
        fn add_flag()
        {
            // --------------------
            // GIVEN
            // an OpenMode object and
            // the object has a non-zero flags and
            // the object has a non-zero kind and
            // --------------------
            let mut obj = openmode()
                .kind(OpenKind::Execute)
                .flags(OpenFlag::ORCLOSE | OpenFlag::OTRUNC)
                .create();

            // --------------------
            // WHEN
            // OpenMode::remove_flags() is called with a different OpenFlag
            //    object
            // --------------------
            obj.remove_flags(OpenFlag::OTRUNC);

            // --------------------
            // THEN
            // the OpenMode object's flags are the original flags - the flag
            //    provided to OpenMode::remove_flags()
            // --------------------
            assert_eq!(obj.kind(), OpenKind::Execute);
            assert_eq!(obj.flags(), OpenFlag::ORCLOSE);
        }

        #[test]
        fn remove_from_zero()
        {
            // --------------------
            // GIVEN
            // an OpenMode object and
            // the object has no flags set and
            // the object is OpenKind::Read
            // --------------------
            let mut obj = openmode().kind(OpenKind::Read).create();
            assert_eq!(obj.bits(), 0);
            assert_eq!(obj.flags(), OpenFlag::ONOFLAG);

            // --------------------
            // WHEN
            // OpenMode::remove_flags() is called with an OpenFlag object
            // --------------------
            obj.remove_flags(OpenFlag::OTRUNC);

            // --------------------
            // THEN
            // no change is made to the object
            // --------------------
            assert_eq!(obj.kind(), OpenKind::Read);
            assert_eq!(obj.flags(), OpenFlag::ONOFLAG);
        }
    }

    mod toggle_flags
    {
        // Stdlib imports

        // Third-party imports

        // Local imports

        use message::v1::{openmode, OpenFlag, OpenKind};

        #[test]
        fn add_flag()
        {
            // --------------------
            // GIVEN
            // an OpenMode object and
            // the object has a non-zero flags and
            // the object has a non-zero kind and
            // --------------------
            let mut obj = openmode()
                .kind(OpenKind::Execute)
                .flags(OpenFlag::ORCLOSE)
                .create();

            // --------------------
            // WHEN
            // OpenMode::toggle_flags() is called with a different OpenFlag
            //    object
            // --------------------
            obj.toggle_flags(OpenFlag::OTRUNC);

            // --------------------
            // THEN
            // the OpenMode object's flags are the original flags + the flag
            //    provided to OpenMode::toggle_flags()
            // --------------------
            assert_eq!(obj.kind(), OpenKind::Execute);
            assert_eq!(obj.flags(), OpenFlag::ORCLOSE | OpenFlag::OTRUNC);
        }

        #[test]
        fn remove_flag()
        {
            // --------------------
            // GIVEN
            // an OpenMode object and
            // the object has a non-zero flags and
            // the object has a non-zero kind and
            // --------------------
            let mut obj = openmode()
                .kind(OpenKind::Execute)
                .flags(OpenFlag::ORCLOSE | OpenFlag::OTRUNC)
                .create();

            // --------------------
            // WHEN
            // OpenMode::toggle_flags() is called with an existing OpenFlag
            //    object
            // --------------------
            obj.toggle_flags(OpenFlag::OTRUNC);

            // --------------------
            // THEN
            // the OpenMode object's flags are the original flags - the flag
            //    provided to OpenMode::toggle_flags()
            // --------------------
            assert_eq!(obj.kind(), OpenKind::Execute);
            assert_eq!(obj.flags(), OpenFlag::ORCLOSE);
        }

        #[test]
        fn toggle_some()
        {
            // --------------------
            // GIVEN
            // an OpenMode object and
            // the object has a non-zero flags and
            // the object has a non-zero kind and
            // --------------------
            let mut obj = openmode()
                .kind(OpenKind::Execute)
                .flags(OpenFlag::ORCLOSE)
                .create();

            // --------------------
            // WHEN
            // OpenMode::toggle_flags() is called with a different OpenFlag
            //    object
            // --------------------
            obj.toggle_flags(OpenFlag::ORCLOSE | OpenFlag::OTRUNC);

            // --------------------
            // THEN
            // the OpenMode object's flags are the original flags + the flag
            //    provided to OpenMode::toggle_flags()
            // --------------------
            assert_eq!(obj.kind(), OpenKind::Execute);
            assert_eq!(obj.flags(), OpenFlag::OTRUNC);
        }

        #[test]
        fn toggle_all_to_noflag()
        {
            // --------------------
            // GIVEN
            // an OpenMode object and
            // the object has a non-zero flags and
            // the object has all flags set
            // --------------------
            let mut obj = openmode()
                .kind(OpenKind::Execute)
                .flags(OpenFlag::all())
                .create();

            // --------------------
            // WHEN
            // OpenMode::toggle_flags() is called with all flags
            // --------------------
            obj.toggle_flags(OpenFlag::all());

            // --------------------
            // THEN
            // the OpenMode object's flags are the original flags + the flag
            //    provided to OpenMode::toggle_flags()
            // --------------------
            assert_eq!(obj.kind(), OpenKind::Execute);
            assert_eq!(obj.flags(), OpenFlag::ONOFLAG);
        }

        #[test]
        fn toggle_noflag_to_all()
        {
            // --------------------
            // GIVEN
            // an OpenMode object and
            // the object has a non-zero flags and
            // the object has no flags set
            // --------------------
            let mut obj = openmode().kind(OpenKind::Execute).create();

            // --------------------
            // WHEN
            // OpenMode::toggle_flags() is called with all flags
            // --------------------
            obj.toggle_flags(OpenFlag::all());

            // --------------------
            // THEN
            // the OpenMode object's flags are all flags
            // --------------------
            assert_eq!(obj.kind(), OpenKind::Execute);
            assert_eq!(obj.flags(), OpenFlag::all());
        }

        #[test]
        fn notoggle_noflag()
        {
            // --------------------
            // GIVEN
            // an OpenMode object and
            // the object has a non-zero flags and
            // the object has no flags set
            // --------------------
            let mut obj = openmode().kind(OpenKind::Execute).create();

            // --------------------
            // WHEN
            // OpenMode::toggle_flags() is called with noflags
            // --------------------
            obj.toggle_flags(OpenFlag::ONOFLAG);

            // --------------------
            // THEN
            // the OpenMode object's flags still has flags
            // --------------------
            assert_eq!(obj.kind(), OpenKind::Execute);
            assert_eq!(obj.flags(), OpenFlag::ONOFLAG);
        }
    }
}


// ===========================================================================
//
// ===========================================================================
