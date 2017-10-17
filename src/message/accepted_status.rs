use super::super::errors::{ErrorKind, Result};

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

impl<T> From<T> for AcceptedStatus<T> {
    fn from(data: T) -> Self {
        AcceptedStatus::Success(data)
    }
}

impl<T> Into<Result<T>> for AcceptedStatus<T> {
    fn into(self) -> Result<T> {
        match self {
            AcceptedStatus::Success(result) => Ok(result),
            AcceptedStatus::ProgramUnavailable => {
                bail!(ErrorKind::ProgramUnavailable);
            }
            AcceptedStatus::ProgramVersionMismatch { min, max } => {
                bail!(ErrorKind::ProgramVersionMismatch(min, max));
            }
            AcceptedStatus::ProcedureUnavailable => {
                bail!(ErrorKind::ProcedureUnavailable);
            }
            AcceptedStatus::GarbageArguments => {
                bail!(ErrorKind::GarbageArguments)
            }
            AcceptedStatus::SystemError => bail!(ErrorKind::SystemError),
        }
    }
}
