use std::io;
use std::io::Cursor;
use std::marker::PhantomData;

use futures::{Future, IntoFuture};
use futures::future::{Flatten, FutureResult};
use serde::Deserialize;
use serde_xdr;
use tokio_service::{NewService, Service};

use super::reply_future::ReplyFuture;
use super::rpc_service_config::RpcServiceConfig;
use super::super::errors::{Error, Result};
use super::super::record::Record;

pub struct RpcServerService<S, P>
where
    S: Service<Request = P::Request, Response = P::Response>,
    P: RpcServiceConfig,
    Error: From<S::Error>,
{
    rpc_service: S,
    _service_parameters: PhantomData<P>,
}

impl<S, P> From<S> for RpcServerService<S, P>
where
    S: Service<Request = P::Request, Response = P::Response>,
    P: RpcServiceConfig,
    Error: From<S::Error>,
{
    fn from(rpc_service: S) -> Self {
        RpcServerService {
            rpc_service,
            _service_parameters: PhantomData,
        }
    }
}

impl<'de, S, P> RpcServerService<S, P>
where
    S: Service<Request = P::Request, Response = P::Response>,
    P: RpcServiceConfig,
    P::Request: Deserialize<'de>,
    Error: From<S::Error>,
{
    fn try_call(
        &self,
        request_record: Record<Vec<u8>>,
    ) -> Result<ReplyFuture<S::Future, P>> {
        let mut request_bytes = Cursor::new(request_record);
        let request = serde_xdr::from_reader(&mut request_bytes)?;

        let response = self.rpc_service.call(request);

        Ok(ReplyFuture::new(response))
    }
}

impl<'de, S, P> Service for RpcServerService<S, P>
where
    S: Service<Request = P::Request, Response = P::Response>,
    P: RpcServiceConfig,
    P::Request: Deserialize<'de>,
    Error: From<S::Error>,
{
    type Request = Record<Vec<u8>>;
    type Response = Record<Vec<u8>>;
    type Error = Error;
    type Future = Flatten<FutureResult<ReplyFuture<S::Future, P>, Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        self.try_call(request).into_future().flatten()
    }
}

impl<'de, S, P> NewService for RpcServerService<S, P>
where
    S: Service<Request = P::Request, Response = P::Response>
        + NewService<
            Request = P::Request,
            Response = P::Response,
            Instance = S,
            Error = <S as Service>::Error,
        >,
    P: RpcServiceConfig,
    P::Request: Deserialize<'de>,
    Error: From<<S as Service>::Error>,
{
    type Request = Record<Vec<u8>>;
    type Response = Record<Vec<u8>>;
    type Error = Error;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(self.rpc_service.new_service()?.into())
    }
}
