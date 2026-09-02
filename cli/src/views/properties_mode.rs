//! Properties Mode view module
//!
//! This module contains the properties mode input handling for the sidebar properties panel.

use crossterm::event::{KeyCode, KeyEvent};

use crate::App;
use crate::AppMode;
use crate::TextInputAction;
use crate::types::get_object_definitions_properties_for_field;

/// Handle input for properties mode
/// 
/// Properties mode allows editing field properties directly in the sidebar.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.mode = AppMode::Edit,
        KeyCode::Up => {
            if app.property_index > 0 {
                app.property_index -= 1;
            }
        }
        KeyCode::Down => {
            app.property_index += 1;
        }
        KeyCode::Char('+') | KeyCode::Right => {
            if let Some(idx) = app.editor.selected_field {
                match app.property_index {
                    0 => app.editor.map.fields[idx].pos.1 += 1, // Column
                    1 => app.editor.map.fields[idx].pos.0 += 1, // Row
                    2 => app.editor.map.fields[idx].length += 1, // Length
                    3 => { // Color
                        app.mode = AppMode::ColorPicker;
                        app.selected_color = app.editor.map.fields[idx].text_color.clone();
                        return;
                    }
                    4 => { // Attributes
                        app.mode = AppMode::AttributePicker;
                        return;
                    }
                    5 => { // INITIAL - open text input
                        let initial = app.editor.map.fields[idx].initial.clone().unwrap_or_default();
                        app.start_text_input("Enter INITIAL value:", &initial, TextInputAction::SetFieldInitial);
                        return;
                    }
                    6 => { // PIC - open text input
                        let pic = app.editor.map.fields[idx].pic.clone().unwrap_or_default();
                        app.start_text_input("Enter PIC value:", &pic, TextInputAction::SetFieldPic);
                        return;
                    }
                    7 => { // Name - open text input
                        let name = app.editor.map.fields[idx].name.clone();
                        app.start_text_input("Enter field name:", &name, TextInputAction::SetFieldName);
                        return;
                    }
                    _ => {}
                }
            }
        }
        KeyCode::Char('-') | KeyCode::Left => {
            if let Some(idx) = app.editor.selected_field {
                match app.property_index {
                    0 => { // Column
                        if app.editor.map.fields[idx].pos.1 > 1 {
                            app.editor.map.fields[idx].pos.1 -= 1;
                        }
                    }
                    1 => { // Row
                        if app.editor.map.fields[idx].pos.0 > 1 {
                            app.editor.map.fields[idx].pos.0 -= 1;
                        }
                    }
                    2 => { // Length
                        if app.editor.map.fields[idx].length > 1 {
                            app.editor.map.fields[idx].length -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        KeyCode::Enter => {
            if let Some(idx) = app.editor.selected_field {
                match app.property_index {
                    5 => { // INITIAL - open text input
                        let initial = app.editor.map.fields[idx].initial.clone().unwrap_or_default();
                        app.start_text_input("Enter INITIAL value:", &initial, TextInputAction::SetFieldInitial);
                        return;
                    }
                    6 => { // PIC - open text input
                        let pic = app.editor.map.fields[idx].pic.clone().unwrap_or_default();
                        app.start_text_input("Enter PIC value:", &pic, TextInputAction::SetFieldPic);
                        return;
                    }
                    7 => { // Name - open text input
                        let name = app.editor.map.fields[idx].name.clone();
                        app.start_text_input("Enter field name:", &name, TextInputAction::SetFieldName);
                        return;
                    }
                    _ => app.mode = AppMode::Edit,
                }
            } else {
                app.mode = AppMode::Edit;
            }
        }
        _ => {}
    }
}