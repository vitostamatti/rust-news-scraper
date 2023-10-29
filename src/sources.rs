use chrono::{DateTime, Utc};
use scraper::{ElementRef, Html, Selector};
use serde::Serialize;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct NewsFeed {
    pub id: String,
    pub url: String,
    pub title: String,
    pub text: String,
    pub datetime: DateTime<Utc>,
    pub source: String,
}

pub trait NewsSource {
    fn fetch(&self) -> Result<Vec<NewsFeed>, Box<dyn Error>>;
}

pub struct Reuters {
    pub base_url: String,
    pub topic_urls: Vec<String>,
}
pub struct BBC {
    pub base_url: String,
    pub topic_urls: Vec<String>,
}

impl Reuters {
    fn fetch_news_text(&self, url: &str) -> Result<String, Box<dyn Error>> {
        println!("{url}");
        let response = reqwest::blocking::get(url)?.text()?;

        let document = Html::parse_document(&response);

        let news_text_selector =
            Selector::parse("#main-content > article > div > div > div > div > div > p")?;

        let text: String = document
            .select(&news_text_selector)
            .map(|e| Html::parse_fragment(&e.html()))
            .flat_map(|frag| {
                frag.tree.into_iter().map(|n| {
                    if let scraper::node::Node::Text(text) = n {
                        Some(text.text.to_string())
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect::<Vec<String>>()
            .join("");

        Ok(text)
    }
    fn fetch_news_feed(&self, topic_url: &str) -> Result<Vec<NewsFeed>, Box<dyn Error>> {
        let datetime = Utc::now();

        let url = format!("{}/{}", &self.base_url, &topic_url);

        let response = reqwest::blocking::get(url)?.text()?;

        let document = Html::parse_document(&response);

        let news_list_selector =
            Selector::parse("#main-content > div:nth-child(3) > ul > li > div > div > h3 > a")?;

        let mut news_feeds: Vec<NewsFeed> = vec![];

        let elements: Vec<ElementRef> = document.select(&news_list_selector).collect();

        for e in elements {
            if let Some(link) = e.attr("href") {
                let url = format!("{}{}", &self.base_url, link);
                let title = e.inner_html().to_string();
                let id = Uuid::new_v4().to_string();
                let text = self.fetch_news_text(&url)?;
                let news_feed = NewsFeed {
                    id,
                    url,
                    title,
                    text,
                    datetime,
                    source: self.base_url.to_string(),
                };
                news_feeds.push(news_feed);
            }
        }
        Ok(news_feeds)
    }
}

impl NewsSource for Reuters {
    fn fetch(&self) -> Result<Vec<NewsFeed>, Box<dyn Error>> {
        let mut news_data: Vec<NewsFeed> = vec![];
        for topic in &self.topic_urls {
            news_data.extend(self.fetch_news_feed(topic)?);
        }
        Ok(news_data)
    }
}

impl BBC {
    fn fetch_news_text(&self, url: &str) -> Result<String, Box<dyn Error>> {
        println!("{url}");

        let response = reqwest::blocking::get(url)?.text()?;

        let document = Html::parse_document(&response);

        let news_text_selector = Selector::parse("#main-content > article > div > div > p")?;

        let text: String = document
            .select(&news_text_selector)
            .map(|e| Html::parse_fragment(&e.html()))
            .flat_map(|frag| {
                frag.tree.into_iter().map(|n| {
                    if let scraper::node::Node::Text(text) = n {
                        Some(text.text.to_string())
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect::<Vec<String>>()
            .join("");

        return Ok(text);
    }
    fn fetch_news_feed(&self, topic_url: &str) -> Result<Vec<NewsFeed>, Box<dyn Error>> {
        let datetime = Utc::now();

        let url = format!("{}/{}", &self.base_url, &topic_url);

        let response = reqwest::blocking::get(url)?.text()?;

        let document = Html::parse_document(&response);

        let news_list_selector =
            Selector::parse("#topos-component > div > div > div:nth-child(2) > div > div > div > div > div > div > div:nth-child(1) > a")?;
        let h3_selector = Selector::parse("h3")?;
        let elements: Vec<ElementRef> = document.select(&news_list_selector).collect();

        let mut news_feeds: Vec<NewsFeed> = vec![];

        for e in elements {
            let title: Option<String> = e.select(&h3_selector).map(|e| e.inner_html()).nth(0);
            if let Some(title) = title {
                if let Some(link) = e.attr("href") {
                    let url = format!("{}{}", &self.base_url, link);
                    let id = Uuid::new_v4().to_string();
                    let text = self.fetch_news_text(&url)?;
                    news_feeds.push(NewsFeed {
                        id,
                        url,
                        title,
                        text,
                        datetime,
                        source: self.base_url.to_string(),
                    })
                }
            }
        }
        Ok(news_feeds)
    }
}

impl NewsSource for BBC {
    fn fetch(&self) -> Result<Vec<NewsFeed>, Box<dyn Error>> {
        let mut news_data: Vec<NewsFeed> = vec![];
        for topic in &self.topic_urls {
            news_data.extend(self.fetch_news_feed(topic)?);
        }
        Ok(news_data)
    }
}
