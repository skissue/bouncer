use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Paragraph, Wrap},
};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

use crate::app::App;
use crate::message::{Action, Message};

use super::{Backend, RunResult};

pub struct TuiBackend;

impl Backend for TuiBackend {
    fn run(self, mut app: App) -> Result<Option<RunResult>, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = event_loop(&mut terminal, &mut app);

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        result
    }
}

fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<Option<RunResult>, Box<dyn std::error::Error>> {
    let mut url_input: Option<Input> = None;

    loop {
        terminal.draw(|frame| draw(frame, app, &url_input))?;

        let ev = event::read()?;
        if let Event::Key(key) = &ev {
            if key.kind != KeyEventKind::Press {
                continue;
            }
        }

        // URL editor mode
        if let Some(ref mut input) = url_input {
            if let Event::Key(key) = &ev {
                match key.code {
                    KeyCode::Enter => {
                        let new_url = input.value().to_string();
                        url_input = None;
                        let msg = Message::SetUrl(new_url);
                        match app.update(msg) {
                            Action::None => {}
                            Action::Quit => return Ok(None),
                            Action::OpenUrl { exec, url } => {
                                return Ok(Some(RunResult { exec, url }));
                            }
                        }
                    }
                    KeyCode::Esc => {
                        url_input = None;
                    }
                    _ => {
                        input.handle_event(&ev);
                    }
                }
            }
            continue;
        }

        if let Event::Key(key) = &ev {
            let msg = if app.show_browser_picker {
                match key.code {
                    KeyCode::Esc => Some(Message::CloseBrowserPicker),
                    KeyCode::Up | KeyCode::Char('k') => Some(Message::SelectPrevious),
                    KeyCode::Down | KeyCode::Char('j') => Some(Message::SelectNext),
                    KeyCode::Enter => Some(Message::ConfirmSelection),
                    _ => None,
                }
            } else {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),
                    KeyCode::Enter => Some(Message::OpenBrowserPicker),
                    KeyCode::Char('e') => {
                        url_input = Some(Input::new(app.url.clone()));
                        None
                    }
                    KeyCode::Char(c @ '1'..='9') => {
                        let idx = (c as usize) - ('1' as usize);
                        Some(Message::ApplyModule(idx))
                    }
                    _ => None,
                }
            };

            if let Some(msg) = msg {
                match app.update(msg) {
                    Action::None => {}
                    Action::Quit => return Ok(None),
                    Action::OpenUrl { exec, url } => {
                        return Ok(Some(RunResult { exec, url }));
                    }
                }
            }
        }
    }
}

fn draw(frame: &mut Frame, app: &App, url_input: &Option<Input>) {
    let chunks = Layout::vertical([
        Constraint::Min(5),
        Constraint::Length(3),
    ])
    .split(frame.area());

    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(" bouncer ");

    let mut text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Original URL:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(format!("  {}", app.original_url)),
    ];

    if app.url != app.original_url {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            "  Current URL:",
            Style::default().add_modifier(Modifier::BOLD),
        )));
        text.push(Line::from(Span::styled(
            format!("  {}", app.url),
            Style::default().fg(Color::Green),
        )));
    }

    if !app.offers.is_empty() {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            "  Available actions:",
            Style::default().add_modifier(Modifier::BOLD),
        )));
        for (i, (module_idx, proposal)) in app.offers.iter().enumerate() {
            let name = app.modules[*module_idx].name();
            text.push(Line::from(format!(
                "  [{}] {} — {}",
                i + 1,
                name,
                proposal
            )));
        }
    }

    let paragraph = Paragraph::new(text)
        .block(main_block)
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, chunks[0]);

    if app.show_browser_picker {
        draw_browser_picker(frame, app, chunks[1]);
    } else {
        draw_footer(frame, app, chunks[1]);
    }

    // URL editor overlay
    if let Some(input) = url_input {
        let width = (frame.area().width - 4).min(80);
        let height = 3;
        let popup = centered_rect(width, height, frame.area());

        frame.render_widget(Clear, popup);

        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Edit URL (Enter to confirm, Esc to cancel) ");

        let visible_width = popup.width.saturating_sub(3) as usize;
        let scroll = input.visual_scroll(visible_width);

        let input_widget = Paragraph::new(input.value())
            .style(Style::default().fg(Color::Yellow))
            .scroll((0, scroll as u16))
            .block(input_block);
        frame.render_widget(input_widget, popup);

        let cursor_x = input.visual_cursor().max(scroll) - scroll;
        frame.set_cursor_position((popup.x + 1 + cursor_x as u16, popup.y + 1));
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(area);
    Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(vertical[0])[0]
}

fn draw_browser_picker(frame: &mut Frame, app: &App, _area: ratatui::layout::Rect) {
    let list_height = app.browsers.len() as u16 + 2;
    let footer_height: u16 = 3;
    let total_height = list_height + footer_height;
    let width = 40;

    let popup = centered_rect(width, total_height, frame.area());

    frame.render_widget(Clear, popup);

    let picker_chunks = Layout::vertical([
        Constraint::Length(list_height),
        Constraint::Length(footer_height),
    ])
    .split(popup);

    let items: Vec<ListItem> = app
        .browsers
        .iter()
        .map(|b| {
            let label = if b.is_default {
                format!("  {} (default)", b.name)
            } else {
                format!("  {}", b.name)
            };
            ListItem::new(label)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Select Browser "),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ")
        .highlight_spacing(HighlightSpacing::Always);

    let mut state = ListState::default();
    state.select(Some(app.selected_browser));
    frame.render_stateful_widget(list, picker_chunks[0], &mut state);

    let footer_block = Block::default().borders(Borders::ALL);
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("  [↑↓]", Style::default().add_modifier(Modifier::DIM)),
        Span::styled(" Select   ", Style::default().add_modifier(Modifier::DIM)),
        Span::styled("[Enter]", Style::default().add_modifier(Modifier::DIM)),
        Span::styled(" Open   ", Style::default().add_modifier(Modifier::DIM)),
        Span::styled("[Esc]", Style::default().add_modifier(Modifier::DIM)),
        Span::styled(" Back", Style::default().add_modifier(Modifier::DIM)),
    ]))
    .block(footer_block);
    frame.render_widget(footer, picker_chunks[1]);
}

fn draw_footer(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let footer_block = Block::default().borders(Borders::ALL);

    let mut spans = vec![
        Span::styled("  [Enter]", Style::default().add_modifier(Modifier::DIM)),
        Span::styled(" Open URL   ", Style::default().add_modifier(Modifier::DIM)),
        Span::styled("[e]", Style::default().add_modifier(Modifier::DIM)),
        Span::styled(" Edit URL   ", Style::default().add_modifier(Modifier::DIM)),
    ];

    if !app.offers.is_empty() {
        if app.offers.len() == 1 {
            spans.push(Span::styled("[1]", Style::default().add_modifier(Modifier::DIM)));
        } else {
            spans.push(Span::styled(
                format!("[1-{}]", app.offers.len()),
                Style::default().add_modifier(Modifier::DIM),
            ));
        }
        spans.push(Span::styled(" Apply module   ", Style::default().add_modifier(Modifier::DIM)));
    }

    spans.push(Span::styled("[q]", Style::default().add_modifier(Modifier::DIM)));
    spans.push(Span::styled(" Quit", Style::default().add_modifier(Modifier::DIM)));

    let footer = Paragraph::new(Line::from(spans)).block(footer_block);
    frame.render_widget(footer, area);
}
