use std::io;

error_chain! {
    foreign_links {
        Io(io::Error);
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

        RemoteCallDenied {
            description("remote call request was denied by remote server")
        }

        SystemError {
            description("remote program reported a system error")
        }
    }
}
