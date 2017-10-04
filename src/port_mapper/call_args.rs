use serde_xdr::VariableLengthOpaqueData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallArgs {
    program: u32,
    version: u32,
    procedure: u32,
    args: VariableLengthOpaqueData,
}
