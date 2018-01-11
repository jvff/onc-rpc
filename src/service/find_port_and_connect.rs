use std::net::{IpAddr, SocketAddr};

use futures::{Future, IntoFuture, Poll};
use futures::future::{AndThen, Flatten, FutureResult, Join};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::multiplex::ClientService;

use super::connect::Connect;
use super::super::errors::{Error, ErrorKind};
use super::super::port_mapper::{GetPortResult, Mapping, PortMapper,
                                PortMapperAsyncClient, PortMapperConnect};
use super::super::record::RecordProtocol;

/// An asynchronous connection attempt (with port discovery) to a remote
/// program.
///
/// The type parameter `T` is the asynchronous client interface to the remote
/// program. An initial connection is made to the port mapper program running on
/// the remote machine in order to obtain the port number of the desired
/// program instance. Connection is then performed through TCP, and
/// communication is through the record protocol.
pub struct FindPortAndConnect<T>
where
    T: From<ClientService<TcpStream, RecordProtocol>>,
{
    future: AndThen<
        GetPortStep,
        Flatten<FutureResult<Connect<T>, Error>>,
        fn((u32, (IpAddr, Handle))) -> Flatten<FutureResult<Connect<T>, Error>>,
    >,
}

impl<T> FindPortAndConnect<T>
where
    T: From<ClientService<TcpStream, RecordProtocol>>,
{
    /// Create a new connection attempt with port discovery to the remote
    /// program.
    ///
    /// An attempt will be made to connect to the port mapper program running on
    /// the remote machine at the IP address specified in `address`. The port
    /// for the program instance specified by `program_id` and `program_version`
    /// will be requested and finally used for a connection attempt to the
    /// program instance.
    ///
    /// Both connection attempts use the event reactor of the given `handle`.
    pub fn new(
        address: IpAddr,
        program_id: u32,
        program_version: u32,
        handle: &Handle,
    ) -> Self {
        let parameters = (
            address.clone(),
            program_id,
            program_version,
            handle.clone()
        );

        let connect_to_port_mapper =
            PortMapperAsyncClient::connect_to_known_port(
                SocketAddr::new(address.clone(), 111),
                handle,
            )
            .join(Ok(parameters).into_future());

        let get_port =
            connect_to_port_mapper.and_then(Self::get_port as GetPortFn);
        let connect = get_port.and_then(
            Self::connect as
                fn(
                    (u32, (IpAddr, Handle)),
                ) -> Flatten<FutureResult<Connect<T>, Error>>,
        );

        FindPortAndConnect {
            future: connect
        }
    }

    fn get_port(
        (port_mapper, (address, program_id, program_version, handle)):
            (PortMapperAsyncClient, (IpAddr, u32, u32, Handle)),
    ) -> GetPort {
        let program = Mapping::of_program(program_id, program_version);
        let parameters = (address, handle);

        port_mapper
            .get_port(program)
            .join(Ok(parameters).into_future())
    }

    fn connect(
        (port, (ip_address, handle)): (u32, (IpAddr, Handle)),
    ) -> Flatten<FutureResult<Connect<T>, Error>> {
        let result = if port <= u16::max_value() as u32 {
            let address = SocketAddr::new(ip_address, port as u16);

            Ok(Connect::new(address, &handle))
        } else {
            Err(ErrorKind::InvalidRemotePort(port).into())
        };

        result.into_future().flatten()
    }
}

impl<T> Future for FindPortAndConnect<T>
where
    T: From<ClientService<TcpStream, RecordProtocol>>,
{
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

type GetPort = Join<GetPortResult, FutureResult<(IpAddr, Handle), Error>>;

type GetPortStep = AndThen<ConnectToPortMapperStep, GetPort, GetPortFn>;
type ConnectToPortMapperStep =
    Join<PortMapperConnect, FutureResult<(IpAddr, u32, u32, Handle), Error>>;

type GetPortFn =
    fn((PortMapperAsyncClient, (IpAddr, u32, u32, Handle))) -> GetPort;
