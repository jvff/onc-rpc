#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum AuthFlavor {
    Null,
    Unix,
    Short,
    Des,
}

#[cfg(test)]
mod tests {
    use serde_xdr;

    use super::AuthFlavor;

    fn serialize(auth_flavor: AuthFlavor, value: u32) {
        let serialized = serde_xdr::to_bytes(&auth_flavor).unwrap();
        let serialized_value: u32 = serde_xdr::from_bytes(&serialized).unwrap();

        assert_eq!(serialized_value, value);
    }

    #[test]
    fn serialize_null() {
        serialize(AuthFlavor::Null, 0);
    }

    #[test]
    fn serialize_unix() {
        serialize(AuthFlavor::Unix, 1);
    }

    #[test]
    fn serialize_short() {
        serialize(AuthFlavor::Short, 2);
    }

    #[test]
    fn serialize_des() {
        serialize(AuthFlavor::Des, 3);
    }

    fn deserialize(value: u32, auth_flavor: AuthFlavor) {
        let serialized = serde_xdr::to_bytes(&value).unwrap();
        let deserialized: AuthFlavor =
            serde_xdr::from_bytes(&serialized).unwrap();

        assert_eq!(deserialized, auth_flavor);
    }

    #[test]
    fn deserialize_null() {
        deserialize(0, AuthFlavor::Null);
    }

    #[test]
    fn deserialize_unix() {
        deserialize(1, AuthFlavor::Unix);
    }

    #[test]
    fn deserialize_short() {
        deserialize(2, AuthFlavor::Short);
    }

    #[test]
    fn deserialize_des() {
        deserialize(3, AuthFlavor::Des);
    }
}
