use std::error::Error; 

use colour::{yellow, magenta, green, cyan, red};

use dotenv::dotenv;

use quotesapi::{get_quotes, Quote};

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
    
    dotenv()?;
    let api_key: String = std::env::var("API_KEY")?;

    println!("=== API KEY to be used: {} ===", api_key);

    let url: &str = "http://localhost:9000/quote/randomTen";
    let quotes: Vec<Quote> = get_quotes(url)?;
    
    render_quotes(&quotes);

    Ok(())
}
