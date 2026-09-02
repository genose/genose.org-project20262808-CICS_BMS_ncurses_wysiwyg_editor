//! Map Type Picker view module
//!
//! This module contains the map type selection dialog rendering and input handling.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Style, Color as TuiColor};
use ratatui::text::{Line, Span, Text};

use cobol_bms_core::FieldType;
use crate::App;
use crate::AppMode;

/// Return scrollable map types
fn get_scrollable_map_types() -> &'static [FieldType] {
    &[
        FieldType::Map,
        FieldType::DFHMSD,
        FieldType::DFHMDF,
        FieldType::DFHMDI,
    ]
}

/// Render the map type picker
/// 
/// Displays a panel for selecting the BMS map type.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let panel_width = 25;
    let panel_area = Rect {
        x: area.x + area.width - panel_width,
        y: area.y,
        width: panel_width,
        height: 12,
    };
    
    let block = Block::default()
        .title(" Map Type [Up/Down:Nav|M/S/D/I:Select|Enter:Ok|Esc:Cancel]")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let inner = Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    };
    
    let map_types = [
        (FieldType::Map, "Standard MAP", "M"),
        (FieldType::DFHMSD, "Scrollable Data (DFHMSD)", "S"),
        (FieldType::DFHMDF, "Scrollable Formatted (DFHMDF)", "D"),
        (FieldType::DFHMDI, "Scrollable Input (DFHMDI)", "I"),
    ];
    
    let mut lines = vec![Line::from(" Select Map Type ".yellow())];
    for (map_type, name, key) in &map_types {
        let selected_type = app.selected_map_type.as_ref();
        let is_selected = selected_type == Some(map_type);
        let prefix = if is_selected { "> " } else { "  " };
        let style = if is_selected {
            Style::default().fg(TuiColor::Black).bg(TuiColor::Yellow)
        } else {
            Style::default().fg(TuiColor::White)
        };
        lines.push(Line::from(Span::styled(format!("{} {} [{}]", prefix, name, key), style)));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("Enter: Select".dim()));
    lines.push(Line::from("Esc: Cancel".dim()));
    
    // Show current map type
    lines.push(Line::from(""));
    lines.push(Line::from(format!("Current: {:?}", app.editor.map.map_type)).dim());
    
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, inner);
}

/// Handle input for map type picker mode
/// 
/// Processes keyboard input for selecting the map type.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.selected_map_type = None;
        }
        KeyCode::Enter => {
            if let Some(map_type) = app.selected_map_type.clone() {
                app.editor.map.map_type = map_type.clone();
                app.mode = AppMode::Edit;
                app.selected_map_type = None;
                app.set_message(&format!("Map type set to: {:?}", map_type));
            }
        }
        KeyCode::Up => {
            let all_types = get_scrollable_map_types();
            if !all_types.is_empty() {
                let new_selection = if let Some(current) = &app.selected_map_type {
                    if let Some(pos) = all_types.iter().position(|t| t == current) {
                        if pos > 0 {
                            Some(all_types[pos - 1].clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    Some(all_types[all_types.len() - 1].clone())
                };
                if let Some(new_type) = new_selection {
                    app.selected_map_type = Some(new_type);
                }
            }
        }
        KeyCode::Down => {
            let all_types = get_scrollable_map_types();
            if !all_types.is_empty() {
                let new_selection = if let Some(current) = &app.selected_map_type {
                    if let Some(pos) = all_types.iter().position(|t| t == current) {
                        if pos + 1 < all_types.len() {
                            Some(all_types[pos + 1].clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    Some(all_types[0].clone())
                };
                if let Some(new_type) = new_selection {
                    app.selected_map_type = Some(new_type);
                }
            }
        }
        KeyCode::Char('M') => app.selected_map_type = Some(FieldType::Map),
        KeyCode::Char('S') => app.selected_map_type = Some(FieldType::DFHMSD),
        KeyCode::Char('D') => app.selected_map_type = Some(FieldType::DFHMDF),
        KeyCode::Char('I') => app.selected_map_type = Some(FieldType::DFHMDI),
        _ => {}
    }
}
