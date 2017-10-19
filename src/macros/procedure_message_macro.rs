#[macro_export]
macro_rules! onc_rpc_program_procedure_message {
    (
        $( $procedure:ident $parameters:tt $( -> $result_type:ty )* ),* $(,)*
    ) => {
        #[allow(non_camel_case_types)]
        #[derive(Serialize)]
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

        impl<'de> Deserialize<'de> for ProcedureMessage {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                const FIELDS: &'static [&'static str] = &[
                    "transaction_id",
                    "call_reply_discriminant",
                    "call_header",
                    "parameters",
                ];

                deserializer.deserialize_struct(
                    "ProcedureMessage",
                    FIELDS,
                    ProcedureMessageVisitor,
                )
            }
        }

        struct ProcedureMessageVisitor;

        impl<'de> Visitor<'de> for ProcedureMessageVisitor {
            type Value = ProcedureMessage;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("struct ProcedureMessage")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let transaction_id = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let discriminant = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                if discriminant != 0u32 {
                    const EXPECTED: &'static [&'static str] = &["call message"];

                    let variant = match discriminant {
                        1 => format!("reply message"),
                        type_id => format!("unknown message type: {}", type_id),
                    };

                    return Err(de::Error::unknown_variant(&variant, EXPECTED));
                }

                let call_header: CallHeader = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                let procedure_id = call_header.procedure();

                const VARIANTS: &'static [&'static str] = &[
                    $( stringify!($procedure), )*
                ];

                $(
                    if procedure_id ==
                        procedures::$procedure::Procedure::procedure()
                    {
                        let parameters = seq.next_element()?
                            .ok_or_else(|| {
                                de::Error::invalid_length(3, &self)
                            })?;

                        Ok(
                            ProcedureMessage::$procedure(
                                RpcMessage::new_call(
                                    transaction_id,
                                    call_header,
                                    parameters,
                                ),
                            )
                        )
                    } else
                )* {
                    let variant =
                        format!("unknown procedure ID: {}", procedure_id);

                    Err(de::Error::unknown_variant(&variant, VARIANTS))
                }
            }
        }
    }
}
