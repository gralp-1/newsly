mod news_scraper;
mod sources;
use simple_tables::Table;

use crate::{sources::{combinator::Combinator, al_jazeera::AlJazeera}, news_scraper::{NewsSource, News}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // print args
    let args: Vec<String> = std::env::args().collect();
    // get the last arg
    let source = args.last().unwrap();
    let mut news: Vec<news_scraper::NewsItem>;
    match source as &str {
        "aljazeera" => {
            news = AlJazeera{}.get_news().await?;
        },
        "combinator" => {
            news = Combinator{}.get_news().await?;
        },
        _ => {
            println!("No news source specified, using default (combinator)");
            news = Combinator{}.get_news().await?;
        }
    }

    news.iter_mut().for_each(|item| {
        item.title = item.title.replace('\u{00AD}', "");
        // set the max length of the title to 50
    });
    let table = News::from_vec(&news);
    println!("{}", table.to_string());
    Ok(())
}
