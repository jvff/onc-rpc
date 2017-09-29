#[macro_export]
macro_rules! onc_rpc_program_procedure {
    ( @processed $procedure:ident $parameters:tt -> $result_data:ty) => {
        use futures::{Async, Future, Poll};

        use $crate::{Error, ErrorKind, ResultExt, RpcCall, RpcProcedure};

        use super::*;

        pub struct Procedure;

        onc_rpc_program_procedure_parameters! $parameters;

        impl RpcProcedure for Procedure {
            type Program = Program;
            type Parameters = Parameters;
            type ResultData = $result_data;

            fn procedure() -> u32 {
                request::RequestId::$procedure.procedure()
            }
        }
    };

    ( $procedure:ident $parameters:tt -> $result_data:ty) => {
        pub mod $procedure {
            onc_rpc_program_procedure! {
                @processed
                $procedure $parameters -> $result_data
            }

            onc_rpc_program_procedure_response_result! {
                $procedure -> $result_data
            }
        }
    };

    ( $procedure:ident $parameters:tt) => {
        pub mod $procedure {
            onc_rpc_program_procedure! {
                @processed
                $procedure $parameters -> ()
            }

            onc_rpc_program_procedure_response_result! {
                $procedure
            }
        }
    };
}
