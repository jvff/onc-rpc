#[macro_export]
macro_rules! onc_rpc_program_procedure_message_from_response {
    (
        ;@end_marker
        $(
            $procedure:ident
            $( ( $result:ident ) )*
            =>
            $conversion:tt
            ,
        )*
    ) => {
        impl From<Response> for ProcedureMessage {
            fn from(response: Response) -> ProcedureMessage {
                match response {
                    $( Response::$procedure $( ($result) )* => $conversion )*
                }
            }
        }
    };

    ( $( $procedure:ident $( -> $result_type:ty )* ),* $(,)* ) => {
        onc_rpc_program_procedure_message_from_response! {
            $( $procedure $( -> $result_type )* ),*
            ;@end_marker
        }
    };

    (
        $procedure:ident
        $(
            ,
            $next_procedure:ident
            $( -> $next_result_type:ty )*
        )*
        ;@end_marker
        $(
            $converted_procedure:ident
            $( ( $converted_result:ident ) )*
            =>
            $conversion:tt
            ,
        )*
    ) => {
        onc_rpc_program_procedure_message_from_response! {
            $( $next_procedure $( -> $next_result_type )* ),*
            ;@end_marker
            $(
                $converted_procedure
                $( ($converted_result) )*
                =>
                $conversion
                ,
            )*
            $procedure => {
                ProcedureMessage::$procedure(RpcMessage::from_reply(()))
            },
        }
    };

    (
        $procedure:ident -> $result_type:ty
        $(
            ,
            $next_procedure:ident
            $( -> $next_result_type:ty )*
        )*
        ;@end_marker
        $(
            $converted_procedure:ident
            $( ( $converted_result:ident ) )*
            =>
            $conversion:tt
            ,
        )*
    ) => {
        onc_rpc_program_procedure_message_from_response! {
            $( $next_procedure $( -> $next_result_type )* ),*
            ;@end_marker
            $(
                $converted_procedure
                $( ($converted_result) )*
                =>
                $conversion
                ,
            )*
            $procedure(result) => {
                ProcedureMessage::$procedure(RpcMessage::from_reply(result))
            },
        }
    };
}
