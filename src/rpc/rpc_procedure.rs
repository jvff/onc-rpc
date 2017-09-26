pub trait RpcProcedure {
    type Parameters: Sized;
    type ResultData: Sized;

    fn program(&self) -> u32;
    fn version(&self) -> u32;
    fn procedure(&self) -> u32;
}
