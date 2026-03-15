use super::{RunResult, RunAction};
use crate::app::App;
use crate::message::{Action, Message};
use crate::module::GuiModuleAction;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub fn run(app: App) -> Result<Option<RunResult>, Box<dyn std::error::Error>> {
    let result = Arc::new(Mutex::new(None::<Option<RunResult>>));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Bouncer")
            .with_app_id("bouncer")
            .with_inner_size([500.0, 350.0])
            .with_window_type(egui::viewport::X11WindowType::Dialog),
        ..Default::default()
    };

    let result_clone = Arc::clone(&result);
    eframe::run_native(
        "bouncer",
        options,
        Box::new(move |_cc| Ok(Box::new(GuiApp::new(app, result_clone)))),
    )
    .map_err(|e| format!("eframe error: {e}"))?;

    let lock = result.lock().unwrap();
    Ok(lock.clone().unwrap_or(None))
}

struct GuiApp {
    app: App,
    result: Arc<Mutex<Option<Option<RunResult>>>>,
    url_edit_buf: String,
    editing_url: bool,
}

impl GuiApp {
    fn new(app: App, result: Arc<Mutex<Option<Option<RunResult>>>>) -> Self {
        let url_edit_buf = app.url.clone();
        Self {
            app,
            result,
            url_edit_buf,
            editing_url: false,
        }
    }

    fn handle_message(&mut self, msg: Message, ctx: &egui::Context) {
        match self.app.update(msg) {
            Action::None => {
                self.url_edit_buf = self.app.url.clone();
            }
            Action::Quit => {
                *self.result.lock().unwrap() = Some(None);
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            Action::OpenUrl { exec, url } => {
                *self.result.lock().unwrap() = Some(Some(RunResult {
                    action: RunAction::Exec(exec),
                    url,
                }));
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            Action::CopyToClipboard { url } => {
                *self.result.lock().unwrap() = Some(Some(RunResult {
                    action: RunAction::CopyToClipboard,
                    url,
                }));
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let url_field_id = egui::Id::new("url_edit_field");

        let msg = if self.app.show_browser_picker {
            ctx.input(|i| {
                if i.key_pressed(egui::Key::Escape) {
                    Some(Message::CloseBrowserPicker)
                } else if i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::K) {
                    Some(Message::SelectPrevious)
                } else if i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::J) {
                    Some(Message::SelectNext)
                } else if i.key_pressed(egui::Key::Enter) {
                    Some(Message::ConfirmSelection)
                } else {
                    None
                }
            })
        } else if self.editing_url {
            let (esc, enter) = ctx.input(|i| {
                (
                    i.key_pressed(egui::Key::Escape),
                    i.key_pressed(egui::Key::Enter),
                )
            });
            if esc {
                self.url_edit_buf = self.app.url.clone();
                self.editing_url = false;
                ctx.memory_mut(|m| m.surrender_focus(url_field_id));
                None
            } else if enter {
                self.editing_url = false;
                ctx.memory_mut(|m| m.surrender_focus(url_field_id));
                Some(Message::SetUrl(self.url_edit_buf.clone()))
            } else {
                None
            }
        } else {
            ctx.input(|i| {
                if i.key_pressed(egui::Key::Q) || i.key_pressed(egui::Key::Escape) {
                    Some(Message::Quit)
                } else if i.key_pressed(egui::Key::Enter) {
                    Some(Message::OpenBrowserPicker)
                } else if i.key_pressed(egui::Key::C) {
                    Some(Message::CopyToClipboard)
                } else if i.key_pressed(egui::Key::E) {
                    self.editing_url = true;
                    None
                } else if i.key_pressed(egui::Key::U) {
                    if i.modifiers.shift {
                        Some(Message::UndoAll)
                    } else {
                        Some(Message::Undo)
                    }
                } else if i.key_pressed(egui::Key::R) {
                    if i.modifiers.shift {
                        Some(Message::RedoAll)
                    } else {
                        Some(Message::Redo)
                    }
                } else {
                    for n in 1..=9u8 {
                        let key = match n {
                            1 => egui::Key::Num1,
                            2 => egui::Key::Num2,
                            3 => egui::Key::Num3,
                            4 => egui::Key::Num4,
                            5 => egui::Key::Num5,
                            6 => egui::Key::Num6,
                            7 => egui::Key::Num7,
                            8 => egui::Key::Num8,
                            9 => egui::Key::Num9,
                            _ => unreachable!(),
                        };
                        if i.key_pressed(key) {
                            return Some(Message::ApplyModule((n - 1) as usize));
                        }
                    }
                    None
                }
            })
        };

        if let Some(msg) = msg {
            self.handle_message(msg, ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("bouncer");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.strong("URL:");
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.url_edit_buf)
                        .id(url_field_id)
                        .desired_width(ui.available_width()),
                );
                if self.editing_url && !response.has_focus() {
                    response.request_focus();
                } else if response.gained_focus() {
                    self.editing_url = true;
                }
            });

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.add_enabled(self.app.can_undo(), egui::Button::new("⟲ Undo")).clicked() {
                    self.handle_message(Message::Undo, ctx);
                }
                if ui.add_enabled(self.app.can_undo(), egui::Button::new("⟲⟲ Undo All")).clicked() {
                    self.handle_message(Message::UndoAll, ctx);
                }
                if ui.add_enabled(self.app.can_redo(), egui::Button::new("⟳ Redo")).clicked() {
                    self.handle_message(Message::Redo, ctx);
                }
                if ui.add_enabled(self.app.can_redo(), egui::Button::new("⟳⟳ Redo All")).clicked() {
                    self.handle_message(Message::RedoAll, ctx);
                }
            });

            ui.add_space(12.0);

            if !self.app.offers.is_empty() {
                ui.strong("Available actions:");
                ui.add_space(4.0);

                let offers: Vec<(usize, usize, String)> = self
                    .app
                    .offers
                    .iter()
                    .enumerate()
                    .map(|(i, (module_idx, proposal))| (i, *module_idx, proposal.clone()))
                    .collect();
                let url = self.app.url.clone();

                let mut clicked_offer = None;
                for (offer_idx, module_idx, proposal) in &offers {
                    let module = &mut self.app.modules[*module_idx];
                    if let Some(GuiModuleAction::Apply) =
                        module.render_offer(ui, *offer_idx, proposal, &url)
                    {
                        clicked_offer = Some(*offer_idx);
                    }
                }

                if let Some(idx) = clicked_offer {
                    self.handle_message(Message::ApplyModule(idx), ctx);
                }

                ui.add_space(8.0);
            }

            ui.horizontal(|ui| {
                if ui.button("Open URL [Enter]").clicked() {
                    let msg = Message::OpenBrowserPicker;
                    self.handle_message(msg, ctx);
                }

                if ui.button("Copy URL [c]").clicked() {
                    let msg = Message::CopyToClipboard;
                    self.handle_message(msg, ctx);
                }

                if ui.button("Quit [q]").clicked() {
                    let msg = Message::Quit;
                    self.handle_message(msg, ctx);
                }
            });
        });

        if self.app.show_browser_picker {
            let mut close_picker = false;
            let mut picked: Option<usize> = None;

            egui::Window::new("Select Browser")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (i, browser) in self.app.browsers.iter().enumerate() {
                            let label = if browser.is_default {
                                format!("{} (default)", browser.name)
                            } else {
                                browser.name.clone()
                            };
                            let selected = i == self.app.selected_browser;
                            let response = ui.selectable_label(selected, &label);
                            if response.clicked() {
                                picked = Some(i);
                            }
                            if response.double_clicked() {
                                self.app.selected_browser = i;
                                picked = None;
                                let msg = Message::ConfirmSelection;
                                self.handle_message(msg, ctx);
                                return;
                            }
                        }
                    });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Open [Enter]").clicked() {
                            let msg = Message::ConfirmSelection;
                            self.handle_message(msg, ctx);
                        }
                        if ui.button("Cancel [Esc]").clicked() {
                            close_picker = true;
                        }
                    });
                });

            if let Some(idx) = picked {
                self.app.selected_browser = idx;
            }
            if close_picker {
                self.handle_message(Message::CloseBrowserPicker, ctx);
            }
        }
    }
}
