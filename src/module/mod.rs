mod tracking_cleaner;

pub use tracking_cleaner::TrackingCleanerModule;

pub trait Module {
    fn name(&self) -> &str;
    fn evaluate(&self, url: &str) -> Option<String>;
    fn transform(&self, url: &str) -> Result<String, String>;
}
