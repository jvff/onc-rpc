use serde::{Deserialize, Serialize};

use super::rpc_body::RpcBody;
use super::super::rpc::{RpcCall, RpcProcedure};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(bound(
    serialize = "P::Parameters: Serialize, P::ResultData: Serialize"
))]
#[serde(bound(
    deserialize =
        "P::Parameters: Deserialize<'de>, P::ResultData: Deserialize<'de>"
))]
pub struct RpcMessage<P>
where
    P: RpcProcedure,
{
    transaction_id: u32,
    body: RpcBody<P::Parameters, P::ResultData>,
}

impl<C> From<C> for RpcMessage<C>
where
    C: RpcCall,
{
    fn from(rpc_call: C) -> Self {
        RpcMessage {
            transaction_id: u32::max_value(),
            body: rpc_call.into(),
        }
    }
}
