use std::error::Error;
use colour::{dark_green, yellow};
use newsapi::{Articles, get_articles};


fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green!("> {}\n", i.title);
        yellow!("> {}\n\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url: &str = "https://newsapi.org/v2/top-headlines?country=gb&apiKey=30beaf465fd44dbe842fea046c826399";
    let articles: Articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}



