use serde_bytes;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallResult {
    port: u32,
    #[serde(with = "serde_bytes")]
    result: Vec<u8>,
}
