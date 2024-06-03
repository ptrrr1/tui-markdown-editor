//pub mod io;


use std::{io, fs, path::Path};
use ratatui::{prelude::*, widgets::*};
use tui_textarea::{Input, Key, TextArea};
use crossterm::{
    event::{
        self, 
        DisableMouseCapture, 
        EnableMouseCapture
    }, 
    execute, 
    terminal::{
        disable_raw_mode, 
        enable_raw_mode, 
        DisableLineWrap, 
        EnableLineWrap, 
        EnterAlternateScreen, 
        LeaveAlternateScreen
    }
};

#[derive(Debug, Clone)]
struct Model {
    mode: Mode,
    filepath: String,
    textarea: TextArea<'static>,
}

impl Model {
    fn new(path: Option<String>) -> Model {
        let mut text_area = TextArea::default();
        let mut filepath = String::new();
        
        // Opens the file content
        match path {
            Some(path) => {
                filepath = path.clone();
                if let Ok(content) = std::fs::read_to_string(&path) {
                    text_area = TextArea::new(content.lines().map(String::from).collect());
                }
            },
            None => {},
        }

        Model {
            mode: Mode::View,
            filepath: filepath,
            textarea: text_area,
        }
    }

    fn mode(self, mode: Mode) -> Model {
        Model { 
            mode: mode,
            filepath: self.filepath, 
            textarea: self.textarea }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Mode {
    Insert,
    View,
    Done,
}

#[derive(Debug)]
enum Message {
    Insert,
    View,
    Done,
    None
}

fn update(model: &Model, msg: Message) -> Model {
    let m = model.clone();

    match msg {
        Message::Insert => { m.mode(Mode::Insert) },

        Message::View => { m.mode(Mode::View) },

        Message::Done => { m.mode(Mode::Done) },

        Message::None => { m }
    }
}

fn view(f: &mut Frame, model: &Model) {
    let filename = Path::new(&model.filepath)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();
    
    let file_name = Line::from(format!("[{}]", filename))
        .alignment(Alignment::Center);

    let cur_mode = match model.mode {
        Mode::Insert => "[INSERT]",

        Mode::View => "[VIEW]",

        _ => "",
    };

	let mode = Line::from(cur_mode)
        .alignment(Alignment::Left);
    
    let pos = Line::from("[X:Y]")
        .alignment(Alignment::Right);

    let line_number_style = Style::new()
                .fg(Color::DarkGray);

    let selection_style = Style::new()
                .fg(Color::Black)
                .bg(Color::Gray);

    let block = Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::top(1))
                .title_top(file_name)
                .title_bottom(mode)
                .title_bottom(pos);

    let mut text_area = model.clone().textarea;
    text_area.set_block(block);
    text_area.set_line_number_style(line_number_style);
    text_area.set_selection_style(selection_style);

    f.render_widget(
        text_area.widget(),
        f.size()
    );
}

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, EnableLineWrap)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize Model
    let path = std::env::args().nth(1);
    let mut model = Model::new(path);

    // Application Loop
    loop {
        // Render
        terminal.draw(|f| view(f, &model))?;

        // Events
        match event::read()?.into() {
            // Navigates through Modes
            Input { key: Key::Esc, .. } => {
                let msg = match model.mode {
                    Mode::Insert => {
                        if !model.filepath.is_empty() {
                            let content = model.textarea.lines().join("\n");
                            fs::write(&model.filepath, content)?;
                        }
                        
                        Message::View
                    },

                    Mode::View => Message::Done,

                    _ => Message::None
                };

                // Updates
                model = update(&model, msg);

                // End app
                if model.mode == Mode::Done { break; }
            },

            // Saving
            Input { key: Key::Char('s'), ctrl: true, .. } if model.mode == Mode::View => { 
                
            }

            // Start Insert Mode
            Input { key: Key::Char('i'), .. } |
            Input { key: Key::Enter, .. } if model.mode == Mode::View => { 
                model = update(&model, Message::Insert) 
            },

            // Go to end of file
            Input { key: Key::Char('j'), ctrl: true, .. } if model.mode == Mode::View => {
                model.textarea.move_cursor(tui_textarea::CursorMove::Bottom)
            },

            // Go to next line
            Input { key: Key::Char('j'), .. }
            | Input { key: Key::Down, .. } if model.mode == Mode::View => {
                model.textarea.move_cursor(tui_textarea::CursorMove::Down)
            },

            // Go to start of ile
            Input { key: Key::Char('k'), ctrl: true, .. } if model.mode == Mode::View => {
                model.textarea.move_cursor(tui_textarea::CursorMove::Top)
            },

            // Go to previous line
            Input { key: Key::Char('k'), .. }
            | Input { key: Key::Up, .. } if model.mode == Mode::View => {
                model.textarea.move_cursor(tui_textarea::CursorMove::Up)
            },

            // Scroll Down
            Input { key: Key::MouseScrollDown, .. } => {
                model.textarea.scroll((1, 0))
            }

            // Scroll Up
            Input { key: Key::MouseScrollUp, .. } => {
                model.textarea.scroll((-1, 0));
            }

            // Read input in Insert Mode
            input if model.mode == Mode::Insert => { 
                model.textarea.input(input);
            },

            _ => {}
        }
    }
 
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        DisableLineWrap
    )?;
    terminal.show_cursor()?;

    Ok(())
}