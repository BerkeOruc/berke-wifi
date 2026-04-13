use crate::app::App;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

pub fn run_ui(app: &mut App) -> io::Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui_layout(f, app))?;

        if let Ok(event) = crossterm::event::read() {
            if app.input_mode {
                if let crossterm::event::Event::Key(crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Char(c),
                    ..
                }) = event
                {
                    app.password_input.push(c);
                } else if let crossterm::event::Event::Key(crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Enter,
                    ..
                }) = event
                {
                    let password = if app.password_input.is_empty() {
                        None
                    } else {
                        Some(app.password_input.clone())
                    };
                    if let Err(e) = app.connect(password.as_deref()) {
                        app.error = Some(e.to_string());
                    }
                    app.input_mode = false;
                    app.password_input.clear();
                } else if let crossterm::event::Event::Key(crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Esc,
                    ..
                }) = event
                {
                    app.input_mode = false;
                    app.password_input.clear();
                }
            } else {
                if let crossterm::event::Event::Key(key) = event {
                    match key.code {
                        crossterm::event::KeyCode::Char('q')
                        | crossterm::event::KeyCode::Char('Q') => break,
                        crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Down => {
                            app.move_selection_down()
                        }
                        crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Up => {
                            app.move_selection_up()
                        }
                        crossterm::event::KeyCode::Char('r') => {
                            if let Err(e) = app.refresh() {
                                app.error = Some(e.to_string());
                            }
                        }
                        crossterm::event::KeyCode::Char('c') => {
                            if let Some(network) = app.selected_network() {
                                if network.security.contains("WPA")
                                    || network.security.contains("WEP")
                                {
                                    app.input_mode = true;
                                    app.password_input.clear();
                                } else if let Err(e) = app.connect(None) {
                                    app.error = Some(e.to_string());
                                }
                            }
                        }
                        crossterm::event::KeyCode::Char('d') => {
                            if let Err(e) = app.disconnect() {
                                app.error = Some(e.to_string());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}

fn ui_layout(frame: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title = Paragraph::new("berke-wifi - WiFi Manager")
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Bearch Linux "),
        );
    frame.render_widget(title, layout[0]);

    let networks: Vec<ListItem> = app
        .networks
        .iter()
        .enumerate()
        .map(|(i, network)| {
            let signal_bars = network.signal_bars();
            let prefix = if network.connected { "*" } else { " " };
            let label = format!(
                "{} {} {:20} {:5} {:>6}%  {}",
                prefix,
                if i == app.selected_index { ">" } else { " " },
                network.ssid,
                signal_bars,
                network.signal,
                network.security
            );
            if i == app.selected_index {
                ListItem::new(label).style(Style::default().fg(Color::Black).bg(Color::White))
            } else {
                ListItem::new(label)
            }
        })
        .collect();

    let list = List::new(networks)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Available Networks "),
        )
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White));

    frame.render_widget(list, layout[1]);

    let status_text = if let Some(ref ssid) = app.connected_ssid {
        format!("Connected: {}", ssid)
    } else {
        "Not connected".to_string()
    };

    let help_text = if app.input_mode {
        "Enter password: "
    } else {
        "j/k: navigate  c: connect  d: disconnect  r: refresh  q: quit"
    };

    let footer = Paragraph::new(format!("{} | {}", status_text, help_text))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, layout[2]);

    if app.input_mode {
        let input_block = Paragraph::new(app.password_input.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(" Password "));
        frame.render_widget(input_block, layout[2]);
    }

    if let Some(ref error) = app.error {
        let error_block = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title(" Error "));
        frame.render_widget(error_block, layout[2]);
    }
}
