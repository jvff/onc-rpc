use super::super::rpc::RpcProgram;

pub struct PortMapperProgram;

impl RpcProgram for PortMapperProgram {
    fn program() -> u32 {
        100_000
    }

    fn version() -> u32 {
        2
    }
}
