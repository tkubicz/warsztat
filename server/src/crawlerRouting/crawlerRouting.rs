use warp::{Filter, Reply};

use select::document::Document;

use crate::{
    crawlerApi::getCdaList::{
        getCdaList,
        CdaListItem,
    },
    utils::{
        render::{
            responseHtml,
            HandlerResponse,
        },
        getFromUrl::getFromUrl,
        HtmlNode::HtmlNode,
    },
};

//https://docs.rs/warp/0.2.3/warp/trait.Filter.html#extracting-tuples

pub fn crawlerRouting() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {

    let mainSwitch = (
        warp::path!("list" / String).and_then(crawler_list)
    ).or(
        warp::path("list2").and_then(async_cda_list2)
    ).or(
        warp::path("htmlselect").and_then(handler_htmlselect)
    );

    mainSwitch
}



async fn handler_htmlselect() -> HandlerResponse {

    let resp = getFromUrl("https://www.cda.pl/video/54190173d").await;

//    let resp = client.get("https://www.cda.pl/video/1509340f3/vfilm").send().await;           //premium
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


async fn async_cda_list2() -> HandlerResponse {
    return Ok(responseHtml(200, "list2".into()));
}

async fn crawler_list(film: String) -> HandlerResponse {

    //"hotel transylwania 2012"

    //let list = crate::crawler::getCdaList("truman show").await;
    //let list = crate::crawler::getCdaList("Człowiek z magicznym pudełkiem").await;
    let list = getCdaList(film.as_str()).await;

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

    //println!("list {:?}", list);

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
        h1 {
            "Wyniki dla: "
            span {
                (film)
            }
        }
        div {
            @for item in itemsHtml.iter() {
                (item)
            }
        }
    };

    return Ok(responseHtml(200, htmlOut.into_string()));
}

