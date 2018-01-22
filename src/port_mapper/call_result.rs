use serde_bytes;

/// Result of a forwarded call request.
///
/// Data returned by the remote procedure of another program called through the
/// port mapper program.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallResult {
    port: u32,
    #[serde(with = "serde_bytes")]
    result: Vec<u8>,
}
