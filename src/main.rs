#![allow(dead_code, unused_imports)] // DEV

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap},
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
mod bins;
use bins::{bin_list, get_code, get_tags};
mod code;
use code::{Code, Tag};
mod tags;
use tags::*;
mod utils;
use utils::copy_payload;

#[macro_use]
extern crate lazy_static; // work in progress

struct BinList<T> {
    state: ListState,
    bins: Vec<T>,
}

struct TabState<'a> {
    titles: Vec<&'a str>,
    index: usize,
}

struct CodeVec<'a> {
    payloads: Vec<Code<'a>>,
    index: usize,
}

impl<'a> CodeVec<'a> {
    fn new(payloads: Vec<Code<'a>>) -> CodeVec<'a> {
        CodeVec { payloads, index: 0 }
    }
    fn set(&mut self, payloads: Vec<Code<'a>>) {
        self.payloads = payloads;
    }
    fn next(&mut self) {
        self.index = (self.index + 1) % self.payloads.len();
    }
    fn reset(&mut self) {
        self.index = 0;
    }
}

impl<'a> TabState<'a> {
    fn new(titles: Vec<&'a str>) -> TabState {
        TabState { titles, index: 0 }
    }
    fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }
    fn reset(&mut self) {
        self.index = 0;
    }
}

impl<T> BinList<T> {
    fn with_items(bins: Vec<T>) -> BinList<T> {
        BinList {
            state: ListState::default(),
            bins,
        }
    }
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.bins.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.bins.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct App<'a> {
    bins: BinList<&'a str>,
    tabs: TabState<'a>,
    payloads: CodeVec<'a>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            bins: BinList::with_items(bin_list()),
            tabs: TabState::new(vec![]),
            payloads: CodeVec::new(vec![]),
        }
    }
    fn on_up(&mut self) {
        self.bins.previous();
        self.set_tabs();
        self.tabs.reset();
        self.set_payloads();
        self.payloads.reset();
    }
    fn on_down(&mut self) {
        self.bins.next();
        self.set_tabs();
        self.tabs.reset();
        self.set_payloads();
        self.payloads.reset();
    }
    fn on_tab(&mut self) {
        if let Some(_) = self.bins.state.selected() {
            self.tabs.next();
            self.set_payloads();
            self.payloads.reset();
        }
    }
    fn get_bin_name(&mut self) -> &str {
        self.bins
            .bins
            .get(self.bins.state.selected().unwrap())
            .unwrap()
    }
    fn set_tabs(&mut self) {
        self.tabs.titles = get_tags(self.get_bin_name());
    }
    fn set_payloads(&mut self) {
        let tab_idx = self.tabs.index;
        let curr_tab = self.tabs.titles.get(tab_idx).unwrap();
        let payloads = get_code(
            self.bins
                .bins
                .get(self.bins.state.selected().unwrap())
                .unwrap(),
        );
        let iter = payloads.into_iter();
        let match_tag = |t| match t {
            Tag::SH => "SHELL",
            Tag::CMD => "CMD",
            Tag::RS => "RS",
            Tag::NIRS => "NIRS",
            Tag::BS => "BS",
            Tag::NIBS => "NIBS",
            Tag::FU => "FU",
            Tag::FD => "FD",
            Tag::FW => "FW",
            Tag::FR => "FR",
            Tag::LL => "LL",
            Tag::SUID => "SUID",
            Tag::SUDO => "SUDO",
            Tag::CB => "CB",
            Tag::LSUID => "LSUID",
        };
        let filtered: Vec<Code> = iter.filter(|x| match_tag(x.tag) == *curr_tab).collect();
        self.payloads.set(filtered);
    }
    fn on_enter(&mut self) {
        if let Some(_) = self.bins.state.selected() {
            self.payloads.next()
        }
    }
    fn on_copy(&mut self) {
        if let Some(_) = self.bins.state.selected() {
            let idx = self.payloads.index;
            let selected_payload = self.payloads.payloads.get(idx).unwrap().code;
            copy_payload(selected_payload);
        }
    }
}

fn _create_code_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Green))
        .title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::DIM),
        ))
}

fn _create_selected_code_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow))
        .title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
}

fn format_code<'a>(c: Code<'a>, selected: bool) -> Paragraph<'a> {
    let style = Style::default().fg(Color::White);
    let code = Paragraph::new(c.code.trim())
        .wrap(Wrap { trim: false }) // ADDED WRAP TO CODE SNIPPETS
        .style(style)
        .block(match selected {
            true => _create_selected_code_block(c.title),
            false => _create_code_block(c.title),
        });
    code
}

fn constraint_len(payload: &str) -> Constraint {
    Constraint::Length(payload.lines().count() as u16)
}

fn render_bin(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(4)
        .vertical_margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);
    let selected = app.bins.state.selected().unwrap();
    let title = app.bins.bins.get(selected).unwrap().to_string();
    let bin = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Green))
        .title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ));
    let tab_titles = app
        .tabs
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();
    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .select(app.tabs.index)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::DIM),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );

    f.render_widget(bin, area);
    f.render_widget(tabs, chunks[0]);
    let render: Vec<Paragraph> = app
        .payloads
        .payloads
        .iter()
        .enumerate()
        .map(|(i, v)| {
            if i == app.payloads.index {
                let p = format_code(*v, true);
                return p;
            } else {
                let p = format_code(*v, false);
                return p;
            }
        })
        .collect();
    let cstrnts = match render.len() {
        1 => Layout::default()
            .direction(Direction::Vertical)
            // .horizontal_margin(4)
            // .vertical_margin(1)
            .constraints(
                [
                    constraint_len(app.payloads.payloads.get(0).unwrap().code),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(chunks[1]),
        2 => Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    constraint_len(app.payloads.payloads.get(0).unwrap().code),
                    constraint_len(app.payloads.payloads.get(1).unwrap().code),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(chunks[1]),
        3 => Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    constraint_len(app.payloads.payloads.get(0).unwrap().code),
                    constraint_len(app.payloads.payloads.get(1).unwrap().code),
                    constraint_len(app.payloads.payloads.get(2).unwrap().code),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(chunks[1]),
        4 => Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    constraint_len(app.payloads.payloads.get(0).unwrap().code),
                    constraint_len(app.payloads.payloads.get(1).unwrap().code),
                    constraint_len(app.payloads.payloads.get(2).unwrap().code),
                    constraint_len(app.payloads.payloads.get(3).unwrap().code),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(chunks[1]),
        5 => Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    constraint_len(app.payloads.payloads.get(0).unwrap().code),
                    constraint_len(app.payloads.payloads.get(1).unwrap().code),
                    constraint_len(app.payloads.payloads.get(2).unwrap().code),
                    constraint_len(app.payloads.payloads.get(3).unwrap().code),
                    constraint_len(app.payloads.payloads.get(4).unwrap().code),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(chunks[1]),
        _ => unreachable!(),
    };
    let mut i = 0;
    for y in render {
        f.render_widget(y, cstrnts[i]);
        i += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let _last_tick = Instant::now();
    loop {
        terminal.draw(|f| client(f, &mut app))?;
        let timeout = tick_rate;
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => app.bins.unselect(),
                    KeyCode::Down => app.on_down(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Tab => app.on_tab(),
                    KeyCode::Enter => app.on_enter(),
                    KeyCode::Char('c') => app.on_copy(),
                    _ => {}
                }
            }
        }
    }
}

fn client(f: &mut Frame, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(18), Constraint::Percentage(82)].as_ref())
        .split(f.size());
    let bins: Vec<ListItem> = app
        .bins
        .bins
        .iter()
        .map(|i| {
            let lines = vec![Line::from(*i)];
            ListItem::new(lines).style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::DIM),
            )
        })
        .collect();
    let bins = List::new(bins)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .border_type(BorderType::Rounded)
                .title("bins"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" ♦ ");
    f.render_stateful_widget(bins, main_layout[0], &mut app.bins.state);
    match app.bins.state.selected() {
        Some(_) => render_bin(f, app, main_layout[1]),
        None => render_root(f, main_layout[1]),
    }
}

fn render_root(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(4),
                Constraint::Percentage(60),
                Constraint::Length(3),
                Constraint::Length(7),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(area);
    let status = r#"This program is currently a prototype. Expect upcoming releases with new features. If you encounter a bug, file an issue on github."#;
    let software_update = r#"cargo install --force fubar-cli"#;
    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().fg(Color::White))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };
    let shell = format!("{}:", SHELL.name);
    let cmd = format!("{}:", COMMAND.name);
    let rs = format!("{}:", REVERSE_SHELL.name);
    let nirs = format!("{}:", NONINTERACTIVE_REVERSE_SHELL.name);
    let bs = format!("{}:", BIND_SHELL.name);
    let nibs = format!("{}:", NONINTERACTIVE_BIND_SHELL.name);
    let fu = format!("{}:", FILE_UPLOAD.name);
    let fd = format!("{}:", FILE_DOWNLOAD.name);
    let fw = format!("{}:", FILE_WRITE.name);
    let fr = format!("{}:", FILE_READ.name);
    let ll = format!("{}:", LIBRARY_LOAD.name);
    let suid = format!("{}:", SUID.name);
    let sudo = format!("{}:", SUDO.name);
    let cb = format!("{}:", CAPABILITIES.name);
    let lsuid = format!("{}:", LIMITED_SUID.name);
    let style = Style::default().fg(Color::Green);
    let root_status = Paragraph::new(Text::styled(status, style))
        .wrap(Wrap { trim: false })
        .block(create_block("root"));
    let update = Paragraph::new(software_update)
        .style(style)
        .block(create_block("update"));
    let name_style = Style::default()
        .fg(Color::Green)
        .bg(Color::Black)
        .add_modifier(Modifier::REVERSED)
        .add_modifier(Modifier::BOLD);
    let descriptions = [
        (shell, SHELL.desc),
        (cmd, COMMAND.desc),
        (rs, REVERSE_SHELL.desc),
        (nirs, NONINTERACTIVE_REVERSE_SHELL.desc),
        (bs, BIND_SHELL.desc),
        (nibs, NONINTERACTIVE_BIND_SHELL.desc),
        (fu, FILE_UPLOAD.desc),
        (fd, FILE_DOWNLOAD.desc),
        (fw, FILE_WRITE.desc),
        (fr, FILE_READ.desc),
        (ll, LIBRARY_LOAD.desc),
        (suid, SUID.desc),
        (sudo, SUDO.desc),
        (cb, CAPABILITIES.desc),
        (lsuid, LIMITED_SUID.desc),
    ];
    let mut desc_vec = vec![];
    for x in descriptions {
        let d = Line::from(vec![
            Span::styled(x.0, name_style),
            Span::styled(format!("    {}", x.1), style),
        ]);
        desc_vec.push(d);
        // desc_vec.push(Spans::from(vec![Span::raw("")]));
    }
    let root = Paragraph::new(desc_vec)
        .alignment(ratatui::layout::Alignment::Left)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    let nav = r#"
        ↑ ↓     navigate bins                    c    copy payload to clipboard
        ⇛       navigate tabs                    r    return to root page
        ↵       navigate payloads                q    quit application
    "#;
    let navigation = Paragraph::new(nav)
        .style(style)
        .block(create_block("navigation"));
    let cr = r#"
        © 2023 irishmaestro
        released under GPL-3.0 license
        https://github.com/irishmaestro/fubar
    "#;
    let copyright = Paragraph::new(cr)
        .style(style)
        .block(create_block("copyright"));
    let anchor = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[3]);
    f.render_widget(root_status, chunks[0]);
    f.render_widget(root, chunks[1]);
    f.render_widget(update, chunks[2]);
    f.render_widget(navigation, anchor[0]);
    f.render_widget(copyright, anchor[1]);
}
