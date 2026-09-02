//! Edit Properties Mode view module
//!
//! This module contains the edit properties mode input handling for the properties panel.

use crossterm::event::{KeyCode, KeyEvent};

use crate::App;
use crate::AppMode;

/// Handle input for edit properties mode
/// 
/// Edit properties mode allows editing all properties of the currently selected field.
/// This is the detailed property editing mode accessible from various dialogs.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    // Handle Enter for saving
    if key.code == KeyCode::Enter {
        if let Some(field) = app.edit_properties_field.take() {
            if let Some(idx) = app.editor.selected_field {
                // Update existing field
                app.editor.map.fields[idx] = field;
                app.mode = AppMode::Edit;
                app.set_message("Properties saved");
            } else {
                // Add new field (came from AddObjectDialog)
                app.editor.add_field(field);
                // Select the newly added field
                if let Some(new_idx) = app.editor.map.fields.len().checked_sub(1) {
                    app.editor.select_field(new_idx);
                }
                app.mode = AppMode::Edit;
                app.set_message("Field inserted");
                app.show_validation_status();
            }
        }
        return;
    }
    
    if let Some(field) = app.edit_properties_field.as_mut() {
        // Get the list of properties for this field
        let properties = crate::get_properties_for_field(field);
        
        match key.code {
            KeyCode::Esc => {
                app.mode = AppMode::Edit;
                app.edit_properties_field = None;
            }
            KeyCode::Up => {
                if app.edit_properties_index > 0 {
                    app.edit_properties_index -= 1;
                    // Scroll up if index is at or above scroll offset
                    if app.edit_properties_index < app.edit_properties_scroll_offset {
                        app.edit_properties_scroll_offset = app.edit_properties_index;
                    }
                }
            }
            KeyCode::Down => {
                if app.edit_properties_index + 1 < properties.len() {
                    app.edit_properties_index += 1;
                    // Scroll down if index is near bottom
                    if app.edit_properties_index >= app.edit_properties_scroll_offset + 18 {
                        app.edit_properties_scroll_offset = app.edit_properties_index.saturating_sub(17);
                    }
                }
            }
            KeyCode::PageUp => {
                if app.edit_properties_scroll_offset > 0 {
                    let step = 10.min(app.edit_properties_scroll_offset);
                    app.edit_properties_scroll_offset = app.edit_properties_scroll_offset.saturating_sub(step);
                    // Keep index visible
                    if app.edit_properties_index < app.edit_properties_scroll_offset {
                        app.edit_properties_index = app.edit_properties_scroll_offset;
                    } else if app.edit_properties_index > app.edit_properties_scroll_offset + 14 {
                        app.edit_properties_index = (app.edit_properties_scroll_offset + 14).min(properties.len().saturating_sub(1));
                    }
                }
            }
            KeyCode::PageDown => {
                let max_scroll = properties.len().saturating_sub(15);
                if app.edit_properties_scroll_offset < max_scroll {
                    let step = 10.min(max_scroll - app.edit_properties_scroll_offset);
                    app.edit_properties_scroll_offset = (app.edit_properties_scroll_offset + step).min(max_scroll);
                    // Keep index visible
                    if app.edit_properties_index < app.edit_properties_scroll_offset {
                        app.edit_properties_index = app.edit_properties_scroll_offset;
                    } else if app.edit_properties_index < app.edit_properties_scroll_offset + 14 && app.edit_properties_index + 10 < properties.len() {
                        app.edit_properties_index = (app.edit_properties_scroll_offset + 14).min(properties.len().saturating_sub(1));
                    }
                }
            }
            KeyCode::Char('+') | KeyCode::Right => {
                if app.edit_properties_index < properties.len() {
                    if let Some(prop) = properties.get(app.edit_properties_index) {
                        prop.modify_value(field, true);
                    }
                }
            }
            KeyCode::Char('-') | KeyCode::Left => {
                if app.edit_properties_index < properties.len() {
                    if let Some(prop) = properties.get(app.edit_properties_index) {
                        prop.modify_value(field, false);
                    }
                }
            }
            KeyCode::Enter => {
                // Regular Enter exits without saving
                app.mode = AppMode::Edit;
                app.edit_properties_field = None;
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                // Trigger image import for ASCII art fields
                if field.ascii_art.is_some() || field.name == "ASCII_ART" {
                    app.mode = AppMode::ImageImport;
                    app.image_import_path.clear();
                    app.image_import_error = None;
                }
            }
            _ => {}
        }
    }
}