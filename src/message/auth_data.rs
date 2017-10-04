use serde_xdr::VariableLengthOpaqueData;

use super::auth_flavor::AuthFlavor;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthData {
    flavor: AuthFlavor,
    body: VariableLengthOpaqueData,
}

impl Default for AuthData {
    fn default() -> Self {
        AuthData {
            flavor: AuthFlavor::None,
            body: VariableLengthOpaqueData::new(),
        }
    }
}
