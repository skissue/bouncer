pub enum Message {
    ApplyModule(usize),
    SetUrl(String),
    OpenBrowserPicker,
    CloseBrowserPicker,
    SelectNext,
    SelectPrevious,
    ConfirmSelection,
    CopyToClipboard,
    Quit,
}

pub enum Action {
    None,
    Quit,
    OpenUrl { exec: String, url: String },
    CopyToClipboard { url: String },
}
