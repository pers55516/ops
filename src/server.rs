use std::net::SocketAddr;
use std::sync::Arc;

use crate::error::Error;
use crate::status::Status;
use crate::Result;

use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};

/// Starts a server and serves the ops endpoints.
pub async fn server<S: Status + 'static>(addr: SocketAddr, status: S) -> Result<()> {
    let status: Arc<S> = Arc::new(status);

    let service = make_service_fn(move |_| {
        let status = status.clone();

        async { Ok::<_, Error>(service_fn(move |req| router(req, status.clone()))) }
    });

    Server::bind(&addr).serve(service).await.map_err(Into::into)
}

async fn router<S: Status + 'static>(req: Request<Body>, status: Arc<S>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/__/about") => about(status.clone()).await,
        (&Method::GET, "/__/metrics") => metrics().await,
        (&Method::GET, "/__/ready") => ready(status.clone()).await,
        (&Method::GET, "/__/health") => health(status.clone()).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("not found"))?),
    }
}

async fn ready<S: Status + 'static>(status: Arc<S>) -> Result<Response<Body>> {
    let resp = match status.ready().await {
        None => Response::builder()
            .header(header::CONTENT_TYPE, "text/plain")
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("not found"))?,
        Some(is_ready) => {
            if is_ready {
                Response::builder()
                    .header(header::CONTENT_TYPE, "text/plain")
                    .status(StatusCode::OK)
                    .body(Body::from("ready\n"))?
            } else {
                Response::builder()
                    .header(header::CONTENT_TYPE, "text/plain")
                    .status(StatusCode::SERVICE_UNAVAILABLE)
                    .body(Body::from("Service unavailable"))?
            }
        }
    };
    Ok(resp)
}

async fn health<S: Status + 'static>(status: Arc<S>) -> Result<Response<Body>> {
    let resp = match status.check().await {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("No health checks"))?,
        Some(resp) => match serde_json::to_string(&resp.to_json()) {
            Ok(payload) => Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(payload))?,
            Err(err) => err_response(err)?,
        },
    };
    Ok(resp)
}

async fn metrics() -> Result<Response<Body>> {
    let resp = match render_metrics() {
        Ok(rendered_metrics) => match String::from_utf8(rendered_metrics) {
            Ok(rendered_metrics) => Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    "text/plain; version=0.0.4; charset=utf-8",
                )
                .body(Body::from(rendered_metrics))?,
            Err(err) => err_response(err)?,
        },
        Err(err) => err_response(err)?,
    };
    Ok(resp)
}

async fn about<S: Status + 'static>(status: Arc<S>) -> Result<Response<Body>> {
    let resp = match serde_json::to_string(&status.about()) {
        Ok(payload) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(payload))?,
        Err(err) => err_response(err)?,
    };
    Ok(resp)
}

fn err_response<I: Into<Error>>(err: I) -> Result<Response<Body>> {
    let resp = Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "text/plain")
        .body(Body::from(err.into().to_string()))?;
    Ok(resp)
}

fn render_metrics() -> Result<Vec<u8>> {
    use prometheus::{gather, Encoder, TextEncoder};

    let metric_family = gather();

    let mut writer = Vec::<u8>::new();
    let encoder = TextEncoder::new();
    encoder.encode(&metric_family, &mut writer)?;

    Ok(writer)
}
