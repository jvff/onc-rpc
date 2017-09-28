use std::io;

use serde_xdr;

error_chain! {
    foreign_links {
        Io(io::Error);
    }

    links {
        XdrError(serde_xdr::Error, serde_xdr::ErrorKind);
    }

    errors {
        CantConvertCallToResult {
            description(
                "can't convert a call request message into a call reply message"
            )
        }

        GarbageArguments {
            description("remote program replied with garbage arguments error")
        }

        GetPortCallFailed {
            description("failed to call port mapper's get_port procedure")
        }

        InvalidGetPortResponse {
            description(
                "failed port mapper returned an invalid response for get_port"
            )
        }

        InvalidRemotePort(port: u32) {
            description("invalid port number received")
            display(
                "invalid port number received: {} (maximum value is {})",
                port,
                u16::max_value(),
            )
        }

        ProcedureUnavailable {
            description("requested procedure is unavailable in remote program")
        }

        ProgramUnavailable {
            description("requested program is unavailable in remote server")
        }

        ProgramVersionMismatch(min: u32, max: u32) {
            description(
                "requested program version is not supported in remote server"
            )
            display(
                "requested program version is not supported in remote server
                 (support versions are from {} to {})",
                min,
                max,
            )
        }

        PortMapperConnectionError(address: String) {
            description("failed to connect to the port mapper program")
            display(
                "failed to connect to the port mapper program at: {}",
                address,
            )
        }

        ProgramConnectionError(program: String, address: String) {
            description("failed to connect to program")
            display(
                "failed to connect to the {} program at: {}",
                program,
                address,
            )
        }

        RemoteCallDenied {
            description("remote call request was denied by remote server")
        }

        SystemError {
            description("remote program reported a system error")
        }
    }
}
