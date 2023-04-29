use crate::news_scraper::{NewsItem, NewsSource};
use async_trait::async_trait;
use scraper::{Html, Selector};

pub struct Combinator {}

#[async_trait]
impl NewsSource for Combinator {
    async fn get_news(
        &self,
    ) -> Result<Vec<crate::news_scraper::NewsItem>, Box<dyn std::error::Error>> {
        let mut news: Vec<NewsItem> = Vec::new();
        let res = reqwest::get("https://news.ycombinator.com/").await?;
        let body = res.text().await?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse("tr.athing").unwrap();
        for element in document.select(&selector) {
            // Link
            let link_selector = Selector::parse(".title a").unwrap();
            let link_element = element.select(&link_selector).next().unwrap();
            //TODO: some links are relative, check for this and add base url
            let url = link_element.value().attr("href").unwrap().to_string();

            if url.starts_with("item?id=") {
                continue;
            }

            // Title
            let title = link_element.inner_html();

            news.push(NewsItem {
                Source: "Hacker News".to_string(),
                Title: title,
                Url: url,
            });
        }
        Ok(news)
    }
}
