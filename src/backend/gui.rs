use std::sync::{Arc, Mutex};

use eframe::egui;

use crate::app::App;
use crate::message::{Action, Message};

use super::{Backend, RunResult};

pub struct GuiBackend;

impl Backend for GuiBackend {
    fn run(self, app: App) -> Result<Option<RunResult>, Box<dyn std::error::Error>> {
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
}

struct GuiApp {
    app: App,
    result: Arc<Mutex<Option<Option<RunResult>>>>,
}

impl GuiApp {
    fn new(app: App, result: Arc<Mutex<Option<Option<RunResult>>>>) -> Self {
        Self { app, result }
    }

    fn handle_message(&mut self, msg: Message, ctx: &egui::Context) {
        match self.app.update(msg) {
            Action::None => {}
            Action::Quit => {
                *self.result.lock().unwrap() = Some(None);
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            Action::OpenUrl { exec, url } => {
                *self.result.lock().unwrap() = Some(Some(RunResult { exec, url }));
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Collect keyboard messages first to avoid borrow conflicts.
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
        } else {
            ctx.input(|i| {
                if i.key_pressed(egui::Key::Q) || i.key_pressed(egui::Key::Escape) {
                    Some(Message::Quit)
                } else if i.key_pressed(egui::Key::C) {
                    Some(Message::ToggleCleaning)
                } else if i.key_pressed(egui::Key::Enter) {
                    Some(Message::OpenBrowserPicker)
                } else {
                    None
                }
            })
        };

        if let Some(msg) = msg {
            self.handle_message(msg, ctx);
        }

        // Draw UI.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("bouncer");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.strong("Original URL:");
                ui.label(&self.app.original_url);
            });

            ui.add_space(4.0);

            let active_label = if self.app.cleaning_enabled {
                "Cleaned URL"
            } else {
                "Original URL (cleaning disabled)"
            };
            ui.horizontal(|ui| {
                ui.strong(format!("{active_label}:"));
                let url_text = self.app.active_url();
                if self.app.cleaning_enabled {
                    ui.colored_label(egui::Color32::from_rgb(100, 200, 100), url_text);
                } else {
                    ui.label(url_text);
                }
            });

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.strong("Status:");
                if self.app.original_url != self.app.cleaned_url {
                    ui.colored_label(egui::Color32::from_rgb(100, 200, 100), "✔ Cleaned");
                } else {
                    ui.colored_label(egui::Color32::from_rgb(200, 200, 100), "— No changes");
                }
            });

            ui.add_space(16.0);

            ui.horizontal(|ui| {
                let toggle_label = if self.app.cleaning_enabled {
                    "Disable Cleaning [c]"
                } else {
                    "Enable Cleaning [c]"
                };
                if ui.button(toggle_label).clicked() {
                    let msg = Message::ToggleCleaning;
                    self.handle_message(msg, ctx);
                }

                if ui.button("Open URL [Enter]").clicked() {
                    let msg = Message::OpenBrowserPicker;
                    self.handle_message(msg, ctx);
                }

                if ui.button("Quit [q]").clicked() {
                    let msg = Message::Quit;
                    self.handle_message(msg, ctx);
                }
            });
        });

        // Browser picker modal.
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
                                picked = None; // skip the single-click select
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
