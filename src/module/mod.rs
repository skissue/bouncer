mod http_to_https;
mod regex_replacer;
mod tracking_cleaner;
mod unshorten;

pub use http_to_https::HttpToHttpsModule;
pub use regex_replacer::RegexReplacerModule;
pub use tracking_cleaner::TrackingCleanerModule;
pub use unshorten::UnshortenModule;

pub trait Module {
    fn name(&self) -> &str;
    fn evaluate(&self, url: &str) -> Option<String>;
    fn transform(&self, url: &str) -> Result<String, String>;
}
