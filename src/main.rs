use std::{io::{self, Write}, path::Path};
use ratatui::{prelude::*, widgets::*};
use tui_textarea::{Input, Key, TextArea};
use crossterm::{
    event::{
        self, 
        DisableMouseCapture, 
        EnableMouseCapture, 
        Event
    }, execute, terminal::{
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
    Save,
    View,
    Done
}

#[derive(Debug, Clone)]
struct File {
    path: Option<String>,
    name: String,
    textarea: TextArea<'static>,
}

impl File {
    fn new(path: Option<String>) -> File {
        // TODO:
        // check if there is a path
        // if yes, return title and path
        // else, open as normal but when saving, open file explorer
        
        // Check if there is a path
        match path {
            // If yes, open content and get file name
            Some(path) => {
                let mut textarea = TextArea::default();
                if let Ok(content) = std::fs::read_to_string(&path) {
                    textarea = TextArea::new(content.lines().map(String::from).collect());
                }

                let name = Path::new(&path)
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .unwrap_or("")
                    .to_string();

                File {
                    path: Some(path),
                    name: name,
                    textarea: textarea,
                }
            },

            // Else, create a default instance and set a generic name            
            None => {
                File {
                    path: None,
                    name: "new_file".to_string(),
                    textarea: TextArea::default(),
                }
            },
        }
    }

    fn save(&self) -> io::Result<()> {
        match &self.path {
            Some(path) => {
                let path = std::path::Path::new(&path);
                let file = std::fs::File::create(&path)?;
                let mut buf_writer = std::io::BufWriter::new(file);

                for line in self.textarea.lines() {
                    buf_writer.write_all(line.as_bytes())?;
                    buf_writer.write_all(b"\n")?; // Adding a newline after each line
                }

                buf_writer.flush()?;
                Ok(())
            },
            None => {
                // TODO: Popup window with destination
                Ok(())
            },
        }
    }

    #[allow(dead_code)]
    fn wrap_text(&mut self) {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
struct Model {
    // Essential
    mode: Mode,
    file: File,

    // Feedback
    is_focused: bool,
    info: Option<String>
}

impl Model {
    fn new(path: Option<String>) -> Model {
        let file = File::new(path);

        Model {
            mode: Mode::View,
            file: file,
            is_focused: true,
            info: None
        }
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::Edit => { 
                self.mode = Mode::Edit; 
                self.info = None; 
            },

            Message::Save => { 
                self.mode = Mode::View;
                let _ = self.file.save(); 
                self.info = Some("File has been saved!".to_string()); 
            },

            Message::View => { 
                self.mode = Mode::View;
            },

            Message::Done => { 
                self.mode = Mode::Done; 
            },
        }
    }

    // TODO: Rewrite in a more readable manner
    fn view(&mut self, f: &mut Frame) {
        //
        let info_space = match &self.info {
            Some(_) => 1,
            None => 0,
        };

        let layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Fill(1),
                                Constraint::Length(info_space)
                            ]
                        );
    
        let chunks = layout.split(f.size());        

        // 
        let file_name = Line::from(format!("[{}]", self.file.name))
            .alignment(Alignment::Center);

        let cur_mode = match self.mode {
            Mode::Edit => "[EDIT]",

            Mode::View => "[VIEW]",

            _ => "",
        };

        let mode = Line::from(cur_mode).alignment(Alignment::Left);

        let (y, x) = self.file.textarea.cursor();
        let pos = Line::from(format!("[{}:{}]", y + 1, x)).alignment(Alignment::Right);

        let line_number_style = Style::new().add_modifier(Modifier::DIM);
        let selection_style = Style::new().add_modifier(Modifier::REVERSED);
        
        let focused_style = if self.is_focused { 
            Style::default() 
        } else { 
            Style::new().add_modifier(Modifier::DIM) 
        };

        let cursor_style = if self.mode == Mode::Edit { 
            Style::new()
                .add_modifier(Modifier::SLOW_BLINK)
                .add_modifier(Modifier::REVERSED) 
        } else {
            Style::new().add_modifier(Modifier::REVERSED) 
        };

        let block = Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(focused_style)
                    .padding(Padding::top(1))
                    .title_top(file_name)
                    .title_bottom(mode)
                    .title_bottom(pos);

        self.file.textarea.set_block(block);
        
        self.file.textarea.set_line_number_style(line_number_style);
        self.file.textarea.set_selection_style(selection_style);
        self.file.textarea.set_style(focused_style);
        self.file.textarea.set_cursor_style(cursor_style);

        f.render_widget(
            self.file.textarea.widget(),
            chunks[0]
        );

        //
        if info_space == 1 {
            let info_block = Block::new()
                                .borders(Borders::NONE)
                                .title(format!("{}", self.info.clone().unwrap()))
                                .title_alignment(Alignment::Center);

            f.render_widget(
                info_block,
                chunks[1]
            );
        }
    }
}

fn input(input: std::io::Result<Event>, model: &mut Model) {
    let i = input.unwrap().clone();

    match i {
        Event::FocusGained => { model.is_focused = true },

        Event::FocusLost => { model.is_focused = false },

        _ => {}
    }

    // 
    match model.mode {
        Mode::Edit => {
            match Input::from(i) {
                // Edit to View
                Input { key: Key::Esc, .. } => {
                    model.update(Message::View);
                },

                // Save
                Input { key: Key::Char('s'), ctrl: true, .. } => {
                    model.update(Message::Save);
                },

                // Undo
                Input { key: Key::Char('z'), ctrl: true, .. } => {
                    model.file.textarea.undo();
                }

                // Redo
                Input { key: Key::Char('y'), ctrl: true, .. } => {
                    model.file.textarea.redo();
                }

                // Copy
                Input { key: Key::Char('c'), ctrl: true, .. } => {
                    model.file.textarea.copy();
                }

                // Paste
                Input { key: Key::Char('v'), ctrl: true, .. } => {
                    // TODO:
                    // I'm having trouble with this 
                    // It does not copy/cut to clipboard and only pastes from clipboard
                    unimplemented!()
                }

                // Cut
                Input { key: Key::Char('x'), ctrl: true, .. } => {
                    model.file.textarea.cut();
                }

                // Read input in Edit Mode
                input => {
                    model.file.textarea.input(input);
                }
            }
        },

        // 
        Mode::View => {
            match Input::from(i) {
                // View to Exit
                Input { key: Key::Esc, .. } |
                Input { key: Key::Char('q'), .. } => { 
                    model.update(Message::Done); 
                },

                // View to Edit
                Input { key: Key::Char('i'), .. } | 
                Input { key: Key::Enter, .. } => { 
                    model.update(Message::Edit); 
                },

                // Save
                Input { key: Key::Char('s'), ctrl: true, .. } => {
                    model.update(Message::Save);
                },

                // Go to start of ile - Shift + k
                Input { key: Key::Char('K'), .. } => {
                    model.file.textarea.move_cursor(tui_textarea::CursorMove::Top)
                },

                // Go to end of file -  Shift + j
                Input { key: Key::Char('J'), .. }  => {
                    model.file.textarea.move_cursor(tui_textarea::CursorMove::Bottom)
                },

                // Move Cursor Down
                Input { key: Key::Char('j'), .. }
                | Input { key: Key::Down, .. }  => {
                    model.file.textarea.move_cursor(tui_textarea::CursorMove::Down)
                },

                // Move Cursor Up
                Input { key: Key::Char('k'), .. } |
                Input { key: Key::Up, .. }  => {
                    model.file.textarea.move_cursor(tui_textarea::CursorMove::Up)
                },

                // Move Cursor Left
                Input { key: Key::Char('h'), .. }
                | Input { key: Key::Left, .. }  => {
                    model.file.textarea.move_cursor(tui_textarea::CursorMove::Back)
                },

                // Move Cursor Right
                Input { key: Key::Char('l'), .. } |
                Input { key: Key::Right, .. }  => {
                    model.file.textarea.move_cursor(tui_textarea::CursorMove::Forward)
                },

                // Scroll Down
                // Allows scrolling past last line
                Input { key: Key::MouseScrollDown, .. } => {
                    model.file.textarea.scroll((1, 0))
                }

                // Scroll Up
                Input { key: Key::MouseScrollUp, .. } => {
                    model.file.textarea.scroll((-1, 0));
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