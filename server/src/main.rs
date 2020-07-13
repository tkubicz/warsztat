#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![feature(trait_alias)]
#![feature(async_closure)]
#![feature(proc_macro_hygiene)]

use warp::{Filter};

pub mod app;
pub mod defaultRouting;
pub mod crawlerApi;
pub mod crawlerRouting;
pub mod utils;

use app::AppState;
use utils::{
    render::{
        responseHtml,
    },
};

use crawlerRouting::crawlerRouting::crawlerRouting;
use defaultRouting::defaultRouting::defaultRouting;

#[tokio::main]
async fn main() {
    let app = AppState::new();

    println!("Server start on 127.0.0.1:3030");

    let routing = (
            defaultRouting(app.clone())
        ).or(
            warp::path("crawler").and(crawlerRouting())
        ).or(
            warp::any().map(|| Ok(responseHtml(404, "Error: 404".into())))
        );

    warp::serve(routing)
        .run(([127, 0, 0, 1], 3030))
        .await;
}