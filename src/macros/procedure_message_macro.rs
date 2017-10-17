#[macro_export]
macro_rules! onc_rpc_program_procedure_message {
    (
        $( $procedure:ident $parameters:tt $( -> $result_type:ty )* ),* $(,)*
    ) => {
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

        onc_rpc_program_procedure_message_from_response! {
            $( $procedure $( -> $result_type )* ),*
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
