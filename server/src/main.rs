#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![feature(trait_alias)]
#![feature(async_closure)]

use std::sync::Arc;
use warp::{Filter, Reply};
use std::convert::Infallible;

pub mod app;
pub mod routerUtil;


use app::AppState;
use routerUtil::injectState;

fn formatHtml(body: &str) -> String {
    let style = "body { white-space: pre-wrap; font-family: monospace;}";
    format!("<html><head><style>{}</style></head><body>{}</body></html>", style, body)
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

async fn handler_static(file_name: String) -> Result<impl Reply, Infallible> {
    let filePath = format!("../client/dist/{}", file_name);
    let file = tokio::fs::read(filePath).await;

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

/*
ab -n 100 -c 100 http://127.0.0.1:3030/hello/das

Spodziewane zwiększenie licznika o 100 i czast trwania tej komeny 10s
*/

fn buildResponse(status: u16, body: String) -> impl Reply { //warp::http::Response<String> {
    let response = warp::http::Response::builder()
        .status(status)
        .header("content-type", "text/html; charset=utf-8")
        .body(body);

    response
}

async fn handler_post() -> Result<impl Reply, Infallible> {

    println!("handler post-a");

    let response = warp::http::Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(formatHtml("Dostaliśmy posta"));

    Ok(response)
}

async fn getSite() -> Result<impl Reply, Infallible> {
    let data = reqwest::get("https://blog.logrocket.com/a-practical-guide-to-async-in-rust/").await;

    let response = match data {
        Ok(data) => data,
        Err(err) => {
            return Ok(buildResponse(200, format!("error czytania {}", err)));   //.into()));
        }
    };

    let text = response.text().await;

    let text = match text {
        Ok(text) => text,
        Err(err) => {
            return Ok(buildResponse(200, format!("error pobierania body {}", err)));
        }
    };

    println!("ddasds {:?}", text);

    let response = buildResponse(200, formatHtml("Dostaliśmy posta"));

    Ok(response)
}

#[tokio::main]
async fn main() {
    let app = AppState::new();

    println!("Server start on 127.0.0.1:3030");

    let filter_mainPage1 = warp::path::end()
        .and(injectState(app.clone()))
        .and_then(handler_index);

    let filter_mainPage2 = warp::path!("index.html")
        .and(injectState(app.clone()))
        .and_then(handler_index);

    let filter_static = warp::path!("static" / String)
        .and_then(handler_static);
        

    let filter_hello = warp::path!("hello" / String)
        .and(injectState(app.clone()))
        .and_then(handler_hello);

    //TODO - dodać handler pokazujący aktualny stan licznika związanego z hello
    
    let filter_post = warp::path!("post")
        .and(warp::post())
        .and_then(handler_post);

    let filter_getSite = warp::path!("get-site")
        .and_then(getSite);

    let routing = filter_mainPage1
        .or(filter_mainPage2)
        .or(filter_static)
        .or(filter_hello)
        .or(filter_getSite)
        .or(filter_post);

    warp::serve(routing)
        .run(([127, 0, 0, 1], 3030))
        .await;
}