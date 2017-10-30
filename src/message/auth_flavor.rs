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

    fn serialize_test(auth_flavor: AuthFlavor, value: u32) {
        let serialized = serde_xdr::to_bytes(&auth_flavor).unwrap();
        let serialized_value: u32 = serde_xdr::from_bytes(&serialized).unwrap();

        assert_eq!(serialized_value, value);
    }

    fn deserialize_test(value: u32, auth_flavor: AuthFlavor) {
        let serialized = serde_xdr::to_bytes(&value).unwrap();
        let deserialized: AuthFlavor =
            serde_xdr::from_bytes(&serialized).unwrap();

        assert_eq!(deserialized, auth_flavor);
    }

    macro_rules! serialization_tests {
        ( $( $name:ident: $value:expr => $variant:expr ),* $(,)* ) => {
            mod serialize {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        serialize_test($variant, $value);
                    }
                )*
            }

            mod deserialize {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        deserialize_test($value, $variant);
                    }
                )*
            }
        };
    }

    serialization_tests! {
        null: 0 => AuthFlavor::Null,
        unix: 1 => AuthFlavor::Unix,
        short: 2 => AuthFlavor::Short,
        des: 3 => AuthFlavor::Des,
    }
}
