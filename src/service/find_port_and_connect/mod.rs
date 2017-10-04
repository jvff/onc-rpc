use std::net::{IpAddr, SocketAddr};

use futures::{Future, IntoFuture, Poll};
use futures::future::{AndThen, Flatten, FutureResult, Join};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::multiplex::ClientService;

use super::connect::Connect;
use super::super::errors::{Error, ErrorKind};
use super::super::port_mapper::{GetPortResult, Mapping, PortMapper,
                                PortMapperConnect};
use super::super::record::RecordProtocol;

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
            PortMapper::connect_to_known_port(
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
            (PortMapper, (IpAddr, u32, u32, Handle)),
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

type GetPortFn = fn((PortMapper, (IpAddr, u32, u32, Handle))) -> GetPort;
