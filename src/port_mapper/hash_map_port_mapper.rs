use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::future::{FutureResult, IntoFuture};

use super::call_args::CallArgs;
use super::call_result::CallResult;
use super::mapping::Mapping;
use super::port_mapper::PortMapper;
use super::protocol::Protocol;
use super::super::errors::Error;

#[derive(Clone, Eq, Hash, PartialEq)]
struct MappingWithoutPort {
    program: u32,
    version: u32,
    protocol: Protocol,
}

impl From<Mapping> for (MappingWithoutPort, u32) {
    fn from(mapping: Mapping) -> Self {
        let mapping_without_port = MappingWithoutPort {
            program: mapping.program,
            version: mapping.version,
            protocol: mapping.protocol,
        };

        (mapping_without_port, mapping.port)
    }
}

impl<'a> From<(&'a MappingWithoutPort, &'a u32)> for Mapping {
    fn from(
        (mapping_without_port, port): (&'a MappingWithoutPort, &'a u32),
    ) -> Self {
        Mapping {
            program: mapping_without_port.program,
            version: mapping_without_port.version,
            protocol: mapping_without_port.protocol,
            port: *port,
        }
    }
}

/// A simple port mapper program implementation using a shared hash map.
///
/// A hash map is used to map registered program instances to their port
/// numbers. The hash map is stored in an `Arc<Mutex<_>>`, which allows it to
/// be shared between different threads.
///
/// The behaviour of `Clone` for this implementation clones the shared reference
/// to the internal hash map, which means that all clones of the program share
/// the same hash map.
///
/// Even though the program instance can be used locally, it is generally
/// served on the network using
/// [`PortMapperServerWrapper`][port_mapper_server_wrapper]. See
/// [`PortMapperServer`][port_mapper_server] for a example that uses a
/// `HashMapPortMapper` internally.
///
/// [port_mapper_server_wrapper]: struct.PortMapperServerWrapper.html
/// [port_mapper_server]: struct.PortMapperServer.html
#[derive(Clone)]
pub struct HashMapPortMapper {
    map: Arc<Mutex<HashMap<MappingWithoutPort, u32>>>,
}

impl HashMapPortMapper {
    /// Create a new program instance with an empty hash map.
    pub fn new() -> Self {
        HashMapPortMapper {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl PortMapper for HashMapPortMapper {
    type Error = Error;
    type NullResult = FutureResult<(), Error>;
    type SetResult = FutureResult<bool, Error>;
    type UnsetResult = FutureResult<bool, Error>;
    type GetPortResult = FutureResult<u32, Error>;
    type DumpResult = FutureResult<Vec<Mapping>, Error>;
    type CallBroadcastResult = FutureResult<CallResult, Error>;

    fn null(&self) -> Self::NullResult {
        Ok(()).into_future()
    }

    fn set<M>(&self, program: M) -> Self::SetResult
    where
        M: Into<Mapping>,
    {
        if let Ok(mut map) = self.map.lock() {
            let mapping: Mapping = program.into();
            let (key, value) = mapping.into();

            if !map.contains_key(&key) {
                map.insert(key, value);

                Ok(true).into_future()
            } else {
                Ok(false).into_future()
            }
        } else {
            Ok(false).into_future()
        }
    }

    fn unset<M>(&self, program: M) -> Self::UnsetResult
    where
        M: Into<Mapping>,
    {
        if let Ok(mut map) = self.map.lock() {
            let mapping: Mapping = program.into();
            let (key, _) = mapping.into();

            map.remove(&key);

            Ok(true).into_future()
        } else {
            Ok(false).into_future()
        }
    }

    fn get_port<M>(&self, program: M) -> Self::GetPortResult
    where
        M: Into<Mapping>,
    {
        if let Ok(map) = self.map.lock() {
            let mapping: Mapping = program.into();
            let (key, _) = mapping.into();

            match map.get(&key) {
                Some(port) => Ok(*port).into_future(),
                None => Ok(0).into_future(),
            }
        } else {
            Ok(0).into_future()
        }
    }

    fn dump(&self) -> Self::DumpResult {
        if let Ok(map) = self.map.lock() {
            let mappings = map.iter().map(Mapping::from).collect();

            Ok(mappings).into_future()
        } else {
            Ok(Vec::new()).into_future()
        }
    }

    fn call_broadcast<A>(&self, _arguments: A) -> Self::CallBroadcastResult
    where
        A: Into<CallArgs>,
    {
        unimplemented!();
    }
}
