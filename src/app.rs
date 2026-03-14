use crate::browser::BrowserEntry;
use crate::message::{Action, Message};
use crate::module::Module;

pub struct App {
    pub original_url: String,
    pub url: String,
    pub should_quit: bool,
    pub browsers: Vec<BrowserEntry>,
    pub selected_browser: usize,
    pub show_browser_picker: bool,
    pub modules: Vec<Box<dyn Module>>,
    pub offers: Vec<(usize, String)>,
}

impl App {
    pub fn new(
        url: String,
        modules: Vec<Box<dyn Module>>,
        browsers: Vec<BrowserEntry>,
    ) -> Self {
        let default_idx = browsers
            .iter()
            .position(|b| b.is_default)
            .unwrap_or(0);

        let offers = Self::evaluate_modules(&modules, &url);

        Self {
            original_url: url.clone(),
            url,
            should_quit: false,
            browsers,
            selected_browser: default_idx,
            show_browser_picker: false,
            modules,
            offers,
        }
    }

    fn evaluate_modules(modules: &[Box<dyn Module>], url: &str) -> Vec<(usize, String)> {
        modules
            .iter()
            .enumerate()
            .filter_map(|(i, m)| m.evaluate(url).map(|proposal| (i, proposal)))
            .collect()
    }

    fn re_evaluate(&mut self) {
        self.offers = Self::evaluate_modules(&self.modules, &self.url);
    }

    pub fn active_url(&self) -> &str {
        &self.url
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::ApplyModule(idx) => {
                if let Some(&(module_idx, _)) = self.offers.get(idx) {
                    if let Ok(new_url) = self.modules[module_idx].transform(&self.url) {
                        self.url = new_url;
                        self.re_evaluate();
                    }
                }
                Action::None
            }
            Message::OpenBrowserPicker => {
                self.show_browser_picker = true;
                Action::None
            }
            Message::CloseBrowserPicker => {
                self.show_browser_picker = false;
                Action::None
            }
            Message::SelectNext => {
                if self.selected_browser + 1 < self.browsers.len() {
                    self.selected_browser += 1;
                }
                Action::None
            }
            Message::SelectPrevious => {
                if self.selected_browser > 0 {
                    self.selected_browser -= 1;
                }
                Action::None
            }
            Message::ConfirmSelection => {
                if let Some(entry) = self.browsers.get(self.selected_browser) {
                    Action::OpenUrl {
                        exec: entry.exec.clone(),
                        url: self.active_url().to_string(),
                    }
                } else {
                    Action::None
                }
            }
            Message::Quit => {
                self.should_quit = true;
                Action::Quit
            }
        }
    }
}
