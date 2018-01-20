use serde_bytes;

/// Arguments for forwarded call request.
///
/// When a port mapper is called to forward a remote call request, these
/// parameters are necessary to select the program instance, the procedure to
/// call and what arguments should be used.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallArgs {
    program: u32,
    version: u32,
    procedure: u32,
    #[serde(with = "serde_bytes")]
    args: Vec<u8>,
}
