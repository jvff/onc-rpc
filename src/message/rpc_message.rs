use super::rpc_body::RpcBody;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RpcMessage<R, C> {
    transaction_id: u32,
    body: RpcBody<R, C>,
}
