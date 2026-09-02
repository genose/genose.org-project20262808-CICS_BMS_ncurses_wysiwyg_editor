//! COBOL BMS WYSIWYG Editor - CLI Interface
//!
//! Interface TUI complete pour l'edition visuelle des maps BMS CICS
//! 
//! Fonctionnalites:
//! - Creation de maps BMS depuis zero
//! - Edition interactive (ajout/suppression/deplacement de champs)
//! - Modification des proprietes (couleurs, attributs, etc.)
//! - Preview en temps reel
//! - Generation de code COBOL
//! - Undo/Redo

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph, Scrollbar},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    Frame,
};
use ratatui::widgets::ScrollbarState;
use ratatui::widgets::ScrollbarOrientation;
use ratatui::style::Color as TuiColor;
use std::{
    collections::HashMap,
    fs,
    io::stdout,
    path::PathBuf,
    time::Duration,
};

use cobol_bms_core::{
    parse_bms_file, generate_cobol, render_bms_text, FieldType, FieldAttribute,
    BmsEditor, BmsField, EditorMode, CursorDirection, ResizeDirection, create_default_map,
    image_to_ascii_simple,
};
use cobol_bms_core::model::{Color as BmsColor, DecorationType, Justify, DataType};

// Types module
mod types;
use types::{FileFilter, InsertableObject, scan_directory_files_with_filter, get_properties_for_field};

// Combo key system
mod combo_keys;
use combo_keys::{ComboKeyManager, ComboAction, ComboContext, TerminalType};

// Views module
mod views;
use views::add_object_dialog::{render as render_add_object_dialog, handle_mode as handle_add_object_dialog_mode};
use views::attribute_picker::{render as render_attribute_picker, handle_mode as handle_attribute_picker_mode};
use views::insert_position_dialog::{render as render_insert_position_dialog, handle_mode as handle_insert_position_mode};
use views::color_picker::{render as render_color_picker, handle_mode as handle_color_picker_mode};
use views::combo_key_help::{render as render_combo_key_help, handle_mode as handle_combo_key_help_mode};
use views::confirm::{render as render_confirm, handle_mode as handle_confirm_mode};
use views::help::{render as render_help, handle_mode as handle_help_mode};
use views::image_import_dialog::{render as render_image_import_dialog, handle_mode as handle_image_import_mode};
use views::map_type_picker::{render as render_map_type_picker, handle_mode as handle_map_type_picker_mode};
use views::open_dialog::{render as render_open_dialog, handle_mode as handle_open_dialog_mode};
use views::save_dialog::{render as render_save_dialog, handle_mode as handle_save_dialog_mode};
use views::status_bar::render as render_status_bar;
use views::text_input::{render as render_text_input, handle_mode as handle_text_input_mode};
use views::edit_properties_mode::handle_mode as handle_edit_properties_mode;
use views::mouse_input::handle_mode as handle_mouse_input;
use views::normal_mode::handle_mode as handle_normal_mode;
use views::properties_mode::handle_mode as handle_properties_mode;
use views::edit_mode::handle_mode as handle_edit_mode;
use views::ui::render as render_ui;
use views::utils::*;
use views::object_definitions_properties::{ObjectDefinitionsPropertyState, handle_object_definitions_properties_mode, render_object_definitions_properties_panel};


/// COBOL BMS WYSIWYG Editor - Editeur visuel pour les maps BMS CICS
#[derive(Parser, Debug)]
#[command(name = "cobol-bms")]
#[command(author = "genose.org")]
#[command(version = "0.1.0")]
#[command(about = "COBOL CICS/BMS WYSIWYG Editor - Creation et edition d'ecrans BMS", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Preview d'une map BMS (affichage texte)
    Preview {
        /// Chemin vers le fichier BMS
        file: PathBuf,
    },
    /// Generer du code COBOL depuis une map BMS
    Generate {
        /// Chemin vers le fichier BMS
        file: PathBuf,
        /// Fichier de sortie (par defaut: <input>.cbl)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Creer une nouvelle map BMS vide
    New {
        /// Nom de la map
        #[arg(short, long, default_value = "NEWMAP")]
        name: String,
        /// Nom du mapset
        #[arg(short, long, default_value = "DEFAULT")]
        mapset: String,
        /// Largeur (colonnes)
        #[arg(short, long, default_value = "80")]
        width: u16,
        /// Hauteur (lignes)
        #[arg(short, long, default_value = "24")]
        height: u16,
        /// Ouvrir dans l'editeur TUI
        #[arg(short, long)]
        edit: bool,
    },
    /// Editeur WYSIWYG interactif
    Edit {
        /// Fichier BMS a editer (optionnel)
        file: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Preview { file } => {
            let map = parse_bms_file(file.to_str().unwrap())?;
            println!("{}", render_bms_text(&map));
        }
        Commands::Generate { file, output } => {
            let map = parse_bms_file(file.to_str().unwrap())?;
            let cobol = generate_cobol(&map);
            
            let output_path = output.unwrap_or_else(|| {
                let mut path = file.clone();
                let new_name = path.file_name().unwrap().to_string_lossy().replace(".bms", ".cbl");
                path.set_file_name(new_name);
                path
            });
            
            fs::write(&output_path, cobol)
                .with_context(|| format!("Failed to write to: {}", output_path.display()))?;
            println!("Generated COBOL: {}", output_path.display());
        }
        Commands::New { name, mapset, width, height, edit } => {
            let mut editor = BmsEditor::new();
            editor.new_map(&name, &mapset, (height, width));
            
            if edit {
                run_editor(editor)?;
            } else {
                let bms = editor.export_to_bms();
                let path = PathBuf::from(format!("{}.bms", name));
                fs::write(&path, bms)?;
                println!("Created new BMS file: {}", path.display());
            }
        }
        Commands::Edit { file } => {
            let editor = if let Some(path) = file {
                let map = parse_bms_file(path.to_str().unwrap())?;
                BmsEditor::from_map(map)
            } else {
                BmsEditor::new()
            };
            run_editor(editor)?;
        }
    }
    
    Ok(())
}

// ==================== EDITEUR WYSIWYG ====================

/// Mode global de l'application
#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    /// Mode normal (navigation)
    Normal,
    /// Mode edition (WYSIWYG)
    Edit,
    /// Mode properties (edition des proprietes)
    Properties,
    /// Mode insert position (pour choisir position avant insertion)
    InsertPosition,
    /// Mode edit properties (edition interactive des proprietes)
    EditProperties,
    /// Mode map type picker
    MapTypePicker,
    /// Mode color picker
    ColorPicker,
    /// Mode attribute picker
    AttributePicker,
    /// Mode save dialog
    SaveDialog,
    /// Mode open dialog
    OpenDialog,
    /// Mode add object dialog
    AddObjectDialog,
    /// Mode text input dialog (for INITIAL, PIC, name, etc.)
    TextInput,
    /// Mode help
    Help,
    /// Mode combo key help
    ComboKeyHelp,
    /// Mode confirm (pour suppression, etc.)
    Confirm,
    /// Mode image import (for AsciiArt fields)
    ImageImport,
}

/// Panel actif pour la navigation (Canvas ou Sidebar)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActivePanel {
    Canvas,
    Sidebar,
}

impl ActivePanel {
    fn toggle(&mut self) {
        *self = match self {
            ActivePanel::Canvas => ActivePanel::Sidebar,
            ActivePanel::Sidebar => ActivePanel::Canvas,
        };
    }
}

/// Actions disponibles dans la sidebar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarAction {
    Edit,
    Delete,
    Move,
    Resize,
    AddField,
    PreviewBms,
    MapType,
}

impl SidebarAction {
    fn all() -> &'static [SidebarAction] {
        &[
            SidebarAction::Edit,
            SidebarAction::Delete,
            SidebarAction::Move,
            SidebarAction::Resize,
            SidebarAction::AddField,
            SidebarAction::PreviewBms,
            SidebarAction::MapType,
        ]
    }

    fn display(&self) -> &'static str {
        match self {
            SidebarAction::Edit => "e: Edit Properties",
            SidebarAction::Delete => "d: Delete",
            SidebarAction::Move => "m: Move",
            SidebarAction::Resize => "r: Resize",
            SidebarAction::AddField => "a: Add field",
            SidebarAction::PreviewBms => "p: Preview BMS",
            SidebarAction::MapType => "T: Map Type",
        }
    }

    #[allow(dead_code)]
    fn from_key(key: char) -> Option<SidebarAction> {
        match key {
            'e' => Some(SidebarAction::Edit),
            'd' => Some(SidebarAction::Delete),
            'm' => Some(SidebarAction::Move),
            'r' => Some(SidebarAction::Resize),
            'a' => Some(SidebarAction::AddField),
            'p' => Some(SidebarAction::PreviewBms),
            'T' => Some(SidebarAction::MapType),
            _ => None,
        }
    }
}

/// Section de la sidebar active
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarSection {
    Actions,
    Objects,
}

impl SidebarSection {
    fn next(&self) -> Self {
        match self {
            SidebarSection::Actions => SidebarSection::Objects,
            SidebarSection::Objects => SidebarSection::Actions,
        }
    }

    #[allow(dead_code)]
    fn previous(&self) -> Self {
        match self {
            SidebarSection::Actions => SidebarSection::Objects,
            SidebarSection::Objects => SidebarSection::Actions,
        }
    }
}

/// Etat de l'application
#[derive(Debug)]
pub struct App {
    editor: BmsEditor,
    mode: AppMode,
    current_file: Option<PathBuf>,
    scroll: u16,
    message: Option<String>,
    message_timeout: Option<usize>,
    exit: bool,
    // Panel actif pour la navigation
    active_panel: ActivePanel,
    // Navigation sidebar
    sidebar_section: SidebarSection,
    sidebar_actions_selected: Option<usize>,
    sidebar_objects_selected: Option<usize>,
    // Affichage canvas: grid ou text BMS
    show_bms_text: bool,
    // Pour le mode map type picker
    pub selected_map_type: Option<FieldType>,
    // Pour le mode properties
    property_index: usize,
    // Pour le mode insert position
    pending_object: Option<types::InsertableObject>,
    pending_position: (u16, u16),
    // Pour le mode edit properties
    edit_properties_field: Option<BmsField>,
    edit_properties_index: usize,
    edit_properties_scroll_offset: usize,
    // Pour le mode object definitions properties
    object_definitions_property_state: Option<ObjectDefinitionsPropertyState>,
    // Pour le mode color picker
    pub selected_color: Option<BmsColor>,
    // Pour le mode attribute picker
    pub selected_attribute: Option<FieldAttribute>,
    // Pour le mode save
    save_path: String,
    // Pour le mode open
    open_path: String,
    // File browser state for open/save dialogs
    file_browser_directory: String,
    file_browser_files: Vec<String>,
    file_browser_selected_index: usize,
    file_browser_filter: types::FileFilter,
    file_browser_scroll: usize,
    // Pour le mode add object
    selected_object_for_add: Option<types::InsertableObject>,
    // Pour le mode text input
    text_input_prompt: String,
    text_input_value: String,
    text_input_action: Option<TextInputAction>,
    // Pour le mode confirm
    confirm_action: ConfirmAction,
    // Mouse support
    mouse_dragging: bool,
    mouse_anchor: Option<(u16, u16)>,
    // Pour le mode image import
    image_import_path: String,
    image_import_error: Option<String>,
    image_import_directory: String,
    image_import_files: Vec<String>,
    image_import_selected_index: usize,
    image_import_show_all_files: bool,
    // Pour le mode help scroll
    help_scroll: usize,
    // Combo key system
    combo_key_manager: ComboKeyManager,
}

/// Action to perform after text input is submitted
#[derive(Debug, Clone)]
pub enum TextInputAction {
    /// Set the initial value of the selected field
    SetFieldInitial,
    /// Set the PIC value of the selected field
    SetFieldPic,
    /// Set the name of the selected field
    SetFieldName,
    /// Set the length of the selected field
    SetFieldLength,
    /// No action (generic text input)
    Custom(String),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ConfirmAction {
    QuitWithoutSave,
    DeleteField,
    ClearMap,
}



impl App {
    fn new(editor: BmsEditor) -> Self {
        Self {
            editor,
            mode: AppMode::Edit,
            current_file: None,
            scroll: 0,
            message: None,
            message_timeout: None,
            exit: false,
            active_panel: ActivePanel::Canvas,
            sidebar_section: SidebarSection::Actions,
            sidebar_actions_selected: None,
            sidebar_objects_selected: None,
            show_bms_text: false,
            selected_map_type: None,
            property_index: 0,
            pending_object: None,
            pending_position: (0, 0),
            edit_properties_field: None,
            edit_properties_index: 0,
            edit_properties_scroll_offset: 0,
            object_definitions_property_state: None,
            selected_color: None,
            selected_attribute: None,
            save_path: String::new(),
            open_path: String::new(),
            // File browser state
            file_browser_directory: String::new(),
            file_browser_files: Vec::new(),
            file_browser_selected_index: 0,
            file_browser_filter: types::FileFilter::AllFiles,
            file_browser_scroll: 0,
            selected_object_for_add: None,
            text_input_prompt: String::new(),
            text_input_value: String::new(),
            text_input_action: None,
            confirm_action: ConfirmAction::QuitWithoutSave,
            mouse_dragging: false,
            mouse_anchor: None,
            image_import_path: String::new(),
            image_import_error: None,
            image_import_directory: String::new(),
            image_import_files: Vec::new(),
            image_import_selected_index: 0,
            image_import_show_all_files: true,
            help_scroll: 0,
            combo_key_manager: Self::create_combo_key_manager(),
        }
    }

    /// Create the combo key manager with appropriate bindings
    fn create_combo_key_manager() -> ComboKeyManager {
        let mut manager = ComboKeyManager::new();
        
        // Register appropriate bindings based on terminal type
        let terminal_type = manager.terminal_type();
        if terminal_type == TerminalType::VSCode {
            manager.register_bindings(ComboKeyManager::vscode_bindings());
        } else {
            manager.register_bindings(ComboKeyManager::default_bindings());
        }
        
        // Set leader key for sequences
        manager.set_leader_key(KeyCode::Char(' '));
        
        manager
    }
    
    /// Reload bindings for current terminal type
    fn reload_combo_key_bindings(&mut self) {
        self.combo_key_manager.clear_bindings();
        
        let terminal_type = self.combo_key_manager.terminal_type();
        if terminal_type == TerminalType::VSCode {
            self.combo_key_manager.register_bindings(ComboKeyManager::vscode_bindings());
        } else {
            self.combo_key_manager.register_bindings(ComboKeyManager::default_bindings());
        }
    }

    /// Update combo key contexts based on current app state
    fn update_combo_key_contexts(&mut self) {
        // Clear existing contexts
        self.combo_key_manager.clear_contexts();
        
        // Always add Global context
        self.combo_key_manager.push_context(ComboContext::Global);
        
        // Add contexts based on mode
        match self.mode {
            AppMode::Edit => {
                self.combo_key_manager.push_context(ComboContext::EditMode);
                
                // Add context based on panel
                match self.active_panel {
                    ActivePanel::Canvas => {
                        self.combo_key_manager.push_context(ComboContext::CanvasPanel);
                        
                        // Add field selection contexts
                        if self.editor.selected_count() > 1 {
                            self.combo_key_manager.push_context(ComboContext::MultipleFieldsSelected);
                        } else if self.editor.selected_field.is_some() {
                            self.combo_key_manager.push_context(ComboContext::FieldSelected);
                        }
                    }
                    ActivePanel::Sidebar => {
                        self.combo_key_manager.push_context(ComboContext::SidebarPanel);
                    }
                }
            }
            AppMode::Properties => {
                self.combo_key_manager.push_context(ComboContext::PropertiesMode);
            }
            AppMode::InsertPosition => {
                self.combo_key_manager.push_context(ComboContext::InsertPositionMode);
            }
            AppMode::TextInput => {
                self.combo_key_manager.push_context(ComboContext::TextInputMode);
            }
            AppMode::ColorPicker => {
                self.combo_key_manager.push_context(ComboContext::ColorPickerMode);
            }
            AppMode::AttributePicker => {
                self.combo_key_manager.push_context(ComboContext::AttributePickerMode);
            }
            // For dialog-like modes
            AppMode::SaveDialog | AppMode::OpenDialog | AppMode::AddObjectDialog | AppMode::Confirm => {
                self.combo_key_manager.push_context(ComboContext::DialogMode);
            }
            _ => {}
        }
    }

    /// Switch to next sidebar section
    fn next_sidebar_section(&mut self) {
        self.sidebar_section = self.sidebar_section.next();
    }
    
    /// Switch to previous sidebar section
    fn prev_sidebar_section(&mut self) {
        self.sidebar_section = self.sidebar_section.previous();
    }

    /// Get help text for combo keys
    fn get_combo_key_help(&self) -> Vec<String> {
        let mut help_lines = Vec::new();
        
        // Add terminal compatibility info
        let terminal_type = self.combo_key_manager.terminal_type();
        if terminal_type.has_limitations() {
            help_lines.push(format!("Terminal: {} (using VSCode-compatible bindings)", terminal_type.as_str()));
            if terminal_type == TerminalType::VSCode {
                help_lines.push("VSCode Note: Many Ctrl+letter combos are intercepted by VSCode".to_string());
                help_lines.push("Using Ctrl+Shift+letter and Ctrl+Alt+letter combinations instead".to_string());
            }
        } else {
            help_lines.push(format!("Terminal: {} (using standard bindings)", terminal_type.as_str()));
        }
        help_lines.push(String::new());
        
        // Add active bindings help
        let help_text = self.combo_key_manager.get_help_text();
        for line in help_text {
            help_lines.push(line);
        }
        
        help_lines
    }

    /// Get all available combo key bindings as help text
    fn get_all_combo_help(&self) -> Vec<String> {
        let mut help_lines = Vec::new();
        help_lines.push("Available Key Bindings:".to_string());
        help_lines.push(String::new());
        
        // Get all bindings and organize by category
        let mut all_bindings = self.combo_key_manager.get_active_bindings();
        
        // Sort bindings by description for consistent display
        all_bindings.sort_by(|a, b| a.description.cmp(&b.description));
        
        for binding in all_bindings {
            help_lines.push(format!("  {}: {}", binding.combo_key.to_string(), binding.description));
        }
        
        help_lines
    }

    /// Force VSCode terminal mode (for testing)
    fn force_vscode_mode(&mut self) {
        self.combo_key_manager.set_terminal_type(TerminalType::VSCode);
    }
    
    /// Check if we're running in VSCode terminal
    fn is_vscode_mode(&self) -> bool {
        self.combo_key_manager.terminal_type() == TerminalType::VSCode
    }

    // Initialize first object for add dialog
    fn init_add_object_dialog(&mut self) {
        let objects = InsertableObject::all();
        if !objects.is_empty() {
            self.selected_object_for_add = Some(objects[0]);
        }
    }
    
    /// Start text input mode with an action
    fn start_text_input(&mut self, prompt: &str, initial_value: &str, action: TextInputAction) {
        self.mode = AppMode::TextInput;
        self.text_input_prompt = prompt.to_string();
        self.text_input_value = initial_value.to_string();
        self.text_input_action = Some(action);
    }
    
    /// Apply the text input action
    fn apply_text_input(&mut self, value: String) {
        if let Some(action) = self.text_input_action.take() {
            if let Some(idx) = self.editor.selected_field {
                match action {
                    TextInputAction::SetFieldInitial => {
                        let value_clone = value.clone();
                        self.editor.map.fields[idx].initial = if value.is_empty() { None } else { Some(value) };
                        self.set_message(&format!("INITIAL set to: {}", value_clone));
                    }
                    TextInputAction::SetFieldPic => {
                        let value_clone = value.clone();
                        self.editor.map.fields[idx].pic = if value.is_empty() { None } else { Some(value) };
                        self.set_message(&format!("PIC set to: {}", value_clone));
                    }
                    TextInputAction::SetFieldName => {
                        self.editor.map.fields[idx].name = value;
                        self.set_message(&format!("Name set to: {}", self.editor.map.fields[idx].name));
                    }
                    TextInputAction::SetFieldLength => {
                        if let Ok(length) = value.parse::<u16>() {
                            self.editor.map.fields[idx].length = length;
                            self.set_message(&format!("Length set to: {}", length));
                        } else {
                            self.set_message("Invalid length - must be a number");
                        }
                    }
                    TextInputAction::Custom(_) => {
                        self.set_message(&format!("Text entered: {}", value));
                    }
                }
            }
        }
    }
    
    /// Validate the current map and display errors if any
    fn validate_map(&mut self) -> bool {
        let errors = self.editor.map.validate();
        if errors.is_empty() {
            true
        } else {
            self.set_message(&format!("Validation errors: {}", errors.join("; ")));
            false
        }
    }
    
    /// Check if a field position is valid
    fn is_valid_field_position(&self, pos: (u16, u16), length: u16) -> bool {
        self.editor.map.is_valid_field_position(pos, length)
    }
    
    /// Show validation status if there are errors
    fn show_validation_status(&mut self) {
        let errors = self.editor.map.validate();
        if !errors.is_empty() {
            self.set_message(&format!("WARNING: {}", errors.join("; ")));
        }
    }
    
    fn is_modified(&self) -> bool {
        self.current_file.is_none() || 
        (self.current_file.as_ref().and_then(|p| {
            fs::read_to_string(p).ok().and_then(|_| {
                parse_bms_file(p.to_str().unwrap()).ok()
            })
        }).map_or(true, |original| original != self.editor.map))
    }
    
    fn set_message(&mut self, msg: &str) {
        self.message = Some(msg.to_string());
        // Error messages are persistent (no timeout)
        // Regular messages have a timeout of 60 frames
        let is_error = msg.starts_with("Error:") || msg.starts_with("Cannot") || msg.starts_with("Failed");
        self.message_timeout = if is_error { None } else { Some(60) };
    }
    
    fn clear_message(&mut self) {
        self.message = None;
        self.message_timeout = None;
    }
    
    fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }
    
    fn scroll_down(&mut self) {
        self.scroll += 1;
    }
    
    fn tick_message(&mut self) {
        if let Some(timeout) = self.message_timeout {
            if timeout == 0 {
                self.clear_message();
            } else {
                self.message_timeout = Some(timeout - 1);
            }
        }
    }
}

/// Handle combo actions
fn handle_combo_action(app: &mut App, action: ComboAction, key: &event::KeyEvent) {
    use ComboAction::*;
    
    let key_desc = format!("{} + {}", 
        if key.modifiers.is_empty() { "".to_string() } else { 
            format!("{:?}", key.modifiers) 
        }, 
        match key.code {
            KeyCode::Char(c) => format!("{}", c),
            _ => format!("{:?}", key.code),
        }
    );
    
    match action {
        // View switching
        TogglePanel => {
            app.active_panel.toggle();
            app.sidebar_actions_selected = None;
            app.sidebar_objects_selected = None;
            app.set_message(match app.active_panel {
                ActivePanel::Canvas => "Canvas mode [Ctrl+P]",
                ActivePanel::Sidebar => "Sidebar mode [Ctrl+P]",
            });
        }
        SwitchToCanvas => {
            app.active_panel = ActivePanel::Canvas;
            app.set_message("Switched to Canvas");
        }
        SwitchToSidebar => {
            app.active_panel = ActivePanel::Sidebar;
            app.set_message("Switched to Sidebar");
        }
        TogglePreview => {
            if app.mode == AppMode::Edit {
                app.show_bms_text = !app.show_bms_text;
                app.set_message(if app.show_bms_text {
                    "BMS text preview ON [Ctrl+Space]"
                } else {
                    "BMS text preview OFF [Ctrl+Space]"
                });
            }
        }
        ToggleHelp => {
            if app.mode == AppMode::Help {
                app.mode = AppMode::Edit;
            } else if app.mode == AppMode::ComboKeyHelp {
                app.mode = AppMode::Edit;
            } else {
                app.mode = AppMode::Help;
            }
        }
        
        // Navigation
        NextField => {
            if app.mode == AppMode::Edit && app.active_panel == ActivePanel::Canvas {
                app.editor.select_next_field();
                if let Some(idx) = app.editor.selected_field {
                    app.editor.cursor_pos = app.editor.map.fields[idx].pos;
                    app.set_message(&format!("Next field: {}", idx));
                }
            }
        }
        PreviousField => {
            if app.mode == AppMode::Edit && app.active_panel == ActivePanel::Canvas {
                app.editor.select_prev_field();
                if let Some(idx) = app.editor.selected_field {
                    app.editor.cursor_pos = app.editor.map.fields[idx].pos;
                    app.set_message(&format!("Previous field: {}", idx));
                }
            }
        }
        NextSection => {
            if app.active_panel == ActivePanel::Sidebar {
                app.next_sidebar_section();
            }
        }
        PreviousSection => {
            if app.active_panel == ActivePanel::Sidebar {
                app.prev_sidebar_section();
            }
        }
        
        FastScrollUp => {
            if app.mode == AppMode::Edit && app.active_panel == ActivePanel::Canvas {
                app.editor.move_cursor(CursorDirection::Up, 5);
                app.editor.select_field_at(app.editor.cursor_pos);
            }
        }
        FastScrollDown => {
            if app.mode == AppMode::Edit && app.active_panel == ActivePanel::Canvas {
                app.editor.move_cursor(CursorDirection::Down, 5);
                app.editor.select_field_at(app.editor.cursor_pos);
            }
        }
        FastScrollLeft => {
            if app.mode == AppMode::Edit && app.active_panel == ActivePanel::Canvas {
                app.editor.move_cursor(CursorDirection::Left, 5);
                app.editor.select_field_at(app.editor.cursor_pos);
            }
        }
        FastScrollRight => {
            if app.mode == AppMode::Edit && app.active_panel == ActivePanel::Canvas {
                app.editor.move_cursor(CursorDirection::Right, 5);
                app.editor.select_field_at(app.editor.cursor_pos);
            }
        }
        
        // Editing
        EnterEditMode => {
            app.mode = AppMode::Edit;
            app.set_message("Edit mode");
        }
        ExitEditMode => {
            app.mode = AppMode::Normal;
            app.set_message("Normal mode");
        }
        
        CopyField => {
            if app.mode == AppMode::Edit {
                let count = app.editor.selected_count();
                app.editor.copy_selected_fields();
                app.set_message(&format!("Copied {} field(s)", count));
            }
        }
        PasteField => {
            if app.mode == AppMode::Edit {
                let count = app.editor.paste_fields_at(app.editor.cursor_pos);
                app.set_message(&format!("Pasted {} field(s)", count));
            }
        }
        
        Undo => {
            if app.mode == AppMode::Edit {
                app.editor.undo();
                app.set_message("Undo");
            }
        }
        Redo => {
            if app.mode == AppMode::Edit {
                app.editor.redo();
                app.set_message("Redo");
            }
        }
        
        // Field operations
        ShowProperties => {
            if app.mode == AppMode::Edit && app.editor.selected_field.is_some() {
                app.mode = AppMode::Properties;
                app.set_message("Field Properties");
            }
        }
        ShowFieldProperties => {
            if app.mode == AppMode::Edit && app.editor.selected_field.is_some() {
                // Show field properties in edit mode
                if let Some(idx) = app.editor.selected_field {
                    app.edit_properties_field = Some(app.editor.map.fields[idx].clone());
                    app.edit_properties_index = 0;
                    // Initialize OBJECTS_DEFINITIONS property state
                    use crate::views::object_definitions_properties::ObjectDefinitionsPropertyState;
                    app.object_definitions_property_state = Some(ObjectDefinitionsPropertyState::new(&app.edit_properties_field.as_ref().unwrap()));
                    app.mode = AppMode::EditProperties;
                    app.set_message("Edit Field Properties");
                }
            }
        }
        ToggleGridSnap => {
            if app.mode == AppMode::Edit {
                app.editor.toggle_snap_to_grid();
                let msg = if app.editor.is_snap_to_grid_enabled() {
                    format!("Grid snap ON (size: {})", app.editor.get_grid_size())
                } else {
                    "Grid snap OFF".to_string()
                };
                app.set_message(&msg);
            }
        }
        AlignToGrid => {
            if app.mode == AppMode::Edit {
                if app.editor.is_snap_to_grid_enabled() && app.editor.get_grid_size() > 0 {
                    let count = app.editor.align_selected_to_grid();
                    app.set_message(&format!("Aligned {} field(s) to grid", count));
                } else {
                    app.set_message("Enable grid snap first (Ctrl+Shift+G)");
                }
            }
        }
        
        // File operations
        NewMap => {
            if app.mode == AppMode::Edit {
                app.mode = AppMode::Confirm;
                app.confirm_action = ConfirmAction::ClearMap;
            }
        }
        SaveMap => {
            if app.mode == AppMode::Edit {
                app.mode = AppMode::SaveDialog;
                app.save_path = app.current_file.as_ref()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "new_map.bms".to_string());
            }
        }
        OpenMap => {
            if app.mode == AppMode::Edit {
                app.mode = AppMode::OpenDialog;
                app.file_browser_directory = std::env::current_dir()
                    .ok()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| ".".to_string());
                app.file_browser_files = scan_directory_files_with_filter(
                    &app.file_browser_directory,
                    app.file_browser_filter
                );
                app.file_browser_selected_index = 0;
                app.file_browser_scroll = 0;
                app.open_path = String::new();
            }
        }
        GenerateCobol => {
            if app.mode == AppMode::Edit {
                let cobol = generate_cobol(&app.editor.map);
                let path = app.current_file.as_ref()
                    .map(|p| p.with_extension("cbl"))
                    .unwrap_or_else(|| PathBuf::from("output.cbl"));
                if let Err(e) = fs::write(&path, cobol) {
                    app.set_message(&format!("Failed: {}", e));
                } else {
                    app.set_message(&format!("Generated: {} [Ctrl+G]", path.display()));
                }
            }
        }
        ValidateMap => {
            if app.mode == AppMode::Edit {
                let errors = app.editor.map.validate();
                if errors.is_empty() {
                    app.set_message("Validation: OK");
                } else {
                    app.set_message(&format!("Validation errors: {}", errors.join("; ")));
                }
            }
        }
        
        // Object operations
        AddObject => {
            if app.mode == AppMode::Edit {
                app.mode = AppMode::AddObjectDialog;
                app.init_add_object_dialog();
                app.set_message("Select object type");
            }
        }
        InsertObject => {
            if app.mode == AppMode::Edit {
                app.mode = AppMode::InsertPosition;
                app.pending_object = None;
                app.set_message("Select position for new object");
            }
        }
        DeleteObject => {
            if app.mode == AppMode::Edit && app.editor.selected_field.is_some() {
                app.mode = AppMode::Confirm;
                app.confirm_action = ConfirmAction::DeleteField;
            }
        }
        MoveObject => {
            if app.mode == AppMode::Edit {
                if let Some(idx) = app.editor.selected_field {
                    app.editor.drag_start = Some(app.editor.map.fields[idx].pos);
                    app.editor.mode = EditorMode::MoveField;
                    app.set_message("Move field - arrows to move, Enter to drop");
                } else {
                    app.set_message("Error: No field selected to move");
                }
            }
        }
        ResizeObject => {
            if app.mode == AppMode::Edit {
                if let Some(idx) = app.editor.selected_field {
                    let field = &app.editor.map.fields[idx];
                    app.editor.drag_start = Some((field.pos.0, field.pos.1 + field.length - 1));
                    app.editor.mode = EditorMode::ResizeField { direction: ResizeDirection::Right };
                    app.set_message("Resize field - Left/Right to resize");
                } else {
                    app.set_message("Error: No field selected to resize");
                }
            }
        }
        
        // Color and attributes
        ShowColorPicker => {
            if app.mode == AppMode::Edit && app.editor.selected_field.is_some() {
                app.mode = AppMode::ColorPicker;
                app.set_message("Select color");
            }
        }
        ShowAttributePicker => {
            if app.mode == AppMode::Edit && app.editor.selected_field.is_some() {
                app.mode = AppMode::AttributePicker;
                app.set_message("Select attribute");
            }
        }
        
        // Text input
        StartTextInput => {
            app.start_text_input("Enter text", "", TextInputAction::Custom("generic".to_string()));
        }
        ConfirmInput => {
            if app.mode == AppMode::TextInput {
                // Confirm text input
                let value = app.text_input_value.clone();
                app.apply_text_input(value);
                app.mode = AppMode::Edit;
            }
        }
        CancelInput => {
            if app.mode == AppMode::TextInput {
                app.mode = AppMode::Edit;
                app.set_message("Input cancelled");
            }
        }
        
        // Sidebar operations
        SwitchToActions => {
            if app.active_panel == ActivePanel::Sidebar {
                app.sidebar_section = SidebarSection::Actions;
                app.sidebar_actions_selected = None;
                app.set_message("Actions section");
            }
        }
        SwitchToObjects => {
            if app.active_panel == ActivePanel::Sidebar {
                app.sidebar_section = SidebarSection::Objects;
                app.sidebar_objects_selected = None;
                app.set_message("Objects section");
            }
        }
        SelectObject => {
            if app.active_panel == ActivePanel::Sidebar {
                app.set_message("Object selected");
            }
        }
        
        // Misc
        ToggleDebug => {
            // Toggle debug mode - for now just show a message
            app.set_message("Debug mode toggled");
        }
        ExitApplication => {
            if app.is_modified() {
                app.mode = AppMode::Confirm;
                app.confirm_action = ConfirmAction::QuitWithoutSave;
            } else {
                app.exit = true;
            }
        }
        ShowAbout => {
            app.set_message("COBOL BMS Editor v0.1.0");
        }
        ShowComboKeyHelp => {
            app.mode = AppMode::ComboKeyHelp;
            app.help_scroll = 0;
            app.set_message("Combo Key Help - Press Q or Esc to return");
        }
        
        // Default case
        _ => {
            app.set_message(&format!("Action: {:?} [{}]", action, key_desc));
        }
    }
}

/// Execution de l'editeur
fn run_editor(editor: BmsEditor) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create app
    let mut app = App::new(editor);
    
    // Main loop
    loop {
        if app.exit {
            break;
        }
        
        terminal.draw(|f| render_ui(f, &app))?;
        
        // Handle input
        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => handle_input(&mut app, key),
                Event::Mouse(mouse_event) => handle_mouse_input(&mut app, mouse_event),
                Event::Resize(_, _) => {
                    // Terminal resize - redraw will handle it
                }
                _ => {}
            }
        }
        
        app.tick_message();
    }
    
    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn handle_input(app: &mut App, key: event::KeyEvent) {
    // Update combo key contexts based on current app state
    app.update_combo_key_contexts();
    
    // Try combo key handling first
    if let Some(action) = app.combo_key_manager.handle_key(&key) {
        handle_combo_action(app, action, &key);
        return;
    }
    
    // Handle leader key sequences
    if let Some(action) = app.combo_key_manager.handle_leader_sequence(key.code) {
        handle_combo_action(app, action, &key);
        return;
    }
    
    // Display message for every key combo (debug mode always on)
    let mods = if key.modifiers.is_empty() {
        String::new()
    } else {
        let mut parts = Vec::new();
        if key.modifiers.contains(KeyModifiers::CONTROL) { parts.push("Ctrl"); }
        if key.modifiers.contains(KeyModifiers::ALT) { parts.push("Alt"); }
        if key.modifiers.contains(KeyModifiers::SHIFT) { parts.push("Shift"); }
        format!("{}", parts.join("+"))
    };
    let key_name = match key.code {
        KeyCode::Char(c) => format!("{}", c),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::BackTab => "BackTab".to_string(),
        KeyCode::F(n) => format!("F{}", n),
        _ => format!("{:?}", key.code),
    };
    let key_desc = if mods.is_empty() {
        key_name
    } else {
        format!("{}+{}", mods, key_name)
    };
    
    // Only show message for modifier keys or special keys
    if !key.modifiers.is_empty() || matches!(key.code, KeyCode::Esc | KeyCode::Enter | KeyCode::Tab | KeyCode::BackTab) {
        app.set_message(&format!("Key: {}", key_desc));
    }
    
    // Handle Ctrl+P for panel toggle (simplified from Ctrl+Alt+O/P)
    // NOTE: This is now handled by combo key system, but kept as fallback
    if key.modifiers.contains(KeyModifiers::CONTROL) && !key.modifiers.contains(KeyModifiers::ALT) {
        match key.code {
            KeyCode::Char('p') | KeyCode::Char('P') => {
                app.active_panel.toggle();
                app.sidebar_actions_selected = None;
                app.sidebar_objects_selected = None;
                app.set_message(match app.active_panel {
                    ActivePanel::Canvas => "Canvas mode [Ctrl+P]",
                    ActivePanel::Sidebar => "Sidebar mode [Ctrl+P]",
                });
                return;
            }
            _ => {}
        }
    }
    
    // Handle Ctrl+Shift+Esc for confirm exit with save prompt
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.modifiers.contains(KeyModifiers::SHIFT) && key.code == KeyCode::Esc {
        if app.is_modified() {
            app.mode = AppMode::Confirm;
            app.confirm_action = ConfirmAction::QuitWithoutSave;
        } else {
            app.exit = true;
        }
        return;
    }
    
    // Handle Ctrl+Space for toggle preview (canvas/code)
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char(' ') {
        if app.mode == AppMode::Edit {
            app.show_bms_text = !app.show_bms_text;
            app.set_message(if app.show_bms_text {
                "BMS text preview ON [Ctrl+Space]"
            } else {
                "BMS text preview OFF [Ctrl+Space]"
            });
        }
        return;
    }
    
    // Handle Ctrl+Shift+P for toggle preview (legacy)
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.modifiers.contains(KeyModifiers::SHIFT) && key.code == KeyCode::Char('p') {
        if app.mode == AppMode::Edit {
            app.show_bms_text = !app.show_bms_text;
            app.set_message(if app.show_bms_text {
                "BMS text preview ON [Ctrl+Shift+P]"
            } else {
                "BMS text preview OFF [Ctrl+Shift+P]"
            });
        }
        return;
    }
    
    // Handle Ctrl+H for toggle help
    if key.modifiers.contains(KeyModifiers::CONTROL) && !key.modifiers.contains(KeyModifiers::ALT) && key.code == KeyCode::Char('h') {
        if app.mode == AppMode::Help {
            app.mode = AppMode::Edit;
        } else {
            app.mode = AppMode::Help;
        }
        return;
    }
    
    // Handle Ctrl keys in all modes
    if key.modifiers.contains(KeyModifiers::CONTROL) && !key.modifiers.contains(KeyModifiers::ALT) {
        match key.code {
            KeyCode::Char('c') => {
                if app.mode == AppMode::Edit {
                    let count = app.editor.selected_count();
                    app.editor.copy_selected_fields();
                    app.set_message(&format!("Copied {} field(s)", count));
                }
                return;
            }
            KeyCode::Char('q') => {
                if app.is_modified() {
                    app.mode = AppMode::Confirm;
                    app.confirm_action = ConfirmAction::QuitWithoutSave;
                } else {
                    app.exit = true;
                }
                return;
            }
            KeyCode::Char('s') => {
                if app.mode == AppMode::Edit {
                    app.mode = AppMode::SaveDialog;
                    app.save_path = app.current_file.as_ref()
                        .map(|p| p.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "new_map.bms".to_string());
                }
                return;
            }
            KeyCode::Char('z') => {
                if app.mode == AppMode::Edit {
                    app.editor.undo();
                    app.set_message("Undo");
                }
                return;
            }
            KeyCode::Char('y') => {
                if app.mode == AppMode::Edit {
                    app.editor.redo();
                    app.set_message("Redo");
                }
                return;
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                // Ctrl+G: Generate COBOL OR Ctrl+Shift+G: Toggle grid snap
                if key.modifiers.contains(KeyModifiers::SHIFT) && app.mode == AppMode::Edit {
                    app.editor.toggle_snap_to_grid();
                    let msg = if app.editor.is_snap_to_grid_enabled() {
                        format!("Grid snap ON (size: {})", app.editor.get_grid_size())
                    } else {
                        "Grid snap OFF".to_string()
                    };
                    app.set_message(&msg);
                } else if app.mode == AppMode::Edit {
                    // Ctrl+G: Generate COBOL
                    let cobol = generate_cobol(&app.editor.map);
                    let path = app.current_file.as_ref()
                        .map(|p| p.with_extension("cbl"))
                        .unwrap_or_else(|| PathBuf::from("output.cbl"));
                    if let Err(e) = fs::write(&path, cobol) {
                        app.set_message(&format!("Failed: {}", e));
                    } else {
                        app.set_message(&format!("Generated: {} [Ctrl+G]", path.display()));
                    }
                }
                return;
            }
            KeyCode::Char('v') | KeyCode::Char('V') => {
                // Ctrl+Shift+V: Validate
                if key.modifiers.contains(KeyModifiers::SHIFT) && app.mode == AppMode::Edit {
                    let errors = app.editor.map.validate();
                    if errors.is_empty() {
                        app.set_message("Validation: OK");
                    } else {
                        app.set_message(&format!("Validation errors: {}", errors.join("; ")));
                    }
                    return;
                }
                // Ctrl+V: Fall through to allow other handlers
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
                // Ctrl+Shift+L: Align selected fields to grid
                if key.modifiers.contains(KeyModifiers::SHIFT) && app.mode == AppMode::Edit {
                    if app.editor.snap_to_grid && app.editor.get_grid_size() > 0 {
                        let count = app.editor.align_selected_to_grid();
                        app.set_message(&format!("Aligned {} field(s) to grid", count));
                    } else {
                        app.set_message("Enable grid snap first (Ctrl+Shift+G)");
                    }
                    return;
                }
                // Ctrl+L: Fall through
            }
            KeyCode::Char('o') | KeyCode::Char('O') => {
                // Ctrl+O: Open file dialog
                if app.mode == AppMode::Edit {
                    app.mode = AppMode::OpenDialog;
                    // Initialize file browser for open dialog
                    app.file_browser_directory = std::env::current_dir()
                        .ok()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|| ".".to_string());
                    app.file_browser_files = scan_directory_files_with_filter(
                        &app.file_browser_directory,
                        app.file_browser_filter
                    );
                    app.file_browser_selected_index = 0;
                    app.file_browser_scroll = 0;
                    app.open_path = String::new();
                    app.set_message("Open file - Use arrows to select, Tab to change filter");
                }
                return;
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                // Ctrl+A: Add object dialog OR Ctrl+Shift+A: Select all
                if key.modifiers.contains(KeyModifiers::SHIFT) && app.mode == AppMode::Edit {
                    app.editor.select_all_fields();
                    app.set_message("All fields selected");
                } else if app.mode == AppMode::Edit {
                    app.mode = AppMode::AddObjectDialog;
                    app.init_add_object_dialog();
                    app.set_message("Select object type");
                }
                return;
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                // Ctrl+D: Delete selected field
                if app.mode == AppMode::Edit && app.editor.selected_field.is_some() {
                    app.mode = AppMode::Confirm;
                    app.confirm_action = ConfirmAction::DeleteField;
                } else if app.mode == AppMode::Edit {
                    app.set_message("Error: No field selected to delete (use arrows to select)");
                }
                return;
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                // Ctrl+M: Move selected field
                if app.mode == AppMode::Edit {
                    if let Some(idx) = app.editor.selected_field {
                        app.editor.drag_start = Some(app.editor.map.fields[idx].pos);
                        app.editor.mode = EditorMode::MoveField;
                        app.set_message("Move field - arrows to move, Enter to drop");
                    } else {
                        app.set_message("Error: No field selected to move (use arrows to select)");
                    }
                }
                return;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // Ctrl+R: Resize selected field
                if app.mode == AppMode::Edit {
                    if let Some(idx) = app.editor.selected_field {
                        app.editor.drag_start = Some((app.editor.map.fields[idx].pos.0, app.editor.map.fields[idx].pos.1 + app.editor.map.fields[idx].length - 1));
                        app.editor.mode = EditorMode::ResizeField { direction: ResizeDirection::Right };
                        app.set_message("Resize field - Left/Right to resize");
                    } else {
                        app.set_message("Error: No field selected to resize (use arrows to select)");
                    }
                }
                return;
            }
            _ => {}
        }
    }
    
    match app.mode {
        AppMode::Edit => handle_edit_mode(app, key),
        AppMode::Properties => handle_properties_mode(app, key),
        AppMode::InsertPosition => handle_insert_position_mode(app, key),
        AppMode::EditProperties => handle_edit_properties_mode(app, key),
        AppMode::MapTypePicker => handle_map_type_picker_mode(app, key),
        AppMode::ColorPicker => handle_color_picker_mode(app, key),
        AppMode::AttributePicker => handle_attribute_picker_mode(app, key),
        AppMode::SaveDialog => handle_save_dialog_mode(app, key),
        AppMode::OpenDialog => handle_open_dialog_mode(app, key),
        AppMode::AddObjectDialog => handle_add_object_dialog_mode(app, key),
        AppMode::TextInput => handle_text_input_mode(app, key),
        AppMode::Help => handle_help_mode(app, key),
        AppMode::ComboKeyHelp => handle_combo_key_help_mode(app, key),
        AppMode::Confirm => handle_confirm_mode(app, key),
        AppMode::ImageImport => handle_image_import_mode(app, key),
        AppMode::Normal => handle_normal_mode(app, key),
    }
}




















// ==================== UI ====================


