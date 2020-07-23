//responseHtml(500, format!("Error build http client {:?}", err))
//responseHtml(500, format!("Error send {:?}", err))
//responseHtml(500, format!("Error get text {:?}", err))

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GetFromUrlError {
    ReqwestBuildClient { err: String },
    ReqwestErrorSend { err: String },
    ReqwestGetError { err: String },
}

impl GetFromUrlError {
    pub fn toString(&self) -> String {
        match self {
            GetFromUrlError::ReqwestBuildClient { err } => {
                format!("CrealerError::ReqwestBuildClient (err: {})", err)
            }
            GetFromUrlError::ReqwestErrorSend { err } => {
                format!("CrealerError::ReqwestErrorSend (err: {})", err)
            }
            GetFromUrlError::ReqwestGetError { err } => {
                format!("CrealerError::ReqwestGetError (err: {})", err)
            }
        }
    }
}

impl fmt::Display for GetFromUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.toString())
    }
}

impl Error for GetFromUrlError {
    fn description(&self) -> &str {
        match self {
            GetFromUrlError::ReqwestBuildClient { err } => err,
            GetFromUrlError::ReqwestErrorSend { err } => err,
            GetFromUrlError::ReqwestGetError { err } => err,
        }
    }
}

pub async fn getFromUrl(url: &str) -> Result<String, GetFromUrlError> {
    let builder = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0");

    let client = builder.build();

    let client = match client {
        Ok(client) => client,
        Err(err) => {
            return Err(GetFromUrlError::ReqwestBuildClient {
                err: format!("{}", err),
            });
        }
    };

    let resp = client.get(url).send().await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(err) => {
            return Err(GetFromUrlError::ReqwestErrorSend {
                err: format!("{}", err),
            });
        }
    };

    let resp = resp.text().await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(err) => {
            return Err(GetFromUrlError::ReqwestGetError {
                err: format!("{}", err),
            });
        }
    };

    Ok(resp)
}
