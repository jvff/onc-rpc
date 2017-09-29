use serde_xdr::OpaqueData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallResult {
    port: u32,
    result: OpaqueData,
}
