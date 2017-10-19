use std::net::SocketAddr;

use super::hash_map_port_mapper::HashMapPortMapper;
use super::port_mapper::PortMapperServerWrapper;

pub struct PortMapperServer {
    server: PortMapperServerWrapper,
}

impl PortMapperServer {
    pub fn new(address: SocketAddr) -> Self {
        PortMapperServer {
            server: PortMapperServerWrapper::new(address),
        }
    }

    pub fn serve(&self) {
        let port_mapper = HashMapPortMapper::new();

        self.server.serve(port_mapper);
    }
}
