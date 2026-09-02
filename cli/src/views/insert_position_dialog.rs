//! Insert Position Dialog view module
//!
//! This module contains the insert position dialog rendering and input handling.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Style, Color as TuiColor};
use ratatui::text::Line;

use crate::types::InsertableObject;
use crate::App;
use crate::AppMode;
use crate::ActivePanel;

/// Render the insert position dialog
/// 
/// Displays a dialog for selecting the position where a new object will be inserted.
/// Shows live preview of the object at the selected position.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let panel_width = 30;
    let panel_area = Rect {
        x: area.x + area.width - panel_width,
        y: area.y,
        width: panel_width,
        height: 10,
    };
    
    let block = Block::default()
        .title(" Insert Position [Arrows:Move|Enter:Confirm|Esc:Cancel|Live Preview]")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let inner = Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    };
    
    let obj_name = app.pending_object.map(|o| o.display()).unwrap_or("Object");
    let (row, col) = app.pending_position;
    
    // Check if position is valid
    let is_valid = if let Some(obj) = app.pending_object {
        app.editor.map.is_valid_field_position(app.pending_position, obj.default_length())
    } else {
        false
    };
    
    let validity_text = if is_valid {
        Line::from("Status: Valid".green())
    } else {
        Line::from("Status: INVALID - will not be inserted".red())
    };
    
    let lines = vec![
        Line::from(format!("Object: {}", obj_name)),
        Line::from(""),
        Line::from(format!("Position: ({}, {})", row, col)),
        Line::from(""),
        validity_text,
        Line::from(""),
        Line::from("Arrows: Move position".dim()),
        Line::from("Enter: Confirm insertion".dim()),
        Line::from("Esc: Cancel".dim()),
    ];
    
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, inner);
}

/// Handle input for insert position dialog mode
/// 
/// Processes keyboard input for selecting insertion position.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    use crate::types::InsertableObject as TypesInsertableObject;
    
    // Handle Enter for confirmation (terminal doesn't support Shift+Enter detection)
    if key.code == KeyCode::Enter {
        let obj = if let Some(obj) = app.pending_object.take() {
            Some(obj)
        } else if app.active_panel == ActivePanel::Sidebar {
            // Fallback: try to get object from sidebar selection
            app.sidebar_objects_selected.and_then(|idx| {
                TypesInsertableObject::all().get(idx).cloned()
            })
        } else {
            None
        };
        
        if let Some(obj) = obj {
            // Check if position is valid before inserting
            let field_length = obj.default_length();
            if !app.is_valid_field_position(app.pending_position, field_length) {
                app.set_message(&format!("Cannot insert: Invalid position ({},{}) for {}", 
                    app.pending_position.0, app.pending_position.1, obj.display()));
                // Keep pending_object for retry
                app.pending_object = Some(obj);
                return;
            }
            
            let field = obj.create_field(app.pending_position);
            app.editor.map.fields.push(field);
            app.mode = AppMode::Edit;
            app.pending_object = None;
            app.sidebar_objects_selected = None;
            app.active_panel = ActivePanel::Canvas;
            app.set_message(&format!("Inserted {}", obj.display()));
        } else {
            app.set_message("No object selected!");
        }
        return;
    }
    
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.pending_object = None;
            app.sidebar_objects_selected = None;
        }
        KeyCode::Up => {
            if app.pending_position.0 > 1 {
                app.pending_position.0 -= 1;
            }
        }
        KeyCode::Down => {
            app.pending_position.0 += 1;
        }
        KeyCode::Left => {
            if app.pending_position.1 > 1 {
                app.pending_position.1 -= 1;
            }
        }
        KeyCode::Right => {
            app.pending_position.1 += 1;
        }
        _ => {}
    }
}