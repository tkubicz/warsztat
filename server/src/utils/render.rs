use std::convert::Infallible;
use warp::http::Response;


pub type HttpResponse = Response<Vec<u8>>;
pub type HandlerResponse = Result<HttpResponse, Infallible>;


pub fn formatHtml(body: &str) -> String {
    let style = "body { white-space: pre-wrap; font-family: monospace;}";
    format!("<html><head><style>{}</style></head><body>{}</body></html>", style, body)
}


pub fn responseHtml(status: u16, body: String) -> HttpResponse {
    let response = warp::http::Response::builder()
        .status(status)
        .header("content-type", "text/html; charset=utf-8")
        .body(formatHtml(&body).into_bytes()).unwrap();

    response
}


pub fn responseVec8(status: u16, body: Vec<u8>) -> HandlerResponse {
    let response = warp::http::Response::builder()
        .status(status)
        .header("content-type", "text/html; charset=utf-8")
        .body(body).unwrap();

    Ok(response)
}
