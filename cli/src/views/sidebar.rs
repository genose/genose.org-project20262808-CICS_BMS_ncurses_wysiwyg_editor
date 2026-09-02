//! Sidebar view module
//!
//! This module contains the sidebar rendering functionality for the BMS editor.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use ratatui::style::Color as TuiColor;
use ratatui::text::{Line, Span, Text};

use crate::types::InsertableObject;
use crate::{App, AppMode, ActivePanel, SidebarSection, SidebarAction};

/// Render the sidebar area
/// 
/// This function renders the sidebar where field information and actions/objects are displayed.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let panel_area = Rect {
        x: area.x + area.width - 24,
        y: area.y,
        width: 24,
        height: area.height,
    };
    
    let title = match app.active_panel {
        ActivePanel::Sidebar => " [>] Sidebar [Ctrl+Alt+P:Toggle|Tab:Switch]",
        ActivePanel::Canvas => " Sidebar [Ctrl+Alt+P:Toggle|Tab:Switch]",
    };
    
    // Couleur du cadre en fonction de l'activation
    let border_color = match app.active_panel {
        ActivePanel::Sidebar => TuiColor::Yellow,
        ActivePanel::Canvas => TuiColor::White,
    };
    
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
    f.render_widget(block, panel_area);
    
    let inner = Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    };
    
    // Field info
    let mut lines: Vec<Line> = Vec::new();
    
    if let Some(idx) = app.editor.selected_field {
        let field = &app.editor.map.fields[idx];
        lines.push(Line::from(" Selected Field "));
        lines.push(Line::from(""));
        lines.push(Line::from(format!("Name: {}", field.name)));
        lines.push(Line::from(format!("Pos: ({},{})", field.pos.0, field.pos.1)));
        lines.push(Line::from(format!("Len: {}", field.length)));
        lines.push(Line::from(""));
        
        let mut attrs_line = String::new();
        for attr in &field.attrb {
            attrs_line.push_str(&format!("{:?} ", attr));
        }
        lines.push(Line::from(attrs_line));
        lines.push(Line::from(""));
    } else {
        lines.push(Line::from(" No field selected ".dim()));
        lines.push(Line::from(""));
    }
    
    // Section title based on active section
    let section_title = match app.sidebar_section {
        SidebarSection::Actions => "> Actions ",
        SidebarSection::Objects => "> Objects ",
    };
    lines.push(Line::from(section_title));
    lines.push(Line::from(""));
    
    // Render appropriate section
    match app.sidebar_section {
        SidebarSection::Actions => {
            // Render sidebar actions with selection highlight
            let actions = SidebarAction::all();
            for (i, action) in actions.iter().enumerate() {
                let display_text = action.display();
                let style = if app.active_panel == ActivePanel::Sidebar && app.sidebar_actions_selected == Some(i) {
                    Style::default().fg(TuiColor::Black).bg(TuiColor::Yellow)
                } else {
                    Style::default().fg(TuiColor::White)
                };
                lines.push(Line::from(Span::styled(display_text, style)));
            }
            
            // Additional actions not in SidebarAction enum
            if app.editor.selected_field.is_none() {
                lines.push(Line::from(""));
                lines.push(Line::from("n: New map"));
                lines.push(Line::from("N: Template"));
                lines.push(Line::from("v: Paste"));
                lines.push(Line::from("g: Gen COBOL"));
            }
        }
        SidebarSection::Objects => {
            // Render insertable objects with selection highlight
            let objects = InsertableObject::all();
            for (i, obj) in objects.iter().enumerate() {
                let display_text = obj.display();
                let style = if app.active_panel == ActivePanel::Sidebar && app.sidebar_objects_selected == Some(i) {
                    Style::default().fg(TuiColor::Black).bg(TuiColor::Yellow)
                } else {
                    Style::default().fg(TuiColor::White)
                };
                lines.push(Line::from(Span::styled(display_text, style)));
            }
        }
    }
    
    // Help hints
    lines.push(Line::from(""));
    lines.push(Line::from("Ctrl+P: Toggle Canvas/Sidebar".dim()));
    lines.push(Line::from("Tab: Next field / Switch section".dim()));
    lines.push(Line::from("Shift+Tab: Previous field".dim()));
    lines.push(Line::from("Alt/Ctrl+Up/Down: Fast scroll (5 lines)".dim()));
    lines.push(Line::from("Alt/Ctrl+Left/Right: Prev/Next field".dim()));
    lines.push(Line::from("Ctrl+Space: Toggle preview".dim()));
    
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, inner);
}
