use std::error::Error; 
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Quote {
    pub id: i16,
    pub csvId: String,
    pub quote: String,
    pub author: String,
    pub genre: String
}

pub fn get_quotes(url: &str) -> Result<Vec<Quote>, Box<dyn Error>> {
    let response: String = ureq::get(url)
        .set("accept", "application/json")
        .call()?
        .into_string()?;
    
    let quotes: Vec<Quote> = serde_json::from_str(&response)?;

    Ok(quotes)
}