// src/server.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Externs
// ===========================================================================


// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

use std::io;
use std::net::SocketAddr;

// Third-party imports

use futures::{Async, Future, Poll, Sink, Stream};
use futures::sync::mpsc;
use tokio_core::net::{Incoming, TcpStream};
use tokio_core::reactor::Handle;

// Local imports


// ===========================================================================
// Server
// ===========================================================================


pub enum ServerMessage {
    // Send(TcpStream, SocketAddr),
    Shutdown,
}


// Helper function to send a server message via a channel
pub fn sendmsg<T>(loop_handle: &Handle, control: mpsc::Sender<T>, msg: T)
where
    T: 'static,
{
    let f = control.send(msg).then(|_| Ok(()));
    loop_handle.spawn(f);
}


// Helper function to send a shutdown message via a channel
pub fn shutdown(loop_handle: &Handle, control: mpsc::Sender<ServerMessage>)
{
    sendmsg::<ServerMessage>(loop_handle, control, ServerMessage::Shutdown);
}


pub struct Server {
    control: (mpsc::Sender<ServerMessage>, mpsc::Receiver<ServerMessage>),
    listener: Incoming, // From TcpListener::incoming(),
    shutdown: bool,
}


impl Server {
    pub fn new(stream: Incoming, channel_size: usize) -> Self
    {
        let control = mpsc::channel::<ServerMessage>(channel_size);

        Self {
            control: control,
            listener: stream,
            shutdown: false,
        }
    }

    pub fn control(&self) -> mpsc::Sender<ServerMessage>
    {
        let (ref tx, _) = self.control;
        tx.clone()
    }

    fn poll_msg(&mut self) -> Poll<Option<(TcpStream, SocketAddr)>, io::Error>
    {
        let msg_poll;
        {
            let (_, ref mut rx) = self.control;
            msg_poll = rx.poll();
        }

        match msg_poll {
            Err(()) => {
                let errmsg = "Error receiving server command";
                let err = io::Error::new(io::ErrorKind::Other, errmsg);
                Err(err)
            }

            // Nothing more will be streamed, close the server down
            Ok(Async::Ready(None)) => Ok(Async::Ready(None)),

            Ok(Async::Ready(Some(ServerMessage::Shutdown))) => {
                {
                    let (_, ref mut rx) = self.control;
                    rx.close();
                    self.shutdown = true;
                }
                Ok(Async::NotReady)
            }

            Ok(Async::NotReady) => Ok(Async::NotReady),
        }
    }
}


impl Stream for Server {
    type Item = (TcpStream, SocketAddr);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error>
    {
        // Poll for a message first
        let msg = self.poll_msg();

        // If no messages, check listener
        if let Ok(Async::NotReady) = msg {
            if self.shutdown {
                Ok(Async::Ready(None))
            } else {
                self.listener.poll()
            }
        }
        // Return message
        else {
            msg
        }
    }
}


// ===========================================================================
//
// ===========================================================================
