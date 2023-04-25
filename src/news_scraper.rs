use scraper::{Html, Selector};
use tabled::Tabled;

#[derive(Tabled)]
pub struct NewsItem {
    // I know capitalized fields are gross but macros
    pub Rank: u32,
    pub Title: String,
    pub Url: String,
    // pub points: u32,
}

pub async fn get_news() -> Result<Vec<NewsItem>, Box<dyn std::error::Error>> {
    let mut news: Vec<NewsItem> = Vec::new();
    let res = reqwest::get("https://news.ycombinator.com/").await?;
    let body = res.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("tr.athing").unwrap();
    for (rank, element) in document.select(&selector).enumerate() {

        // Link
        let link_selector = Selector::parse(".title a").unwrap();
        let link_element = element.select(&link_selector).next().unwrap();
        let Url = link_element.value().attr("href").unwrap().to_string();

        // Title
        let Title = link_element.inner_html();

        news.push(NewsItem {
            Rank: (rank + 1) as u32,
            Title,
            Url,
        });
    }
    Ok(news)
}
