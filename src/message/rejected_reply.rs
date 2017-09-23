use super::auth_status::AuthStatus;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RejectedReply {
    RpcMismatch {
        low: u32,
        high: u32,
    },
    AuthFailure(AuthStatus),
}
