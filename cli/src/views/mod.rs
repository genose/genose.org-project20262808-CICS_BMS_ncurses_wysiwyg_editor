//! Views module - Contains all UI views/windows/menus for the application
//!
//! This module organizes the application's UI into separate, maintainable components.
//! Each view typically has:
//! - A render function: `render_<view_name>(f: &mut Frame, app: &App, area: Rect)`
//! - A handler function: `handle_<view_name>_mode(app: &mut App, key: KeyEvent)`
//!
//! Current views:
//! - canvas: The main BMS map canvas
//! - sidebar: The sidebar with objects and actions
//! - properties: Property editing panel
//! - help: Help view with keyboard shortcuts (template in help.rs)
//! - combo_key_help: Combo key bindings help (fully extracted)
//! - dialogs: Various dialog boxes (save, open, confirm, etc.)
//!
//! Future: Each view will be extracted to its own file in this directory.

pub mod add_object_dialog;
pub mod attribute_picker;
pub mod color_picker;
pub mod combo_key_help;
pub mod confirm;
pub mod edit_mode;
pub mod help;
pub mod image_import_dialog;
pub mod insert_position_dialog;
pub mod edit_properties_mode;
pub mod mouse_input;
pub mod normal_mode;
pub mod ui;
pub mod properties_mode;
pub mod utils;
pub mod map_type_picker;
pub mod open_dialog;
pub mod save_dialog;
pub mod canvas;
pub mod properties;
pub mod sidebar;
pub mod status_bar;
pub mod text_input;
pub mod object_definitions_properties;
// pub mod sidebar;
// pub mod properties;
// pub mod dialogs;
