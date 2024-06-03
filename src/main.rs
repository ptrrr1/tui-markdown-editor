use std::{io, fs, path::Path};
use ratatui::{prelude::*, widgets::*};
use tui_textarea::{Input, Key, TextArea};
use crossterm::{
    event::{
        self, 
        DisableMouseCapture, 
        EnableMouseCapture, Event
    }, 
    execute, 
    terminal::{
        disable_raw_mode, 
        enable_raw_mode,
        EnterAlternateScreen, 
        LeaveAlternateScreen
    }
};


#[derive(Debug, Clone, PartialEq)]
enum Mode {
    Edit,
    View,
    Done,
}

#[derive(Debug)]
enum Message {
    Edit,
    View,
    Done
}

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

    fn update(&mut self, msg: Message) {
        match msg {
            Message::Edit => { self.mode = Mode::Edit; },

            Message::View => { self.mode = Mode::View; },

            Message::Done => { self.mode = Mode::Done; }
        }
    }

    fn view(&mut self, f: &mut Frame) {
        let filename = Path::new(&self.filepath)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_string();
        
        let file_name = Line::from(format!("[{}]", filename))
            .alignment(Alignment::Center);

        let cur_mode = match self.mode {
            Mode::Edit => "[EDIT]",

            Mode::View => "[VIEW]",

            _ => "",
        };

        let mode = Line::from(cur_mode).alignment(Alignment::Left);
        let pos = Line::from("[X:Y]").alignment(Alignment::Right);

        let line_number_style = Style::new().add_modifier(Modifier::DIM);
        let selection_style = Style::new().add_modifier(Modifier::REVERSED);

        let block = Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::top(1))
                    .title_top(file_name)
                    .title_bottom(mode)
                    .title_bottom(pos);

        self.textarea.set_block(block);
        self.textarea.set_line_number_style(line_number_style);
        self.textarea.set_selection_style(selection_style);

        f.render_widget(
            self.textarea.widget(),
            f.size()
        );
    }
}

fn input(input: std::io::Result<Event>, model: &mut Model) {
    let i = input.unwrap().clone();

    // 
    match model.mode {
        Mode::Edit => {
            match Input::from(i) {
                // Edit to View
                Input { key: Key::Esc, .. } => {
                    /*
                    if !model.filepath.is_empty() {
                        let content = model.textarea.lines().join("\n");
                        fs::write(&model.filepath, content)?;
                    }
                    */

                    model.update(Message::View);
                },

                // Read input in Edit Mode
                input => { 
                    model.textarea.input(input);
                }
            }
        },

        // 
        Mode::View => {
            match Input::from(i) {
                // View to Exit
                Input { key: Key::Esc, .. } => { model.update(Message::Done); },

                // View to Edit
                Input { key: Key::Char('i'), .. } |
                Input { key: Key::Enter, .. } => { 
                    model.update(Message::Edit); 
                },

                // Go to start of ile - Ctrl + k
                Input { key: Key::Char('k'), ctrl: true, .. } if model.mode == Mode::View => {
                    model.textarea.move_cursor(tui_textarea::CursorMove::Top)
                },

                // Go to end of file - Ctrl + j
                Input { key: Key::Char('j'), ctrl: true, .. } if model.mode == Mode::View => {
                    model.textarea.move_cursor(tui_textarea::CursorMove::Bottom)
                },

                // Go to next line
                Input { key: Key::Char('j'), .. }
                | Input { key: Key::Down, .. } if model.mode == Mode::View => {
                    model.textarea.move_cursor(tui_textarea::CursorMove::Down)
                },

                // Go to previous line
                Input { key: Key::Char('k'), .. }
                | Input { key: Key::Up, .. } if model.mode == Mode::View => {
                    model.textarea.move_cursor(tui_textarea::CursorMove::Up)
                },

                // Scroll Down
                // Allows scrolling past last line
                Input { key: Key::MouseScrollDown, .. } => {
                    model.textarea.scroll((1, 0))
                }

                // Scroll Up
                Input { key: Key::MouseScrollUp, .. } => {
                    model.textarea.scroll((-1, 0));
                }

                _ => {}
            }
        },

        _ => {}
    }
}

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    
    execute!(
        stdout, 
        EnterAlternateScreen, 
        EnableMouseCapture
        )?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize Model
    let path = std::env::args().nth(1);
    let mut model = Model::new(path);

    // Application Loop
    loop {
        // Render
        terminal.draw(|f| model.view(f))?;

        // Input Events
        input(event::read(), &mut model);

        // End app
        if model.mode == Mode::Done { break; }
    }
 
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}