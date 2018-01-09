use serde::{Deserialize, Serialize};

use super::call_body::CallBody;
use super::call_header::CallHeader;
use super::rpc_body::RpcBody;
use super::super::errors::Result;
use super::super::rpc::{RpcCall, RpcProcedure};

/// Representation of an ONC-RPC message.
///
/// This is a representation data contained in an ONC-RPC message, either when
/// requesting or responding a procedure call.
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

impl<C> From<C> for RpcMessage<C::Procedure>
where
    C: RpcCall,
    C::Procedure: RpcProcedure<Parameters = C>,
{
    fn from(rpc_call: C) -> Self {
        RpcMessage {
            transaction_id: u32::max_value(),
            body: rpc_call.into(),
        }
    }
}

impl<P> RpcMessage<P>
where
    P: RpcProcedure,
{
    /// Create a new ONC-RPC message for a remote procedure call request.
    pub fn new_call(
        transaction_id: u32,
        call_header: CallHeader,
        parameters: P::Parameters,
    ) -> Self {
        let call_body = CallBody::new(call_header, parameters);

        RpcMessage {
            transaction_id,
            body: RpcBody::Call(call_body),
        }
    }

    /// Create a new ONC-RPC message for a remote procedure call reply.
    ///
    /// The transaction ID is configured later by the service responsible for
    /// matching requests and replies.
    pub fn from_reply(rpc_reply: P::ResultData) -> Self {
        RpcMessage {
            transaction_id: u32::max_value(),
            body: RpcBody::Reply(rpc_reply.into()),
        }
    }

    /// Obtain the parameters of a remote procedure call request.
    ///
    /// Fails if the message is a remote procedure call reply.
    pub fn into_parameters(self) -> Result<P::Parameters> {
        self.body.into_parameters()
    }

    /// Obtain the result of a remote procedure call.
    ///
    /// Fails if the message is a remote procedure call request.
    pub fn into_reply(self) -> Result<P::ResultData> {
        self.body.into_reply()
    }
}
