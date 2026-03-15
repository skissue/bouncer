use super::{GuiModule, Module};

const SHORT_DOMAINS: &[&str] = &[
    "bit.ly",
    "t.co",
    "goo.gl",
    "tinyurl.com",
    "ow.ly",
    "is.gd",
    "buff.ly",
    "dlvr.it",
    "j.mp",
];

pub struct UnshortenModule;

impl UnshortenModule {
    fn is_shortened(&self, url: &str) -> bool {
        if let Ok(parsed) = url::Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                return SHORT_DOMAINS.iter().any(|d| host == *d);
            }
        }
        false
    }
}

impl GuiModule for UnshortenModule {}

impl Module for UnshortenModule {
    fn name(&self) -> &str {
        "Unshorten"
    }

    fn evaluate(&self, url: &str) -> Option<String> {
        if self.is_shortened(url) {
            Some("Resolve shortened URL".to_string())
        } else {
            None
        }
    }

    fn transform(&self, url: &str) -> Result<String, String> {
        let api_url = format!("https://unshorten.me/json/{}", url);
        let resp = reqwest::blocking::get(&api_url).map_err(|e| e.to_string())?;
        let body: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
        body["resolved_url"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "No resolved_url in response".to_string())
    }
}
