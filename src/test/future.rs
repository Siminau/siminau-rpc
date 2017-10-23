// src/test/future.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Tests
// ===========================================================================


mod read_to_block {
    // --------------------
    // Imports
    // --------------------
    // Stdlib imports

    use std::io;
    use std::io::{Cursor, Read, Result};

    // Third-party imports

    use futures::Future;
    use futures::future::{Either, ok as noop_future};
    use quickcheck::TestResult;
    use tokio_core::reactor::Core;
    use tokio_io::AsyncRead;

    // Local imports

    use future::read_to_block;

    // --------------------
    // Helpers
    // --------------------
    // Custom Read object that always returns io::ErrorKind::WouldBlock
    #[derive(Debug)]
    struct MockRead {
        inner: Vec<String>,
        read_cursor: bool,
    }

    impl MockRead {
        fn new(buf: Vec<String>) -> MockRead
        {
            MockRead {
                inner: buf,
                read_cursor: false,
            }
        }
    }

    impl Read for MockRead {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize>
        {
            if self.read_cursor || self.inner.is_empty() {
                self.read_cursor = false;
                Err(io::Error::new(io::ErrorKind::WouldBlock, "would block"))
            } else {
                self.read_cursor = true;
                let data = self.inner.remove(0).into_bytes();
                let mut cur = Cursor::new(data);
                cur.read(buf)
            }
        }
    }

    impl AsyncRead for MockRead {}

    // --------------------
    // Tests
    // --------------------

    quickcheck! {
        fn to_eof(val: String) -> TestResult {
            if val.is_empty() {
                return TestResult::discard()
            }

            // --------------------
            // GIVEN
            // a cursor over a string and
            // an empty buffer
            // --------------------
            let expected = val.clone();
            let cursor = Cursor::new(val.into_bytes());
            let buf: Vec<u8> = Vec::new();

            // --------------------
            // WHEN
            // read_to_block's future is run and
            // the future completes
            // --------------------
            let fut = read_to_block(cursor, buf)
                .and_then(|(_, buf)| Ok(buf));

            let mut core = Core::new().unwrap();
            let buf = core.run(fut).unwrap();

            // --------------------
            // THEN
            // a non-empty string is returned and
            // the returned string is the expected value
            // --------------------
            let result = String::from_utf8(buf).unwrap();
            TestResult::from_bool(!result.is_empty() && result == expected)
        }
    }

    #[test]
    fn would_block()
    {
        // --------------------
        // GIVEN
        // a vec of strings and
        // a reader that implements the std::io::Read trait and
        // the reader operates over the vec of strings and
        // an empty buffer
        // --------------------
        let val = vec![String::from("hello"), String::from("world")];
        let expected = val[0].clone();
        let reader = MockRead::new(val);

        let buf: Vec<u8> = Vec::new();

        // --------------------
        // WHEN
        // read_to_block's future is run and
        // the future completes
        // --------------------
        let fut = read_to_block(reader, buf).and_then(|(_, buf)| Ok(buf));

        let mut core = Core::new().unwrap();
        let buf = core.run(fut).unwrap();

        // --------------------
        // THEN
        // a non-empty string result is returned and
        // the returned string matches the first element of the vec of strings
        // --------------------
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn would_block_empty_buffer()
    {
        // --------------------
        // GIVEN
        // an empty vec and
        // a reader that implements the std::io::Read trait and
        // the reader operates over the empty vec and
        // an empty buffer
        // --------------------
        let val = Vec::new();
        let reader = MockRead::new(val);

        let buf: Vec<u8> = Vec::new();

        // --------------------
        // WHEN
        // read_to_block's future is run and
        // the future completes
        // --------------------
        let fut = read_to_block(reader, buf)

            // It's expected that read_to_block will block (ie poll will
            // return Async::NotReady), so need to select the noop future
            .select2(noop_future::<(MockRead, Vec<u8>), io::Error>(
                (MockRead::new(Vec::new()), vec![42]),
            ))

            // Need to unwrap select2's returned Either value into just the
            // buffer. This will panic if the read_to_block's future completes
            // first
            .map(|i| {
                match i {
                    Either::A(_) => unreachable!(),
                    Either::B(((_, buf), _)) => buf,
                }
            });

        let mut core = Core::new().unwrap();
        let buf = core.run(fut).unwrap();

        // --------------------
        // THEN
        // the value from the noop future is returned
        // --------------------
        assert_eq!(buf.len(), 1);
        assert_eq!(buf, vec![42]);
    }
}


// ===========================================================================
//
// ===========================================================================
