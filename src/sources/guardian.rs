use async_trait::async_trait;
use scraper::{Html, Selector};

use crate::news_scraper::{NewsItem, NewsSource};

pub struct Guardian {}

#[async_trait]
impl NewsSource for Guardian {
    async fn get_news(&self) -> Result<Vec<NewsItem>, Box<dyn std::error::Error>> {
        let mut news: Vec<NewsItem> = Vec::new();
        let base_url = "https://www.theguardian.com/world";
        let res = reqwest::get(base_url.to_owned()).await?;
        let body = res.text().await?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse("a").unwrap();
        for element in document.select(&selector) {
            // Link
            // get the href attribute
            if !element
                .value()
                .attr("data-link-name")
                .unwrap_or("")
                .contains("article")
            {
                continue;
            }
            let url = element.value().attr("href").unwrap();
            
            // Sometimes nested span's, sometimes not
            let title_selector = Selector::parse("span").unwrap();
            let mut title_element = element.select(&title_selector).next().unwrap_or(element);
            title_element = title_element.select(&title_selector).next().unwrap_or(title_element);

            // get the element span
            let title = title_element.inner_html().trim().to_string();

            // clean up url and title so there's no ending whitespace
            let url = url.trim().to_string();
            let title = title.trim().to_string();

            news.push(NewsItem {
                Source: "Guardian".to_string(),
                Title: title,
                Url: url,
            });
        }
        // contains duplicates, so dedup
        news.dedup_by(|item, other| item.Title == other.Title);
        Ok(news)
    }
}
