mod rules;

use super::Module;
use rules::Cleaner;

pub struct TrackingCleanerModule {
    cleaner: Cleaner,
}

impl TrackingCleanerModule {
    pub fn new() -> Self {
        let data = include_str!("../../../resources/data.minify.json");
        let cleaner = Cleaner::from_json(data);
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
