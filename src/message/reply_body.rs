use super::accepted_reply::AcceptedReply;
use super::rejected_reply::RejectedReply;
use super::super::errors::{ErrorKind, Result};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReplyBody<T> {
    Accepted(AcceptedReply<T>),
    Denied(RejectedReply),
}

impl<T> From<T> for ReplyBody<T> {
    fn from(data: T) -> Self {
        ReplyBody::Accepted(data.into())
    }
}

impl<T> Into<Result<T>> for ReplyBody<T> {
    fn into(self) -> Result<T> {
        match self {
            ReplyBody::Accepted(reply) => reply.into(),
            ReplyBody::Denied(_) => bail!(ErrorKind::RemoteCallDenied),
        }
    }
}
