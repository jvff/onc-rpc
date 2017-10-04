use std::mem;
use std::net::{IpAddr, SocketAddr};

use futures::{Async, Future, Poll};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::multiplex::ClientService;

use self::port_mapper_status::PortMapperStatus;
use super::connect::Connect;
use super::super::errors::{Error, ErrorKind};
use super::super::port_mapper::{GetPortResult, Mapping, PortMapper};
use super::super::record::RecordProtocol;

enum PortStatus {
    Waiting,
    Requested(GetPortResult),
    Ready(u16),
}

enum ConnectStatus<T>
where
    T: From<ClientService<TcpStream, RecordProtocol>>,
{
    Waiting,
    Connecting(Connect<T>)
}

pub struct FindPortAndConnect<T>
where
    T: From<ClientService<TcpStream, RecordProtocol>>,
{
    address: IpAddr,
    program_id: u32,
    program_version: u32,
    handle: Handle,
    port_mapper: PortMapperStatus,
    port: PortStatus,
    connect: ConnectStatus<T>,
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
        let port_mapper = PortMapper::connect_to_known_port(
            SocketAddr::new(address.clone(), 111),
            handle,
        );

        FindPortAndConnect {
            address,
            program_id,
            program_version,
            handle: handle.clone(),
            port_mapper: PortMapperStatus::Connecting(port_mapper),
            port: PortStatus::Waiting,
            connect: ConnectStatus::Waiting,
        }
    }
}

impl<T> Future for FindPortAndConnect<T>
where
    T: From<ClientService<TcpStream, RecordProtocol>>,
{
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        poll_connect_status(
            &self.address,
            &self.handle,
            self.program_id,
            self.program_version,
            &mut self.connect,
            &mut self.port,
            &mut self.port_mapper,
        )
    }
}

fn poll_connect_status<T>(
    ip_address: &IpAddr,
    handle: &Handle,
    program_id: u32,
    program_version: u32,
    connect_status: &mut ConnectStatus<T>,
    port_status: &mut PortStatus,
    port_mapper_status: &mut PortMapperStatus,
) -> Poll<T, Error>
where
    T: From<ClientService<TcpStream, RecordProtocol>>,
{
    match *connect_status {
        ConnectStatus::Waiting => {
            let port = try_ready!(
                poll_port_status(
                    program_id,
                    program_version,
                    port_status,
                    port_mapper_status,
                )
            );

            let address = SocketAddr::new(ip_address.clone(), port);

            let new_connect_status = ConnectStatus::Connecting(
                Connect::new(address, handle),
            );

            mem::replace(connect_status, new_connect_status);

            poll_connect_status(
                ip_address,
                handle,
                program_id,
                program_version,
                connect_status,
                port_status,
                port_mapper_status,
            )
        }
        ConnectStatus::Connecting(ref mut connect) => {
            let program = try_ready!(connect.poll());

            Ok(Async::Ready(program))
        }
    }
}

fn poll_port_status(
    program_id: u32,
    program_version: u32,
    port_status: &mut PortStatus,
    port_mapper_status: &mut PortMapperStatus,
) -> Poll<u16, Error> {
    let moved_port_status = mem::replace(port_status, PortStatus::Waiting);

    let (poll_result, new_status) = match moved_port_status {
        PortStatus::Waiting => {
            let port_mapper_poll_result = port_mapper_status.poll();

            match port_mapper_poll_result {
                Ok(Async::Ready(())) => {
                    if let PortMapperStatus::Connected(ref mut port_mapper) =
                        *port_mapper_status
                    {
                        let program =
                            Mapping::of_program(program_id, program_version);

                        let port = port_mapper.get_port(program);
                        let new_status = PortStatus::Requested(port);

                        (None, new_status)
                    } else {
                        unreachable!(
                            "PortMapperStatus was polled and returned
                             Async::Ready"
                        );
                    }
                }
                Ok(Async::NotReady) => {
                    let poll_result = Ok(Async::NotReady);
                    let new_status = PortStatus::Waiting;

                    (Some(poll_result), new_status)
                }
                Err(error) => {
                    let poll_result = Err(error);
                    let new_status = PortStatus::Waiting;

                    (Some(poll_result), new_status)
                }
            }
        }
        PortStatus::Requested(mut result) => {
            let result_poll_result = result.poll();

            match result_poll_result {
                Ok(Async::Ready(port)) => {

                    if port > u16::max_value() as u32 {
                        bail!(ErrorKind::InvalidRemotePort(port));
                    }

                    let new_status = PortStatus::Ready(port as u16);

                    (None, new_status)
                }
                Ok(Async::NotReady) => {
                    let poll_result = Ok(Async::NotReady);
                    let new_status = PortStatus::Requested(result);

                    (Some(poll_result), new_status)
                }
                Err(error) => {
                    let poll_result = Err(error);
                    let new_status = PortStatus::Requested(result);

                    (Some(poll_result), new_status)
                }
            }
        }
        PortStatus::Ready(port) => {
            let poll_result = Ok(Async::Ready(port));
            let new_status = PortStatus::Ready(port);

            (Some(poll_result), new_status)
        }
    };

    mem::replace(port_status, new_status);

    if let Some(poll_result) = poll_result {
        poll_result
    } else {
        poll_port_status(
            program_id,
            program_version,
            port_status,
            port_mapper_status,
        )
    }
}

mod port_mapper_status;
