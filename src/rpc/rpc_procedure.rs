use super::rpc_program::RpcProgram;

/// Specification of a remote procedure interface.
pub trait RpcProcedure {
    /// The program that the procedure is associated to.
    type Program: RpcProgram;
    /// The type that contains all the parameters for the procedure.
    type Parameters: Sized;
    /// The type of the resulting data.
    type ResultData: Sized;

    /// The procedure identification code.
    fn procedure() -> u32;
}
