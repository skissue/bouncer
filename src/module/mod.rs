mod http_to_https;
mod regex_replacer;
mod tracking_cleaner;
mod unshorten;

use eframe::egui;

pub use http_to_https::HttpToHttpsModule;
pub use regex_replacer::RegexReplacerModule;
pub use tracking_cleaner::TrackingCleanerModule;
pub use unshorten::UnshortenModule;

pub trait Module {
    fn name(&self) -> &str;
    fn evaluate(&self, url: &str) -> Option<String>;
    fn transform(&self, url: &str) -> Result<String, String>;
}

pub enum GuiModuleAction {
    Apply,
}

pub trait GuiModule: Module {
    fn render_offer(
        &mut self,
        ui: &mut egui::Ui,
        offer_idx: usize,
        offer: &str,
        _url: &str,
    ) -> Option<GuiModuleAction> {
        let label = format!("[{}] {} — {}", offer_idx + 1, self.name(), offer);
        if ui.button(&label).clicked() {
            Some(GuiModuleAction::Apply)
        } else {
            None
        }
    }
}
