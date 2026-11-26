use limner::{MarkdownStyle, render_markdown};
use ratatui::{
    DefaultTerminal, Frame, border,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Wrap,
    },
};
use ratatui_notifications::{Level, Notification, Notifications};

pub fn run(name: String, entry: crate::rss::Entry, current: usize, total_entries: usize) -> bool {
    let mut app = App::new(name, entry, current, total_entries);
    ratatui::run(|terminal| {
        app.run(terminal)
    })
}

struct App {
    running: bool,
    feed_name: String,
    entry_name: String,
    content: String,
    url: String,
    full: bool,
    current: usize,
    total_entries: usize,
    vertical_scroll: u16,
    total_lines: usize,
    notifications: Notifications,
}

impl App {
    fn new(name: String, entry: crate::rss::Entry, current: usize, total_entries: usize) -> Self {
        Self {
            running: true,
            feed_name: name,
            entry_name: entry.title,
            content: entry.content,
            url: entry.url,
            full: false,
            current,
            total_entries,
            vertical_scroll: 0,
            total_lines: 0,
            notifications: Notifications::new(),
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> bool {
        while self.running {
            terminal
                .draw(|frame| {
                    self.draw(frame);
                })
                .unwrap();
            if !self.keybinds() {
                return false;
            }
        }

        true
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame
            .area()
            .centered(Constraint::Percentage(85), Constraint::Percentage(75));
        let master_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(100), Constraint::Length(4)])
            .split(area);

        frame.render_widget(
            Paragraph::new("<l>").block(
                Block::default()
                    .borders(border!(TOP, RIGHT, BOTTOM))
                    .border_style(Style::default().fg(Color::Gray))
                    .border_type(BorderType::Double),
            ),
            master_layout[1],
        );

        let line = match self.full {
            true => Line::from(format!(
                " <j/k> scroll | <o> read later | <{}/{}> ",
                self.current, self.total_entries
            ))
            .right_aligned(),
            false => Line::from(format!(
                " <j> download full article | <o> read later | <{}/{}> ",
                self.current, self.total_entries
            ))
            .right_aligned(),
        };
        let rendered = render_markdown(&self.content, &MarkdownStyle::default(), area.width);
        self.total_lines = rendered.lines.len();
        frame.render_widget(
            Paragraph::new(rendered.lines)
                .block(
                    Block::default()
                        .title_top(Line::from(vec![
                            Span::from(" ".to_string() + &self.entry_name.clone() + " |"),
                            Span::from(" ".to_string() + &self.feed_name.clone() + " "),
                        ]))
                        .title_bottom(line)
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Magenta))
                        .border_type(BorderType::Double),
                )
                .wrap(Wrap { trim: false })
                .scroll((self.vertical_scroll, 0)),
            master_layout[0],
        );

        // scrollbar
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("󱦲"))
            .end_symbol(Some("󱦳"));
        let mut scrollbar_state =
            ScrollbarState::new(self.total_lines).position(self.vertical_scroll as usize);
        frame.render_stateful_widget(
            scrollbar,
            master_layout[0].inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );

        // notifs
        self.notifications.tick(std::time::Duration::from_millis(16));
        self.notifications.render(frame, frame.area());
    }

    fn keybinds(&mut self) -> bool {
        if let Event::Key(key) = event::read().unwrap()
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('l') => self.running = false,
                KeyCode::Char('q') => {
                    self.running = false;
                    return false;
                }
                KeyCode::Char('j') if !self.full => {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    self.content = rt.block_on(crate::rss::get(&self.url.clone()));
                    self.full = true;
                }
                KeyCode::Char('j') if self.full => {
                    self.vertical_scroll += 1;
                }
                KeyCode::Char('k') if self.full => {
                    self.vertical_scroll -= 1;
                }
                KeyCode::Char('o') => {
                    match crate::linkding::bookmark(
                        self.url.clone(),
                        self.entry_name.clone(),
                        match self.full {
                            true => self.entry_name.clone(),
                            false => self.content.clone(),
                        },
                    ) {
                        Ok(_) => {
                            let notif = Notification::new("Article saved to Linkding successfully")
                                .title(" Saved")
                                .level(Level::Info)
                                .build()
                                .unwrap();

                            self.notifications.add(notif).unwrap();

                            println!("saved article {} to linkding", self.entry_name.clone());
                        }
                        Err(e) => {
                            let notif = Notification::new(format!("Failed to save article due to error: {e}"))
                                .title(" Failed")
                                .level(Level::Error)
                                .build()
                                .unwrap();

                            self.notifications.add(notif).unwrap();

                            println!("failed to save article {} to linkding due to: {}", self.entry_name.clone(), e);
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    self.running = false;
                }
                _ => (),
            }
        }

        true
    }
}
