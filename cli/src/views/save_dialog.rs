//! Save Dialog view module
//!
//! This module contains the save dialog rendering and input handling.

use std::fs;
use std::path::PathBuf;

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Style, Color as TuiColor};

use crate::App;
use crate::AppMode;

/// Render the save dialog
/// 
/// Displays a dialog for saving the current map to a file.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let dialog_width = 40;
    let dialog_height = 5;
    let dialog_area = Rect {
        x: area.x + (area.width.saturating_sub(dialog_width)) / 2,
        y: area.y + (area.height.saturating_sub(dialog_height)) / 2,
        width: dialog_width,
        height: dialog_height,
    };
    
    let block = Block::default()
        .title(" Save File ")
        .borders(Borders::ALL);
    f.render_widget(block, dialog_area);
    
    let inner = Rect {
        x: dialog_area.x + 1,
        y: dialog_area.y + 1,
        width: dialog_area.width.saturating_sub(2),
        height: dialog_area.height.saturating_sub(2),
    };
    
    let prompt = Paragraph::new("File path: ")
        .style(Style::default().fg(TuiColor::Yellow));
    f.render_widget(prompt, Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 });
    
    let path_text = Paragraph::new(app.save_path.as_str())
        .style(Style::default().fg(TuiColor::White));
    f.render_widget(path_text, Rect { x: inner.x, y: inner.y + 1, width: inner.width, height: 1 });
    
    let help = Paragraph::new("Enter: Save | Esc: Cancel")
        .style(Style::default().fg(TuiColor::Cyan));
    f.render_widget(help, Rect { x: inner.x, y: inner.y + 2, width: inner.width, height: 1 });
}

/// Handle input for save dialog mode
/// 
/// Processes keyboard input for the save dialog.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.mode = AppMode::Edit,
        KeyCode::Enter => {
            // Validate before saving
            let errors = app.editor.map.validate();
            if !errors.is_empty() {
                app.set_message(&format!("Cannot save: {}", errors.join("; ")));
                return;
            }
            
            // Prevent saving empty BMS maps (no fields)
            if app.editor.map.fields.is_empty() {
                app.set_message("Cannot save: Empty BMS map has no fields");
                return;
            }
            let path = PathBuf::from(&app.save_path);
            match fs::write(&path, app.editor.export_to_bms()) {
                Ok(_) => {
                    app.current_file = Some(path.clone());
                    app.mode = AppMode::Edit;
                    app.set_message(&format!("Saved: {}", path.display()));
                }
                Err(e) => {
                    app.set_message(&format!("Failed: {}", e));
                }
            }
        }
        KeyCode::Backspace => {
            app.save_path.pop();
        }
        KeyCode::Char(c) => {
            app.save_path.push(c);
        }
        _ => {}
    }
}