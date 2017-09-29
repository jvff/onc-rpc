macro_rules! onc_rpc_program_procedures {
    (
        $( $procedure:ident $parameters:tt $( -> $result_type:ty )* ),*
        $(,)*
    ) => {
        pub mod procedures {
            use serde::{Deserialize, Deserializer};

            use $crate::{DeserializeWithHint, RpcMessage};

            use super::*;
            use super::request::{Request, RequestId};

            $(
                onc_rpc_program_procedure! {
                    $procedure $parameters $( -> $result_type )*
                }
            )*

            onc_rpc_program_procedure_message! {
                $( $procedure $parameters ),*
            }
        }
    };
}
