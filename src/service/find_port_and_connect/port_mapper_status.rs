use std::mem;

use futures::{Async, Future, Poll};

use super::super::super::errors::Error;
use super::super::super::port_mapper::{PortMapper, PortMapperConnect};

pub enum PortMapperStatus {
    Waiting,
    Connecting(PortMapperConnect),
    Connected(PortMapper),
}

impl Future for PortMapperStatus {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let moved_port_mapper_status =
            mem::replace(self, PortMapperStatus::Waiting);

        let (poll_result, new_status) = match moved_port_mapper_status {
            PortMapperStatus::Waiting => {
                unreachable!("connection to PortMapper has already started");
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
                        let new_status = PortMapperStatus::Connecting(connect);

                        (Some(poll_result), new_status)
                    }
                    Err(error) => {
                        let poll_result = Err(error);
                        let new_status = PortMapperStatus::Connecting(connect);

                        (Some(poll_result), new_status)
                    }
                }
            }
            PortMapperStatus::Connected(port_mapper) => {
                let poll_result = Ok(Async::Ready(()));
                let new_status = PortMapperStatus::Connected(port_mapper);

                (Some(poll_result), new_status)
            }
        };

        mem::replace(self, new_status);

        if let Some(poll_result) = poll_result {
            poll_result
        } else {
            self.poll()
        }
    }
}
