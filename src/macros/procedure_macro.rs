macro_rules! onc_rpc_program_procedure {
    ( $procedure:ident $parameters:tt -> $result_data:ty) => {
        mod $procedure {
            use $crate::{RpcCall, RpcProcedure};
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
        }
    };

    ( $procedure:ident $parameters:tt) => {
        onc_rpc_program_procedure!($procedure $parameters -> ());
    };
}
