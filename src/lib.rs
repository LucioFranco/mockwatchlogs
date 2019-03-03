#[macro_use]
extern crate tokio_trace;

mod actions;
mod streams;
mod types;

use bytes::Bytes;
use futures::{future, Future, Poll, Stream as _};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio_tcp::TcpListener;
use tokio_trace::field;
use tokio_trace_futures::Instrument;
use tokio_trace_tower_http::InstrumentedMakeService;
use tower_hyper::server::Server;
use tower_service::Service;

use streams::Context;

pub fn serve(addr: SocketAddr) -> impl Future<Item = (), Error = ()> {
    let bind = TcpListener::bind(&addr).expect("bind");

    let mut serve_span = span!(
        "serve",
        local_ip = field::debug(addr.ip()),
        local_port = addr.port() as u64
    );

    let service = MockCloudwatchLogs::default();
    let service = InstrumentedMakeService::new(service, serve_span.clone());

    let serve_span2 = serve_span.clone();
    serve_span.enter(|| {
        info!("Listening on {}", addr);

        let server = Server::new(service);

        bind.incoming()
            .fold(server, |mut server, stream| {
                let peer_addr = stream.peer_addr().unwrap();
                let mut conn_span = span!("connection", peer = &field::debug(peer_addr));
                let conn_span2 = conn_span.clone();

                conn_span.enter(|| {
                    trace!("Incoming Tcp connection from {}", peer_addr);

                    if let Err(e) = stream.set_nodelay(true) {
                        return Err(e);
                    }

                    let serve = server
                        .serve(stream)
                        .map_err(|e| panic!("Server error {:?}", e))
                        .and_then(|_| {
                            debug!("response finished");
                            future::ok(())
                        })
                        .instrument(conn_span2);

                    hyper::rt::spawn(serve);

                    Ok(server)
                })
            })
            .map_err(|e| {
                error!({ error = field::display(e) }, "serve error");
            })
            .map(|_| ())
            .instrument(serve_span2)
    })
}

pub type Body = hyper::Body;
pub type Request = hyper::Request<Body>;
pub type Response = hyper::Response<Body>;
pub type Error = hyper::Error;

#[derive(Default)]
struct MockCloudwatchLogs {
    context: Arc<Mutex<Context>>,
}

impl Service<()> for MockCloudwatchLogs {
    type Response = Router;
    type Error = hyper::Error;
    type Future = future::FutureResult<Self::Response, Self::Error>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(().into())
    }

    fn call(&mut self, _: ()) -> Self::Future {
        let context = self.context.clone();
        future::ok(Router::new(context))
    }
}

struct Router {
    context: Arc<Mutex<Context>>,
}

impl Router {
    pub fn new(context: Arc<Mutex<Context>>) -> Self {
        Router { context }
    }

    fn dispatch(
        action: &str,
        context: Arc<Mutex<Context>>,
        body: Bytes,
    ) -> Result<Response, Error> {
        use actions::*;

        info!("Incoming action: {}", action);

        let mut action_span = span!("action", action = &field::debug(action));

        action_span.enter(|| {
            match action {
                "Logs_20140328.DescribeLogStreams" => {
                    // TODO: make this a 500
                    let mut context = context.lock().unwrap();
                    let req = extract(&body).unwrap();
                    describe_streams(&mut context, req).or_else(|e| Ok(e.into()))
                }

                "Logs_20140328.CreateLogGroup" => {
                    let mut context = context.lock().unwrap();
                    let req = extract(&body).unwrap();
                    create_group(&mut context, req).or_else(|e| Ok(e.into()))
                }

                "Logs_20140328.CreateLogStream" => {
                    let mut context = context.lock().unwrap();
                    let req = extract(&body).unwrap();
                    create_stream(&mut context, req).or_else(|e| Ok(e.into()))
                }

                "Logs_20140328.PutLogEvents" => {
                    let mut context = context.lock().unwrap();
                    let req = extract(&body).unwrap();
                    put_logs(&mut context, req).or_else(|e| Ok(e.into()))
                }

                "Logs_20140328.GetLogEvents" => {
                    let mut context = context.lock().unwrap();
                    let req = extract(&body).unwrap();
                    get_logs(&mut context, req).or_else(|e| Ok(e.into()))
                }

                _ => unimplemented!("404"),
            }
        })
    }
}

impl Service<Request> for Router {
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error> + Send + 'static>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(().into())
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let mut http_span = span!(
            "request",
            method = &field::debug(req.method()),
            uri = &field::debug(req.uri()),
            headers = &field::debug(req.headers())
        );

        http_span.enter(|| {
            info!("Incoming HTTP Request");
            trace!("HTTP Request: {:?}", req);

            let (parts, body) = req.into_parts();
            let context = self.context.clone();

            let fut = body.concat2().and_then(move |body| {
                // TODO: check that it is a post request
                let amz_target_header = parts.headers.get("X-Amz-Target");

                if let Some(Ok(action)) = amz_target_header.map(|a| a.to_str()) {
                    Router::dispatch(action, context, body.into_bytes())
                } else {
                    unimplemented!("Bad request")
                }
            });

            Box::new(fut)
        })
    }
}

fn extract<'a, T>(body: &'a Bytes) -> Result<T, ()>
where
    T: Deserialize<'a>,
{
    serde_json::from_slice(&body[..]).map_err(|_| ())
}
