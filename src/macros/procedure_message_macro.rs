#[macro_export]
macro_rules! onc_rpc_program_procedure_message {
    ( $( $procedure:ident $parameters:tt ),* $(,)* ) => {
        #[allow(non_camel_case_types)]
        #[derive(Deserialize, Serialize)]
        #[serde(untagged)]
        pub enum ProcedureMessage {
            $( $procedure(RpcMessage<$procedure::Procedure>), )*
        }

        $(
            impl From<RpcMessage<$procedure::Procedure>> for ProcedureMessage {
                fn from(message: RpcMessage<$procedure::Procedure>) -> Self {
                    ProcedureMessage::$procedure(message)
                }
            }
        )*

        onc_rpc_program_procedure_message_from_request! {
            $( $procedure $parameters ),*
        }

        impl DeserializeWithHint<RequestId> for ProcedureMessage {
            fn deserialize_with_hint<'de, D>(
                hint: RequestId,
                deserializer: D,
            ) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                match hint {
                    $(
                        RequestId::$procedure => {
                            Ok(
                                RpcMessage::<$procedure::Procedure>
                                    ::deserialize(deserializer)?.into()
                            )
                        }
                    )*
                }
            }
        }
    }
}

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
        $(
            ,
            $next_procedure:ident
            ( $( $next_name:ident : $next_type:ty ),* )
        )*
        @end_marker
        $(
            $converted_procedure:ident
            $( ( $converted_parameter:ident ) )*
            $( { $( $converted_field:ident ),* } )*
            =>
            $conversion:tt
            ,
        )*
    ) => {
        onc_rpc_program_procedure_message_from_request! {
            $( $next_procedure( $( $next_name: $next_type ),* ) ),*
            @end_marker
            $(
                $converted_procedure
                $( ( $converted_parameter ) )*
                $( { $( $converted_field ),* } )*
                =>
                $conversion
                ,
            )*
            $procedure => {
                let parameters = $procedure::Parameters::default();

                ProcedureMessage::$procedure(parameters.into())
            },
        }
    };

    (
        $procedure:ident ( $name:ident : $type:ty )
        $(
            ,
            $next_procedure:ident
            ( $( $next_name:ident : $next_type:ty ),* )
        )*
        @end_marker
        $(
            $converted_procedure:ident
            $( ( $converted_parameter:ident ) )*
            $( { $( $converted_field:ident ),* } )*
            =>
            $conversion:tt
            ,
        )*
    ) => {
        onc_rpc_program_procedure_message_from_request! {
            $( $next_procedure( $( $next_name: $next_type ),* ) ),*
            @end_marker
            $(
                $converted_procedure
                $( ( $converted_parameter ) )*
                $( { $( $converted_field ),* } )*
                =>
                $conversion
                ,
            )*
            $procedure($name) => {
                let parameters = $procedure::Parameters::from($name);

                ProcedureMessage::$procedure(parameters.into())
            },
        }
    };

    (
        $procedure:ident ( $( $name:ident : $type:ty ),* )
        $(
            ,
            $next_procedure:ident
            ( $( $next_name:ident : $next_type:ty ),* )
        )*
        @end_marker
        $(
            $converted_procedure:ident
            $( ( $converted_parameter:ident ) )*
            $( { $( $converted_field:ident ),* } )*
            =>
            $conversion:tt
            ,
        )*
    ) => {
        onc_rpc_program_procedure_message_from_request! {
            $( $next_procedure( $( $next_name: $next_type ),* ) ),*
            @end_marker
            $(
                $converted_procedure
                $( ( $converted_parameter ) )*
                $( { $( $converted_field ),* } )*
                =>
                $conversion
                ,
            )*
            $procedure { $( $name ),* } => {
                let parameters = $procedure::Parameters::new( $( $name ),* );

                ProcedureMessage::$procedure(parameters.into())
            },
        }
    };
}
