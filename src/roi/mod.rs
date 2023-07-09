mod data;

use data::InitiativeData;
use log::info;
use reqwest::Client;
use scraper::Html;
use scraper::Selector;
use std::error::Error;
use std::fmt::Display;
use std::format;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct RoiClient {
    inner: Arc<Mutex<RoiClientInner>>,
}

impl RoiClient {
    pub fn new(timeout: u64) -> Result<Self, Box<dyn Error>> {
        let header_selector = Selector::parse("div.col-left > h1")?;
        let archive_selector = Selector::parse("li.lock")?;
        let positive_selector = Selector::parse("b.js-voting-info-affirmative")?;
        let negative_selector = Selector::parse("b.js-voting-info-negative")?;
        let timeout = Duration::from_secs(timeout);
        let client = Client::builder()
            .timeout(timeout)
            .danger_accept_invalid_certs(true)
            .build()?;
        let inner = RoiClientInner::new(
            header_selector,
            archive_selector,
            positive_selector,
            negative_selector,
            client,
        );

        Ok(Self {
            inner: Arc::new(Mutex::new(inner)),
        })
    }

    pub async fn load(&self, id: u32) -> Result<InitiativeData, Box<dyn Error>> {
        info!("Loading initiative: id = {}", id);

        let inner = self.inner.lock().await;
        let text = inner
            .client
            .get(format!("https://www.roi.ru/{}/", id))
            .send()
            .await?
            .text()
            .await?;
        let html = Html::parse_document(&text);
        let name = select_text(&html, &inner.header_selector)?;
        let is_archived = html.select(&inner.archive_selector).next().is_some();
        let positive = select_number(&html, &inner.positive_selector)?;
        let negative = select_number(&html, &inner.negative_selector)?;

        Ok(InitiativeData::new(
            id,
            &name,
            is_archived,
            positive,
            negative,
        ))
    }
}

fn select_number<T>(html: &Html, selector: &Selector) -> Result<T, Box<dyn Error>>
where
    T: FromStr,
    T::Err: Display,
{
    let text = html
        .select(selector)
        .next()
        .ok_or_else(|| format!("Element not found: selector = {:?}", selector))?
        .text()
        .flat_map(|text| text.chars().filter(|&ch| char::is_numeric(ch)))
        .fold(String::default(), |mut acc, ch| {
            acc.push(ch);
            acc
        });

    Ok(T::from_str(&text)
        .map_err(|error| format!("Failed to parse number `{}`: {}", text, error))?)
}

fn select_text(html: &Html, selector: &Selector) -> Result<String, Box<dyn Error>> {
    let text = html
        .select(selector)
        .next()
        .ok_or_else(|| format!("Element not found: selector = {:?}", selector))?
        .text()
        .fold(String::default(), |mut acc, text| {
            if !acc.is_empty() {
                acc.push(' ');
            }

            acc.push_str(text.trim());
            acc
        });

    Ok(text)
}

#[derive(Debug)]
struct RoiClientInner {
    header_selector: Selector,
    archive_selector: Selector,
    positive_selector: Selector,
    negative_selector: Selector,
    client: Client,
}

impl RoiClientInner {
    fn new(
        header_selector: Selector,
        archive_selector: Selector,
        positive_selector: Selector,
        negative_selector: Selector,
        client: Client,
    ) -> Self {
        Self {
            header_selector,
            archive_selector,
            positive_selector,
            negative_selector,
            client,
        }
    }
}
