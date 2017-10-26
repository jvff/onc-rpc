use serde_bytes;

use super::auth_flavor::AuthFlavor;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthData {
    flavor: AuthFlavor,
    #[serde(with = "serde_bytes")]
    body: Vec<u8>,
}

impl Default for AuthData {
    fn default() -> Self {
        AuthData {
            flavor: AuthFlavor::None,
            body: Vec::new(),
        }
    }
}
