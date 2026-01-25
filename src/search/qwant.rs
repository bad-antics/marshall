// Copyright (c) 2026 bad-antics
// OSINT Search - Qwant API Client

use serde::{Deserialize, Serialize};
use reqwest::Client;

/// Qwant API client for privacy-respecting search
pub struct QwantClient {
    client: Client,
    region: String,
    base_url: String,
}

impl QwantClient {
    pub fn new(region: &str) -> Self {
        Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:128.0) Gecko/20100101 Firefox/128.0")
                .build()
                .unwrap_or_default(),
            region: region.to_string(),
            base_url: "https://api.qwant.com/v3/search/web".to_string(),
        }
    }

    /// Search Qwant and return parsed results
    pub async fn search(&self, query: &str, count: usize) -> Result<Vec<super::WebResult>, String> {
        let url = format!(
            "{}?q={}&count={}&locale={}&offset=0&device=desktop",
            self.base_url,
            urlencoding::encode(query),
            count.min(50),
            self.region
        );

        let response = self.client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Qwant returned status: {}", response.status()));
        }

        let json: QwantResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(self.parse_results(json))
    }

    /// Parse Qwant response into our WebResult format
    fn parse_results(&self, response: QwantResponse) -> Vec<super::WebResult> {
        let mut results = Vec::new();

        if let Some(data) = response.data {
            if let Some(items) = data.result.items {
                for item in items {
                    let domain = extract_domain(&item.url);
                    results.push(super::WebResult {
                        title: item.title,
                        url: item.url.clone(),
                        domain,
                        snippet: item.desc.unwrap_or_default(),
                        favicon_url: item.favicon,
                        cached_url: None,
                        osint: None,
                    });
                }
            }
        }

        results
    }

    /// Search for images
    pub async fn search_images(&self, query: &str, count: usize) -> Result<Vec<ImageResult>, String> {
        let url = format!(
            "https://api.qwant.com/v3/search/images?q={}&count={}&locale={}&offset=0",
            urlencoding::encode(query),
            count.min(50),
            self.region
        );

        let response = self.client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: QwantImageResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(json.data.map(|d| d.result.items.unwrap_or_default()).unwrap_or_default())
    }

    /// Search for news
    pub async fn search_news(&self, query: &str, count: usize) -> Result<Vec<NewsResult>, String> {
        let url = format!(
            "https://api.qwant.com/v3/search/news?q={}&count={}&locale={}&offset=0",
            urlencoding::encode(query),
            count.min(50),
            self.region
        );

        let response = self.client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: QwantNewsResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(json.data.map(|d| d.result.items.unwrap_or_default()).unwrap_or_default())
    }
}

/// Extract domain from URL
fn extract_domain(url: &str) -> String {
    url::Url::parse(url)
        .map(|u| u.host_str().unwrap_or("unknown").to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

// Qwant API Response structures

#[derive(Debug, Deserialize)]
pub struct QwantResponse {
    pub status: String,
    pub data: Option<QwantData>,
}

#[derive(Debug, Deserialize)]
pub struct QwantData {
    pub result: QwantResult,
}

#[derive(Debug, Deserialize)]
pub struct QwantResult {
    pub items: Option<Vec<QwantItem>>,
    pub total: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct QwantItem {
    pub title: String,
    pub url: String,
    pub desc: Option<String>,
    pub favicon: Option<String>,
    pub source: Option<String>,
}

// Image search response
#[derive(Debug, Deserialize)]
pub struct QwantImageResponse {
    pub data: Option<QwantImageData>,
}

#[derive(Debug, Deserialize)]
pub struct QwantImageData {
    pub result: QwantImageResult,
}

#[derive(Debug, Deserialize)]
pub struct QwantImageResult {
    pub items: Option<Vec<ImageResult>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResult {
    pub title: String,
    pub url: String,
    pub thumbnail: String,
    pub media: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

// News search response
#[derive(Debug, Deserialize)]
pub struct QwantNewsResponse {
    pub data: Option<QwantNewsData>,
}

#[derive(Debug, Deserialize)]
pub struct QwantNewsData {
    pub result: QwantNewsResult,
}

#[derive(Debug, Deserialize)]
pub struct QwantNewsResult {
    pub items: Option<Vec<NewsResult>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsResult {
    pub title: String,
    pub url: String,
    pub desc: Option<String>,
    pub source: Option<String>,
    pub date: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain("https://example.com/path"), "example.com");
        assert_eq!(extract_domain("https://sub.example.com/"), "sub.example.com");
    }
}
