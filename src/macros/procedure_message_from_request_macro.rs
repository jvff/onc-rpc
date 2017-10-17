#[macro_export]
macro_rules! onc_rpc_program_procedure_message_from_request {
    (
        @end_marker
        $(
            $procedure:ident
            $( ( $parameter:ident ) )*
            $( { $( $field:ident ),* } )*
            =>
            $conversion:tt
            ,
        )*
    ) => {
        impl From<Request> for ProcedureMessage {
            fn from(request: Request) -> ProcedureMessage {
                match request {
                    $(
                        Request::$procedure
                            $( ($parameter) )*
                            $( { $( $field ),* } )*
                        => $conversion
                    )*
                }
            }
        }
    };

    (
        $( $procedure:ident ( $( $name:ident : $type:ty ),* $(,)* ) ),* $(,)*
    ) => {
        onc_rpc_program_procedure_message_from_request! {
            $( $procedure( $( $name: $type ),* ) ),*
            @end_marker
        }
    };

    (
        $procedure:ident ()
        $( , $next_procedure:ident $next_parameters:tt )*
        @end_marker
        $( $ready:tt )*
    ) => {
        onc_rpc_program_procedure_message_from_request! {
            $( $next_procedure $next_parameters ),*
            @end_marker
            $( $ready )*
            $procedure => {
                let parameters = $procedure::Parameters::default();

                ProcedureMessage::$procedure(parameters.into())
            },
        }
    };

    (
        $procedure:ident ( $name:ident : $type:ty )
        $( , $next_procedure:ident $next_parameters:tt )*
        @end_marker
        $( $ready:tt )*
    ) => {
        onc_rpc_program_procedure_message_from_request! {
            $( $next_procedure $next_parameters ),*
            @end_marker
            $( $ready )*
            $procedure($name) => {
                let parameters = $procedure::Parameters::from($name);

                ProcedureMessage::$procedure(parameters.into())
            },
        }
    };

    (
        $procedure:ident ( $( $name:ident : $type:ty ),* )
        $( , $next_procedure:ident $next_parameters:tt )*
        @end_marker
        $( $ready:tt )*
    ) => {
        onc_rpc_program_procedure_message_from_request! {
            $( $next_procedure $next_parameters ),*
            @end_marker
            $( $ready )*
            $procedure { $( $name ),* } => {
                let parameters = $procedure::Parameters::new( $( $name ),* );

                ProcedureMessage::$procedure(parameters.into())
            },
        }
    };
}
