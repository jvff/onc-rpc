use super::accepted_reply::AcceptedReply;
use super::rejected_reply::RejectedReply;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReplyBody<T> {
    Accepted(AcceptedReply<T>),
    Denied(RejectedReply),
}
