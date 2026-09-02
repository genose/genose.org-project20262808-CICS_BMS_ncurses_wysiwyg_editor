//! Color Picker view module
//!
//! This module contains the color selection dialog rendering and input handling.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::text::{Line, Text};

use cobol_bms_core::model::Color as BmsColor;
use crate::App;
use crate::AppMode;

/// Render the color picker
/// 
/// Displays a panel for selecting field colors.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let panel_width = 28;
    let panel_area = Rect {
        x: area.x + area.width - panel_width,
        y: area.y,
        width: panel_width,
        height: 13,
    };
    
    let block = Block::default()
        .title(" Colors [B/G/R/Y/C/M/W/K/O/P:Select|Space:None|Enter:Apply|Esc:Cancel] ")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let colors = vec![
        (BmsColor::Black, "Black", "K"),
        (BmsColor::Blue, "Blue", "B"),
        (BmsColor::Green, "Green", "G"),
        (BmsColor::Cyan, "Cyan", "C"),
        (BmsColor::Red, "Red", "R"),
        (BmsColor::Magenta, "Magenta", "M"),
        (BmsColor::Yellow, "Yellow", "Y"),
        (BmsColor::White, "White", "W"),
        (BmsColor::Orange, "Orange", "O"),
        (BmsColor::Pink, "Pink", "P"),
    ];
    
    let mut lines = vec![Line::from(" Select: ".yellow())];
    for (color, name, key) in &colors {
        let prefix = if Some(color) == app.selected_color.as_ref() { "> " } else { "  " };
        lines.push(Line::from(format!("{}{} [{}]", prefix, name, key)));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("Space: None".to_string()));
    lines.push(Line::from("Enter: Apply".to_string()));
    
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    });
}

/// Handle input for color picker mode
/// 
/// Processes keyboard input for selecting colors.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    use BmsColor::*;
    
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.selected_color = None;
        }
        KeyCode::Enter => {
            if let Some(color) = app.selected_color.clone() {
                if app.editor.selected_field.is_some() {
                    app.editor.set_selected_field_color(Some(color));
                }
            }
            app.mode = AppMode::Edit;
            app.selected_color = None;
        }
        KeyCode::Char('b') => app.selected_color = Some(Blue),
        KeyCode::Char('g') => app.selected_color = Some(Green),
        KeyCode::Char('r') => app.selected_color = Some(Red),
        KeyCode::Char('y') => app.selected_color = Some(Yellow),
        KeyCode::Char('w') => app.selected_color = Some(White),
        KeyCode::Char('c') => app.selected_color = Some(Cyan),
        KeyCode::Char('m') => app.selected_color = Some(Magenta),
        KeyCode::Char('k') => app.selected_color = Some(Black),
        KeyCode::Char('o') => app.selected_color = Some(Orange),
        KeyCode::Char('p') => app.selected_color = Some(Pink),
        KeyCode::Char(' ') => app.selected_color = None,
        _ => {}
    }
}
