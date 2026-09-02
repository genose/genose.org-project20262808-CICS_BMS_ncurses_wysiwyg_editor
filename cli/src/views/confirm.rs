//! Confirm dialog view module
//!
//! This module contains the confirmation dialog rendering and input handling.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Style, Color as TuiColor};

use crate::App;
use crate::AppMode;
use crate::ConfirmAction;

/// Render the confirmation dialog
/// 
/// Displays a confirmation prompt with Yes/No options.
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
        .title(" Confirm ")
        .borders(Borders::ALL);
    f.render_widget(block, dialog_area);
    
    let inner = Rect {
        x: dialog_area.x + 1,
        y: dialog_area.y + 1,
        width: dialog_area.width.saturating_sub(2),
        height: dialog_area.height.saturating_sub(2),
    };
    
    let message = match app.confirm_action {
        ConfirmAction::QuitWithoutSave => "Quit without saving?",
        ConfirmAction::DeleteField => "Delete selected field?",
        ConfirmAction::ClearMap => "Clear all fields?",
    };
    
    let prompt = Paragraph::new(message)
        .style(Style::default().fg(TuiColor::Yellow));
    f.render_widget(prompt, Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 });
    
    let help = Paragraph::new("Y/Enter: Yes | N/Esc: No")
        .style(Style::default().fg(TuiColor::Cyan));
    f.render_widget(help, Rect { x: inner.x, y: inner.y + 1, width: inner.width, height: 1 });
}

/// Handle input for confirm mode
/// 
/// Processes keyboard input for the confirmation dialog.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Enter => {
            match app.confirm_action {
                ConfirmAction::QuitWithoutSave => app.exit = true,
                ConfirmAction::DeleteField => {
                    if app.editor.remove_selected_field().is_some() {
                        app.set_message("Field deleted");
                        app.show_validation_status();
                    }
                    app.mode = AppMode::Edit;
                }
                ConfirmAction::ClearMap => {
                    app.editor.map.fields.clear();
                    app.editor.selected_field = None;
                    app.set_message("Map cleared");
                    app.mode = AppMode::Edit;
                }
            }
        }
        KeyCode::Char('n') | KeyCode::Esc => app.mode = AppMode::Edit,
        _ => {}
    }
}
