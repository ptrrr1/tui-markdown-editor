use super::{Model, Mode, Message};

use crossterm::event::Event;
use tui_textarea::{Input, Key};

pub fn read_input(input: std::io::Result<Event>, model: &mut Model) {
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
/*
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
*/
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