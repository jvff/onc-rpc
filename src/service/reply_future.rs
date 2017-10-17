use std::marker::PhantomData;

use futures::{Async, Future, Poll};
use serde_xdr;

use super::rpc_service_config::RpcServiceConfig;
use super::super::errors::Error;
use super::super::record::Record;

pub struct ReplyFuture<R, P>
where
    P: RpcServiceConfig,
    R: Future<Item = P::Response>,
    R::Error: Into<Error>,
{
    response: R,
    _service_parameters: PhantomData<P>,
}

impl<R, P> ReplyFuture<R, P>
where
    P: RpcServiceConfig,
    R: Future<Item = P::Response>,
    R::Error: Into<Error>,
{
    pub fn new(response: R) -> Self {
        ReplyFuture {
            response,
            _service_parameters: PhantomData,
        }
    }
}

impl<R, P> Future for ReplyFuture<R, P>
where
    P: RpcServiceConfig,
    R: Future<Item = P::Response>,
    R::Error: Into<Error>,
{
    type Item = Record<Vec<u8>>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let response = match self.response.poll() {
            Ok(Async::Ready(response)) => response,
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(error) => return Err(error.into()),
        };

        let response_message: P::ProcedureMessage = response.into();
        let response_record = serde_xdr::to_bytes(&response_message)?.into();

        Ok(Async::Ready(response_record))
    }
}
