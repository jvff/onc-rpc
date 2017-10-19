use std::io;
use std::io::Cursor;
use std::marker::PhantomData;

use futures::{Future, IntoFuture};
use futures::future::{Flatten, FutureResult};
use serde_xdr;
use tokio_service::{NewService, Service};

use super::reply_future::ReplyFuture;
use super::rpc_service_config::RpcServiceConfig;
use super::try_from::TryFrom;
use super::super::errors::{Error, Result};
use super::super::record::Record;

pub struct RpcServerService<S, P>
where
    S: Service<Request = P::Request, Response = P::Response>,
    P: RpcServiceConfig,
{
    rpc_service: S,
    _service_parameters: PhantomData<P>,
}

impl<S, P> From<S> for RpcServerService<S, P>
where
    S: Service<Request = P::Request, Response = P::Response>,
    P: RpcServiceConfig,
    S::Error: Into<Error>,
    <P::Request as TryFrom<P::ProcedureMessage>>::Error: Into<Error>,
{
    fn from(rpc_service: S) -> Self {
        RpcServerService {
            rpc_service,
            _service_parameters: PhantomData,
        }
    }
}

impl<S, P> RpcServerService<S, P>
where
    S: Service<Request = P::Request, Response = P::Response>,
    P: RpcServiceConfig,
    S::Error: Into<Error>,
    Error: From<<P::Request as TryFrom<P::ProcedureMessage>>::Error>,
    //<P::Request as TryFrom<P::ProcedureMessage>>::Error: Into<Error>,
{
    fn try_call(
        &self,
        request_record: Record<Vec<u8>>,
    ) -> Result<ReplyFuture<S::Future, P>> {
        let mut bytes = Cursor::new(request_record);
        let message: P::ProcedureMessage = serde_xdr::from_reader(&mut bytes)?;
        let request = P::Request::try_from(message)?;

        let response = self.rpc_service.call(request);

        Ok(ReplyFuture::new(response))
    }
}

impl<S, P> Service for RpcServerService<S, P>
where
    S: Service<Request = P::Request, Response = P::Response>,
    P: RpcServiceConfig,
    S::Error: Into<Error>,
    Error: From<<P::Request as TryFrom<P::ProcedureMessage>>::Error>,
{
    type Request = Record<Vec<u8>>;
    type Response = Record<Vec<u8>>;
    type Error = Error;
    type Future = Flatten<FutureResult<ReplyFuture<S::Future, P>, Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        self.try_call(request).into_future().flatten()
    }
}

impl<S, P> NewService for RpcServerService<S, P>
where
    S: Service<Request = P::Request, Response = P::Response>
        + NewService<
            Request = P::Request,
            Response = P::Response,
            Instance = S,
            Error = <S as Service>::Error,
        >,
    P: RpcServiceConfig,
    <S as Service>::Error: Into<Error>,
    Error: From<<P::Request as TryFrom<P::ProcedureMessage>>::Error>,
{
    type Request = Record<Vec<u8>>;
    type Response = Record<Vec<u8>>;
    type Error = Error;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(self.rpc_service.new_service()?.into())
    }
}
