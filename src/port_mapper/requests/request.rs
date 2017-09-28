use super::call_args::CallArgs;
use super::mapping::Mapping;
use super::request_id::RequestId;
use super::super::super::service::RpcRequest;

pub enum Request {
    Null,
    Set(Mapping),
    Unset(Mapping),
    GetPort(Mapping),
    Dump,
    CallBroadcast(CallArgs),
}

impl RpcRequest for Request {
    type ResponseHint = RequestId;

    fn response_hint(&self) -> RequestId {
        RequestId::from(self)
    }
}
