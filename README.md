# ONC-RPC Rust Crate

This crate implements asynchronous client and server communication using
[Open Network Computing Remote Procedure Calls][rfc].

[rfc]: https://tools.ietf.org/html/rfc1057

## Usage

Add the following in your `Cargo.toml` file in order to add it as a dependency
to your project:

    [dependencies]
    onc-rpc = "0.1"

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
                fn(10) set(value: u32) -> SetResult<bool>,
                fn(20) get() -> GetResult<Option<u32>>,
            }
        }
    }

From there, you can implement the generated `MyProgram` trait and serve it
using the generated `program_module::Server`, or use the generated
`program_module::SyncClient` to connect to a remote server and execute procedure
calls.

There is also `program_module::AsyncClient` that immediately returns after each
call and returns a future that will send the request and retrieve the response
when polled. This allows more control over the execution of the remote calls and
integrates with asynchronous code using futures.

## Status

This crate is **currently unstable**. The macro syntax has a high probability of
changing in the near future and the crate will only be considered stable after
more tests have been developed and real-world usage also shows that it is
stable. If you find any bugs or inconsistencies, please report them as GitHub
issues.
