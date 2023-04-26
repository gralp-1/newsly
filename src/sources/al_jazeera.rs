use async_trait::async_trait;
use scraper::{Html, Selector};
use crate::news_scraper::{NewsSource, NewsItem};


pub struct AlJazeera {}

#[async_trait]
impl NewsSource for AlJazeera {
    async fn get_news(
        &self,
    ) -> Result<Vec<crate::news_scraper::NewsItem>, Box<dyn std::error::Error>> {
        let mut news: Vec<NewsItem> = Vec::new();
        let base_url = "https://www.aljazeera.com";
        let res = reqwest::get(base_url.to_owned() + "/news/").await?;
        let body = res.text().await?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse("a.u-clickable-card__link").unwrap();
        for element in document.select(&selector) {
            // Link
            // get the href attribute
            let url =
                base_url.to_owned() + &element.value().attr("href").unwrap().to_string();

            // get the element span
            let title_selector = Selector::parse("span").unwrap();
            let title_element = element.select(&title_selector).next().unwrap();
            let title = title_element.inner_html().trim().to_string();

            // clean up url and title so there's no ending whitespace
            let url = url.trim().to_string();
            let title = title.trim().to_string();
            

            news.push(NewsItem { title, url });
        }
        Ok(news)
    }
}
