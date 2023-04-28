use crate::news_scraper::{NewsItem, NewsSource};
use async_trait::async_trait;
use scraper::{Html, Selector};
pub struct BBC {}

// #[async_trait]
// impl NewsSource for BBC {
//     async fn get_news(
//         &self,
//     ) -> Result<Vec<crate::news_scraper::NewsItem>, Box<dyn std::error::Error>> {
//         let mut news: Vec<NewsItem> = Vec::new();
//         let body_url = "https://www.bbc.co.uk";
//         let res = reqwest::get(body_url.to_owned() + "/news/world").await?;
//         let body = res.text().await?;
//         let document = Html::parse_document(&body);
//         let selector = Selector::parse("*").unwrap();
//
//         for element in document.select(&selector) {
//             // Link
//             let link_selector = Selector::parse("a.gs-c-promo-heading").unwrap();
//             let link_element = element.select(&link_selector).next().unwrap();
//             let url = body_url.to_string() + link_element.value().attr("href").unwrap();
//
//             // Title
//             let title = "hi".to_string(); // link_element.inner_html();
//
//             news.push(NewsItem { title, url });
//         }
//         Ok(news)
//     }
// }
