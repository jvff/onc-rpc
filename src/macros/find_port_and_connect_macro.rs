#[macro_export]
macro_rules! onc_rpc_program_find_port_and_connect {
    ( $program:ident, $id:expr, $version:expr ) => {
        mod find_port_and_connect {
            use std::mem;
            use std::net::{IpAddr, SocketAddr};

            use futures::{Async, Future, Poll};
            use tokio_core::reactor::Handle;

            use $crate::port_mapper::{GetPortResult, Mapping, PortMapper,
                                      PortMapperConnect};
            use $crate::{Error, ErrorKind};

            use super::{Connect, $program as Program};

            enum PortMapperStatus {
                Waiting,
                Connecting(PortMapperConnect),
                Connected(PortMapper),
            }

            enum PortStatus {
                Waiting,
                Requested(GetPortResult),
                Ready(u16),
            }

            enum ConnectStatus {
                Waiting,
                Connecting(Connect)
            }

            pub struct FindPortAndConnect {
                address: IpAddr,
                handle: Handle,
                port_mapper: PortMapperStatus,
                port: PortStatus,
                connect: ConnectStatus,
            }

            impl FindPortAndConnect {
                pub fn new(address: IpAddr, handle: &Handle) -> Self {
                    let port_mapper = PortMapper::connect_to_known_port(
                        SocketAddr::new(address.clone(), 111),
                        handle,
                    );

                    FindPortAndConnect {
                        address,
                        handle: handle.clone(),
                        port_mapper: PortMapperStatus::Connecting(port_mapper),
                        port: PortStatus::Waiting,
                        connect: ConnectStatus::Waiting,
                    }
                }
            }

            impl Future for FindPortAndConnect {
                type Item = Program;
                type Error = Error;

                fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
                    poll_connect_status(
                        &self.address,
                        &self.handle,
                        &mut self.connect,
                        &mut self.port,
                        &mut self.port_mapper,
                    )
                }
            }

            fn poll_connect_status(
                ip_address: &IpAddr,
                handle: &Handle,
                connect_status: &mut ConnectStatus,
                port_status: &mut PortStatus,
                port_mapper_status: &mut PortMapperStatus,
            ) -> Poll<Program, Error> {
                match *connect_status {
                    ConnectStatus::Waiting => {
                        let port = try_ready!(
                            poll_port_status(port_status, port_mapper_status)
                        );

                        let address = SocketAddr::new(ip_address.clone(), port);

                        let new_connect_status = ConnectStatus::Connecting(
                            Connect::new(address, handle),
                        );

                        mem::replace(connect_status, new_connect_status);

                        poll_connect_status(
                            ip_address,
                            handle,
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
                port_status: &mut PortStatus,
                port_mapper_status: &mut PortMapperStatus,
            ) -> Poll<u16, Error> {
                let moved_port_status =
                    mem::replace(port_status, PortStatus::Waiting);

                let (poll_result, new_status) = match moved_port_status {
                    PortStatus::Waiting => {
                        let port_mapper_poll_result =
                            poll_port_mapper_status(port_mapper_status);

                        match port_mapper_poll_result {
                            Ok(Async::Ready(())) => {
                                if let
                                    PortMapperStatus::Connected(
                                        ref mut port_mapper
                                    ) = *port_mapper_status
                                {
                                    let program =
                                        Mapping::of_program($id, $version);

                                    let port = port_mapper.get_port(program);
                                    let new_status =
                                        PortStatus::Requested(port);

                                    (None, new_status)
                                } else {
                                    unreachable!(
                                        "PortMapperStatus was polled and
                                         returned Async::Ready"
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
                    poll_port_status(port_status, port_mapper_status)
                }
            }

            fn poll_port_mapper_status(
                port_mapper_status: &mut PortMapperStatus,
            ) -> Poll<(), Error> {
                let moved_port_mapper_status =
                    mem::replace(port_mapper_status, PortMapperStatus::Waiting);

                let (poll_result, new_status) = match moved_port_mapper_status {
                    PortMapperStatus::Waiting => {
                        unreachable!(
                            "connection to PortMapper has already started"
                        );
                    }
                    PortMapperStatus::Connecting(mut connect) => {
                        let connect_poll_result = connect.poll();

                        match connect_poll_result {
                            Ok(Async::Ready(port_mapper)) => {
                                let new_status =
                                    PortMapperStatus::Connected(port_mapper);

                                (None, new_status)
                            }
                            Ok(Async::NotReady) => {
                                let poll_result = Ok(Async::NotReady);
                                let new_status =
                                    PortMapperStatus::Connecting(connect);

                                (Some(poll_result), new_status)
                            }
                            Err(error) => {
                                let poll_result = Err(error);
                                let new_status =
                                    PortMapperStatus::Connecting(connect);

                                (Some(poll_result), new_status)
                            }
                        }
                    }
                    PortMapperStatus::Connected(port_mapper) => {
                        let poll_result = Ok(Async::Ready(()));
                        let new_status =
                            PortMapperStatus::Connected(port_mapper);

                        (Some(poll_result), new_status)
                    }
                };

                mem::replace(port_mapper_status, new_status);

                if let Some(poll_result) = poll_result {
                    poll_result
                } else {
                    poll_port_mapper_status(port_mapper_status)
                }
            }
        }

        pub use self::find_port_and_connect::FindPortAndConnect;
    }
}
