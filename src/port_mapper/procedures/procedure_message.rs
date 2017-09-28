use serde::{Deserialize, Deserializer};

use super::call_broadcast;
use super::call_broadcast::CallBroadcast;
use super::dump;
use super::dump::Dump;
use super::get_port;
use super::get_port::GetPort;
use super::null;
use super::null::Null;
use super::set;
use super::set::Set;
use super::unset;
use super::unset::Unset;
use super::super::requests::{Request, RequestId};
use super::super::super::message::RpcMessage;
use super::super::super::service::DeserializeWithHint;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProcedureMessage {
    Null(RpcMessage<Null>),
    Set(RpcMessage<Set>),
    Unset(RpcMessage<Unset>),
    GetPort(RpcMessage<GetPort>),
    Dump(RpcMessage<Dump>),
    CallBroadcast(RpcMessage<CallBroadcast>),
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

impl From<RpcMessage<Null>> for ProcedureMessage {
    fn from(message: RpcMessage<Null>) -> Self {
        ProcedureMessage::Null(message)
    }
}

impl From<RpcMessage<Set>> for ProcedureMessage {
    fn from(message: RpcMessage<Set>) -> Self {
        ProcedureMessage::Set(message)
    }
}

impl From<RpcMessage<Unset>> for ProcedureMessage {
    fn from(message: RpcMessage<Unset>) -> Self {
        ProcedureMessage::Unset(message)
    }
}

impl From<RpcMessage<GetPort>> for ProcedureMessage {
    fn from(message: RpcMessage<GetPort>) -> Self {
        ProcedureMessage::GetPort(message)
    }
}

impl From<RpcMessage<Dump>> for ProcedureMessage {
    fn from(message: RpcMessage<Dump>) -> Self {
        ProcedureMessage::Dump(message)
    }
}

impl From<RpcMessage<CallBroadcast>> for ProcedureMessage {
    fn from(message: RpcMessage<CallBroadcast>) -> Self {
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
                Ok(RpcMessage::<Null>::deserialize(deserializer)?.into())
            },
            RequestId::set => {
                Ok(RpcMessage::<Set>::deserialize(deserializer)?.into())
            }
            RequestId::unset => {
                Ok(RpcMessage::<Unset>::deserialize(deserializer)?.into())
            }
            RequestId::get_port => {
                Ok(RpcMessage::<GetPort>::deserialize(deserializer)?.into())
            }
            RequestId::dump => {
                Ok(RpcMessage::<Dump>::deserialize(deserializer)?.into())
            }
            RequestId::call_broadcast => {
                Ok(
                    RpcMessage::<CallBroadcast>::deserialize(deserializer)?
                        .into()
                )
            }
        }
    }
}
