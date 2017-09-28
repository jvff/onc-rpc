use serde::Serialize;

use super::deserialize_with_hint::DeserializeWithHint;
use super::rpc_request::RpcRequest;
use super::try_from::TryFrom;

pub trait RpcServiceConfig {
    type Request: RpcRequest;
    type Response: TryFrom<Self::ProcedureMessage>;
    type ProcedureMessage:
        From<Self::Request>
            + Serialize
            + DeserializeWithHint<<Self::Request as RpcRequest>::ResponseHint>;
}