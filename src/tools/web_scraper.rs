use crate::tools::Tool;
use anyhow::Result;
use async_trait::async_trait;
use scraper::{Html, Selector};

/// A tool for scraping websites.
///
/// This tool allows the agent to fetch the content of a URL and parse it to
/// extract the text from the body of the HTML.
pub struct WebScraperTool;

#[async_trait]
impl Tool for WebScraperTool {
    /// Returns the name of the tool, "WebScraperTool".
    fn name(&self) -> &str {
        "WebScraperTool"
    }

    /// Executes the web scraping command.
    ///
    /// # Arguments
    ///
    /// * `args` - The URL of the website to scrape.
    ///
    /// # Returns
    ///
    /// A `Result` with the scraped text content of the website's body, or an
    /// error if the website cannot be scraped.
    async fn execute(&self, args: &str) -> Result<String> {
        let url = args.trim();
        let resp = reqwest::get(url).await?.text().await?;
        let document = Html::parse_document(&resp);
        let selector = Selector::parse("body").unwrap();
        let body = match document.select(&selector).next() {
            Some(body) => body,
            None => return Ok("".to_string()),
        };
        Ok(body.text().collect::<Vec<_>>().join("\n"))
    }
}
