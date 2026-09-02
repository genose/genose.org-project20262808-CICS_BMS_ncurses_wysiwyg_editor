//! Attribute Picker view module
//!
//! This module contains the attribute selection dialog rendering and input handling.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::text::{Line, Text};

use cobol_bms_core::FieldAttribute;
use crate::App;
use crate::AppMode;

/// Render the attribute picker
/// 
/// Displays a panel for selecting field attributes.
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
        height: 14,
    };
    
    let block = Block::default()
        .title(" Attributes ")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let attrs = vec![
        (FieldAttribute::Prot, "PROT", "P"),
        (FieldAttribute::Norm, "NORM", "N"),
        (FieldAttribute::Num, "NUM", "U"),
        (FieldAttribute::Alph, "ALPH", "A"),
        (FieldAttribute::AlphaNum, "ALNUM", "L"),
        (FieldAttribute::Intens, "INTENS", "I"),
        (FieldAttribute::Blink, "BLINK", "B"),
        (FieldAttribute::Reverse, "REVERSE", "V"),
        (FieldAttribute::Dark, "DARK", "D"),
    ];
    
    let mut lines = vec![Line::from(" Select: ".yellow())];
    for (attr, name, key) in &attrs {
        let prefix = if Some(attr) == app.selected_attribute.as_ref() { "> " } else { "  " };
        lines.push(Line::from(format!("{}{} [{}]", prefix, name, key)));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("Enter: Add attribute".to_string()));
    lines.push(Line::from("Esc: Cancel".to_string()));
    
    // Show current field attributes
    if let Some(idx) = app.editor.selected_field {
        let field = &app.editor.map.fields[idx];
        if !field.attrb.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(format!("Current: {:?}", field.attrb)).dim());
        }
    }
    
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

/// Handle input for attribute picker mode
/// 
/// Processes keyboard input for selecting field attributes.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    use FieldAttribute::*;
    
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.selected_attribute = None;
        }
        KeyCode::Enter => {
            if let Some(attr) = app.selected_attribute.clone() {
                if app.editor.selected_field.is_some() {
                    app.editor.add_selected_field_attribute(attr);
                }
            }
            app.mode = AppMode::Edit;
            app.selected_attribute = None;
        }
        KeyCode::Char('p') => app.selected_attribute = Some(Prot),
        KeyCode::Char('n') => app.selected_attribute = Some(Norm),
        KeyCode::Char('u') => app.selected_attribute = Some(Num),
        KeyCode::Char('a') => app.selected_attribute = Some(Alph),
        KeyCode::Char('l') => app.selected_attribute = Some(AlphaNum),
        KeyCode::Char('i') => app.selected_attribute = Some(Intens),
        KeyCode::Char('b') => app.selected_attribute = Some(Blink),
        KeyCode::Char('v') => app.selected_attribute = Some(Reverse),
        KeyCode::Char('d') => app.selected_attribute = Some(Dark),
        _ => {}
    }
}
