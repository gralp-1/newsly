use async_trait::async_trait;
use simple_tables::macros::{table, table_row};

#[table_row]
pub struct NewsItem {
    pub Source: String,
    pub Title: String,
    pub Url: String,
}
#[table(rows=NewsItem)]
pub struct News {}

#[async_trait]
pub trait NewsSource {
    // name field in struct
    async fn get_news(&self) -> Result<Vec<NewsItem>, Box<dyn std::error::Error>>;
}
