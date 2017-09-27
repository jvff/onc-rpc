use super::call_broadcast::CallBroadcast;
use super::dump::Dump;
use super::get_port::GetPort;
use super::null::Null;
use super::set::Set;
use super::unset::Unset;
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
