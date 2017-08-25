# siminau-rpc

A library that defines the RPC protocol used by all siminau projects.
Currently, the protocol is largely similar to MessagePack-RPC with the
exception of using integers instead of strings for the method parameter.

## Getting started

### Install Rust toolchain

To build, first install the Rust toolchain. While the toolchain may be
installable from a package management system depending on your platform, it
currently is best to install via [rustup][1]. Please visit [www.rustup.rs][1]
to download and run the installer.

[1]: https://www.rustup.rs

### Run safesec

Once Rust is installed, simply enter these commands to confirm that the test
suite succeeds:

```shell
$ git clone https://github.com/siminau/siminau-rpc.git
$ cd siminau-rpc
$ cargo test
```

This will run all unit, integration, and doc tests.

## Features

* Core RPC traits
* RPC structs based on MessagePack

## Licensing

This project is licensed under the MIT license.
