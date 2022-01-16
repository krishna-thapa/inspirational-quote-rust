use std::error::Error; 

use serde::Deserialize;

use colour::{yellow, magenta, green, cyan, red};

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

fn render_quotes(quotes: &Vec<Quote>) {
    for i in quotes {
        yellow!("> {}\n", i.id);
        red!("-- CsvId: {}\n", i.csvId);
        magenta!("-- Quote: {}\n", i.quote);
        cyan!("-- >> Author: {}\n", i.author);
        green!("-- >> Genre: {}\n\n", i.genre);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Running the main function! ===");
    let url: &str = "http://localhost:9000/quote/randomTen";
    let quotes: Vec<Quote> = get_quotes(url)?;
    
    render_quotes(&quotes);

    Ok(())
}
