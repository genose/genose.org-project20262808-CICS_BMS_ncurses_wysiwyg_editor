//! Open Dialog view module
//!
//! This module contains the open dialog rendering and input handling.

use std::path::PathBuf;

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Style, Color as TuiColor};

use cobol_bms_core::{parse_bms_file, BmsEditor};
use crate::types::{FileFilter, scan_directory_files_with_filter};
use crate::App;
use crate::AppMode;

/// Render the open dialog
/// 
/// Displays a file browser dialog for opening BMS files.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
/// * `area` - The area to render in
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let dialog_width = area.width.min(60);
    let dialog_height = area.height.min(16);
    let dialog_area = Rect {
        x: area.x + (area.width.saturating_sub(dialog_width)) / 2,
        y: area.y + (area.height.saturating_sub(dialog_height)) / 2,
        width: dialog_width,
        height: dialog_height,
    };
    
    let block = Block::default()
        .title(" Open File [Enter:Select|Esc:Cancel|Tab:Filter|Arrows:Nav] ")
        .borders(Borders::ALL);
    f.render_widget(block, dialog_area);
    
    let inner = Rect {
        x: dialog_area.x + 1,
        y: dialog_area.y + 1,
        width: dialog_area.width.saturating_sub(2),
        height: dialog_area.height.saturating_sub(2),
    };
    
    let mut current_y = inner.y;
    
    // Display current directory
    let dir_display = if app.file_browser_directory.is_empty() {
        ".".to_string()
    } else {
        app.file_browser_directory.clone()
    };
    let dir_para = Paragraph::new(format!("Directory: {}", dir_display))
        .style(Style::default().fg(TuiColor::Cyan));
    f.render_widget(dir_para, Rect { x: inner.x, y: current_y, width: inner.width, height: 1 });
    current_y += 1;
    
    // Display filter mode at the bottom
    let filter_text = Paragraph::new(format!("Filter: {}", app.file_browser_filter.display_name()))
        .style(Style::default().fg(TuiColor::Yellow));
    let filter_height = 1;
    let filter_y = inner.y + inner.height.saturating_sub(filter_height + 1);
    
    // Display file list with scroll
    let file_list_height = (filter_y - current_y) as usize;
    if !app.file_browser_files.is_empty() {
        for (idx, filename) in app.file_browser_files.iter().enumerate() {
            if idx >= app.file_browser_scroll && idx < app.file_browser_scroll + file_list_height {
                let is_selected = idx == app.file_browser_selected_index;
                let file_style = if is_selected {
                    Style::default().fg(TuiColor::Black).bg(TuiColor::Yellow)
                } else {
                    Style::default().fg(TuiColor::White)
                };
                
                let file_para = Paragraph::new(format!("  {}", filename))
                    .style(file_style);
                f.render_widget(file_para, Rect { x: inner.x, y: current_y as u16, width: inner.width, height: 1 });
                current_y += 1;
            }
        }
    } else {
        let no_files = Paragraph::new("  No files found")
            .style(Style::default().fg(TuiColor::Gray));
        f.render_widget(no_files, Rect { x: inner.x, y: current_y, width: inner.width, height: 1 });
        current_y += 1;
    }
    
    // Display filter info at bottom
    f.render_widget(filter_text, Rect { x: inner.x, y: filter_y, width: inner.width, height: 1 });
    
    // Display manual path entry
    if !app.open_path.is_empty() {
        let path_display = format!("Path: {}", app.open_path);
        let path_para = Paragraph::new(path_display)
            .style(Style::default().fg(TuiColor::Green));
        let path_y = filter_y + 1;
        if path_y < dialog_area.y + dialog_area.height {
            f.render_widget(path_para, Rect { x: inner.x, y: path_y, width: inner.width, height: 1 });
        }
    }
}

/// Handle input for open dialog mode
/// 
/// Processes keyboard input for the file browser.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    use crate::types::FileFilter as TypesFileFilter;
    
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.open_path.clear();
            app.file_browser_directory.clear();
            app.file_browser_files.clear();
        }
        KeyCode::Enter => {
            // Open selected file or use manual path
            if !app.file_browser_files.is_empty() && app.file_browser_selected_index < app.file_browser_files.len() {
                let filename = &app.file_browser_files[app.file_browser_selected_index];
                let full_path = std::path::Path::new(&app.file_browser_directory).join(filename);
                let path = PathBuf::from(full_path);
                
                if path.exists() {
                    match parse_bms_file(path.to_str().unwrap()) {
                        Ok(map) => {
                            app.editor = BmsEditor::from_map(map);
                            app.current_file = Some(path.clone());
                            app.mode = AppMode::Edit;
                            app.open_path.clear();
                            app.file_browser_directory.clear();
                            app.file_browser_files.clear();
                            app.set_message(&format!("Opened: {}", path.display()));
                        }
                        Err(e) => {
                            app.set_message(&format!("Failed to open: {}", e));
                        }
                    }
                } else {
                    app.set_message("File does not exist");
                }
            } else if !app.open_path.is_empty() {
                // Try manual path entry
                let path = PathBuf::from(&app.open_path);
                if path.exists() {
                    match parse_bms_file(path.to_str().unwrap()) {
                        Ok(map) => {
                            app.editor = BmsEditor::from_map(map);
                            app.current_file = Some(path.clone());
                            app.mode = AppMode::Edit;
                            app.open_path.clear();
                            app.file_browser_directory.clear();
                            app.file_browser_files.clear();
                            app.set_message(&format!("Opened: {}", path.display()));
                        }
                        Err(e) => {
                            app.set_message(&format!("Failed to open: {}", e));
                        }
                    }
                } else {
                    app.set_message("File does not exist");
                }
            }
        }
        KeyCode::Tab => {
            // Cycle through file filters
            app.file_browser_filter = TypesFileFilter::next(app.file_browser_filter);
            app.file_browser_files = scan_directory_files_with_filter(
                &app.file_browser_directory,
                app.file_browser_filter
            );
            app.file_browser_selected_index = 0;
            app.file_browser_scroll = 0;
        }
        KeyCode::Up => {
            if !app.file_browser_files.is_empty() {
                if app.file_browser_selected_index > 0 {
                    app.file_browser_selected_index -= 1;
                    if app.file_browser_selected_index < app.file_browser_scroll {
                        app.file_browser_scroll = app.file_browser_selected_index;
                    }
                }
            }
        }
        KeyCode::Down => {
            if !app.file_browser_files.is_empty() {
                if app.file_browser_selected_index + 1 < app.file_browser_files.len() {
                    app.file_browser_selected_index += 1;
                    // Scroll down if selected item is below visible area
                    if app.file_browser_selected_index >= app.file_browser_scroll + 10 {
                        app.file_browser_scroll = app.file_browser_selected_index.saturating_sub(9);
                    }
                }
            }
        }
        KeyCode::Backspace => {
            app.open_path.pop();
        }
        KeyCode::Char(c) => {
            app.open_path.push(c);
        }
        _ => {}
    }
}