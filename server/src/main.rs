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

    let response = warp::http::Response::builder()s
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(file);

    Ok(response)
}

async fn handler_hello(name: String) -> Result<impl Reply, Infallible> {
    let body = format!("Hello, {}!", name);

    let response = warp::http::Response::builder()
        .status(500)
        .header("content-type", "text/html; charset=utf-8")
        .body(formatHtml(body.as_str()));

    Ok(response)
}

async fn handler_post() -> Result<impl Reply, Infallible> {

    println!("handler post-a");

    let response = warp::http::Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(formatHtml("Dostaliśmy posta"));

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


    //TODO - do zaimplementowania czytanie innych zasobów statycznych z dist
    // requesty na /static/:jakis_plik mają czytać z katalogu ../client/dist/:jakiś_plik
    // trzeba uwzględnić mime pliku


    let filter_hello = warp::path!("hello" / String)
        .and_then(handler_hello);
    
    let filter_post = warp::path!("post")
        .and(warp::post())
        .and_then(handler_post);

    let routing = filter_mainPage1
        .or(filter_mainPage2)
        .or(filter_hello)
        .or(filter_post);

    warp::serve(routing)
        .run(([127, 0, 0, 1], 3030))
        .await;
}