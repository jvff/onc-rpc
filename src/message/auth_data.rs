use serde_xdr::OpaqueData;

use super::auth_flavor::AuthFlavor;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthData {
    flavor: AuthFlavor,
    body: OpaqueData,
}

impl Default for AuthData {
    fn default() -> Self {
        AuthData {
            flavor: AuthFlavor::None,
            body: OpaqueData::new(),
        }
    }
}
