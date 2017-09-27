use super::super::program::PortMapperProgram;
use super::super::requests::Request;
use super::super::super::rpc::RpcProcedure;

pub trait PortMapperProcedure {
    type ResultData: Sized;

    fn procedure(&self) -> u32;
}

impl<T> RpcProcedure for T
where
    T: PortMapperProcedure
{
    type Program = PortMapperProgram;
    type Parameters = Request;
    type ResultData = <Self as PortMapperProcedure>::ResultData;

    fn procedure(&self) -> u32 {
        PortMapperProcedure::procedure(self)
    }
}
