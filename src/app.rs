use crate::browser::BrowserEntry;
use crate::message::{Action, Message};
use crate::module::Module;

pub struct App {
    pub url: String,
    pub undo_stack: Vec<String>,
    pub redo_stack: Vec<String>,
    pub browsers: Vec<BrowserEntry>,
    pub selected_browser: usize,
    pub show_browser_picker: bool,
    pub modules: Vec<Box<dyn Module>>,
    pub offers: Vec<(usize, String)>,
}

impl App {
    pub fn new(url: String, modules: Vec<Box<dyn Module>>, browsers: Vec<BrowserEntry>) -> Self {
        let default_idx = browsers.iter().position(|b| b.is_default).unwrap_or(0);

        let offers = Self::evaluate_modules(&modules, &url);

        Self {
            url,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
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

    fn push_url(&mut self, new_url: String) {
        self.undo_stack.push(std::mem::replace(&mut self.url, new_url));
        self.redo_stack.clear();
        self.re_evaluate();
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn active_url(&self) -> &str {
        &self.url
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::ApplyModule(idx) => {
                if let Some(&(module_idx, _)) = self.offers.get(idx) {
                    if let Ok(new_url) = self.modules[module_idx].transform(&self.url) {
                        self.push_url(new_url);
                    }
                }
                Action::None
            }
            Message::SetUrl(new_url) => {
                self.push_url(new_url);
                Action::None
            }
            Message::Undo => {
                if let Some(prev) = self.undo_stack.pop() {
                    self.redo_stack.push(std::mem::replace(&mut self.url, prev));
                    self.re_evaluate();
                }
                Action::None
            }
            Message::Redo => {
                if let Some(next) = self.redo_stack.pop() {
                    self.undo_stack.push(std::mem::replace(&mut self.url, next));
                    self.re_evaluate();
                }
                Action::None
            }
            Message::UndoAll => {
                if let Some(first) = self.undo_stack.first().cloned() {
                    self.undo_stack.push(std::mem::replace(&mut self.url, first));
                    let all = self.undo_stack.drain(..).collect::<Vec<_>>();
                    // all contains: [original, ..., prev_current]
                    // redo_stack should let RedoAll restore to prev_current (the last entry)
                    self.redo_stack.extend(all.into_iter().skip(1));
                    self.re_evaluate();
                }
                Action::None
            }
            Message::RedoAll => {
                if let Some(last) = self.redo_stack.last().cloned() {
                    self.redo_stack.push(std::mem::replace(&mut self.url, last));
                    let all = self.redo_stack.drain(..).collect::<Vec<_>>();
                    // all contains: [oldest_redo, ..., prev_current]
                    // undo_stack should let UndoAll restore back
                    let len = all.len();
                    self.undo_stack.extend(all.into_iter().take(len - 1));
                    self.re_evaluate();
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
            Message::CopyToClipboard => Action::CopyToClipboard {
                url: self.active_url().to_string(),
            },
            Message::Quit => Action::Quit,
        }
    }
}
