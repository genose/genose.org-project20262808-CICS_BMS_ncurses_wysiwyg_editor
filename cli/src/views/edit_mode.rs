//! Edit mode module
//!
//! This module contains the handle_mode function for the main editing functionality.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use cobol_bms_core::{BmsEditor, EditorMode, CursorDirection, ResizeDirection, create_default_map};

use crate::types::InsertableObject;
use crate::{App, AppMode, ActivePanel, SidebarAction, SidebarSection, ConfirmAction};

/// Handle input for edit mode
/// 
/// This is the main input handler for the edit mode, handling all keyboard input
/// for editing BMS maps including navigation, field selection, and field manipulation.
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
    
    // Handle Shift+Arrow for multi-selection (range selection)
    if key.modifiers.contains(KeyModifiers::SHIFT) && !key.modifiers.contains(KeyModifiers::CONTROL) && !key.modifiers.contains(KeyModifiers::ALT) {
        if app.active_panel == ActivePanel::Canvas {
            // Remember the current anchor point for range selection
            let anchor_idx = app.editor.selected_field;
            
            match key.code {
                KeyCode::Up => {
                    app.editor.move_cursor(CursorDirection::Up, 1);
                }
                KeyCode::Down => {
                    app.editor.move_cursor(CursorDirection::Down, 1);
                }
                KeyCode::Left => {
                    app.editor.move_cursor(CursorDirection::Left, 1);
                }
                KeyCode::Right => {
                    app.editor.move_cursor(CursorDirection::Right, 1);
                }
                _ => {}
            }
            
            // Extend selection to the field at the new cursor position
            if let Some(new_idx) = app.editor.field_at(app.editor.cursor_pos) {
                if let Some(anchor_idx) = anchor_idx {
                    // Ensure selected_fields is initialized with anchor
                    if app.editor.selected_fields.is_empty() {
                        app.editor.selected_fields = vec![anchor_idx];
                    }
                    app.editor.extend_selection_to(new_idx);
                    app.set_message(&format!("Selected {} field(s)", app.editor.selected_count()));
                } else {
                    // No anchor, just select the field at new position
                    app.editor.select_field_at(app.editor.cursor_pos);
                }
            }
            return;
        }
    }
    
    // Handle special actions (Shift+Enter when supported)
    if key.modifiers.contains(KeyModifiers::SHIFT) && key.code == KeyCode::Enter {
        app.set_message("validation combokey shift enter");
        if app.active_panel == ActivePanel::Sidebar && app.sidebar_section == SidebarSection::Objects {
            // Direct insert in Objects sidebar (Shift+Enter when supported)
            if let Some(selected_idx) = app.sidebar_objects_selected {
                let objects = InsertableObject::all();
                if selected_idx < objects.len() {
                    let obj = objects[selected_idx];
                    let field = obj.create_field(app.editor.cursor_pos);
                    app.editor.map.fields.push(field);
                    app.set_message(&format!("Inserted {}", obj.display()));
                }
            }
            // Also handle pending object (from Enter then confirmation)
            if let Some(obj) = app.pending_object.take() {
                if let Some(pos_idx) = app.sidebar_objects_selected {
                    let objects = InsertableObject::all();
                    if pos_idx < objects.len() {
                        let field = obj.create_field(app.pending_position);
                        app.editor.map.fields.push(field);
                        app.set_message(&format!("Inserted {}", obj.display()));
                    }
                }
            }
        } else if app.active_panel == ActivePanel::Canvas {
            // Open EditProperties on selected field in Canvas
            if let Some(idx) = app.editor.selected_field {
                let field = app.editor.map.fields[idx].clone();
                app.edit_properties_field = Some(field);
                app.edit_properties_index = 0;
                app.edit_properties_scroll_offset = 0;
                app.mode = AppMode::EditProperties;
                app.set_message("Edit properties - Enter to save");
            }
        }
        return;
    }
    
    match key.code {
        // Navigation
        KeyCode::Char('j') | KeyCode::Down => {
            if app.active_panel == ActivePanel::Canvas {
                app.editor.move_cursor(CursorDirection::Down, 1);
                app.editor.select_field_at(app.editor.cursor_pos);
            } else {
                // Sidebar navigation
                match app.sidebar_section {
                    SidebarSection::Actions => {
                        let actions = SidebarAction::all();
                        if let Some(current) = app.sidebar_actions_selected {
                            let next = (current + 1).min(actions.len().saturating_sub(1));
                            app.sidebar_actions_selected = Some(next);
                        } else if !actions.is_empty() {
                            app.sidebar_actions_selected = Some(0);
                        }
                    }
                    SidebarSection::Objects => {
                        let objects = InsertableObject::all();
                        if let Some(current) = app.sidebar_objects_selected {
                            let next = (current + 1).min(objects.len().saturating_sub(1));
                            app.sidebar_objects_selected = Some(next);
                        } else if !objects.is_empty() {
                            app.sidebar_objects_selected = Some(0);
                        }
                    }
                }
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if app.active_panel == ActivePanel::Canvas {
                app.editor.move_cursor(CursorDirection::Up, 1);
                app.editor.select_field_at(app.editor.cursor_pos);
            } else {
                // Sidebar navigation
                match app.sidebar_section {
                    SidebarSection::Actions => {
                        if let Some(current) = app.sidebar_actions_selected {
                            let prev = current.saturating_sub(1);
                            app.sidebar_actions_selected = if prev > 0 || current == 0 { Some(prev) } else { None };
                        } else {
                            let actions = SidebarAction::all();
                            if !actions.is_empty() {
                                app.sidebar_actions_selected = Some(actions.len() - 1);
                            }
                        }
                    }
                    SidebarSection::Objects => {
                        if let Some(current) = app.sidebar_objects_selected {
                            let prev = current.saturating_sub(1);
                            app.sidebar_objects_selected = if prev > 0 || current == 0 { Some(prev) } else { None };
                        } else {
                            let objects = InsertableObject::all();
                            if !objects.is_empty() {
                                app.sidebar_objects_selected = Some(objects.len() - 1);
                            }
                        }
                    }
                }
            }
        }
        KeyCode::Char('h') | KeyCode::Left => {
            if app.active_panel == ActivePanel::Canvas {
                app.editor.move_cursor(CursorDirection::Left, 1);
                app.editor.select_field_at(app.editor.cursor_pos);
            }
        }
        KeyCode::Char('l') | KeyCode::Right => {
            if app.active_panel == ActivePanel::Canvas {
                app.editor.move_cursor(CursorDirection::Right, 1);
                app.editor.select_field_at(app.editor.cursor_pos);
            }
        }
        
        // Field navigation with Tab/Shift+Tab
        KeyCode::Tab => {
            if app.active_panel == ActivePanel::Canvas {
                app.editor.select_next_field();
                if let Some(idx) = app.editor.selected_field {
                    let field = &app.editor.map.fields[idx];
                    app.editor.cursor_pos = field.pos;
                }
            } else {
                // Toggle between Actions and Objects sections in Sidebar
                app.sidebar_section = app.sidebar_section.next();
                app.sidebar_actions_selected = None;
                app.sidebar_objects_selected = None;
                app.set_message(match app.sidebar_section {
                    SidebarSection::Actions => "Actions section",
                    SidebarSection::Objects => "Objects section",
                });
            }
        }
        KeyCode::BackTab => {
            if app.active_panel == ActivePanel::Canvas {
                app.editor.select_prev_field();
                if let Some(idx) = app.editor.selected_field {
                    let field = &app.editor.map.fields[idx];
                    app.editor.cursor_pos = field.pos;
                }
            }
        }
        
        // Execute selected sidebar action or insert object
        KeyCode::Enter => {
            if app.active_panel == ActivePanel::Sidebar {
                match app.sidebar_section {
                    SidebarSection::Actions => {
                        if let Some(selected_idx) = app.sidebar_actions_selected {
                            let actions = SidebarAction::all();
                            if selected_idx < actions.len() {
                                match actions[selected_idx] {
                                    SidebarAction::Edit => {
                                        if app.editor.selected_field.is_some() {
                                            app.mode = AppMode::Properties;
                                            app.property_index = 0;
                                        }
                                    }
                                    SidebarAction::Delete => {
                                        if app.editor.selected_field.is_some() {
                                            app.mode = AppMode::Confirm;
                                            app.confirm_action = ConfirmAction::DeleteField;
                                        }
                                    }
                                    SidebarAction::Move => {
                                        if let Some(idx) = app.editor.selected_field {
                                            app.editor.drag_start = Some(app.editor.map.fields[idx].pos);
                                            app.editor.mode = EditorMode::MoveField;
                                            app.set_message("Move field - arrows to move, Enter to drop");
                                        }
                                    }
                                    SidebarAction::Resize => {
                                        if let Some(idx) = app.editor.selected_field {
                                            app.editor.drag_start = Some((app.editor.map.fields[idx].pos.0, app.editor.map.fields[idx].pos.1 + app.editor.map.fields[idx].length - 1));
                                            app.editor.mode = EditorMode::ResizeField { direction: ResizeDirection::Right };
                                            app.set_message("Resize field - Left/Right to resize");
                                        }
                                    }
                                    SidebarAction::AddField => {
                                        app.editor.add_field_at_cursor(10);
                                        app.set_message("Added field");
                                        app.show_validation_status();
                                    }
                                    SidebarAction::MapType => {
                                        app.mode = AppMode::MapTypePicker;
                                        app.set_message("Select map type");
                                    }
                                    SidebarAction::PreviewBms => {
                                        app.show_bms_text = !app.show_bms_text;
                                        app.set_message(if app.show_bms_text {
                                            "BMS text preview ON"
                                        } else {
                                            "BMS text preview OFF"
                                        });
                                    }
                                }
                            }
                        }
                    }
                    SidebarSection::Objects => {
                        if let Some(selected_idx) = app.sidebar_objects_selected {
                            let objects = InsertableObject::all();
                            if selected_idx < objects.len() {
                                let obj = objects[selected_idx];
                                app.pending_object = Some(obj);
                                app.pending_position = app.editor.cursor_pos;
                                app.mode = AppMode::InsertPosition;
                                app.set_message("Set position with arrows, Enter to confirm");
                            }
                        }
                    }
                }
            }
        }
        
        // Single-letter shortcuts (kept for workflow compatibility)
        // Navigation and special keys
        KeyCode::Char('?') => app.mode = AppMode::Help,
        KeyCode::Char(' ') => app.mode = AppMode::Normal,
        
        // New map commands
        KeyCode::Char('n') => {
            app.editor.new_map("NEWMAP", "DEFAULT", (24, 80));
            app.current_file = None;
            app.set_message("New map created");
        }
        _ => {}
    }
}
