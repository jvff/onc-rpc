use serde::Deserializer;

/// Additional deserialization method with a hint parameter.
pub trait DeserializeWithHint<H> {
    /// Attempts to construct a new type by deserializing using a hint
    /// parameter.
    ///
    /// The hint parameter is normally used as extra information to use during
    /// the deserialization process.
    fn deserialize_with_hint<'de, D>(
        hint: H,
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
        Self: Sized,
    ;
}
