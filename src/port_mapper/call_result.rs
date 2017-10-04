use serde_xdr::VariableLengthOpaqueData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallResult {
    port: u32,
    result: VariableLengthOpaqueData,
}
