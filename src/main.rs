use std::error::Error; 

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Quote {
    id: i16,
    csvId: String,
    quote: String,
    author: String,
    genre: String
}

fn get_quotes(url: &str) -> Result<Vec<Quote>, Box<dyn Error>> {
    let response: String = ureq::get(url)
        .set("accept", "application/json")
        .call()?
        .into_string()?;
    
    let quotes: Vec<Quote> = serde_json::from_str(&response)?;

    Ok(quotes)
}

fn main() {
    println!("=== Running the main function! ===");
    let url: &str = "http://localhost:9000/quote/randomTen";
    let quotes: Result<Vec<Quote>, Box<dyn Error>> = get_quotes(url);
    dbg!(quotes);
}
