use serde::Deserializer;

pub trait DeserializeWithHint<H> {
    fn deserialize_with_hint<'de, D>(
        hint: H,
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
        Self: Sized,
    ;
}
