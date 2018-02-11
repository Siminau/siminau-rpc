// src/future.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Stdlib imports

use std::io;
use std::mem;

// Third-party imports

use futures::{Async, Future, Poll};
use tokio_io::AsyncRead;

// Local imports

// ===========================================================================
//
// ===========================================================================

#[derive(Debug)]
enum ReadToBlockState<A>
{
    Reading
    {
        a: A,
        buf: Vec<u8>,
    },
    Empty,
}

#[derive(Debug)]
pub struct ReadToBlock<A>
{
    state: ReadToBlockState<A>,
}

/// Creates a future which will read all the bytes associated with the I/O
/// object `A` into the buffer provided until either the read operation will
/// block or EOF is reached.
///
/// In the case of an error the buffer and the object will be discarded, with
/// the error yielded. In the case of success the object and the buffer will
/// be returned, with all data read from the stream appended to the buffer.
pub fn read_to_block<A>(a: A, buf: Vec<u8>) -> ReadToBlock<A>
where
    A: AsyncRead,
{
    ReadToBlock {
        state: ReadToBlockState::Reading { a: a, buf: buf },
    }
}

impl<A> Future for ReadToBlock<A>
where
    A: AsyncRead,
{
    type Item = (A, Vec<u8>);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(A, Vec<u8>), io::Error>
    {
        match self.state {
            ReadToBlockState::Reading {
                ref mut a,
                ref mut buf,
            } => {
                // If we get `Ok`, then we know the stream hit EOF and we're
                // done. If we hit "would block" then all the read data so far
                // is in our buffer and the future completes. Otherwise we
                // propagate errors
                match a.read_to_end(buf) {
                    Ok(_num_bytes_read) => {}

                    // If would block and there's nothing in the buffer, then
                    // return not ready
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if buf.len() == 0 {
                            return Ok(Async::NotReady);
                        }
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
            ReadToBlockState::Empty => {
                panic!("poll ReadToBlock after it's done")
            }
        }

        match mem::replace(&mut self.state, ReadToBlockState::Empty) {
            ReadToBlockState::Reading { a, buf } => Ok((a, buf).into()),
            ReadToBlockState::Empty => unreachable!(),
        }
    }
}

// ===========================================================================
//
// ===========================================================================
