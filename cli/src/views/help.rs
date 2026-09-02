//! Help view module - Reference implementation
//!
//! This module demonstrates the intended pattern for view modules.
//! When ready to use this pattern, the help functions should be moved from main.rs
//! and this file should be updated to import the App struct properly.
//!
//! Current status: The help functions are at the end of main.rs with the paging fixes applied.
//! This file serves as a template for future extraction.

// Use this when App is moved to a separate module:
// use crate::app::{App, AppMode};
// use crate::combo_keys::ComboKeyManager;

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::text::Line;

/// Get all help lines for the help view
/// This will be used by both the render and handler functions
pub fn get_help_lines() -> Vec<Line<'static>> {
    vec![
        Line::from(" WYSIWYG Editor - Help ".bold()),
        Line::from(""),
        Line::from(" Navigation: ".yellow()),
        Line::from("  j/k/Down/Up: Move cursor (1 line)"),
        Line::from("  h/l/Left/Right: Move cursor"),
        Line::from("  Alt/Ctrl+Up/Down: Move cursor (5 lines)"),
        Line::from("  Alt/Ctrl+Left/Right: Prev/Next field"),
        Line::from("  Tab/Shift+Tab: Next/Prev field"),
        Line::from("  Shift+Arrow: Extend selection"),
        Line::from("  Ctrl+P: Toggle Canvas/Sidebar"),
        Line::from("  Ctrl+Space: Toggle preview (canvas/code)"),
        Line::from("  Key triggers displayed in message bar"),
        Line::from(""),
        Line::from(" Mouse: ".yellow()),
        Line::from("  Left-click: Select field"),
        Line::from("  Left-click + drag: Multi-select fields"),
        Line::from("  Right-click: Select and show info"),
        Line::from("  Scroll: Scroll canvas"),
        Line::from(""),
        Line::from(" Selection: ".yellow()),
        Line::from("  Ctrl+Shift+A: Select all fields"),
        Line::from("  Shift+Arrow: Multi-select fields"),
        Line::from(""),
        Line::from(" Grid: ".yellow()),
        Line::from("  Ctrl+Shift+G: Toggle grid snap"),
        Line::from("  Ctrl+Shift+L: Align selected to grid"),
        Line::from(""),
        Line::from(" Field Ops: ".yellow()),
        Line::from("  a/A: Add field (10/20 chars) - legacy"),
        Line::from("  Ctrl+A: Add object (select type, then configure properties)"),
        Line::from("  d: Delete field (or Ctrl+D)"),
        Line::from("  m: Move field (or Ctrl+M)"),
        Line::from("  r: Resize field (or Ctrl+R)"),
        Line::from(""),
        Line::from(" Properties: ".yellow()),
        Line::from("  e: Edit properties"),
        Line::from("  C: Change color"),
        Line::from("  t: Change attributes"),
        Line::from(""),
        Line::from(" Clipboard: ".yellow()),
        Line::from("  c: Copy (or Ctrl+C)"),
        Line::from("  x: Cut"),
        Line::from("  v: Paste"),
        Line::from(""),
        Line::from(" File: ".yellow()),
        Line::from("  n: New map"),
        Line::from("  N: Template"),
        Line::from("  Ctrl+S: Save"),
        Line::from("  Ctrl+O: Open file"),
        Line::from("  g: Generate COBOL (or Ctrl+G)"),
        Line::from(""),
        Line::from(" Undo/Redo: ".yellow()),
        Line::from("  Ctrl+Z: Undo"),
        Line::from("  Ctrl+Y: Redo"),
        Line::from(""),
        Line::from(" Validation: ".yellow()),
        Line::from("  Ctrl+Shift+V: Validate map"),
        Line::from(""),
        Line::from(" Exit: ".yellow()),
        Line::from("  Esc: Quit with confirm"),
        Line::from("  Ctrl+Q: Quit with confirm"),
        Line::from("  Ctrl+Shift+Esc: Quit with confirm"),
        Line::from(""),
        Line::from(" Other: ".yellow()),
        Line::from("  ? or Ctrl+H: Toggle help"),
        Line::from("  Ctrl+Shift+H: Show combo key bindings"),
        Line::from(""),
        Line::from(" Combo Key Bindings:".yellow()),
        Line::from("  (See combo key help with Ctrl+Shift+H)"),
        Line::from(""),
        Line::from(" Note: Both legacy (letter) and new (Ctrl+letter) shortcuts work".dim()),
    ]
}

/// Render the help view
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render(f: &mut Frame, app: &super::App, area: Rect) {
    let help_area = area;
    let block = Block::default()
        .title(" Help (Scroll: Up/Down/PgUp/PgDn/Home/End) ")
        .borders(Borders::ALL);
    f.render_widget(block, help_area);
    
    let inner = Rect {
        x: help_area.x + 1,
        y: help_area.y + 1,
        width: help_area.width.saturating_sub(2),
        height: help_area.height.saturating_sub(2),
    };
    
    let all_help_lines = get_help_lines();
    let total_lines = all_help_lines.len();
    let visible_height = inner.height as usize;
    
    if visible_height == 0 {
        return;
    }
    
    let start_line = app.help_scroll.min(total_lines.saturating_sub(visible_height));
    let end_line = (start_line + visible_height).min(total_lines);
    
    let visible_lines: Vec<Line> = all_help_lines.into_iter()
        .skip(start_line)
        .take(end_line - start_line)
        .collect();
    
    let help_text = Text::from(visible_lines);
    
    let paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, inner);
    
    if total_lines > visible_height {
        let mut scrollbar_state = ScrollbarState::new(total_lines)
            .position(app.help_scroll)
            .viewport_content_length(visible_height);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .thumb_symbol("\u{2588}")
            .track_symbol(Some(" "))
            .begin_symbol(None)
            .end_symbol(None);
        f.render_stateful_widget(scrollbar, inner, &mut scrollbar_state);
    }
}

/// Handle input for help mode
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut super::App, key: KeyEvent) {
    let total_lines = get_help_lines().len();
    let max_scroll = total_lines.saturating_sub(1);
    
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app.mode = super::AppMode::Edit,
        KeyCode::Up | KeyCode::Char('k') => {
            if app.help_scroll > 0 {
                app.help_scroll -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.help_scroll < max_scroll {
                app.help_scroll += 1;
            }
        }
        KeyCode::PageUp => {
            let page_amount = 10.min(app.help_scroll);
            app.help_scroll = app.help_scroll.saturating_sub(page_amount);
        }
        KeyCode::PageDown => {
            let page_amount = 10;
            app.help_scroll = (app.help_scroll + page_amount).min(max_scroll);
        }
        KeyCode::Home => app.help_scroll = 0,
        KeyCode::End => app.help_scroll = max_scroll,
        _ => {}
    }
}
