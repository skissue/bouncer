#[cfg(feature = "tui")]
pub mod tui;
#[cfg(feature = "gui")]
pub mod gui;

#[cfg(all(feature = "tui", feature = "gui"))]
compile_error!("Features `tui` and `gui` are mutually exclusive.");
#[cfg(not(any(feature = "tui", feature = "gui")))]
compile_error!("Either the `tui` or `gui` feature must be enabled.");

use crate::app::App;

#[derive(Clone)]
pub struct RunResult {
    pub exec: String,
    pub url: String,
}

pub trait Backend {
    fn run(self, app: App) -> Result<Option<RunResult>, Box<dyn std::error::Error>>;
}
