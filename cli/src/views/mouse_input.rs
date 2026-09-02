//! Mouse Input view module
//!
//! This module contains the mouse input handling for the canvas.

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::App;
use crate::ActivePanel;
use crate::AppMode;

/// Handle mouse input for field selection and drag selection
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `mouse_event` - The mouse event to handle
pub fn handle_mode(app: &mut App, mouse_event: MouseEvent) {
    // Only handle mouse events in Edit mode when Canvas is active
    if app.mode != AppMode::Edit || app.active_panel != ActivePanel::Canvas {
        return;
    }
    
    match mouse_event.kind {
        MouseEventKind::Down(button) => {
            if button == MouseButton::Left {
                // Store the anchor position for potential drag selection
                app.mouse_anchor = Some((mouse_event.column, mouse_event.row));
                app.mouse_dragging = true;
                
                // Try to select the field at the clicked position
                // Note: mouse coordinates are 0-indexed, BMS coordinates are 1-indexed
                let pos = (mouse_event.row.saturating_add(1), mouse_event.column.saturating_add(1));
                if let Some(field_idx) = app.editor.field_at(pos) {
                    // If Shift is being held, extend the selection
                    // For now, just select the field (we'll check for Shift in key modifiers separately)
                    // Since mouse events don't have modifier info in crossterm 0.27,
                    // we'll use a simple click for single selection
                    app.editor.select_field(field_idx);
                    app.editor.cursor_pos = app.editor.map.fields[field_idx].pos;
                    app.set_message(&format!("Selected field {}", field_idx));
                } else {
                    // Clicked on empty space - clear selection and move cursor
                    app.editor.selected_field = None;
                    app.editor.selected_fields.clear();
                    app.editor.cursor_pos = pos;
                }
            } else if button == MouseButton::Right {
                // Right-click: select field and show properties (or context menu in future)
                let pos = (mouse_event.row.saturating_add(1), mouse_event.column.saturating_add(1));
                if let Some(field_idx) = app.editor.field_at(pos) {
                    app.editor.select_field(field_idx);
                    // In the future, we could show a context menu here
                    app.set_message(&format!("Right-clicked field {}", field_idx));
                }
            }
        }
        MouseEventKind::Up(button) => {
            if button == MouseButton::Left {
                app.mouse_dragging = false;
                app.mouse_anchor = None;
            }
        }
        MouseEventKind::Drag(button) => {
            if button == MouseButton::Left && app.mouse_dragging {
                // Drag selection - extend selection to current position
                if let Some(anchor) = app.mouse_anchor {
                    let current_pos = (mouse_event.column, mouse_event.row);
                    
                    // Convert to 1-indexed BMS coordinates
                    let anchor_bms = (anchor.1.saturating_add(1), anchor.0.saturating_add(1));
                    let current_bms = (current_pos.1.saturating_add(1), current_pos.0.saturating_add(1));
                    
                    // Find fields at both positions
                    if let Some(_anchor_idx) = app.editor.field_at(anchor_bms) {
                        if let Some(current_idx) = app.editor.field_at(current_bms) {
                            // Extend selection from anchor to current field
                            app.editor.extend_selection_to(current_idx);
                            app.set_message(&format!("Selected {} field(s)", app.editor.selected_count()));
                        }
                    }
                }
            }
        }
        MouseEventKind::ScrollDown | MouseEventKind::ScrollUp => {
            // Handle scroll wheel
            if mouse_event.kind == MouseEventKind::ScrollDown {
                app.scroll_down();
            } else {
                app.scroll_up();
            }
        }
        _ => {}
    }
}