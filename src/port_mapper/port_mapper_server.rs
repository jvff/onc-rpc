use std::net::SocketAddr;

use super::hash_map_port_mapper::HashMapPortMapper;
use super::port_mapper::PortMapperServerWrapper;

/// A simple port mapper server implementation.
///
/// Uses a hash map to map registered programs and their port numbers.
///
/// # Example
///
/// ```
/// use onc_rpc::port_mapper::PortMapperServer;
/// # use std::thread;
///
/// let address = "0.0.0.0:111".parse().unwrap();
/// let server = PortMapperServer::new(address);
///
/// # thread::spawn(move || {
/// server.serve();
/// # });
/// ```
pub struct PortMapperServer {
    server: PortMapperServerWrapper,
}

impl PortMapperServer {
    /// Create a new server instance listening on a socket address.
    ///
    /// The standard port for the port mapper program as defined by the
    /// [RFC][port_mapper_rfc] is 111.
    ///
    /// [port_mapper_rfc]: https://tools.ietf.org/html/rfc1057#appendix-A.1
    pub fn new(address: SocketAddr) -> Self {
        PortMapperServer {
            server: PortMapperServerWrapper::new(address),
        }
    }

    /// Start the port mapper server.
    ///
    /// This method will block until the server is shut down.
    pub fn serve(&self) {
        let port_mapper = HashMapPortMapper::new();

        self.server.serve(port_mapper);
    }
}
