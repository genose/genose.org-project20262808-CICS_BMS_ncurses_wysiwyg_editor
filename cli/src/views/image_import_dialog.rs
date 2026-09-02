//! Image Import Dialog view module
//!
//! This module contains the image import dialog rendering and input handling.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Style, Color as TuiColor};

use crate::App;
use crate::AppMode;

/// Render the image import dialog
/// 
/// Displays a file browser dialog for importing images for ASCII art conversion.
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
        .title(" Import Image for ASCII Art [Enter:Select|Esc:Cancel|Tab:Toggle Filter] ")
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
    let dir_display = if app.image_import_directory.is_empty() {
        ".".to_string()
    } else {
        app.image_import_directory.clone()
    };
    let dir_para = Paragraph::new(format!("Directory: {}", dir_display))
        .style(Style::default().fg(TuiColor::Cyan));
    f.render_widget(dir_para, Rect { x: inner.x, y: current_y, width: inner.width, height: 1 });
    current_y += 1;
    
    // Display filter mode
    let filter_mode = if app.image_import_show_all_files {
        "Showing ALL files"
    } else {
        "Showing IMAGE files only"
    };
    let filter_para = Paragraph::new(filter_mode)
        .style(Style::default().fg(TuiColor::Yellow));
    f.render_widget(filter_para, Rect { x: inner.x, y: current_y, width: inner.width, height: 1 });
    current_y += 1;
    
    // Display file list
    if !app.image_import_files.is_empty() {
        let visible_files = &app.image_import_files;
        
        // Show files in a scrollable list
        for (idx, filename) in visible_files.iter().enumerate() {
            if (current_y - inner.y) as usize >= inner.height as usize - 3 {
                break; // Stop if we run out of space
            }
            
            let is_selected = idx == app.image_import_selected_index && app.image_import_selected_index < visible_files.len();
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
    } else {
        let no_files = Paragraph::new("  No files found")
            .style(Style::default().fg(TuiColor::Gray));
        f.render_widget(no_files, Rect { x: inner.x, y: current_y, width: inner.width, height: 1 });
        current_y += 1;
    }
    
    // Display selected file path (for manual entry)
    if !app.image_import_path.is_empty() {
        let path_display = format!("Path: {}", app.image_import_path);
        let path_para = Paragraph::new(path_display)
            .style(Style::default().fg(TuiColor::Green));
        f.render_widget(path_para, Rect { x: inner.x, y: current_y, width: inner.width, height: 1 });
        current_y += 1;
    }
    
    // Display error message if any
    if let Some(error) = &app.image_import_error {
        let error_para = Paragraph::new(format!("Error: {}", error))
            .style(Style::default().fg(TuiColor::Red));
        f.render_widget(error_para, Rect { x: inner.x, y: current_y, width: inner.width, height: 1 });
        current_y += 1;
    }
    
    // Display help at the bottom
    let help_text = vec![
        "Up/Down: Navigate files",
        "Enter: Select file",
        "Tab: Toggle image/all files",
        "Esc: Cancel",
    ];
    for help_line in help_text.iter().rev() {
        if current_y < inner.y + inner.height {
            let help_para = Paragraph::new(*help_line)
                .style(Style::default().fg(TuiColor::Cyan).dim());
            f.render_widget(help_para, Rect { x: inner.x, y: current_y, width: inner.width, height: 1 });
        }
        current_y += 1;
    }
}

/// Handle input for image import dialog mode
/// 
/// Processes keyboard input for selecting images to import.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    use std::path::PathBuf;
    use cobol_bms_core::model::Color;
    
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.edit_properties_field = None;
            app.image_import_error = None;
            app.image_import_path.clear();
            app.image_import_directory.clear();
            app.image_import_files.clear();
            app.image_import_selected_index = 0;
        }
        KeyCode::Enter => {
            // Determine the full path based on current directory and selection
            let full_path = if !app.image_import_directory.is_empty() && app.image_import_selected_index < app.image_import_files.len() {
                let filename = &app.image_import_files[app.image_import_selected_index];
                std::path::Path::new(&app.image_import_directory).join(filename)
            } else if !app.image_import_path.is_empty() {
                std::path::PathBuf::from(&app.image_import_path)
            } else {
                app.image_import_error = Some("No file selected".to_string());
                return;
            };
            
            let path_str = full_path.to_string_lossy().to_string();
            
            // Try to load the image and convert to ASCII art
            match crate::image_to_ascii_simple(&path_str, app.edit_properties_field.as_ref().map_or(40, |f| f.length as u32), None) {
                Ok(ascii_art) => {
                    // Set the ASCII art on the current field
                    if let Some(field) = app.edit_properties_field.as_mut() {
                        field.ascii_art = Some(ascii_art);
                        // Update field dimensions to match ASCII art
                        if let Some(ascii_art_data) = &field.ascii_art {
                            field.length = ascii_art_data.width;
                            field.height = Some(ascii_art_data.height);
                        }
                    }
                    app.mode = AppMode::Edit;
                    app.edit_properties_field = None;
                    app.image_import_error = None;
                    app.image_import_path.clear();
                    app.image_import_directory.clear();
                    app.image_import_files.clear();
                    app.set_message("Image converted to ASCII art!");
                }
                Err(e) => {
                    app.image_import_error = Some(format!("Error: {}", e));
                }
            }
        }
        KeyCode::Tab => {
            // Toggle between showing all files and image files only
            app.image_import_show_all_files = !app.image_import_show_all_files;
            // Refresh the file list
            if !app.image_import_directory.is_empty() {
                app.image_import_files = crate::scan_directory_files(&app.image_import_directory, !app.image_import_show_all_files);
            }
            app.image_import_selected_index = 0;
            // Ensure index is valid after filter change
            if !app.image_import_files.is_empty() {
                app.image_import_selected_index = app.image_import_selected_index.min(app.image_import_files.len() - 1);
            }
            app.image_import_error = None;
        }
        KeyCode::Up => {
            if !app.image_import_files.is_empty() {
                if app.image_import_selected_index > 0 {
                    app.image_import_selected_index -= 1;
                } else {
                    app.image_import_selected_index = app.image_import_files.len() - 1;
                }
                // Clamp index to valid range
                app.image_import_selected_index = app.image_import_selected_index.min(app.image_import_files.len().saturating_sub(1));
                // Update the path to show the selected file
                if !app.image_import_files.is_empty() && app.image_import_selected_index < app.image_import_files.len() {
                    app.image_import_path = app.image_import_files[app.image_import_selected_index].clone();
                }
                app.image_import_error = None;
            }
        }
        KeyCode::Down => {
            if !app.image_import_files.is_empty() {
                if app.image_import_selected_index < app.image_import_files.len() - 1 {
                    app.image_import_selected_index += 1;
                } else {
                    app.image_import_selected_index = 0;
                }
                // Clamp index to valid range
                app.image_import_selected_index = app.image_import_selected_index.min(app.image_import_files.len().saturating_sub(1));
                // Update the path to show the selected file
                if !app.image_import_files.is_empty() && app.image_import_selected_index < app.image_import_files.len() {
                    app.image_import_path = app.image_import_files[app.image_import_selected_index].clone();
                }
                app.image_import_error = None;
            }
        }
        KeyCode::Backspace => {
            app.image_import_path.pop();
            app.image_import_error = None;
        }
        KeyCode::Char(c) => {
            app.image_import_path.push(c);
            app.image_import_error = None;
        }
        _ => {}
    }
}