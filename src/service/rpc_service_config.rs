use serde::{Deserialize, Serialize};

use super::deserialize_with_hint::DeserializeWithHint;
use super::rpc_request::RpcRequest;
use super::try_from::TryFrom;

/// Associated types that define an RPC service configuration.
///
/// Defines the request and response types and the message type that's used for
/// transfering requests and responses.
pub trait RpcServiceConfig {
    /// The request type.
    type Request: RpcRequest + TryFrom<Self::ProcedureMessage>;

    /// The response type.
    type Response: TryFrom<Self::ProcedureMessage>;

    /// The procedure message type.
    ///
    /// Requests and responses are converted to and from messages of this type.
    type ProcedureMessage:
        From<Self::Request>
            + From<Self::Response>
            + Serialize
            + Deserialize<'static>
            + DeserializeWithHint<<Self::Request as RpcRequest>::ResponseHint>;
}
