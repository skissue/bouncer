use crate::rules::Cleaner;

pub trait Module {
    fn name(&self) -> &str;
    fn evaluate(&self, url: &str) -> Option<String>;
    fn transform(&self, url: &str) -> Result<String, String>;
}

pub struct TrackingCleanerModule {
    cleaner: Cleaner,
}

impl TrackingCleanerModule {
    pub fn new(cleaner: Cleaner) -> Self {
        Self { cleaner }
    }
}

impl Module for TrackingCleanerModule {
    fn name(&self) -> &str {
        "Tracking Cleaner"
    }

    fn evaluate(&self, url: &str) -> Option<String> {
        let cleaned = self.cleaner.clean(url);
        if cleaned != url {
            Some("Remove tracking parameters".to_string())
        } else {
            None
        }
    }

    fn transform(&self, url: &str) -> Result<String, String> {
        Ok(self.cleaner.clean(url))
    }
}
