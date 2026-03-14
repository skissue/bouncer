mod app;
mod backend;
mod browser;
mod message;
mod module;

use backend::Backend;
use module::TrackingCleanerModule;

use crate::backend::{RunAction, RunResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: bouncer <url>");
        std::process::exit(1);
    }

    let url = args[1].clone();

    let modules: Vec<Box<dyn module::Module>> = vec![Box::new(TrackingCleanerModule::new())];

    let browsers = browser::discover_browsers("bouncer");
    let app = app::App::new(url, modules, browsers);

    #[cfg(feature = "tui")]
    let backend = backend::tui::TuiBackend;
    #[cfg(feature = "gui")]
    let backend = backend::gui::GuiBackend;

    let result = backend.run(app)?;

    if let Some(RunResult { action, url }) = result {
        match action {
            RunAction::Exec(command) => {
                browser::open_url_with(&command, &url);
            }
            RunAction::CopyToClipboard => {
                let mut clipboard = arboard::Clipboard::new()
                    .expect("Failed to access clipboard");
                clipboard
                    .set_text(&url)
                    .expect("Failed to copy to clipboard");
            }
        }
    }

    Ok(())
}
