use std::sync::Arc;

use crate::status::Status;
use crate::Result;

use serde::Serialize;
use trillium::{conn_try, conn_unwrap, Conn, Handler, KnownHeaderName::ContentType, State};
use trillium_router::Router;

/// Routes to be attached to a Trillium app runtime
pub fn router<S: Status + 'static>(status: S) -> impl Handler {
    let status = Arc::new(status);

    (State::new(status), routes::<S>())
}

fn routes<S: Status + 'static>() -> impl Handler {
    Router::new()
        .get("/about", about::<S>)
        .get("/metrics", metrics)
        .get("/ready", ready::<S>)
        .get("/health", health::<S>)
}

async fn ready<S: Status + 'static>(conn: Conn) -> Conn {
    let status = conn_unwrap!(conn.state::<Arc<S>>(), conn);

    match status.ready().await {
        Some(is_ready) => {
            if is_ready {
                conn.with_status(200).with_body("ready\n")
            } else {
                conn.with_status(503).with_body("Service unavailable")
            }
        }
        None => conn.with_status(404).with_body("not found"),
    }
}

async fn health<S: Status + 'static>(conn: Conn) -> Conn {
    let status = conn_unwrap!(conn.state::<Arc<S>>(), conn);

    match status.check().await {
        Some(resp) => conn.with_status(200).with_json(resp.to_json()),
        None => conn.with_status(404).with_body("No health checks"),
    }
}

async fn about<S: Status + 'static>(conn: Conn) -> Conn {
    let status = conn_unwrap!(conn.state::<Arc<S>>(), conn);

    let about = status.about();

    conn.with_status(200).with_json(about)
}

async fn metrics(conn: Conn) -> Conn {
    let metrics = conn_try!(render_metrics(), conn);

    conn.with_status(200)
        .with_header(ContentType, "text/plain; version=0.0.4; charset=utf-8")
        .with_body(metrics)
}

trait JsonConnExt {
    fn with_json(self, t: impl Serialize + Send + Sync + 'static) -> Self;
}

impl JsonConnExt for Conn {
    fn with_json(self, t: impl Serialize + Send + Sync + 'static) -> Self {
        let body = conn_try!(serde_json::to_string(&t), self);
        self.with_header(ContentType, "application/json")
            .with_body(body)
    }
}

fn render_metrics() -> Result<Vec<u8>> {
    use prometheus::{gather, Encoder, TextEncoder};

    let metric_family = gather();

    let mut writer = Vec::<u8>::new();
    let encoder = TextEncoder::new();
    encoder.encode(&metric_family, &mut writer)?;

    Ok(writer)
}
