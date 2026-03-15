pub mod gui;

use crate::app::App;

#[derive(Clone)]
pub struct RunResult {
    pub action: RunAction,
    pub url: String,
}

#[derive(Clone)]
pub enum RunAction {
    Exec(String),
    CopyToClipboard,
}

pub trait Backend {
    fn run(self, app: App) -> Result<Option<RunResult>, Box<dyn std::error::Error>>;
}
