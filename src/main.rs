mod app;
mod backend;
mod browser;
mod config;
mod message;
mod module;

use backend::Backend;
use module::{HttpToHttpsModule, RegexReplacerModule, TrackingCleanerModule, UnshortenModule};

use crate::backend::{RunAction, RunResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: bouncer <url>");
        std::process::exit(1);
    }

    let url = args[1].clone();
    let config = config::Config::load();

    let mut modules: Vec<Box<dyn module::Module>> = Vec::new();
    for slug in &config.enabled_modules {
        let m: Box<dyn module::Module> = match slug.as_str() {
            "https" => Box::new(HttpToHttpsModule),
            "tracking_cleaner" => Box::new(TrackingCleanerModule::new()),
            "unshorten" => Box::new(UnshortenModule),
            "regex_replacer" => Box::new(RegexReplacerModule::new(&config)),
            unknown => {
                eprintln!("Error: unknown module '{unknown}' in config");
                std::process::exit(1);
            }
        };
        modules.push(m);
    }

    let browsers = browser::discover_browsers("bouncer");
    let app = app::App::new(url, modules, browsers);

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
