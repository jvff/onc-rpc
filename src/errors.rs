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
        /// Failure when attempting to convert an RPC message into a reply
        /// message.
        CantConvertCallToResult {
            description(
                "can't convert a call request message into a call reply message"
            )
        }

        /// Failure when attempting to convert an RPC message into a call
        /// request.
        CantConvertResultToCall {
            description(
                "can't convert a call reply message into a call request message"
            )
        }

        /// Remote program replied with a "garbage arguments" error.
        GarbageArguments {
            description("remote program replied with garbage arguments error")
        }

        /// Received an invalid response from a procedure.
        InvalidProcedureResponse(procedure: String) {
            description("received an invalid response from a procedure call")
            display(
                "received an invalid response from procedure: {}",
                procedure,
            )
        }

        /// An invalid port number was received (it has more than 16 bits).
        InvalidRemotePort(port: u32) {
            description("invalid port number received")
            display(
                "invalid port number received: {} (maximum value is {})",
                port,
                u16::max_value(),
            )
        }

        /// Failure to call a remote procedure.
        ProcedureCallError(procedure: String) {
            description("failed to call procedure")
            display("failed to call procedure: {}", procedure)
        }

        /// Requested procedure is unavailable in remote program.
        ProcedureUnavailable {
            description("requested procedure is unavailable in remote program")
        }

        /// Requested program is unavailable in remote server.
        ProgramUnavailable {
            description("requested program is unavailable in remote server")
        }

        /// Requested program version is not supported in remote server.
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

        /// Failed to connect to remote program.
        ProgramConnectionError(address: String) {
            description("failed to connect to program")
            display("failed to connect to program at: {}", address)
        }

        /// Remote Proceduce Call request was denied by the remote server.
        RemoteCallDenied {
            description("remote call request was denied by remote server")
        }

        /// Remote program reported a system error.
        SystemError {
            description("remote program reported a system error")
        }
    }
}
