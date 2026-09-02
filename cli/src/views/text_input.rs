//! Text Input view module
//!
//! This module contains the text input dialog rendering and input handling.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Style, Color as TuiColor};

use crate::App;
use crate::AppMode;

/// Render the text input dialog
/// 
/// Displays a dialog for entering text values (field properties, etc.).
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let dialog_width = 50;
    let dialog_height = 5;
    let dialog_area = Rect {
        x: area.x + (area.width.saturating_sub(dialog_width)) / 2,
        y: area.y + (area.height.saturating_sub(dialog_height)) / 2,
        width: dialog_width,
        height: dialog_height,
    };
    
    let block = Block::default()
        .title(" Text Input ")
        .borders(Borders::ALL);
    f.render_widget(block, dialog_area);
    
    let inner = Rect {
        x: dialog_area.x + 1,
        y: dialog_area.y + 1,
        width: dialog_area.width.saturating_sub(2),
        height: dialog_area.height.saturating_sub(2),
    };
    
    let prompt = Paragraph::new(app.text_input_prompt.as_str())
        .style(Style::default().fg(TuiColor::Yellow));
    f.render_widget(prompt, Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 });
    
    let value_text = Paragraph::new(app.text_input_value.as_str())
        .style(Style::default().fg(TuiColor::White));
    f.render_widget(value_text, Rect { x: inner.x, y: inner.y + 1, width: inner.width, height: 1 });
    
    let help = Paragraph::new("Enter: OK | Esc: Cancel | Backspace: Delete")
        .style(Style::default().fg(TuiColor::Cyan));
    f.render_widget(help, Rect { x: inner.x, y: inner.y + 2, width: inner.width, height: 1 });
}

/// Handle input for text input mode
/// 
/// Processes keyboard input for the text input dialog.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.text_input_prompt.clear();
            app.text_input_value.clear();
            app.text_input_action = None;
        }
        KeyCode::Enter => {
            let value = std::mem::take(&mut app.text_input_value);
            app.apply_text_input(value);
            app.mode = AppMode::Edit;
            app.text_input_prompt.clear();
        }
        KeyCode::Backspace => {
            app.text_input_value.pop();
        }
        KeyCode::Char(c) => {
            app.text_input_value.push(c);
        }
        _ => {}
    }
}
