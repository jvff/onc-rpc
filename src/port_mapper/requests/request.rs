use super::call_args::CallArgs;
use super::mapping::Mapping;
use super::request_id::RequestId;
use super::super::super::service::RpcRequest;

#[allow(non_camel_case_types)]
pub enum Request {
    null,
    set(Mapping),
    unset(Mapping),
    get_port(Mapping),
    dump,
    call_broadcast(CallArgs),
}

impl RpcRequest for Request {
    type ResponseHint = RequestId;

    fn response_hint(&self) -> RequestId {
        RequestId::from(self)
    }
}
