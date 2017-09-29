#[macro_export]
macro_rules! onc_rpc_program_connect {
    ( $program:ident ) => {
        mod connect {
            use super::$program;

            use std::net::SocketAddr;

            use futures::{Async, Future, Poll};
            use tokio_core::reactor::Handle;
            use tokio_proto::Connect as TcpConnect;
            use tokio_proto::TcpClient;
            use tokio_proto::multiplex::Multiplex;

            use $crate::{Error, ErrorKind, RecordProtocol, ResultExt};

            pub struct Connect {
                address: SocketAddr,
                connect: TcpConnect<Multiplex, RecordProtocol>,
            }

            impl Connect {
                pub fn new(address: SocketAddr, handle: &Handle) -> Self {
                    let client = TcpClient::new(RecordProtocol);

                    Connect {
                        address,
                        connect: client.connect(&address, handle),
                    }
                }
            }

            impl Future for Connect {
                type Item = $program;
                type Error = Error;

                fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
                    let poll_result = self.connect.poll()
                        .chain_err(|| {
                            ErrorKind::ProgramConnectionError(
                                stringify!($program).to_string(),
                                self.address.to_string(),
                            )
                        });
                    let record_service = try_ready!(poll_result);

                    Ok(Async::Ready($program::from(record_service)))
                }
            }
        }

        pub use self::connect::Connect;
    };
}
