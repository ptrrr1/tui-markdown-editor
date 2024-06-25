use ratatui::prelude::*;
use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture
    }, execute, terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};

pub mod file;
use file::File;

pub mod ui;

pub mod input;

#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Edit,
    Save,
    View,
    Done
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
	Edit, // ing...
	Save, // ing...
	View, // ing...
	Exit, // ing...
}

#[allow(dead_code)]
pub struct Model {
	mode: Mode, // Default: View
	file: File,
	is_focused: bool // Feedback
}

impl Model {
    pub fn new(file :File) -> Model {
    	Model {
    		mode: Mode::View,
    		file: file,
    		is_focused: true,
    	}
    }

    // TODO:
    pub fn init(mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();

        execute!(
            stdout,
            EnterAlternateScreen,
            EnableMouseCapture
        )?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // App loop
        loop {
            // Render
            terminal.draw(|f| ui::render_frame(&mut self, f))?;

            // Input Events
            input::read_input(event::read(), &mut self);

            // End app
            if self.mode == Mode::Exit { break; }
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

    // TODO: Rework into multiple functions
    // Ex.: Model::to_edit()
    #[allow(dead_code)]
    fn update(&mut self, msg: Message) {
        match msg {
            Message::Edit => {
                self.mode = Mode::Edit;
                //self.info = None;
            },

            Message::Save => {
                self.mode = Mode::View;
                let _ = self.file.save();
                //self.info = Some("File has been saved!".to_string());
            },

            Message::View => {
                self.mode = Mode::View;
            },

            Message::Done => {
                self.mode = Mode::Exit;
            },
        }
    }
}