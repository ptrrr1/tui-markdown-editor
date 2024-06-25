use super::{Model, Mode};

use ratatui::{prelude::*, widgets::*};
use ratatui::terminal::Frame;

pub fn render_frame(model: &mut Model, f: &mut Frame) {
	// 
	/*
    let info_space = match &model.info {
        Some(_) => 1,
        None => 0,
    };
	*/
    // Define layout parts
    let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Fill(1),
                            //Constraint::Length(info_space)
                        ]
                    );

    let chunks = layout.split(f.size());        

    // 
    let file_name = Line::from(format!("[{}]", model.file.name))
        .alignment(Alignment::Center);

    let cur_mode = match model.mode {
        Mode::Edit => "[EDIT]",

        Mode::View => "[VIEW]",

        _ => "",
    };

    let mode = Line::from(cur_mode).alignment(Alignment::Left);

    let (y, x) = model.file.textarea.cursor();
    let pos = Line::from(format!("[{}:{}]", y + 1, x)).alignment(Alignment::Right);

    let line_number_style = Style::new().add_modifier(Modifier::DIM);
    let selection_style = Style::new().add_modifier(Modifier::REVERSED);
    
    let focused_style = if model.is_focused { 
        Style::default() 
    } else { 
        Style::new().add_modifier(Modifier::DIM) 
    };

    let cursor_style = if model.mode == Mode::Edit { 
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

    model.file.textarea.set_block(block);
    
    model.file.textarea.set_line_number_style(line_number_style);
    model.file.textarea.set_selection_style(selection_style);
    model.file.textarea.set_style(focused_style);
    model.file.textarea.set_cursor_style(cursor_style);

    f.render_widget(
        model.file.textarea.widget(),
        chunks[0]
    );

    //
    /*
    if info_space == 1 {
        let info_block = Block::new()
                            .borders(Borders::NONE)
                            .title(format!("{}", model.info.clone().unwrap()))
                            .title_alignment(Alignment::Center);

        f.render_widget(
            info_block,
            chunks[1]
        );
    }
    */
}