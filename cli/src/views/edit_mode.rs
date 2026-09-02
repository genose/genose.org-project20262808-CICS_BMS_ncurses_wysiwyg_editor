//! Edit Mode view module
//!
//! This module contains the edit mode input handling for the main editor.

use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::event::KeyEvent;

use crate::App;
use crate::ActivePanel;
use crate::AppMode;
use crate::ComboAction;
use crate::CursorDirection;
use crate::TextInputAction;

/// Handle input for edit mode
/// 
/// Edit mode is the main editing mode for the BMS map editor.
/// It handles navigation, field manipulation, and various editing operations.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    // F9 no longer used - replaced by Ctrl+Alt+P in handle_input
    
    // Handle Alt+Arrow and Ctrl+Arrow keys for navigation
    // Note: Alt+Left/Right and Ctrl+Left/Right are captured by VSCode, but work in native terminals
    if key.modifiers.contains(KeyModifiers::ALT) || key.modifiers.contains(KeyModifiers::CONTROL) {
        let is_alt = key.modifiers.contains(KeyModifiers::ALT);
        let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        
        // Only handle if exactly one modifier is pressed (Alt OR Ctrl, not both)
        if is_alt != is_ctrl {
            match key.code {
                KeyCode::Up => {
                    if app.active_panel == ActivePanel::Canvas {
                        app.editor.move_cursor(CursorDirection::Up, 5);
                        app.editor.select_field_at(app.editor.cursor_pos);
                        return;
                    }
                }
                KeyCode::Down => {
                    if app.active_panel == ActivePanel::Canvas {
                        app.editor.move_cursor(CursorDirection::Down, 5);
                        app.editor.select_field_at(app.editor.cursor_pos);
                        return;
                    }
                }
                KeyCode::Left => {
                    if app.active_panel == ActivePanel::Canvas {
                        app.editor.select_prev_field();
                        if let Some(idx) = app.editor.selected_field {
                            let field = &app.editor.map.fields[idx];
                            app.editor.cursor_pos = field.pos;
                        }
                        return;
                    }
                }
                KeyCode::Right => {
                    if app.active_panel == ActivePanel::Canvas {
                        app.editor.select_next_field();
                        if let Some(idx) = app.editor.selected_field {
                            let field = &app.editor.map.fields[idx];
                            app.editor.cursor_pos = field.pos;
                        }
                        return;
                    }
                }
                _ => {}
            }
        }
    }
