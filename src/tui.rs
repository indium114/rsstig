use limner::{render_markdown, MarkdownStyle};
use ratatui::{
    border,
  DefaultTerminal, Frame,
  crossterm::event::{self, Event, KeyCode, KeyEventKind},
  layout::{Constraint, Direction, Layout},
  style::{Color, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

pub fn run(name: String, entry: crate::rss::Entry, current: usize, total_entries: usize) -> bool {
    let mut app = App::new(name, entry, current, total_entries);
    ratatui::run(|terminal| if !app.run(terminal) {
        return false;
    } else {
        return true;
    })
}

struct App {
    running: bool,
    feed_name: String,
    entry_name: String,
    content: String,
    full: bool,
    current: usize,
    total_entries: usize,
    vertical_scroll: u16,
    total_lines: usize
}

impl App {
    fn new(name: String, entry: crate::rss::Entry, current: usize, total_entries: usize) -> Self {
        Self {
            running: true,
            feed_name: name,
            entry_name: entry.title,
            content: entry.content,
            full: false,
            current,
            total_entries,
            vertical_scroll: 0,
            total_lines: 0,
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

        return true;
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame
            .area()
            .centered(Constraint::Percentage(80), Constraint::Percentage(60));
        let master_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(100), Constraint::Length(4)])
            .split(area);

        frame.render_widget(
            Paragraph::new("<l>")
                .block(
                    Block::default()
                        .borders(border!(TOP, RIGHT, BOTTOM))
                        .border_style(Style::default().fg(Color::Gray))
                        .border_type(BorderType::Double)
                ),
            master_layout[1]
        );

        let rendered = render_markdown(&self.content, &MarkdownStyle::default(), area.width);
        self.total_lines = rendered.lines.len();
        frame.render_widget(
            Paragraph::new(rendered.lines)
                .block(
                    Block::default()
                        .title_top(
                            Line::from(vec![
                                Span::from(" ".to_string() + &self.entry_name.clone() + " |"),
                                Span::from(" ".to_string() + &self.feed_name.clone() + " "),
                            ])
                        )
                        .title_bottom(
                            Line::from(format!(" <{}/{}> ", self.current, self.total_entries)).right_aligned()
                        )
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Magenta))
                        .border_type(BorderType::Double)
                )
                .wrap(Wrap { trim: false })
                .scroll((self.vertical_scroll, 0)),
            master_layout[0]
        )
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
                _ => (),
            }
        }

        true
    }
}
