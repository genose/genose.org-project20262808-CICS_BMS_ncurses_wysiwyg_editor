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

pub mod attribute_picker;
pub mod color_picker;
pub mod combo_key_help;
pub mod confirm;
pub mod map_type_picker;
pub mod status_bar;
pub mod text_input;
// pub mod help;  // Template available, not yet activated
// pub mod canvas;
// pub mod sidebar;
// pub mod properties;
// pub mod dialogs;
