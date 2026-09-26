use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};
use std::{io, process::Command, time::Duration};

struct App {
    tabs: Vec<String>,
    selected_tab: usize,
    address: String,
    input_buffer: String,
    editing_address: bool,
    status: String,
}

impl App {
    fn new() -> Self {
        Self {
            tabs: vec!["Home".to_string(), "Search".to_string(), "Images".to_string()],
            selected_tab: 0,
            address: "https://example.com".to_string(),
            input_buffer: "https://example.com".to_string(),
            editing_address: false,
            status: "Kitty terminal mode ready".to_string(),
        }
    }

    fn start_search(&mut self) {
        self.editing_address = true;
        self.input_buffer = self.address.clone();
        self.status = "Editing address... press Enter to search".to_string();
    }

    fn resolve_target(&self, query: &str) -> String {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return String::new();
        }

        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            return trimmed.to_string();
        }

        if trimmed.contains('.') && !trimmed.contains(' ') {
            return format!("https://{trimmed}");
        }

        format!("https://duckduckgo.com/?q={}", urlencoding::encode(trimmed))
    }

    fn submit_search(&mut self) {
        let query = self.input_buffer.trim();
        if query.is_empty() {
            self.status = "Address is empty".to_string();
            self.editing_address = false;
            self.input_buffer.clear();
            return;
        }

        let target = self.resolve_target(query);
        self.address = target.clone();
        self.status = format!("Opening in Firefox: {target}");
        self.editing_address = false;
        self.input_buffer.clear();

        if let Err(err) = open_firefox(&target) {
            self.status = format!("Firefox launch failed: {err}");
        }
    }

    fn cancel_search(&mut self) {
        self.editing_address = false;
        self.input_buffer = self.address.clone();
        self.status = "Address edit cancelled".to_string();
    }

    fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % self.tabs.len();
    }

    fn prev_tab(&mut self) {
        if self.selected_tab == 0 {
            self.selected_tab = self.tabs.len() - 1;
        } else {
            self.selected_tab -= 1;
        }
    }

    fn add_tab(&mut self) {
        let next = self.tabs.len() + 1;
        self.tabs.push(format!("Tab {next}"));
        self.selected_tab = self.tabs.len() - 1;
    }
}

fn render_ui(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    let tab_titles: Vec<&str> = app.tabs.iter().map(String::as_str).collect();
    let selected = app.selected_tab;
    let tabs = Tabs::new(tab_titles)
        .select(selected)
        .block(Block::default().title("Astra Browser").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(tabs, chunks[0]);

    let address_value = if app.editing_address {
        app.input_buffer.clone()
    } else {
        app.address.clone()
    };

    let address = Paragraph::new(address_value)
        .block(Block::default().title("Address").borders(Borders::ALL))
        .style(Style::default().fg(Color::LightBlue));
    frame.render_widget(address, chunks[1]);

    let inner = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(chunks[2]);

    let body = Paragraph::new(
        "Astra TUI\n\nThis terminal browser shell is designed for Kitty terminals.\nUse the terminal image protocol to display richer media directly inside the app.\n\n• q to quit\n• → / ← to switch tabs\n• n to add a tab",
    )
    .block(Block::default().title("Content").borders(Borders::ALL))
    .alignment(Alignment::Left)
    .style(Style::default().fg(Color::LightGreen));
    frame.render_widget(body, inner[0]);

    let list = List::new(vec![
        ListItem::new("Home / Search results"),
        ListItem::new("Bookmarks"),
        ListItem::new("Downloads"),
        ListItem::new("Kitty image protocol: ready"),
        ListItem::new("Status: terminal-native browsing shell"),
    ])
    .block(Block::default().title("Sidebar").borders(Borders::ALL))
    .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow));
    frame.render_widget(list, inner[1]);

    let status = Paragraph::new(Line::from(vec![
        Span::raw("Status: "),
        Span::styled(&app.status, Style::default().fg(Color::LightMagenta)),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Status"));
    frame.render_widget(status, chunks[3]);
}

fn open_firefox(url: &str) -> Result<()> {
    let firefox = ["firefox", "firefox-esr"].iter().find(|cmd| {
        Command::new(*cmd)
            .arg("--version")
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false)
    });

    match firefox {
        Some(browser) => {
            Command::new(*browser)
                .arg(url)
                .spawn()?;
            Ok(())
        }
        None => {
            Command::new("xdg-open").arg(url).spawn()?;
            Ok(())
        }
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| render_ui(frame, app))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }

                    if app.editing_address {
                        match key.code {
                            KeyCode::Esc => app.cancel_search(),
                            KeyCode::Enter => app.submit_search(),
                            KeyCode::Backspace => {
                                app.input_buffer.pop();
                            }
                            KeyCode::Char(ch) => app.input_buffer.push(ch),
                            _ => {}
                        }
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Right => app.next_tab(),
                        KeyCode::Left => app.prev_tab(),
                        KeyCode::Char('n') => app.add_tab(),
                        KeyCode::Char('/') => app.start_search(),
                        KeyCode::Char('s') => app.start_search(),
                        KeyCode::Char('r') => app.status = "Reloading page".to_string(),
                        KeyCode::Char('h') => app.status = "Home view".to_string(),
                        KeyCode::Enter => app.start_search(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, Show)?;

    result
}

// Kitty image protocol note:
// A terminal running Kitty can display inline image payloads with sequences like:
// \x1b_Ga=T,f=100,s=320,v=200;BASE64PNGDATA\x1b\\
// This is the path to richer media inside the TUI without a desktop window.
