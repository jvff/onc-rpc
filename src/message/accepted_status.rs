#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AcceptedStatus<T> {
    Success(T),
    ProgramUnavailable,
    ProgramMismatch {
        low: u32,
        high: u32,
    },
    ProcedureUnavailable,
    GarbageArguments,
    SystemError,
}
