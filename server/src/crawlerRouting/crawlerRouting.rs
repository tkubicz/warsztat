use warp::{Filter, Reply};

use crate::{
    crawlerApi::{
        getCdaList::{
            getCdaList,
            CdaListItem,
        },
        isPremium::isPremium,
    },
    utils::{
        render::{
            responseHtml,
            HandlerResponse,
        },
    },
};


//https://docs.rs/warp/0.2.3/warp/trait.Filter.html#extracting-tuples

pub fn crawlerRouting() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {

    let mainSwitch = (
        warp::path!("list" / String).and_then(crawler_list)
    ).or(
        warp::path("list2").and_then(async_cda_list2)
    ).or(
        warp::path!("show" / String).and_then(handler_show)
    );

    mainSwitch
}


async fn handler_show(film: String) -> HandlerResponse {
    //let isPremium = isPremium("https://www.cda.pl/video/54190173d").await;
    let filmUrl = format!("https://www.cda.pl/video/{}/vfilm", film);
    let isPremium = isPremium(filmUrl.as_str()).await;

    //let resp = getFromUrl("https://www.cda.pl/video/54190173d").await;

    //let resp = client.get("https://www.cda.pl/video/1509340f3/vfilm").send().await;           //premium
    //let resp = client.get("https://www.cda.pl/video/4300682b6/vfilm").send().await;           //premium
    //let resp = reqwest::get("https://www.cda.pl/video/4300682b6/vfilm").await;

    return Ok(responseHtml(200, format!("Film ---> isPremium={} filmUrl={}", isPremium, filmUrl)));
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

