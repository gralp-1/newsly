use std::error::Error;

use async_trait::async_trait;
use scraper::{Html, Selector};

use crate::news_scraper::{NewsItem, NewsSource};

pub struct Independent {}

#[async_trait]
impl NewsSource for Independent {
    async fn get_news(&self) -> Result<Vec<NewsItem>, Box<dyn Error>> {
        let mut news: Vec<NewsItem> = Vec::new();
        let base_url = "https://www.independent.co.uk/news/";
        let res = reqwest::get(base_url.to_owned() + "world/").await?;
        let body = res.text().await?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse("a.title").unwrap();
        for element in document.select(&selector) {
            // Link
            // get the href attribute
            let url = base_url.to_owned() + &element.value().attr("href").unwrap().to_string();

            // get the element span
            let title = element.inner_html().trim().to_string();

            // clean up url and title so there's no ending whitespace
            let url = url.trim().to_string();
            let title = title.trim().to_string();

            news.push(NewsItem {
                Source: "The Independent".to_string(),
                Title: title,
                Url: url,
            });
        }
        Ok(news)
    }
}