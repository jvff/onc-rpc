use super::protocol::Protocol;

/// A mapping between a program instance and a port number.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mapping {
    /// Program ID number.
    pub program: u32,

    /// Program version.
    pub version: u32,

    /// Protocol used to communicate.
    pub protocol: Protocol,

    /// Port used to communicate.
    pub port: u32,
}

impl Mapping {
    /// Create a mapping representing a program instance.
    pub fn of_program(program: u32, version: u32) -> Self {
        Mapping {
            program,
            version,
            protocol: Protocol::Tcp,
            port: u32::max_value(),
        }
    }
}
