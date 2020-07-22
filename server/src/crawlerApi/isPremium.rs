use select::document::Document;

use crate::{
    utils::{
        getFromUrl::getFromUrl,
        HtmlNode::HtmlNode,
    },
};


pub async fn isPremium(film: &str) -> bool {
    
    //"https://www.cda.pl/video/54190173d"

    let resp = getFromUrl(film).await;

//    let resp = client.get("https://www.cda.pl/video/1509340f3/vfilm").send().await;           //premium
//    let resp = client.get("https://www.cda.pl/video/4300682b6/vfilm").send().await;           //premium
    //let resp = reqwest::get("https://www.cda.pl/video/4300682b6/vfilm").await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(_errResp) => {
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
            return true;
        }
    }

    false
}

