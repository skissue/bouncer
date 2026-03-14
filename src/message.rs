pub enum Message {
    ApplyModule(usize),
    OpenBrowserPicker,
    CloseBrowserPicker,
    SelectNext,
    SelectPrevious,
    ConfirmSelection,
    Quit,
}

pub enum Action {
    None,
    Quit,
    OpenUrl { exec: String, url: String },
}
