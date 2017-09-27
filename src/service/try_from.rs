pub trait TryFrom<T> {
    type Error;

    fn try_from(value: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
    ;
}
