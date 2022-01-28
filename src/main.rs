mod theme;

use std::error::Error; 

use colour::{cyan};

use dotenv::dotenv;

use quotesapi::{get_quotes, Quote};

fn render_quotes(quotes: &Vec<Quote>) {
    let theme = theme::default();
    theme.print_text("# 10 Random Inspirational Quotes");
    cyan!(">> Total quotes: {}\n\n", quotes.len());
    for i in quotes {
        theme.print_text(&format!("# {}", i.csvId));
        theme.print_text(&format!("> `{}`", i.id));
        theme.print_text(&format!("> *{}*", i.quote));
        theme.print_text(&format!("> **{}**", i.author));
        theme.print_text(&format!("* {}", i.genre));
        theme.print_text("---");
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
