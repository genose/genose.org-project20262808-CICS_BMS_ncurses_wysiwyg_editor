//! Normal Mode view module
//!
//! This module contains the normal mode input handling.
//! Normal mode is a fallback mode that mostly delegates to edit mode.

use crossterm::event::{KeyCode, KeyEvent};

use crate::App;
use crate::AppMode;
use crate::ConfirmAction;

/// Handle input for normal mode
/// 
/// Normal mode is a simple mode that delegates most input to edit mode.
/// It primarily handles the 'q' key for quitting and 'e' or Esc to return to edit mode.
/// 
/// # Arguments
/// * `app` - The application state (mutable)
/// * `key` - The key event to handle
pub fn handle_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('e') | KeyCode::Esc => app.mode = AppMode::Edit,
        KeyCode::Char('q') => {
            if app.is_modified() {
                app.mode = AppMode::Confirm;
                app.confirm_action = ConfirmAction::QuitWithoutSave;
            } else {
                app.exit = true;
            }
        }
        _ => crate::handle_edit_mode(app, key),
    }
}