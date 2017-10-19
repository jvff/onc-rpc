use serde::{Deserialize, Serialize};

use super::deserialize_with_hint::DeserializeWithHint;
use super::rpc_request::RpcRequest;
use super::try_from::TryFrom;

pub trait RpcServiceConfig {
    type Request: RpcRequest + TryFrom<Self::ProcedureMessage>;
    type Response: TryFrom<Self::ProcedureMessage>;
    type ProcedureMessage:
        From<Self::Request>
            + From<Self::Response>
            + Serialize
            + Deserialize<'static>
            + DeserializeWithHint<<Self::Request as RpcRequest>::ResponseHint>;
}
