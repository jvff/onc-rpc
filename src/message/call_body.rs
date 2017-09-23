use super::auth_data::AuthData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallBody<T> {
    rpc_version: u32,
    program: u32,
    version: u32,
    procedure: u32,
    credentials: AuthData,
    verifier: AuthData,
    parameters: T,
}
