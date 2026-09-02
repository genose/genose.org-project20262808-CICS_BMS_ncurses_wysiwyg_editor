//! Status Bar view module
//!
//! This module contains the status bar rendering.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use ratatui::style::{Style, Color as TuiColor};
use ratatui::layout::{Layout, Direction, Constraint};

use crate::App;
use crate::AppMode;

/// Detect if running inside VS Code integrated terminal
fn is_vscode_terminal() -> bool {
    let term_program = std::env::var("TERM_PROGRAM").unwrap_or_default();
    term_program == "vscode" || term_program.contains("vscode")
}

/// Render the status bar
/// 
/// Displays mode, message, cursor position, selection count, and file info.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let status_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(40),
            Constraint::Percentage(35),
        ])
        .split(area);
    
    // Mode
    let mode_text = match app.mode {
        AppMode::Edit => "EDIT",
        AppMode::Properties => "PROPERTIES",
        AppMode::InsertPosition => "INSERT_POS",
        AppMode::EditProperties => "EDIT_PROPS",
        AppMode::MapTypePicker => "MAP_TYPE",
        AppMode::ColorPicker => "COLOR",
        AppMode::AttributePicker => "ATTRS",
        AppMode::SaveDialog => "SAVE",
        AppMode::OpenDialog => "OPEN",
        AppMode::AddObjectDialog => "ADD_OBJ",
        AppMode::TextInput => "TEXT_IN",
        AppMode::Help => "HELP",
        AppMode::ComboKeyHelp => "COMBO_HELP",
        AppMode::Confirm => "CONFIRM",
        AppMode::ImageImport => "IMG_IMPORT",
        AppMode::Normal => "PREVIEW",
    };
    
    let mode = Paragraph::new(format!(" MODE: {}", mode_text))
        .style(Style::default().fg(TuiColor::Green).bold())
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(mode, status_layout[0]);
    
    // Message and cursor position
    let message_text = app.message.as_deref().unwrap_or("");
    let cursor_info = format!(" Row:{} Col:{} ", app.editor.cursor_pos.0, app.editor.cursor_pos.1);
    let status_text = if message_text.is_empty() {
        cursor_info
    } else {
        format!("{}{}", cursor_info, message_text)
    };
    let message = Paragraph::new(status_text)
        .style(Style::default().fg(TuiColor::Red))
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(message, status_layout[1]);
    
    // Selection count and file info
    let selection_count = app.editor.selected_count();
    let selection_text = if selection_count > 0 {
        format!(" [{}] ", selection_count)
    } else {
        String::new()
    };
    
    let file_info = if let Some(ref path) = app.current_file {
        format!(" {} ", path.file_name().unwrap_or_default().to_string_lossy())
    } else {
        " NEW MAP ".to_string()
    };
    
    let modified = if app.is_modified() { "[MODIFIED]" } else { "" };
    let vscode_indicator = if is_vscode_terminal() { "[VSCode]" } else { "" };
    let file = Paragraph::new(format!("{}{}{}{}", selection_text, file_info, modified, vscode_indicator))
        .style(Style::default().fg(TuiColor::Cyan))
        .alignment(ratatui::layout::Alignment::Right)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(file, status_layout[2]);
}
