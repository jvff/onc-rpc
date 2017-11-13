# ONC-RPC Rust Crate

This crate implements asynchronous client and server communication using [Open
Network Computing Remote Procedure Calls][rfc].

[rfc]: https://tools.ietf.org/html/rfc1057

## Usage

This crate is currently not available on crates.io because documentation and
tests are incomplete. Once it reaches a minimum threshold of having at least
some documentation for every public item, it will be uploaded. In the meantime,
you can configure your project to use this repository directly, but be warned
that the crate is **currently unstable**. To do so, add the following lines to
your `Cargo.toml` file:

    [dependencies]
    onc-rpc = { git = "https://github.com/jvff/onc-rpc" }

To use this crate for remote procedure calls, define the interface using the
`onc_rpc!` macro, as follows:

    extern crate serde;
    extern crate tokio_core;
    extern crate tokio_proto;
    extern crate tokio_service;

    #[macro_use] extern crate error_chain;
    #[macro_use] extern crate futures;
    #[macro_use] extern crate onc_rpc;
    #[macro_use] extern crate serde_derive;

    onc_rpc! {
        program(program_module::MyProgram) {
            id = 0x2017_2017;
            version = 1;

            procedures {
                10 => set(value: u32) -> SetResult<bool>,
                20 => get() -> GetResult<Option<u32>>,
            }
        }
    }

From there, you can implement the generated `MyProgram` trait and serve it
using the generated `program_module::Server`, or use the generated
`program_module::SyncClient` to connect to a remote server and execute procedure
calls.

## Status

This crate should not be considered stable before development of its internal
tests is complete and more thorough real-world tests have been made. If you find
any bugs or inconsistencies, please report them as GitHub issues.
