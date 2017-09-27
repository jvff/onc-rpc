use super::protocol::Protocol;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mapping {
    pub program: u32,
    pub version: u32,
    pub protocol: Protocol,
    pub port: u32,
}
