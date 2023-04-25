mod news_scraper;
use news_scraper::{get_news};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let news = get_news().await?;
    let table = tabled::Table::new(&news);
    println!("{}", table.to_string());
    Ok(())
}
