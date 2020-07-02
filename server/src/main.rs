#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![feature(trait_alias)]
#![feature(async_closure)]

use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use std::convert::Infallible;

pub mod app;

use app::AppState;

async fn async_route_ws_index(_appState: Arc<AppState>) -> Result<impl warp::Reply, Infallible> {
    let body = "dasdas";
    let style = "body { white-space: pre-wrap; font-family: monospace;}";

    let response = warp::http::Response::builder()
        .header("content-type", "text/html; charset=utf-8")
        .body(format!("<html><head><style>{}</style></head><body>{}</body></html>", style, body));

    Ok(response)
}

fn injectState<T: Clone + Sized>(state: T) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::any().map(move || state.clone())
}

fn route_ws_index(appAddr: Arc<AppState>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let appAddrFilter = warp::any().map(move || appAddr.clone());

    warp::path!("ws")
        .and(appAddrFilter)
        .and_then(async_route_ws_index)
}

async fn async_route_ws_index2(name: String) -> Result<impl warp::Reply, Infallible> {
    let body = format!("Hello, {}!", name);
    let style = "body { white-space: pre-wrap; font-family: monospace;}";

    let response = warp::http::Response::builder()
        .status(500)
        .header("content-type", "text/html; charset=utf-8")
        .body(format!("<html><head><style>{}</style></head><body>{}</body></html>", style, body));

    Ok(response)
}

fn route_ws_index2(_appAddr: Arc<AppState>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("hello" / String)
        .and_then(async_route_ws_index2)
}

#[tokio::main]
async fn main() {
    let app = AppState::new();

    println!("Server start on 127.0.0.1:3030");

    let hello = route_ws_index2(app.clone());

    let routing = hello.or(route_ws_index(app.clone()));

    warp::serve(routing)
        .run(([127, 0, 0, 1], 3030))
        .await;
}