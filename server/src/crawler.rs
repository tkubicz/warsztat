use select::document::Document;
use crate::HtmlNode;

pub enum CrawlerError {
    ReqwestBuildClient { err: String },
    ReqwestErrorSend { err: String },
    ReqwestGetError { err: String },
}

impl CrawlerError {
    pub fn toString(&self) -> String {
        match self {
            CrawlerError::ReqwestBuildClient { err } => format!("CrealerError::ReqwestBuildClient (err: {})", err),
            CrawlerError::ReqwestErrorSend { err } => format!("CrealerError::ReqwestErrorSend (err: {})", err),
            CrawlerError::ReqwestGetError { err } => format!("CrealerError::ReqwestGetError (err: {})", err),
        }
    }
}

//responseHtml(500, format!("Error build http client {:?}", err))
//responseHtml(500, format!("Error send {:?}", err))
//responseHtml(500, format!("Error get text {:?}", err))

pub async fn getFromUrl(url: &str) -> Result<String, CrawlerError> {
    let builder = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0");

    let client = builder.build();

    let client = match client {
        Ok(client) => client,
        Err(err) => {
            return Err(CrawlerError::ReqwestBuildClient { err: format!("{}", err)});
        }
    };

    let resp = client.get(url).send().await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(err) => {
            return Err(CrawlerError::ReqwestErrorSend { err: format!("{}", err)});
        }
    };

    let resp = resp.text().await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(err) => {
            return Err(CrawlerError::ReqwestGetError { err: format!("{}", err)});
        }
    };

    Ok(resp)
}

use url::form_urlencoded;


#[derive(Debug)]
pub struct CdaListItem {
    pub link: String,
    pub title: String,
    pub img: String,
    pub description: String,
    pub time: u64,
    //pub time: String,
}

fn formatDigits(time: u64) -> String {
    if time < 10 {
        format!("0{}", time)
    } else {
        format!("{}", time)
    }
}

impl CdaListItem {
    pub fn timeToStr(&self) -> String {
        let time0 = self.time;

        let hour: u64 = time0 / 3600;
        let time1: u64 = time0 % 3600;
     
        let minute: u64 = time1 / 60;
        let seconds: u64 = time1 % 60;
        
        format!("{}:{}:{}", formatDigits(hour), formatDigits(minute), formatDigits(seconds))
    }
}

fn convertToU64(value: &str) -> u64 {
    value.parse().unwrap()
}

fn parseTime(time: String) -> u64 {
    let chunks = time.as_str().trim().split(":");
    
    let mut time: u64 = 0;

    for item in chunks {
        time = time * 60 + convertToU64(item);
    }

    time
}

pub async fn getCdaList(phrase: &str) -> Result<Vec<CdaListItem>, CrawlerError> {

    // let encoded: String = form_urlencoded::Serializer::new(String::new())
    //     .append_pair("foo", "bar & baz")
    //     .append_pair("saison", "Été+hiver")
    //     .finish();
    
    let url: String = form_urlencoded::Serializer::new(format!("https://www.cda.pl/info/{}", phrase))
        .finish();
    
    //"https://www.cda.pl/info/truman_show"

    let resp = getFromUrl(url.as_str()).await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(errResponse) => {
            return Err(errResponse);
        }
    };

    let document = Document::from(resp.as_str());
    let root = HtmlNode::HtmlNode::fromDocument(&document);

    let mut out = Vec::<CdaListItem>::new();

    for node in root.findByClass("video-clip-wrapper") {

        let mut linkTitleNodeList = node.findByClass("video-clip-link");

        if linkTitleNodeList.len() > 1 {
            panic!("Too many");
        }

        let linkTitleNode = linkTitleNodeList.pop();

        let linkTitleNode = match linkTitleNode {
            Some(linkTitleNode) => linkTitleNode,
            None => {
                continue;
            }
        };
    

        let link = linkTitleNode.attr("href");

        let img = node.findByClassOne("video-clip-image").attr("src");

        let description = node.findByNameExpectOne("label").attr("title");

        let titleNode = node.findByClassOne("link-title-visit");
        let title = titleNode.text();

        //TODO
        let timeElem = node.findByClassOne("timeElem");
        let time = timeElem.text();
        //content wziąć, to będzie czas
        
        //TODO - zrobić pełne linki do elementów ...

        if let (Some(link), title, Some(img), Some(description)) = (link, title, img, description) {
            out.push(CdaListItem {
                link: String::from(link),
                title: title.clone(),
                img: String::from(img),
                description: String::from(description),
                time: parseTime(time)
            })
        } else {
            panic!("asadsadasdas");
        }
    }
    
    Ok(out)
}
