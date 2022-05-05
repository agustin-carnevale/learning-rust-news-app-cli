mod theme;

use newsapi::{get_articles, Articles};
use std::error::Error;
// use colour::{dark_green,yellow};

fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for article in &articles.articles {
        theme.print_text(&format!("`{}`", article.title));
        theme.print_text(&format!("> *{}*", article.url));
        theme.print_text("---");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("API_KEY")?;

    let url = "https://newsapi.org/v2/top-headlines?country=us&category=business&apiKey=";

    let url = format!("{}{}", url, api_key);

    let articles = get_articles(&url)?;

    render_articles(&articles);
    Ok(())
}
