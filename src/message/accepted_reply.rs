use super::accepted_status::AcceptedStatus;
use super::auth_data::AuthData;
use super::super::errors::Result;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AcceptedReply<T> {
    verifier: AuthData,
    reply_data: AcceptedStatus<T>,
}

impl<T> From<T> for AcceptedReply<T> {
    fn from(data: T) -> Self {
        AcceptedReply {
            verifier: AuthData::default(),
            reply_data: data.into(),
        }
    }
}

impl<T> Into<Result<T>> for AcceptedReply<T> {
    fn into(self) -> Result<T> {
        self.reply_data.into()
    }
}
