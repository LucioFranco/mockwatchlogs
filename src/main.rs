#[macro_use]
extern crate tokio_trace;

use hyper::rt;
use mockwatchlogs::serve;

use tokio_trace::field;
use tokio_trace_futures::Instrument;

fn main() {
    let subscriber = tokio_trace_fmt::FmtSubscriber::builder()
        .with_filter(tokio_trace_fmt::filter::EnvFilter::from(
            "mockwatchlogs=trace",
        ))
        .full()
        .finish();
    tokio_trace_env_logger::try_init().expect("init log adapter");

    tokio_trace::subscriber::with_default(subscriber, || {
        let addr = "0.0.0.0:6000".parse().unwrap();
        let mut server_span = span!("server", local = &field::debug(addr));
        let serve = serve(addr).instrument(server_span.clone());

        server_span.enter(|| rt::run(serve));
    });
}
