use super::accepted_status::AcceptedStatus;
use super::auth_data::AuthData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AcceptedReply<T> {
    verifier: AuthData,
    reply_data: AcceptedStatus<T>,
}
