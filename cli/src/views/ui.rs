//! UI rendering module
//!
//! This module contains the main UI rendering logic.

use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use ratatui::style::Color as TuiColor;

use crate::App;
use crate::AppMode;
use crate::ActivePanel;
use super::canvas::render as render_canvas;
use super::sidebar::render as render_sidebar;
use super::properties::render_properties_panel as render_properties_panel;
use super::properties::render_edit_properties_panel as render_edit_properties_panel;
use super::insert_position_dialog::render as render_insert_position_dialog;
use super::map_type_picker::render as render_map_type_picker;
use super::color_picker::render as render_color_picker;
use super::attribute_picker::render as render_attribute_picker;
use super::save_dialog::render as render_save_dialog;
use super::open_dialog::render as render_open_dialog;
use super::add_object_dialog::render as render_add_object_dialog;
use super::text_input::render as render_text_input;
use super::help::render as render_help;
use super::combo_key_help::render as render_combo_key_help;
use super::confirm::render as render_confirm;
use super::image_import_dialog::render as render_image_import_dialog;
use super::status_bar::render as render_status_bar;

/// Main UI rendering function
/// 
/// Renders the complete user interface based on the current application state.
/// This includes the header, main content area (canvas/sidebar or dialogs), and status bar.
/// 
/// # Arguments
/// * `f` - The frame to render to
/// * `app` - The application state
pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();
    
    // Main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),    // Header
            Constraint::Min(1),       // Canvas + Sidebar
            Constraint::Length(2),    // Status bar
        ])
        .split(size);
    
    // Header
    let header_title = match app.mode {
        AppMode::Edit => " WYSIWYG EDITOR ",
        AppMode::Properties => " PROPERTIES ",
        AppMode::InsertPosition => " INSERT POSITION ",
        AppMode::EditProperties => " EDIT PROPERTIES ",
        AppMode::MapTypePicker => " MAP TYPE ",
        AppMode::ColorPicker => " COLOR PICKER ",
        AppMode::AttributePicker => " ATTRIBUTES ",
        AppMode::SaveDialog => " SAVE FILE ",
        AppMode::OpenDialog => " OPEN FILE ",
        AppMode::AddObjectDialog => " ADD OBJECT ",
        AppMode::TextInput => " TEXT INPUT ",
        AppMode::Help => " HELP ",
        AppMode::ComboKeyHelp => " COMBO KEY HELP ",
        AppMode::Confirm => " CONFIRM ",
        AppMode::ImageImport => " IMAGE IMPORT ",
        AppMode::Normal => " PREVIEW ",
    };
    
    let header = Block::default()
        .title(header_title)
        .title_alignment(ratatui::layout::Alignment::Center)
        .borders(Borders::TOP)
        .style(Style::default().bg(TuiColor::Blue).fg(TuiColor::White));
    f.render_widget(header, main_layout[0]);
    
    // Main content area
    let content_area = main_layout[1];
    
    match app.mode {
        AppMode::Edit | AppMode::Normal => {
            render_canvas(f, app, content_area);
            render_sidebar(f, app, content_area);
        }
        AppMode::Properties => {
            render_canvas(f, app, content_area);
            render_properties_panel(f, app, content_area);
        }
        AppMode::InsertPosition => {
            render_canvas(f, app, content_area);
            render_insert_position_dialog(f, app, content_area);
        }
        AppMode::EditProperties => {
            render_canvas(f, app, content_area);
            render_edit_properties_panel(f, app, content_area);
        }
        AppMode::MapTypePicker => {
            render_canvas(f, app, content_area);
            render_map_type_picker(f, app, content_area);
        }
        AppMode::ColorPicker => {
            render_canvas(f, app, content_area);
            render_color_picker(f, app, content_area);
        }
        AppMode::AttributePicker => {
            render_canvas(f, app, content_area);
            render_attribute_picker(f, app, content_area);
        }
        AppMode::SaveDialog => {
            render_save_dialog(f, app, content_area);
        }
        AppMode::OpenDialog => {
            render_canvas(f, app, content_area);
            render_open_dialog(f, app, content_area);
        }
        AppMode::AddObjectDialog => {
            render_canvas(f, app, content_area);
            render_add_object_dialog(f, app, content_area);
        }
        AppMode::TextInput => {
            render_canvas(f, app, content_area);
            render_text_input(f, app, content_area);
        }
        AppMode::Help => {
            render_help(f, app, content_area);
        }
        AppMode::ComboKeyHelp => {
            render_combo_key_help(f, app, content_area);
        }
        AppMode::Confirm => {
            render_confirm(f, app, content_area);
        }
        AppMode::ImageImport => {
            render_canvas(f, app, content_area);
            render_image_import_dialog(f, app, content_area);
        }
    }
    
    // Status bar
    render_status_bar(f, app, main_layout[2]);
}