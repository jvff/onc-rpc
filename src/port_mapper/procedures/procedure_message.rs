use serde::{Deserialize, Deserializer};

use super::call_broadcast;
use super::dump;
use super::get_port;
use super::null;
use super::set;
use super::unset;
use super::super::requests::{Request, RequestId};
use super::super::super::message::RpcMessage;
use super::super::super::service::DeserializeWithHint;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProcedureMessage {
    Null(RpcMessage<null::Procedure>),
    Set(RpcMessage<set::Procedure>),
    Unset(RpcMessage<unset::Procedure>),
    GetPort(RpcMessage<get_port::Procedure>),
    Dump(RpcMessage<dump::Procedure>),
    CallBroadcast(RpcMessage<call_broadcast::Procedure>),
}

impl From<Request> for ProcedureMessage {
    fn from(request: Request) -> Self {
        match request {
            Request::null => {
                ProcedureMessage::Null(null::Parameters::default().into())
            }
            Request::set(mapping) => {
                ProcedureMessage::Set(set::Parameters::from(mapping).into())
            }
            Request::unset(mapping) => {
                ProcedureMessage::Unset(unset::Parameters::from(mapping).into())
            }
            Request::get_port(mapping) => {
                let parameters = get_port::Parameters::from(mapping);

                ProcedureMessage::GetPort(parameters.into())
            }
            Request::dump => {
                ProcedureMessage::Dump(dump::Parameters::default().into())
            }
            Request::call_broadcast(call_args) => {
                let parameters = call_broadcast::Parameters::from(call_args);

                ProcedureMessage::CallBroadcast(parameters.into())
            }
        }
    }
}

impl From<RpcMessage<null::Procedure>> for ProcedureMessage {
    fn from(message: RpcMessage<null::Procedure>) -> Self {
        ProcedureMessage::Null(message)
    }
}

impl From<RpcMessage<set::Procedure>> for ProcedureMessage {
    fn from(message: RpcMessage<set::Procedure>) -> Self {
        ProcedureMessage::Set(message)
    }
}

impl From<RpcMessage<unset::Procedure>> for ProcedureMessage {
    fn from(message: RpcMessage<unset::Procedure>) -> Self {
        ProcedureMessage::Unset(message)
    }
}

impl From<RpcMessage<get_port::Procedure>> for ProcedureMessage {
    fn from(message: RpcMessage<get_port::Procedure>) -> Self {
        ProcedureMessage::GetPort(message)
    }
}

impl From<RpcMessage<dump::Procedure>> for ProcedureMessage {
    fn from(message: RpcMessage<dump::Procedure>) -> Self {
        ProcedureMessage::Dump(message)
    }
}

impl From<RpcMessage<call_broadcast::Procedure>> for ProcedureMessage {
    fn from(message: RpcMessage<call_broadcast::Procedure>) -> Self {
        ProcedureMessage::CallBroadcast(message)
    }
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
            RequestId::null => {
                Ok(
                    RpcMessage::<null::Procedure>::deserialize(deserializer)?
                        .into()
                )
            },
            RequestId::set => {
                Ok(
                    RpcMessage::<set::Procedure>::deserialize(deserializer)?
                        .into()
                )
            }
            RequestId::unset => {
                Ok(
                    RpcMessage::<unset::Procedure>::deserialize(deserializer)?
                        .into()
                )
            }
            RequestId::get_port => {
                Ok(
                    RpcMessage::<get_port::Procedure>::deserialize(
                        deserializer,
                    )?.into()
                )
            }
            RequestId::dump => {
                Ok(
                    RpcMessage::<dump::Procedure>::deserialize(deserializer)?
                        .into()
                )
            }
            RequestId::call_broadcast => {
                Ok(
                    RpcMessage::<call_broadcast::Procedure>::deserialize(
                        deserializer,
                    )?.into()
                )
            }
        }
    }
}
