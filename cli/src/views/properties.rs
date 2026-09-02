//! Properties panel view module
//!
//! This module contains the properties panel rendering functionality.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use ratatui::text::{Line, Text};

use crate::App;

/// Render the properties panel (read-only view)
/// 
/// This shows the properties of the currently selected field in a read-only format.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render_properties_panel(f: &mut Frame, app: &App, area: Rect) {
    let panel_width = area.width.min(35);
    let panel_area = Rect {
        x: area.x + area.width - panel_width,
        y: area.y,
        width: panel_width,
        height: area.height.min(15),
    };
    
    let block = Block::default()
        .title(" Properties [Read-only|Esc:Close] ")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    if let Some(idx) = app.editor.selected_field {
        let field = &app.editor.map.fields[idx];
        let lines = vec![
            Line::from("> Position ".yellow()),
            Line::from(format!("  Row: {} ", field.pos.0)),
            Line::from(format!("  Col: {} ", field.pos.1)),
            Line::from(""),
            Line::from(" Size ".yellow()),
            Line::from(format!("  Length: {} ", field.length)),
            Line::from(""),
            Line::from(" Appearance ".yellow()),
            Line::from(format!("  Color: {:?}", field.text_color)),
            Line::from(format!("  Attrs: {:?}", field.attrb)),
            Line::from(""),
            Line::from(" Type ".yellow()),
            Line::from(format!("  {:?}", field.field_type)),
        ];
        
        let text = Text::from(lines);
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::NONE));
        f.render_widget(paragraph, Rect {
            x: panel_area.x + 1,
            y: panel_area.y + 1,
            width: panel_area.width.saturating_sub(2),
            height: panel_area.height.saturating_sub(2),
        });
    }
}

/// Render the edit properties panel
/// 
/// This shows the properties of a field in an editable format with navigation.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render_edit_properties_panel(f: &mut Frame, app: &App, area: Rect) {

    
    let panel_width = (area.width * 3 / 4).min(50).max(40);
    let panel_area = Rect {
        x: (area.width - panel_width) / 2 + area.x,
        y: area.y + 2,
        width: panel_width,
        height: area.height.saturating_sub(4),
    };
    
    let block = Block::default()
        .title(" Edit Properties [Tab:Next|Shift+Tab:Prev|Enter:Edit|Esc:Close] ")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let inner = Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    };
    
    if let Some(field) = &app.edit_properties_field {
        // Basic field properties display
        let lines = vec![
            Line::from("> Position ".yellow()),
            Line::from(format!("  Row: {} ", field.pos.0)),
            Line::from(format!("  Col: {} ", field.pos.1)),
            Line::from(""),
            Line::from(" Size ".yellow()),
            Line::from(format!("  Length: {} ", field.length)),
            Line::from(""),
            Line::from(" Appearance ".yellow()),
            Line::from(format!("  Color: {:?}", field.text_color)),
            Line::from(format!("  Attrs: {:?}", field.attrb)),
            Line::from(""),
            Line::from(" Type ".yellow()),
            Line::from(format!("  {:?}", field.field_type)),
            Line::from(""),
            Line::from(" Name ".yellow()),
            Line::from(format!("  {}", field.name)),
            Line::from(""),
            if let Some(ref initial) = field.initial {
                Line::from(" Initial Value ".yellow())
            } else {
                Line::from("")
            },
            if let Some(ref initial) = field.initial {
                Line::from(format!("  {}", initial))
            } else {
                Line::from("")
            },
        ];
        
        let text = Text::from(lines);
        let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::NONE));
        f.render_widget(paragraph, inner);
    }
}
