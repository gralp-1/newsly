mod news_scraper;
mod sources;
mod tui;
mod util;

use cursive::{views::{Dialog, TextView}, view::Margins, theme::Theme};
use rand::seq::SliceRandom;
use simple_tables::Table;
use util::custom_table;

use crate::{
    news_scraper::{News, NewsSource},
    sources::{al_jazeera::AlJazeera, combinator::Combinator},
    util::clean_news_vec,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // print args
    let mut siv = cursive::default();
    // Starts the event loop.

    let mut news: Vec<news_scraper::NewsItem>;
    let aj_news = AlJazeera {}.get_news().await?;
    let ycomb_news = Combinator {}.get_news().await?;
    // news is aj_news + ycomb_news
    news = aj_news;
    news.extend(ycomb_news);
    // shuffle news
    news.shuffle(&mut rand::thread_rng());

    // clean news
    clean_news_vec(&mut news);

    siv.add_layer(
        Dialog::new()
        .content(TextView::new(News::from_vec(&news).to_string()))
        .title("News")
        .padding(Margins::lrtb(1, 1, 1, 1))
    );
    let mut theme = Theme::default();
    theme.shadow = false;
    theme.borders = cursive::theme::BorderStyle::Simple;
    siv.set_theme(theme);
    siv.run();
    Ok(())
}
