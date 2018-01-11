/// Attempt to construct a type by conversion.
///
/// This is similar to `From`, but may fail.
pub trait TryFrom<T> {
    /// The error type of a failed attempt.
    type Error;

    /// Attempt to convert a value into a new instance.
    fn try_from(value: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
    ;
}
