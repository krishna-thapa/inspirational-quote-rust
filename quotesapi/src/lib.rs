use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum QuotesApiError{
    #[error("Failed fetching quotes")]
    RequestFailed(ureq::Error),
    #[error("Failed convedrting response to string")]
    FailedResponseToString(std::io::Error),
    #[error("Quote Parsing Failed")]
    QuoteParseFailed(serde_json::Error)
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Quote {
    pub id: i16,
    pub csvId: String,
    pub quote: String,
    pub author: String,
    pub genre: String
}

pub fn get_quotes(url: &str) -> Result<Vec<Quote>, QuotesApiError> {
    let response: String = ureq::get(url)
        .set("accept", "application/json")
        .call().map_err(|e| QuotesApiError::RequestFailed(e))?
        .into_string().map_err(|e| QuotesApiError::FailedResponseToString(e))?;
    
    let quotes: Vec<Quote> = serde_json::from_str(&response).map_err(|e| QuotesApiError::QuoteParseFailed(e))?;

    Ok(quotes)
}