//! Combo Key Help view module
//!
//! This module contains the combo key help view rendering and input handling.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::text::Line;
use ratatui::style::{Style, Color as TuiColor};

use crate::combo_keys::ComboKeyManager;

/// Render the combo key help view
/// 
/// Displays all available combo key bindings with scrolling support.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render(f: &mut Frame, app: &crate::App, area: Rect) {
    let help_area = area;
    let block = Block::default()
        .title(" Combo Key Help (Scroll: Up/Down/PgUp/PgDn/Home/End/Q to quit) ")
        .borders(Borders::ALL);
    f.render_widget(block, help_area);
    
    let inner = Rect {
        x: help_area.x + 1,
        y: help_area.y + 1,
        width: help_area.width.saturating_sub(2),
        height: help_area.height.saturating_sub(2),
    };
    
    // Get combo key help lines
    let combo_help_lines = app.get_combo_key_help();
    let all_help_lines: Vec<Line> = combo_help_lines
        .into_iter()
        .map(|s| Line::from(s).style(Style::default().fg(TuiColor::White)))
        .collect();
    
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

/// Handle input for combo key help mode
/// 
/// Processes keyboard input for navigating the combo key help view.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut crate::App, key: KeyEvent) {
    // Get combo key help lines count for bounds checking
    let combo_help_lines = app.get_combo_key_help();
    let total_lines = combo_help_lines.len();
    let max_scroll = total_lines.saturating_sub(1);
    
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app.mode = crate::AppMode::Edit,
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
