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
pub mod crawler;
pub mod utils;

use app::AppState;
use utils::{
    injectState::injectState,
    getFromUrl::{getFromUrl},
    HtmlNode::HtmlNode,
};
use crawler::{
    getCdaList::{
        getCdaList,
        CdaListItem
    },
};

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

    //let list = crate::crawler::getCdaList("truman show").await;
    //let list = crate::crawler::getCdaList("Człowiek z magicznym pudełkiem").await;
    let list = getCdaList("hotel transylwania 2012").await;

    let mut list = match list {
        Ok(list) => list,
        Err(err) => {
            return Ok(responseHtml(500, format!("{}", err.toString())));
        }
    };

    use std::cmp::Ordering;

    list.sort_by(|a: &CdaListItem, b: &CdaListItem| {
        if b.time < a.time{
            return Ordering::Less;
        }

        if b.time > a.time {
            return Ordering::Greater;
        }

        Ordering::Equal
    });

    println!("list {:?}", list);

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
                        br {}
                        (item.timeToStr())
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

    let resp = getFromUrl("https://www.cda.pl/video/54190173d").await;

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
    let root = HtmlNode::fromDocument(&document);

    //println!("document {:?}", document);

    for node in root.findByClass("reg-premium-load-js") {
        if node.hasClass("btn-premium") {
            //let a: String = node;
            println!("aaaaa {:?}", node);
        }
    }

    return Ok(responseHtml(200, "ooołłl je".into()));
}

// fn cda_router() {

// }


async fn async_cda_list1() -> HandlerResponse {
    return Ok(responseHtml(200, "list1".into()));
}


async fn async_cda_list2() -> HandlerResponse {
    return Ok(responseHtml(200, "list2".into()));
}




fn cda_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    let mainSwitch = (
        warp::path("list1").and_then(async_cda_list1)
    ).or(
        warp::path("list2").and_then(async_cda_list2)
    );

    let filter_cda = warp::path("cda").and(
        mainSwitch
    );

    filter_cda
}


#[tokio::main]
async fn main() {
    let app = AppState::new();

    println!("Server start on 127.0.0.1:3030");


    let routing = (
            warp::path::end()
                .and(injectState(app.clone()))
                .and_then(handler_index)
        ).or(
            cda_filter()
            //warp::path("cda").and(cda_filter())
        ).or(
            warp::path!("index.html")
                .and(injectState(app.clone()))
                .and_then(handler_index)
        ).or(
            warp::path!("static" / String)
                .and_then(handler_static)
        ).or(
            warp::path!("hello" / String)
                .and(injectState(app.clone()))
                .and_then(handler_hello)
        ).or(
            warp::path("htmlselect")
                .and(warp::get())
                .and_then(handler_htmlselect)
        ).or(
            warp::path("cdalist")
                .and(warp::get())
                .and_then(handler_cda_list)
        ).or(
            warp::path!("post")
                .and(warp::post())
                .and_then(handler_post)
        );

    warp::serve(routing)
        .run(([127, 0, 0, 1], 3030))
        .await;
}