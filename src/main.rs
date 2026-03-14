mod app;
mod backend;
mod browser;
mod message;
mod module;
mod rules;

use backend::Backend;
use module::TrackingCleanerModule;

use crate::backend::RunResult;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: bouncer <url>");
        std::process::exit(1);
    }

    let data = include_str!("../data.minify.json");
    let cleaner = rules::Cleaner::from_json(data);
    let url = args[1].clone();

    let modules: Vec<Box<dyn module::Module>> = vec![Box::new(TrackingCleanerModule::new(cleaner))];

    let browsers = browser::discover_browsers("bouncer");
    let app = app::App::new(url, modules, browsers);

    #[cfg(feature = "tui")]
    let backend = backend::tui::TuiBackend;
    #[cfg(feature = "gui")]
    let backend = backend::gui::GuiBackend;

    let result = backend.run(app)?;

    if let Some(RunResult { action, url }) = result {
        match action {
            backend::RunAction::Exec(command) => {
                browser::open_url_with(&command, &url);
            }
        }
    }

    Ok(())
}
