use super::protocol::Protocol;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mapping {
    pub program: u32,
    pub version: u32,
    pub protocol: Protocol,
    pub port: u32,
}

impl Mapping {
    pub fn of_program(program: u32, version: u32) -> Self {
        Mapping {
            program,
            version,
            protocol: Protocol::Tcp,
            port: u32::max_value(),
        }
    }
}
