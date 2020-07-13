
use std::sync::Arc;
use warp::{Filter, Reply};
use std::convert::Infallible;

use crate::{
    app::AppState,
    utils::{
        injectState::injectState,
        render::{
            formatHtml,
            responseHtml,
            HandlerResponse,
            responseVec8,
        }
    },
};

pub fn defaultRouting(app: Arc<AppState>) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {


    let default_routing = (
        warp::path::end()
            .and(injectState(app.clone()))
            .and_then(handler_index)
    ).or(
        warp::path!("index.html")
            .and(injectState(app.clone()))
            .and_then(handler_index)
    ).or(
        warp::path!("static" / String)
            .and_then(handler_static)
    ).or(
        warp::path!("post")
            .and(warp::post())
            .and_then(handler_post)
    ).or(
        warp::path!("hello" / String)
            .and(injectState(app.clone()))
            .and_then(handler_hello)
    );

    default_routing
}


async fn handler_post() -> Result<impl Reply, Infallible> {

    println!("handler post-a");

    let response = warp::http::Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(formatHtml("Dostaliśmy posta"));

    Ok(response)
}


async fn handler_index(_appState: Arc<AppState>) -> Result<impl Reply, Infallible> {

    let file = tokio::fs::read("../client/dist/index.html").await;

    let file = match file {
        Ok(file) => file,
        Err(err) => {
            println!("{:?}", err);
            return Ok(
                warp::http::Response::builder()
                    .status(500)
                    .header("content-type", "text/html; charset=utf-8")
                    .body(formatHtml("Error 500").into_bytes())
            );
        }
    };

    let response = warp::http::Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(file);

    Ok(response)
}

async fn handler_static(file_name: String) -> HandlerResponse {
    let filePath = format!("../client/dist/{}", file_name);
    let file = tokio::fs::read(filePath).await;

    let file = match file {
        Ok(file) => file,
        Err(err) => {
            println!("{:?}", err);
            return Ok(responseHtml(500, "Error 500".into()));
        }
    };

    responseVec8(200, file)
}


async fn handler_hello(name: String, appState: Arc<AppState>) -> Result<impl Reply, Infallible> {
    appState.incrementCounter().await;
    let newCounter = appState.getCounter().await;

    let body = format!("Hello, {}! - Jesteś {}-tą osobą urucamiającą ten handler", name, newCounter);

    let response = warp::http::Response::builder()
        .status(500)
        .header("content-type", "text/html; charset=utf-8")
        .body(formatHtml(body.as_str()));

    Ok(response)
}
