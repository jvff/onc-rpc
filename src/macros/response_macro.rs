#[macro_export]
macro_rules! onc_rpc_program_response {
    (
        $( $procedure:ident $( -> $result_type:ty )*),*
        $(,)*
    ) => {
        mod response {
            use $crate::{Error, Result, TryFrom};

            use super::*;
            use super::procedures::ProcedureMessage;

            #[allow(non_camel_case_types)]
            pub enum Response {
                $( $procedure $( ($result_type) )* ),*
            }

            impl TryFrom<ProcedureMessage> for Response {
                type Error = Error;

                onc_rpc_program_response_conversion! {
                    $( $procedure $( -> $result_type )* ),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! onc_rpc_program_response_conversion {
    (
        ; end_marker ;
        $( $procedure:ident ($message:pat) => $conversion:tt, )*
    ) => {
        fn try_from(message: ProcedureMessage) -> Result<Self> {
            match message {
                $( ProcedureMessage::$procedure($message) => $conversion )*
            }
        }
    };

    ( $( $procedure:ident $( -> $result_type:ty )* ),* ) => {
        onc_rpc_program_response_conversion! {
            $( $procedure $( -> $result_type )* ),*
            ; end_marker ;
        }
    };

    (
        $procedure:ident
        $( , $next_procedure:ident $( -> $next_result_type:ty )* )*
        ; end_marker ;
        $( $converted_procedure:ident ($message:pat) => $conversion:tt, )*
    ) => {
        onc_rpc_program_response_conversion! {
            $( $next_procedure $( -> $next_result_type )* ),*
            ; end_marker ;
            $( $converted_procedure($message) => $conversion, )*
            $procedure(_) => {
                Ok(Response::$procedure)
            },
        }
    };

    (
        $procedure:ident -> $result_type:ty
        $( , $next_procedure:ident $( -> $next_result_type:ty )* )*
        ; end_marker ;
        $( $converted_procedure:ident ($message:pat) => $conversion:tt, )*
    ) => {
        onc_rpc_program_response_conversion! {
            $( $next_procedure $( -> $next_result_type )* ),*
            ; end_marker ;
            $( $converted_procedure($message) => $conversion, )*
            $procedure(message) => {
                Ok(Response::$procedure(message.into_reply()?))
            },
        }
    };
}
