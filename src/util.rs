use crate::news_scraper::NewsItem;

pub fn clean_news_vec(news_vec: &mut Vec<NewsItem>) {
    for news_item in news_vec {
        clean_news_item(news_item);
    }
}

fn clean_news_item(news_item: &mut NewsItem) {
    news_item.Title = news_item.Title.trim().to_string();
    news_item.Url = news_item.Url.trim().to_string();
    news_item.Title = news_item.Title.replace("\u{00AD}", "");
    news_item.Url = news_item.Url.replace("\u{00AD}", "");
}

fn clickable_link(news_item: &NewsItem) -> String {
    format!(
        "\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
        news_item.Url, news_item.Title
    )
}

pub fn custom_table(news_vec: &Vec<NewsItem>) -> String {
    let mut table = String::new();
    let mut longest_source = 0;
    for news_item in news_vec {
        // get the longest source name
        if news_item.Source.len() > longest_source {
            longest_source = news_item.Source.len();
        }
    }
    // now we have the longest source name, we can format the table
    for news_item in news_vec {
        let source = format!("{:width$}", news_item.Source, width = longest_source);
        table.push_str(&format!("{} | {}\n", source, clickable_link(news_item)));
    }
    table
}
