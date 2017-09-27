use super::rpc_program::RpcProgram;

pub trait RpcProcedure {
    type Program: RpcProgram;
    type Parameters: Sized;
    type ResultData: Sized;

    fn procedure(&self) -> u32;
}
