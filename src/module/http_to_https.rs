use super::{GuiModule, Module};

pub struct HttpToHttpsModule;

impl GuiModule for HttpToHttpsModule {}

impl Module for HttpToHttpsModule {
    fn name(&self) -> &str {
        "HTTP to HTTPS"
    }

    fn evaluate(&self, url: &str) -> Option<String> {
        if url.starts_with("http://") {
            Some("Upgrade to HTTPS".to_string())
        } else {
            None
        }
    }

    fn transform(&self, url: &str) -> Result<String, String> {
        Ok(url.replacen("http://", "https://", 1))
    }
}
