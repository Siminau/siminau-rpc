// test_server.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Externs
// ===========================================================================
#![recursion_limit = "1024"]

// Stdlib externs

// Third-party externs
extern crate bytes;
extern crate futures;

extern crate rmp;
extern crate rmp_serde as rmps;
extern crate rmpv;
extern crate serde;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_service;

// Local externs
extern crate siminau_rpc;

#[macro_use]
extern crate siminau_rpc_derive;


// ===========================================================================
// Imports
// ===========================================================================


// Stdlib imports

use std::io;
use std::net::Shutdown;

use std::thread;
use std::time::Duration;

// Third-party imports

use bytes::BytesMut;
use futures::{Future, Sink, Stream, future};
use futures::sync::mpsc;
use rmpv::Value;
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::{Core, Handle};
use tokio_io::AsyncRead;
use tokio_io::codec::{Decoder, Encoder};
use tokio_io::io::{read_to_end, write_all};
use tokio_service::{NewService, Service};

// Local imports

use siminau_rpc::codec::MsgPackCodec;
use siminau_rpc::error::{RpcErrorKind, RpcResult};
use siminau_rpc::core::{CodeConvert, Message};
use siminau_rpc::core::request::{RequestMessage, RpcRequest};
use siminau_rpc::core::response::{ResponseMessage, RpcResponse};
use siminau_rpc::server::{Server, ServerMessage, shutdown};


// ===========================================================================
// Service
// ===========================================================================

// Define request and response codes


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum RequestMethod {
    Get,
}


type Request = RequestMessage<RequestMethod>;


#[derive(Debug, PartialEq, Clone, CodeConvert)]
pub enum ErrorResponse {
    Nil,
    InvalidRequest,
    InvalidRequestID,
}


type Response = ResponseMessage<ErrorResponse>;


pub trait ServiceWithShutdown<T>: Service {
    fn set_server_control(&mut self, mpsc::Sender<T>, Handle);
    fn server_control(&self) -> Option<(Handle, mpsc::Sender<T>)>;
    fn shutdown(&self);
}


pub struct RpcService<T> {
    control: Option<(Handle, mpsc::Sender<T>)>,
}


impl<T> RpcService<T> {
    pub fn new() -> Self
    {
        Self { control: None }
    }
}


impl Service for RpcService<ServerMessage> {
    type Request = Value;
    type Response = Value;
    type Error = io::Error;
    type Future = Box<Future<Item = Value, Error = io::Error>>;

    fn call(&self, val: Self::Request) -> Self::Future
    {
        // Convert Value into a Message
        let msg = match Message::from(val) {

            // Return error response if invalid message
            Err(e) => {
                let msgid = 0;
                let errcode = ErrorResponse::InvalidRequest;
                let result = Value::from(e.to_string());
                let res = Response::new(msgid, errcode, result);
                let val: Value = res.into();
                self.shutdown();
                let fut = future::ok::<Value, io::Error>(val);
                return Box::new(fut);
            }
            Ok(m) => m,
        };

        // Convert Message into a Request
        let req = match Request::from(msg) {

            // Return error response if invalid request
            Err(e) => {
                let errcode = match e.kind() {
                    &RpcErrorKind::InvalidRequestID => {
                        ErrorResponse::InvalidRequestID
                    }
                    _ => ErrorResponse::InvalidRequest,
                };
                let msgid = 0;
                let errmsg = Value::from(e.to_string());
                let res = Response::new(msgid, errcode, errmsg);
                let val: Value = res.into();
                self.shutdown();
                let fut = future::ok::<Value, io::Error>(val);
                return Box::new(fut);
            }
            Ok(req) => req,
        };

        // Return an ok response
        let req_args = req.message_args();
        let result = req_args[0].clone();

        let msgid = req.message_id();
        let errcode = ErrorResponse::Nil;
        // let result = Value::Nil;
        let res = Response::new(msgid, errcode, result);
        let val: Value = res.into();

        self.shutdown();
        let fut = future::ok::<Value, io::Error>(val);
        Box::new(fut)
    }
}


impl ServiceWithShutdown<ServerMessage> for RpcService<ServerMessage> {
    fn set_server_control(&mut self, s: mpsc::Sender<ServerMessage>, loop_handle: Handle)
    {
        self.control = Some((loop_handle, s));
    }

    fn server_control(&self) -> Option<(Handle, mpsc::Sender<ServerMessage>)>
    {
        if let Some((ref h, ref tx)) = self.control {
            Some((h.clone(), tx.clone()))
        } else {
            None
        }
    }

    fn shutdown(&self)
    {
        // Request shutdown
        let control = self.server_control();
        if let Some((h, tx)) = control {
            shutdown(&h, tx);
        }
    }
}


pub fn serve<S, I>(s: S) -> io::Result<()>
    where S: NewService<Request=Value,
                        Response=Value,
                        Error=io::Error,
                        Instance=I> + 'static,
          I: Service<Request=S::Request,
                     Response=S::Response,
                     Error=S::Error> +
             ServiceWithShutdown<ServerMessage> + 'static
{
    // Create event loop
    let mut core = Core::new()?;
    let handle = core.handle();

    // Create server stream
    // Bind to localhost only
    let address = "127.0.0.1:12345".parse().unwrap();
    let listener = match TcpListener::bind(&address, &handle) {
        Ok(l) => l,
        Err(e) => {
            let errmsg =
                format!("Unable to bind to address {}: {}", address, e);
            let err = io::Error::new(io::ErrorKind::ConnectionRefused, errmsg);
            return Err(err);
        }
    };

    let server = Server::new(listener.incoming(), 1);
    let tx = server.control();

    // Set up server future
    let server = server
        .for_each(|(socket, _peer_addr)| {
            let (writer, reader) = socket.framed(MsgPackCodec).split();
            let mut service = match s.new_service() {
                Ok(service) => service,
                Err(_) => unreachable!(),
            };
            service.set_server_control(tx.clone(), handle.clone());

            let responses = reader.and_then(move |req| service.call(req));
            let server = writer.send_all(responses).then(|_| Ok(()));

            handle.spawn(server);

            Ok(())
        })
        .map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "connection handler error")
        });

    core.run(server)
}


fn client() -> io::Result<String>
{
    // Create event loop
    let mut core = Core::new()?;
    let handle = core.handle();

    // Encode a message
    let req_text = Value::from("Hello World");
    let req = Request::new(42, RequestMethod::Get, vec![req_text.clone()]);
    let val: Value = req.into();
    let mut buf = BytesMut::new();
    let mut codec = MsgPackCodec;
    codec.encode(val, &mut buf).unwrap();

    // Connect to remote server
    let address = "127.0.0.1:12345".parse().unwrap();
    let socket = TcpStream::connect(&address, &handle);

    // Set up request and response
    let request = socket.and_then(|socket| write_all(socket, &buf[..]));
    let response = request.and_then(|(socket, _req)| {
        // Close socket's writer to prevent deadlock
        socket.shutdown(Shutdown::Write).expect(
            "Could not shutdown",
        );

        // Read response from server into a buffer
        read_to_end(socket, Vec::new())
    });

    // Send request and get response
    let (_socket, data) = core.run(response).unwrap();

    // Decode data
    let mut buf = BytesMut::from(data);
    let res = match codec.decode(&mut buf)? {
        Some(m) => m,
        _ => panic!("Should not get here"),
    };

    // Turn into a Response
    let msg = Message::from(res).unwrap();
    let res = Response::from(msg).unwrap();
    let res_text = res.result();

    if res_text == &req_text {
        let t = format!("Got message: {}", res_text);
        Ok(t)
    } else {
        let errmsg = format!("Expected: {} || Got: {}", res_text, req_text);
        let err = io::Error::new(io::ErrorKind::Other, errmsg);
        Err(err)
    }
}


#[test]
fn expected_server_test()
{

    // Start server
    let child = thread::spawn(
        move || if let Err(e) = serve(|| Ok(RpcService::new())) {
            println!("Server failed with {}", e);
        },
    );

    thread::sleep(Duration::from_millis(500));

    // Start client
    let res = client();

    let val = match res {
        Ok(t) => {
            println!("{}", t);
            true
        }
        Err(e) => {
            println!("Client error: {}", e);
            false
        }
    };

    child.join().unwrap();
    assert!(val);
}


// ===========================================================================
//
// ===========================================================================
