pub mod gui;

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
