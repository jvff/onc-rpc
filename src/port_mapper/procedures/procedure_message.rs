use serde::{Deserialize, Deserializer};

use super::call_broadcast::CallBroadcast;
use super::dump::Dump;
use super::get_port::GetPort;
use super::null::Null;
use super::set::Set;
use super::unset::Unset;
use super::super::requests::RequestId;
use super::super::super::message::RpcMessage;

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

impl ProcedureMessage {
    pub fn deserialize_with_hint<'de, D>(
        hint: RequestId,
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match hint {
            RequestId::Null => {
                Ok(RpcMessage::<Null>::deserialize(deserializer)?.into())
            },
            RequestId::Set => {
                Ok(RpcMessage::<Set>::deserialize(deserializer)?.into())
            }
            RequestId::Unset => {
                Ok(RpcMessage::<Unset>::deserialize(deserializer)?.into())
            }
            RequestId::GetPort => {
                Ok(RpcMessage::<GetPort>::deserialize(deserializer)?.into())
            }
            RequestId::Dump => {
                Ok(RpcMessage::<Dump>::deserialize(deserializer)?.into())
            }
            RequestId::CallBroadcast => {
                Ok(
                    RpcMessage::<CallBroadcast>::deserialize(deserializer)?
                        .into()
                )
            }
        }
    }
}
