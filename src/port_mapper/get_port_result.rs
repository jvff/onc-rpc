use futures::{Async, Future, Poll};

use super::requests::RequestResult;
use super::super::errors::{Error, ErrorKind, ResultExt};

pub struct GetPortResult<F>
where
    F: Future<Item = RequestResult, Error = Error>,
{
    result: F,
}

impl<F> From<F> for GetPortResult<F>
where
    F: Future<Item = RequestResult, Error = Error>,
{
    fn from(result: F) -> Self {
        GetPortResult { result }
    }
}

impl<F> Future for GetPortResult<F>
where
    F: Future<Item = RequestResult, Error = Error>,
{
    type Item = u16;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let poll_result =
            self.result.poll().chain_err(|| ErrorKind::GetPortCallFailed);

        match try_ready!(poll_result) {
            RequestResult::get_port(port) => {
                ensure!(
                    port < u16::max_value() as u32,
                    ErrorKind::InvalidRemotePort(port)
                );

                Ok(Async::Ready(port as u16))
            }
            _ => bail!(ErrorKind::InvalidGetPortResponse)
        }
    }
}
