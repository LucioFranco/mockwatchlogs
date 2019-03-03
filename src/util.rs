use futures::{Future, Poll, Async, Stream};
use hyper::{Body, Error, Request, Response, body::Chunk};
use tower_service::Service;

pub struct Collect<S> {
    inner: S,
}

pub struct CollectFuture {
    concat: Concat2<Chunk>,
}

impl<S> Collect<S> {
    pub fn new(inner: S) -> Self {
        Collect { inner }
    }
}

impl<S> Service<Request<Body>> for Collect<S>
where
    S: Service<Request<Bytes>, Response = Response<Body>>,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(().into())
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let (parts, body) = request.into_parts();

        let fut = body
            .concat2()
            .and_then(move |body| {
                
            })
    }
}
