use sources::{NewsFeed, NewsSource, Reuters, BBC};
use std::{error::Error, fs::File};
mod sources;

fn main() -> Result<(), Box<dyn Error>> {
    let reuters = Reuters {
        base_url: "https://www.reuters.com".to_string(),
        topic_urls: vec![
            String::from("world"),
            String::from("technology"),
            String::from("business"),
        ],
    };
    let reuters_news_feed = reuters.fetch()?;
    let bbc = BBC {
        base_url: "https://www.bbc.com".to_string(),
        topic_urls: vec![
            String::from("news/world"),
            String::from("news/technology"),
            String::from("news/business"),
        ],
    };
    let bbc_news_feed = bbc.fetch()?;

    let news_feed: Vec<NewsFeed> = vec![reuters_news_feed, bbc_news_feed]
        .into_iter()
        .flatten()
        .collect();
    let f = File::create("./data/news_feed.json")?;
    serde_json::to_writer(f, &news_feed)?;

    Ok(())
}
