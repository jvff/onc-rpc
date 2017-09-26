#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AcceptedStatus<T> {
    Success(T),
    ProgramUnavailable,
    ProgramVersionMismatch {
        min: u32,
        max: u32,
    },
    ProcedureUnavailable,
    GarbageArguments,
    SystemError,
}
