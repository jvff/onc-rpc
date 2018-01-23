use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};

/// Protocol to use for remote procedure calls.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Protocol {
    /// TCP/IP.
    Tcp,

    /// UDP/IP.
    Udp,
}

impl Serialize for Protocol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let protocol_code = match *self {
            Protocol::Tcp => 6,
            Protocol::Udp => 17,
        };

        serializer.serialize_u32(protocol_code)
    }
}

impl<'de> Deserialize<'de> for Protocol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u32(ProtocolVisitor)
    }
}

struct ProtocolVisitor;

impl<'de> Visitor<'de> for ProtocolVisitor {
    type Value = Protocol;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("either the value 6 or the value 17")
    }

    fn visit_u32<E>(self, value: u32) -> Result<Protocol, E>
    where
        E: Error,
    {
        match value {
            6 => Ok(Protocol::Tcp),
            17 => Ok(Protocol::Udp),
            _ => {
                Err(
                    E::custom(format!("{} is not a valid protocol code", value))
                )
            }
        }
    }
}
