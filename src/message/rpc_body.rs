use super::call_body::CallBody;
use super::reply_body::ReplyBody;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RpcBody<C, R> {
    Call(CallBody<C>),
    Reply(ReplyBody<R>),
}
