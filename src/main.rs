#[allow(non_snake_case)]
mod news_scraper;
mod sources;
mod util;

use cursive::{
    theme::Theme,
    views::{Dialog, TextView},
};
use news_scraper::NewsItem;
use rand::seq::SliceRandom;
use simple_tables::Table;
use sources::{guardian::Guardian, independent::Independent};

use crate::{
    news_scraper::{News, NewsSource},
    sources::{al_jazeera::AlJazeera, combinator::Combinator},
    util::clean_news_vec,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // NEWS STUFF
    let mut news: Vec<NewsItem> = Vec::new();
    // TODO: find a better way of doing this
    set_news(&mut news).await?;
    // only keep the first 10 news items

    // UPDATE STUFF
    // start a new thread which updates news every minute with tokio

    // TUI STUFF
    let mut siv = cursive::default();
    siv.add_layer(
        Dialog::new()
            .content(TextView::new(News::from_vec(&news).to_string()))
            .title("News"),
    );
    siv.add_global_callback('q', |s| s.quit());
    let mut theme = Theme::default();
    theme.shadow = true;
    theme.borders = cursive::theme::BorderStyle::Simple;
    siv.set_theme(theme);
    siv.run();

    Ok(())
}

async fn set_news(news: &mut Vec<NewsItem>) -> Result<(), Box<dyn std::error::Error>> {
    let sources: Vec<Box<dyn NewsSource>> = vec![Box::new(AlJazeera {}), Box::new(Combinator {}), Box::new(Guardian {}), Box::new(Independent{})];
    for source in sources {
        let mut source_news = source.get_news().await?;
        source_news.truncate(15);
        news.extend(source_news);
    }
    news.shuffle(&mut rand::thread_rng());
    news.truncate(35);
    clean_news_vec(news);
    Ok(())
}
