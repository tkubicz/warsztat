#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![feature(trait_alias)]
#![feature(async_closure)]
#![feature(proc_macro_hygiene)]

use std::sync::Arc;
use warp::{Filter, Reply};
use std::convert::Infallible;
use warp::http::Response;
use select::document::Document;

pub mod app;
pub mod routerUtil;
pub mod HtmlNode;
pub mod crawler;

use app::AppState;
use routerUtil::injectState;

type HttpResponse = Response<Vec<u8>>;
type HandlerResponse = Result<HttpResponse, Infallible>;


fn formatHtml(body: &str) -> String {
    let style = "body { white-space: pre-wrap; font-family: monospace;}";
    format!("<html><head><style>{}</style></head><body>{}</body></html>", style, body)
}


fn responseHtml(status: u16, body: String) -> HttpResponse {
    let response = warp::http::Response::builder()
        .status(status)
        .header("content-type", "text/html; charset=utf-8")
        .body(formatHtml(&body).into_bytes()).unwrap();

    response
}

fn responseV8(status: u16, body: Vec<u8>) -> HandlerResponse {
    let response = warp::http::Response::builder()
        .status(status)
        .header("content-type", "text/html; charset=utf-8")
        .body(body).unwrap();

    Ok(response)
}



/*
ab -n 100 -c 100 http://127.0.0.1:3030/hello/das

Spodziewane zwiększenie licznika o 100 i czast trwania tej komeny 10s
*/




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

    responseV8(200, file)
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

async fn handler_post() -> Result<impl Reply, Infallible> {

    println!("handler post-a");

    let response = warp::http::Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(formatHtml("Dostaliśmy posta"));

    Ok(response)
}

async fn handler_cda_list() -> HandlerResponse {

//    let list = crate::crawler::getCdaList("truman show").await;
    let list = crate::crawler::getCdaList("La La Land").await;

    let list = match list {
        Ok(list) => list,
        Err(err) => {
            return Ok(responseHtml(500, format!("{}", err.toString())));
        }
    };

    println!("list {:?}", list);

    // let fragment1 = maud::html! {
    //     a href="http://onet.pl/" {
    //         "To jakiś fikuśny link 1"
    //     }
    // };

    // let fragment2 = maud::html! {
    //     a href="http://onet.pl/" {
    //         "To jakiś fikuśny link 2"
    //     }
    // };

    let mut itemsHtml: Vec<maud::Markup> = Vec::new();

    for item in list {
        itemsHtml.push(maud::html! {
            div {
                div {
                    div {
                        img src=(item.img) {}
                    }
                    div {
                        (item.title)
                        (item.time)
                    }
                }
                br {}
                br {}
            }
        });
    }

    let htmlOut = maud::html! {
        h1 { "Lista" }
        div {
            @for item in itemsHtml.iter() {
                (item)
            }
        }
    };

    return Ok(responseHtml(200, htmlOut.into_string()));
}

async fn handler_htmlselect() -> HandlerResponse {

    let resp = crate::crawler::getFromUrl("https://www.cda.pl/video/54190173d").await;

//    let resp = client.get("h"btn-premium"ttps://www.cda.pl/video/1509340f3/vfilm").send().await;           //premium
//    let resp = client.get("https://www.cda.pl/video/4300682b6/vfilm").send().await;           //premium

    //let resp = reqwest::get("https://www.cda.pl/video/4300682b6/vfilm").await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(errResp) => {
            todo!();
//            return Ok(errResp);
        }
    };

    let document = Document::from(resp.as_str());
    let root = HtmlNode::HtmlNode::fromDocument(&document);

    //println!("document {:?}", document);

    for node in root.findByClass("reg-premium-load-js") {
        if node.hasClass("btn-premium") {
            //let a: String = node;
            println!("aaaaa {:?}", node);
        }
    }

    return Ok(responseHtml(200, "ooołłl je".into()));
}

async fn getSite() -> Result<impl Reply, Infallible> {

    let response = crate::crawler::getFromUrl("https://blog.logrocket.com/a-practical-guide-to-async-in-rust/").await;

    let response = match response {
        Ok(response) => response,
        Err(errResp) => {
            todo!();
//            return Ok(errResp);
        }
    };

    println!("ddasds {:?}", response);

    let response = responseHtml(200, formatHtml("Dostaliśmy posta"));

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

    let filter_htmlselect = warp::path("htmlselect")
        .and(warp::get())
        .and_then(handler_htmlselect);

    let filter_cdalist = warp::path("cdalist")
        .and(warp::get())
        .and_then(handler_cda_list);

    let filter_getSite = warp::path!("get-site")
        .and_then(getSite);

    let routing = filter_mainPage1
        .or(filter_mainPage2)
        .or(filter_static)
        .or(filter_hello)
        .or(filter_htmlselect)
        .or(filter_cdalist)
        .or(filter_getSite)
        .or(filter_post);

    warp::serve(routing)
        .run(([127, 0, 0, 1], 3030))
        .await;
}