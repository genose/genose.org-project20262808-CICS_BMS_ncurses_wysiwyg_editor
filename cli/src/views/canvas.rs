//! Canvas view module
//!
//! This module contains the canvas rendering functionality for the BMS editor.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use ratatui::style::Color as TuiColor;
use ratatui::text::Span;
use std::collections::HashMap;

use cobol_bms_core::{BmsField, FieldType, FieldAttribute};
use cobol_bms_core::model::{Color as BmsColor, DecorationType, Justify};
use crate::types::InsertableObject;
use crate::{App, AppMode, ActivePanel};

/// Render the main canvas area
/// 
/// This function renders the canvas where BMS fields are displayed and edited.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let canvas_width = area.width.saturating_sub(25);
    let canvas_area = Rect {
        x: area.x,
        y: area.y,
        width: canvas_width,
        height: area.height,
    };
    
    // Draw border
    let canvas_title = match app.active_panel {
        ActivePanel::Canvas => format!(" [>] Canvas ({}x{}) [Ctrl+P:Toggle|Tab:Next|Shift+Tab:Prev|Alt/Ctrl+Arrows:Nav|Ctrl+Space:Preview]", app.editor.map.size.0, app.editor.map.size.1),
        ActivePanel::Sidebar => format!(" Canvas ({}x{}) [Ctrl+P:Toggle|Tab:Next|Shift+Tab:Prev|Alt/Ctrl+Arrows:Nav|Ctrl+Space:Preview]", app.editor.map.size.0, app.editor.map.size.1),
    };
    
    // Couleur du cadre en fonction de l'activation
    let border_color = match app.active_panel {
        ActivePanel::Canvas => TuiColor::Yellow,
        ActivePanel::Sidebar => TuiColor::White,
    };
    
    let canvas_block = Block::default()
        .title(canvas_title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
    f.render_widget(canvas_block, canvas_area);
    
    // Render content based on mode
    let content_area = Rect {
        x: canvas_area.x + 1,
        y: canvas_area.y + 1,
        width: canvas_area.width.saturating_sub(2),
        height: canvas_area.height.saturating_sub(2),
    };
    
    if app.show_bms_text {
        render_bms_text_preview(f, app, content_area);
    } else {
        render_bms_grid(f, app, content_area, canvas_area);
    }
}

/// Render the BMS grid view
/// 
/// This renders the BMS map as a grid where each field is represented visually.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render_bms_grid(f: &mut Frame, app: &App, area: Rect, canvas_area: Rect) {
    let map = &app.editor.map;
    
    // Build list of fields to display (including preview field)
    let mut fields_to_render: Vec<(BmsField, bool)> = map.fields.iter().map(|f| (f.clone(), false)).collect();
    
    // Add preview field for InsertPosition mode
    if let Some(obj) = app.pending_object {
        let preview_field = obj.create_field(app.pending_position);
        fields_to_render.push((preview_field, true));
    }
    
    // Add preview field for EditProperties mode
    if let Some(edit_field) = &app.edit_properties_field {
        fields_to_render.push((edit_field.clone(), true));
    }
    
    // Create a grid based on the visible area
    let visible_rows = area.height as usize;
    let visible_cols = area.width as usize;
    
    let start_row = app.scroll as usize;
    let end_row = (start_row + visible_rows).min(map.size.0 as usize);
    
    for grid_row in start_row..end_row {
        let mut spans = Vec::<Span>::new();
        
        for col in 1..=visible_cols {
            let mut c = ' ';
            let mut style = Style::default();
            let mut is_selected = false;
            
            // Check if any field covers this cell
            for (field, is_preview) in &fields_to_render {
                let (field_row, field_col) = field.pos;
                let field_row = field_row as usize;
                let field_col = field_col as usize;
                let field_end_col = field_col + field.length as usize - 1;
                
                // Check if this cell is within the field's area (considering height for multi-row fields)
                let field_end_row = if let Some(height) = field.height {
                    field_row + height as usize - 1
                } else {
                    field_row
                };
                
                if (grid_row + 1 >= field_row && grid_row + 1 <= field_end_row) && col >= field_col && col <= field_end_col {
                    // Determine the character based on field type and position within field
                    let field_start = field_col;
                    let field_end = field_end_col;
                    let is_first_col = col == field_start;
                    let is_last_col = col == field_end;
                    let is_first_row = grid_row + 1 == field_row;
                    let is_last_row = grid_row + 1 == field_end_row;
                    
                    // Pre-compute fieldset decoration for color handling
                    let fieldset_chars = if matches!(field.field_type, FieldType::Group) && field.height.is_some() {
                        let dec_type = field.fieldset_decoration.clone().unwrap_or(DecorationType::Brackets);
                        let border_type = field.fieldset_border.clone().unwrap_or(DecorationType::Dashes);
                        let title_align = field.fieldset_title_align.clone().unwrap_or(Justify::Left);
                        let fill_char = if let Some(fill_dec) = field.fieldset_title_fill_decoration.clone() {
                            match fill_dec {
                                DecorationType::Brackets => '[',
                                DecorationType::Parentheses => '(',
                                DecorationType::Plus => '+',
                                DecorationType::Asterisk => '*',
                                DecorationType::Hash => '#',
                                DecorationType::Dashes => '-',
                                DecorationType::Equals => '=',
                            }
                        } else {
                            ' '  // Default: space
                        };
                        let (open_dec, close_dec) = match dec_type {
                            DecorationType::Brackets => ('[', ']'),
                            DecorationType::Parentheses => ('(', ')'),
                            DecorationType::Plus => ('+', '+'),
                            DecorationType::Asterisk => ('*', '*'),
                            DecorationType::Hash => ('#', '#'),
                            DecorationType::Dashes => ('-', '-'),
                            DecorationType::Equals => ('=', '='),
                        };
                        let line_dec = match border_type {
                            DecorationType::Brackets => '-',
                            DecorationType::Parentheses => '-',
                            DecorationType::Plus => '-',
                            DecorationType::Asterisk => '*',
                            DecorationType::Hash => '#',
                            DecorationType::Dashes => '-',
                            DecorationType::Equals => '=',
                        };
                        Some((open_dec, close_dec, line_dec, fill_char, title_align))
                    } else {
                        None
                    };
                    
                    c = if *is_preview {
                        // Preview fields use special characters
                        if is_first_col {
                            '['
                        } else if is_last_col {
                            ']'
                        } else {
                            '-'
                        }
                    } else if let Some((open_dec, close_dec, line_dec, fill_char, title_align)) = fieldset_chars {
                        // Fieldset/Group rendering
                        let relative_col = col - field_col;
                        
                        // First row: title row
                        if is_first_row {
                            if is_first_col {
                                open_dec
                            } else if is_last_col {
                                close_dec
                            } else if field.length > 2 {
                                // Title row: check if this is the title position
                                let title_col = match title_align {
                                    Justify::Left => 1,
                                    Justify::Center => (field.length as usize) / 2,
                                    Justify::Right => (field.length as usize) - 2,
                                };
                                if relative_col == title_col.saturating_sub(1) {
                                    // This is where the title goes - use fill char or space
                                    fill_char
                                } else {
                                    line_dec
                                }
                            } else {
                                line_dec
                            }
                        } else if is_last_row {
                            // Bottom border row
                            if is_first_col {
                                // For box-drawing: ┌─┐ for top-left, └─┘ for bottom-left
                                match open_dec {
                                    '[' => '└',
                                    '(' => '╰',
                                    '+' => '└',
                                    _ => line_dec,
                                }
                            } else if is_last_col {
                                match close_dec {
                                    ']' => '┘',
                                    ')' => '╯',
                                    '+' => '┘',
                                    _ => line_dec,
                                }
                            } else {
                                line_dec
                            }
                        } else {
                            // Middle rows (content area)
                            if is_first_col || is_last_col {
                                match open_dec {
                                    '[' | '+' => '│',
                                    '(' => '│',
                                    _ => '|',
                                }
                            } else {
                                ' '
                            }
                        }
                    } else {
                        // Regular field rendering
                        if is_first_col {
                            if is_first_row {
                                // Top-left corner
                                match field.field_type {
                                    FieldType::Group => '╭',
                                    _ => '[',
                                }
                            } else if is_last_row {
                                // Bottom-left corner
                                match field.field_type {
                                    FieldType::Group => '╰',
                                    _ => '[',
                                }
                            } else {
                                // Left edge
                                match field.field_type {
                                    FieldType::Group => '│',
                                    _ => '[',
                                }
                            }
                        } else if is_last_col {
                            if is_first_row {
                                // Top-right corner
                                match field.field_type {
                                    FieldType::Group => '╮',
                                    _ => ']',
                                }
                            } else if is_last_row {
                                // Bottom-right corner
                                match field.field_type {
                                    FieldType::Group => '╯',
                                    _ => ']',
                                }
                            } else {
                                // Right edge
                                match field.field_type {
                                    FieldType::Group => '│',
                                    _ => ']',
                                }
                            }
                        } else {
                            // Middle of field
                            if is_first_row || is_last_row {
                                match field.field_type {
                                    FieldType::Group => '─',
                                    _ => '-',
                                }
                            } else {
                                ' '
                            }
                        }
                    };
                    
                    // Set style based on field state
                    if *is_preview {
                        style = Style::default().fg(TuiColor::Cyan).bg(TuiColor::DarkGray);
                    } else {
                        // Check if this field is selected
                        is_selected = false;
                        if let Some(selected_idx) = app.editor.selected_field {
                            if let Some(selected_field) = app.editor.map.fields.get(selected_idx) {
                                if selected_field.pos == field.pos {
                                    is_selected = true;
                                }
                            }
                        }
                        // Also check if in selected_fields (multi-select)
                        if !is_selected {
                            for &selected_idx in &app.editor.selected_fields {
                                if let Some(selected_field) = app.editor.map.fields.get(selected_idx) {
                                    if selected_field.pos == field.pos {
                                        is_selected = true;
                                        break;
                                    }
                                }
                            }
                        }
                        
                        if is_selected {
                            // Selected field - use bright colors
                            let bg_color = match field.text_color {
                                Some(BmsColor::Red) => TuiColor::Red,
                                Some(BmsColor::Green) => TuiColor::Green,
                                Some(BmsColor::Blue) => TuiColor::Blue,
                                Some(BmsColor::Yellow) => TuiColor::Yellow,
                                Some(BmsColor::White) => TuiColor::White,
                                Some(BmsColor::Turquoise) => TuiColor::Cyan,
                                Some(BmsColor::Pink) => TuiColor::Magenta,
                                Some(BmsColor::Neutral) | Some(BmsColor::Default) | None => TuiColor::White,
                                _ => TuiColor::White,
                            };
                            style = Style::default().fg(TuiColor::Black).bg(bg_color);
                        } else {
                            // Normal field - use field color
                            let fg_color = match field.text_color {
                                Some(BmsColor::Red) => TuiColor::Red,
                                Some(BmsColor::Green) => TuiColor::Green,
                                Some(BmsColor::Blue) => TuiColor::Blue,
                                Some(BmsColor::Yellow) => TuiColor::Yellow,
                                Some(BmsColor::White) => TuiColor::White,
                                Some(BmsColor::Turquoise) => TuiColor::Cyan,
                                Some(BmsColor::Pink) => TuiColor::Magenta,
                                Some(BmsColor::Neutral) | Some(BmsColor::Default) | None => TuiColor::White,
                                _ => TuiColor::White,
                            };
                            style = Style::default().fg(fg_color);
                        }
                    }
                    
                    // Don't check other fields once we found one
                    break;
                }
            }
            
            spans.push(Span::from(format!("{}", c)).style(style));
        }
        
        // Create the line and render it
        let line = Line::from(spans);
        let paragraph = Paragraph::new(line);
        f.render_widget(paragraph, Rect {
            x: area.x,
            y: (grid_row - start_row) as u16 + area.y,
            width: area.width,
            height: 1,
        });
    }
    
    // Render cursor
    render_cursor(f, app, canvas_area);
}

/// Render cursor on the canvas
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `canvas_area` - The canvas area
fn render_cursor(f: &mut Frame, app: &App, canvas_area: Rect) {
    let map = &app.editor.map;
    
    // Only render cursor if canvas is active
    if app.active_panel != ActivePanel::Canvas {
        return;
    }
    
    let (cursor_row, cursor_col) = app.editor.cursor_pos;
    let scroll_row = app.scroll as u16;
    
    // Check if cursor is visible in the current viewport
    if cursor_row >= scroll_row && cursor_row < scroll_row + canvas_area.height - 1 {
        let visible_row = cursor_row - scroll_row;
        let visible_col = cursor_col;
        
        if visible_col < canvas_area.width {
            // Create cursor character
            let cursor_char = '▋'; // Block cursor
            
            // Position cursor at the correct location
            f.set_cursor(
                canvas_area.x + visible_col + 1,
                canvas_area.y + visible_row + 1,
            );
        }
    }
}

/// Render BMS text preview
/// 
/// This renders the BMS map as text (similar to the original BMS format).
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render_bms_text_preview(f: &mut Frame, app: &App, area: Rect) {
    let map = &app.editor.map;
    let bms_text = cobol_bms_core::render_bms_text(map);
    let lines: Vec<&str> = bms_text.lines().collect();
    
    let visible_rows = area.height as usize;
    let start_line = app.scroll as usize;
    let end_line = (start_line + visible_rows).min(lines.len());
    
    let mut text_lines = Vec::new();
    for i in start_line..end_line {
        let line = lines.get(i).unwrap_or(&"");
        text_lines.push(Line::from(line.to_string()));
    }
    
    let text = Text::from(text_lines);
    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, area);
}
