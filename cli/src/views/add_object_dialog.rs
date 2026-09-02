//! Add Object Dialog view module
//!
//! This module contains the add object dialog rendering and input handling.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Style, Color as TuiColor};
use ratatui::text::{Line, Span};

use crate::types::InsertableObject;
use crate::App;
use crate::AppMode;

/// Render the add object dialog
/// 
/// Displays a panel for selecting the type of object to add to the map.
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
        height: area.height.min(15),
    };
    
    let block = Block::default()
        .title(" Add Object [Up/Down:Nav|Enter:Select|Esc:Cancel] ")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let inner = Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    };
    
    let objects = InsertableObject::all();
    let mut lines = vec![Line::from(" Select Object Type ".yellow())];
    
    for (_i, obj) in objects.iter().enumerate() {
        let display_text = obj.display();
        let is_selected = app.selected_object_for_add == Some(*obj);
        let prefix = if is_selected { "> " } else { "  " };
        let style = if is_selected {
            Style::default().fg(TuiColor::Black).bg(TuiColor::Yellow)
        } else {
            Style::default().fg(TuiColor::White)
        };
        lines.push(Line::from(Span::styled(format!("{} {}", prefix, display_text), style)));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from("Enter: Select".dim()));
    lines.push(Line::from("Esc: Cancel".dim()));
    
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, inner);
}

/// Handle input for add object dialog mode
/// 
/// Processes keyboard input for selecting object types.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    use crate::types::InsertableObject as TypesInsertableObject;
    
    let objects = InsertableObject::all();
    
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.selected_object_for_add = None;
        }
        KeyCode::Enter => {
            if let Some(obj) = app.selected_object_for_add {
                if obj == TypesInsertableObject::AsciiArt || obj == TypesInsertableObject::Image {
                    // For AsciiArt and Image, go directly to image import
                    let field = obj.create_field(app.editor.cursor_pos);
                    app.edit_properties_field = Some(field);
                    
                    // Initialize image import with current directory
                    let current_dir = std::env::current_dir()
                        .ok()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|| ".".to_string());
                    
                    app.mode = AppMode::ImageImport;
                    app.image_import_path.clear();
                    app.image_import_directory = current_dir;
                    app.image_import_files = crate::scan_directory_files(&app.image_import_directory, true); // Show image files by default
                    app.image_import_selected_index = 0;
                    // Ensure index is valid if no files found
                    if app.image_import_files.is_empty() {
                        app.image_import_selected_index = 0;
                    } else {
                        app.image_import_selected_index = app.image_import_selected_index.min(app.image_import_files.len() - 1);
                    }
                    app.image_import_error = None;
                    app.image_import_show_all_files = false;
                    app.selected_object_for_add = None;
                    app.set_message(&format!("Import image for {} - Use arrows to select, Tab to show all files", obj.display()));
                } else {
                    // Create a field from the selected object
                    let mut field = obj.create_field(app.editor.cursor_pos);
                    
                    // Instead of inserting immediately, go to EditProperties mode
                    // to allow configuring the field properties
                    app.edit_properties_field = Some(field);
                    app.edit_properties_index = 0;
                    app.mode = AppMode::EditProperties;
                    app.selected_object_for_add = None;
                    app.set_message(&format!("Configure {}", obj.display()));
                }
            }
        }
        KeyCode::Up => {
            let objects = InsertableObject::all();
            let current_idx = app.selected_object_for_add
                .and_then(|obj| objects.iter().position(|&o| o == obj))
                .unwrap_or(0);
            
            if current_idx > 0 {
                let new_obj = objects[current_idx - 1];
                app.selected_object_for_add = Some(new_obj);
            } else {
                // Wrap around to bottom
                let last_obj = objects.last().copied();
                app.selected_object_for_add = last_obj;
            }
        }
        KeyCode::Down => {
            let objects = InsertableObject::all();
            let current_idx = app.selected_object_for_add
                .and_then(|obj| objects.iter().position(|&o| o == obj))
                .unwrap_or(objects.len() - 1);
            
            if current_idx + 1 < objects.len() {
                let new_obj = objects[current_idx + 1];
                app.selected_object_for_add = Some(new_obj);
            } else {
                // Wrap around to top
                let first_obj = objects.first().copied();
                app.selected_object_for_add = first_obj;
            }
        }
        _ => {}
    }
}