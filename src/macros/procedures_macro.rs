#[macro_export]
macro_rules! onc_rpc_program_procedures {
    (
        $( $procedure:ident $parameters:tt $( -> $result_type:ty )* ),*
        $(,)*
    ) => {
        pub mod procedures {
            use std::fmt;
            use std::fmt::Formatter;

            use serde::de;
            use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess};

            use $crate::{CallHeader, DeserializeWithHint, RpcMessage,
                         RpcProcedure};

            use super::*;
            use super::request::{Request, RequestId};

            $(
                onc_rpc_program_procedure! {
                    $procedure $parameters $( -> $result_type )*
                }
            )*

            onc_rpc_program_procedure_message! {
                $( $procedure $parameters $( -> $result_type )* ),*
            }
        }
    };
}
