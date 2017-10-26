use serde_bytes;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallArgs {
    program: u32,
    version: u32,
    procedure: u32,
    #[serde(with = "serde_bytes")]
    args: Vec<u8>,
}
