use super::call_args::CallArgs;
use super::call_result::CallResult;
use super::mapping::Mapping;

onc_rpc! {
    program(port_mapper::PortMapper) {
        id = 100_000;
        version = 2;
        connect = Connect;

        procedures {
            0 => null(),
            1 => set(program: Mapping) -> bool => SetResult,
            2 => unset(program: Mapping) -> bool => UnsetResult,
            3 => get_port(program: Mapping) -> u32 => GetPortResult,
            4 => dump() -> Vec<Mapping> => DumpResult,
            5 => call_broadcast(arguments: CallArgs) -> CallResult
                => CallBroadcastResult,
        }
    }
}
