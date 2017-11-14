use super::call_args::CallArgs;
use super::call_result::CallResult;
use super::mapping::Mapping;
use super::super::service::Connect;

onc_rpc! {
    program(port_mapper::PortMapper) {
        id = 100_000;
        version = 2;

        export {
            AsyncClient as PortMapperAsyncClient;
            SyncClient as PortMapperSyncClient;
            Server as PortMapperServerWrapper;
        }

        procedures {
            0 => null() -> NullResult<()>,
            1 => set(program: Mapping) -> SetResult<bool>,
            2 => unset(program: Mapping) -> UnsetResult<bool>,
            3 => get_port(program: Mapping) -> GetPortResult<u32>,
            4 => dump() -> DumpResult<Vec<Mapping>>,
            5 => call_broadcast(arguments: CallArgs)
                -> CallBroadcastResult<CallResult>,
        }
    }
}

pub type PortMapperConnect = Connect<port_mapper::AsyncClient>;
