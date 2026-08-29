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
    event::{self, Event, KeyCode, KeyModifiers, MouseButton, MouseEvent, MouseEventKind, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    Frame,
};
use ratatui::style::Color as TuiColor;
use std::{
    fs,
    io::stdout,
    path::PathBuf,
    time::Duration,
};

use cobol_bms_core::{
    parse_bms_file, generate_cobol, render_bms_text, FieldType, FieldAttribute,
    BmsEditor, BmsField, EditorMode, CursorDirection, ResizeDirection, create_default_map,
};
use cobol_bms_core::model::Color as BmsColor;

// ==================== UTILITIES ====================

/// Detect if running inside VS Code integrated terminal
fn is_vscode_terminal() -> bool {
    let term_program = std::env::var("TERM_PROGRAM").unwrap_or_default();
    term_program == "vscode" || term_program.contains("vscode")
}

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
enum AppMode {
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
    /// Mode confirm (pour suppression, etc.)
    Confirm,
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
enum SidebarAction {
    Edit,
    Delete,
    Move,
    Resize,
    Color,
    Attributes,
    AddField,
    AddLongField,
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
            SidebarAction::Color,
            SidebarAction::Attributes,
            SidebarAction::AddField,
            SidebarAction::AddLongField,
            SidebarAction::PreviewBms,
            SidebarAction::MapType,
        ]
    }

    fn display(&self) -> &'static str {
        match self {
            SidebarAction::Edit => "e: Edit",
            SidebarAction::Delete => "d: Delete",
            SidebarAction::Move => "m: Move",
            SidebarAction::Resize => "r: Resize",
            SidebarAction::Color => "C: Color",
            SidebarAction::Attributes => "t: Attrs",
            SidebarAction::AddField => "a: Add field",
            SidebarAction::AddLongField => "A: Add long",
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
            'C' => Some(SidebarAction::Color),
            't' => Some(SidebarAction::Attributes),
            'a' => Some(SidebarAction::AddField),
            'A' => Some(SidebarAction::AddLongField),
            'p' => Some(SidebarAction::PreviewBms),
            'T' => Some(SidebarAction::MapType),
            _ => None,
        }
    }
}

/// Types d'objets insérables dans le Canvas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertableObject {
    AlphanumericField,
    NumericField,
    DateField,
    TimeField,
    BooleanField,
    Literal,
    ProtectedLiteral,
    Group,
    Line,
    Box,
}

impl InsertableObject {
    pub fn all() -> &'static [InsertableObject] {
        &[
            InsertableObject::AlphanumericField,
            InsertableObject::NumericField,
            InsertableObject::DateField,
            InsertableObject::TimeField,
            InsertableObject::BooleanField,
            InsertableObject::Literal,
            InsertableObject::ProtectedLiteral,
            InsertableObject::Group,
            InsertableObject::Line,
            InsertableObject::Box,
        ]
    }

    pub fn display(&self) -> &'static str {
        match self {
            InsertableObject::AlphanumericField => "Alphanumeric Field",
            InsertableObject::NumericField => "Numeric Field",
            InsertableObject::DateField => "Date Field",
            InsertableObject::TimeField => "Time Field",
            InsertableObject::BooleanField => "Boolean Field",
            InsertableObject::Literal => "Literal",
            InsertableObject::ProtectedLiteral => "Protected Literal",
            InsertableObject::Group => "Group",
            InsertableObject::Line => "Horizontal Line",
            InsertableObject::Box => "Box",
        }
    }
    
    pub fn default_length(&self) -> u16 {
        match self {
            InsertableObject::AlphanumericField => 20,
            InsertableObject::NumericField => 10,
            InsertableObject::DateField => 8,
            InsertableObject::TimeField => 6,
            InsertableObject::BooleanField => 1,
            InsertableObject::Literal | InsertableObject::ProtectedLiteral => 20,
            InsertableObject::Group => 1,
            InsertableObject::Line | InsertableObject::Box => 40,
        }
    }

    pub fn create_field(&self, pos: (u16, u16)) -> BmsField {
        let mut field = BmsField::default();
        field.pos = pos;
        field.length = match self {
            InsertableObject::AlphanumericField => 20,
            InsertableObject::NumericField => 10,
            InsertableObject::DateField => 8,
            InsertableObject::TimeField => 6,
            InsertableObject::BooleanField => 1,
            InsertableObject::Literal | InsertableObject::ProtectedLiteral => 20,
            InsertableObject::Group => 1,
            InsertableObject::Line | InsertableObject::Box => 40,
        };
        field.name = match self {
            InsertableObject::AlphanumericField => "ALNUM_FIELD".to_string(),
            InsertableObject::NumericField => "NUM_FIELD".to_string(),
            InsertableObject::DateField => "DATE_FIELD".to_string(),
            InsertableObject::TimeField => "TIME_FIELD".to_string(),
            InsertableObject::BooleanField => "BOOL_FIELD".to_string(),
            InsertableObject::Literal => "LITERAL".to_string(),
            InsertableObject::ProtectedLiteral => "PROT_LITERAL".to_string(),
            InsertableObject::Group => "GROUP".to_string(),
            InsertableObject::Line => "HLINE".to_string(),
            InsertableObject::Box => "BOX".to_string(),
        };
        field.field_type = FieldType::Field;
        field.attrb = match self {
            InsertableObject::ProtectedLiteral => vec![FieldAttribute::Prot],
            InsertableObject::NumericField => vec![FieldAttribute::Num],
            InsertableObject::DateField => vec![FieldAttribute::Date],
            InsertableObject::TimeField => vec![FieldAttribute::Time],
            InsertableObject::BooleanField => vec![FieldAttribute::Bool],
            _ => vec![FieldAttribute::Norm],
        };
        field.pic = match self {
            InsertableObject::NumericField => Some("9(10)".to_string()),
            InsertableObject::DateField => Some("X(8)".to_string()),
            InsertableObject::TimeField => Some("X(6)".to_string()),
            InsertableObject::BooleanField => Some("X(1)".to_string()),
            _ => None,
        };
        field
    }
}

/// Section de la sidebar active
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SidebarSection {
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
struct App {
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
    selected_map_type: Option<FieldType>,
    // Pour le mode properties
    property_index: usize,
    // Pour le mode insert position
    pending_object: Option<InsertableObject>,
    pending_position: (u16, u16),
    // Pour le mode edit properties
    edit_properties_field: Option<BmsField>,
    edit_properties_index: usize,
    // Pour le mode color picker
    selected_color: Option<BmsColor>,
    // Pour le mode attribute picker
    selected_attribute: Option<FieldAttribute>,
    // Pour le mode save
    save_path: String,
    // Pour le mode open
    open_path: String,
    // Pour le mode add object
    selected_object_for_add: Option<InsertableObject>,
    // Pour le mode text input
    text_input_prompt: String,
    text_input_value: String,
    text_input_action: Option<TextInputAction>,
    // Pour le mode confirm
    confirm_action: ConfirmAction,
    // Mouse support
    mouse_dragging: bool,
    mouse_anchor: Option<(u16, u16)>,
}

/// Action to perform after text input is submitted
#[derive(Debug, Clone)]
enum TextInputAction {
    /// Set the initial value of the selected field
    SetFieldInitial,
    /// Set the PIC value of the selected field
    SetFieldPic,
    /// Set the name of the selected field
    SetFieldName,
    /// No action (generic text input)
    Custom(String),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum ConfirmAction {
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
            selected_color: None,
            selected_attribute: None,
            save_path: String::new(),
            open_path: String::new(),
            selected_object_for_add: None,
            text_input_prompt: String::new(),
            text_input_value: String::new(),
            text_input_action: None,
            confirm_action: ConfirmAction::QuitWithoutSave,
            mouse_dragging: false,
            mouse_anchor: None,
        }
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
        self.message_timeout = Some(60);
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
        
        terminal.draw(|f| ui(f, &app))?;
        
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
    
    // Handle Ctrl+Shift+P for toggle preview
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
                    app.save_path = String::new(); // Reuse save_path for open path
                    app.set_message("Open file - Enter path");
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
        AppMode::Confirm => handle_confirm_mode(app, key),
        AppMode::Normal => handle_normal_mode(app, key),
    }
}

/// Handle mouse input for field selection and drag selection
fn handle_mouse_input(app: &mut App, mouse_event: MouseEvent) {
    // Only handle mouse events in Edit mode when Canvas is active
    if app.mode != AppMode::Edit || app.active_panel != ActivePanel::Canvas {
        return;
    }
    
    match mouse_event.kind {
        MouseEventKind::Down(button) => {
            if button == MouseButton::Left {
                // Store the anchor position for potential drag selection
                app.mouse_anchor = Some((mouse_event.column, mouse_event.row));
                app.mouse_dragging = true;
                
                // Try to select the field at the clicked position
                // Note: mouse coordinates are 0-indexed, BMS coordinates are 1-indexed
                let pos = (mouse_event.row.saturating_add(1), mouse_event.column.saturating_add(1));
                if let Some(field_idx) = app.editor.field_at(pos) {
                    // If Shift is being held, extend the selection
                    // For now, just select the field (we'll check for Shift in key modifiers separately)
                    // Since mouse events don't have modifier info in crossterm 0.27,
                    // we'll use a simple click for single selection
                    app.editor.select_field(field_idx);
                    app.editor.cursor_pos = app.editor.map.fields[field_idx].pos;
                    app.set_message(&format!("Selected field {}", field_idx));
                } else {
                    // Clicked on empty space - clear selection and move cursor
                    app.editor.selected_field = None;
                    app.editor.selected_fields.clear();
                    app.editor.cursor_pos = pos;
                }
            } else if button == MouseButton::Right {
                // Right-click: select field and show properties (or context menu in future)
                let pos = (mouse_event.row.saturating_add(1), mouse_event.column.saturating_add(1));
                if let Some(field_idx) = app.editor.field_at(pos) {
                    app.editor.select_field(field_idx);
                    // In the future, we could show a context menu here
                    app.set_message(&format!("Right-clicked field {}", field_idx));
                }
            }
        }
        MouseEventKind::Up(button) => {
            if button == MouseButton::Left {
                app.mouse_dragging = false;
                app.mouse_anchor = None;
            }
        }
        MouseEventKind::Drag(button) => {
            if button == MouseButton::Left && app.mouse_dragging {
                // Drag selection - extend selection to current position
                if let Some(anchor) = app.mouse_anchor {
                    let current_pos = (mouse_event.column, mouse_event.row);
                    
                    // Convert to 1-indexed BMS coordinates
                    let anchor_bms = (anchor.1.saturating_add(1), anchor.0.saturating_add(1));
                    let current_bms = (current_pos.1.saturating_add(1), current_pos.0.saturating_add(1));
                    
                    // Find fields at both positions
                    if let Some(_anchor_idx) = app.editor.field_at(anchor_bms) {
                        if let Some(current_idx) = app.editor.field_at(current_bms) {
                            // Extend selection from anchor to current field
                            app.editor.extend_selection_to(current_idx);
                            app.set_message(&format!("Selected {} field(s)", app.editor.selected_count()));
                        }
                    }
                }
            }
        }
        MouseEventKind::ScrollDown | MouseEventKind::ScrollUp => {
            // Handle scroll wheel
            if mouse_event.kind == MouseEventKind::ScrollDown {
                app.scroll_down();
            } else {
                app.scroll_up();
            }
        }
        _ => {}
    }
}

fn handle_edit_mode(app: &mut App, key: event::KeyEvent) {
    // F9 no longer used - replaced by Ctrl+Alt+P in handle_input
    
    // Handle Alt+Arrow and Ctrl+Arrow keys for navigation
    // Note: Alt+Left/Right and Ctrl+Left/Right are captured by VSCode, but work in native terminals
    if key.modifiers.contains(KeyModifiers::ALT) || key.modifiers.contains(KeyModifiers::CONTROL) {
        let is_alt = key.modifiers.contains(KeyModifiers::ALT);
        let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        
        // Only handle if exactly one modifier is pressed (Alt OR Ctrl, not both)
        if is_alt != is_ctrl {
            match key.code {
                KeyCode::Up => {
                    if app.active_panel == ActivePanel::Canvas {
                        app.editor.move_cursor(CursorDirection::Up, 5);
                        app.editor.select_field_at(app.editor.cursor_pos);
                        return;
                    }
                }
                KeyCode::Down => {
                    if app.active_panel == ActivePanel::Canvas {
                        app.editor.move_cursor(CursorDirection::Down, 5);
                        app.editor.select_field_at(app.editor.cursor_pos);
                        return;
                    }
                }
                KeyCode::Left => {
                    if app.active_panel == ActivePanel::Canvas {
                        app.editor.select_prev_field();
                        if let Some(idx) = app.editor.selected_field {
                            let field = &app.editor.map.fields[idx];
                            app.editor.cursor_pos = field.pos;
                        }
                        return;
                    }
                }
                KeyCode::Right => {
                    if app.active_panel == ActivePanel::Canvas {
                        app.editor.select_next_field();
                        if let Some(idx) = app.editor.selected_field {
                            let field = &app.editor.map.fields[idx];
                            app.editor.cursor_pos = field.pos;
                        }
                        return;
                    }
                }
                _ => {}
            }
        }
    }
    
    // Handle Shift+Arrow for multi-selection (range selection)
    if key.modifiers.contains(KeyModifiers::SHIFT) && !key.modifiers.contains(KeyModifiers::CONTROL) && !key.modifiers.contains(KeyModifiers::ALT) {
        if app.active_panel == ActivePanel::Canvas {
            // Remember the current anchor point for range selection
            let anchor_idx = app.editor.selected_field;
            
            match key.code {
                KeyCode::Up => {
                    app.editor.move_cursor(CursorDirection::Up, 1);
                }
                KeyCode::Down => {
                    app.editor.move_cursor(CursorDirection::Down, 1);
                }
                KeyCode::Left => {
                    app.editor.move_cursor(CursorDirection::Left, 1);
                }
                KeyCode::Right => {
                    app.editor.move_cursor(CursorDirection::Right, 1);
                }
                _ => {}
            }
            
            // Extend selection to the field at the new cursor position
            if let Some(new_idx) = app.editor.field_at(app.editor.cursor_pos) {
                if let Some(anchor_idx) = anchor_idx {
                    // Ensure selected_fields is initialized with anchor
                    if app.editor.selected_fields.is_empty() {
                        app.editor.selected_fields = vec![anchor_idx];
                    }
                    app.editor.extend_selection_to(new_idx);
                    app.set_message(&format!("Selected {} field(s)", app.editor.selected_count()));
                } else {
                    // No anchor, just select the field at new position
                    app.editor.select_field_at(app.editor.cursor_pos);
                }
            }
            return;
        }
    }
    
    // Handle special actions (Shift+Enter when supported)
    if key.modifiers.contains(KeyModifiers::SHIFT) && key.code == KeyCode::Enter {
        if app.active_panel == ActivePanel::Sidebar && app.sidebar_section == SidebarSection::Objects {
            // Direct insert in Objects sidebar (Shift+Enter when supported)
            if let Some(selected_idx) = app.sidebar_objects_selected {
                let objects = InsertableObject::all();
                if selected_idx < objects.len() {
                    let obj = objects[selected_idx];
                    let field = obj.create_field(app.editor.cursor_pos);
                    app.editor.map.fields.push(field);
                    app.set_message(&format!("Inserted {}", obj.display()));
                }
            }
            // Also handle pending object (from Enter then confirmation)
            if let Some(obj) = app.pending_object.take() {
                if let Some(pos_idx) = app.sidebar_objects_selected {
                    let objects = InsertableObject::all();
                    if pos_idx < objects.len() {
                        let field = obj.create_field(app.pending_position);
                        app.editor.map.fields.push(field);
                        app.set_message(&format!("Inserted {}", obj.display()));
                    }
                }
            }
        } else if app.active_panel == ActivePanel::Canvas {
            // Open EditProperties on selected field in Canvas
            if let Some(idx) = app.editor.selected_field {
                let field = app.editor.map.fields[idx].clone();
                app.edit_properties_field = Some(field);
                app.edit_properties_index = 0;
                app.mode = AppMode::EditProperties;
                app.set_message("Edit properties - Enter to save");
            }
        }
        return;
    }
    
    match key.code {
        // Navigation
        KeyCode::Char('j') | KeyCode::Down => {
            if app.active_panel == ActivePanel::Canvas {
                app.editor.move_cursor(CursorDirection::Down, 1);
                app.editor.select_field_at(app.editor.cursor_pos);
            } else {
                // Sidebar navigation
                match app.sidebar_section {
                    SidebarSection::Actions => {
                        let actions = SidebarAction::all();
                        if let Some(current) = app.sidebar_actions_selected {
                            let next = (current + 1).min(actions.len().saturating_sub(1));
                            app.sidebar_actions_selected = Some(next);
                        } else if !actions.is_empty() {
                            app.sidebar_actions_selected = Some(0);
                        }
                    }
                    SidebarSection::Objects => {
                        let objects = InsertableObject::all();
                        if let Some(current) = app.sidebar_objects_selected {
                            let next = (current + 1).min(objects.len().saturating_sub(1));
                            app.sidebar_objects_selected = Some(next);
                        } else if !objects.is_empty() {
                            app.sidebar_objects_selected = Some(0);
                        }
                    }
                }
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if app.active_panel == ActivePanel::Canvas {
                app.editor.move_cursor(CursorDirection::Up, 1);
                app.editor.select_field_at(app.editor.cursor_pos);
            } else {
                // Sidebar navigation
                match app.sidebar_section {
                    SidebarSection::Actions => {
                        if let Some(current) = app.sidebar_actions_selected {
                            let prev = current.saturating_sub(1);
                            app.sidebar_actions_selected = if prev > 0 || current == 0 { Some(prev) } else { None };
                        } else {
                            let actions = SidebarAction::all();
                            if !actions.is_empty() {
                                app.sidebar_actions_selected = Some(actions.len() - 1);
                            }
                        }
                    }
                    SidebarSection::Objects => {
                        if let Some(current) = app.sidebar_objects_selected {
                            let prev = current.saturating_sub(1);
                            app.sidebar_objects_selected = if prev > 0 || current == 0 { Some(prev) } else { None };
                        } else {
                            let objects = InsertableObject::all();
                            if !objects.is_empty() {
                                app.sidebar_objects_selected = Some(objects.len() - 1);
                            }
                        }
                    }
                }
            }
        }
        KeyCode::Char('h') | KeyCode::Left => {
            if app.active_panel == ActivePanel::Canvas {
                app.editor.move_cursor(CursorDirection::Left, 1);
                app.editor.select_field_at(app.editor.cursor_pos);
            }
        }
        KeyCode::Char('l') | KeyCode::Right => {
            if app.active_panel == ActivePanel::Canvas {
                app.editor.move_cursor(CursorDirection::Right, 1);
                app.editor.select_field_at(app.editor.cursor_pos);
            }
        }
        
        // Field navigation with Tab/Shift+Tab
        KeyCode::Tab => {
            if app.active_panel == ActivePanel::Canvas {
                app.editor.select_next_field();
                if let Some(idx) = app.editor.selected_field {
                    let field = &app.editor.map.fields[idx];
                    app.editor.cursor_pos = field.pos;
                }
            } else {
                // Toggle between Actions and Objects sections in Sidebar
                app.sidebar_section = app.sidebar_section.next();
                app.sidebar_actions_selected = None;
                app.sidebar_objects_selected = None;
                app.set_message(match app.sidebar_section {
                    SidebarSection::Actions => "Actions section",
                    SidebarSection::Objects => "Objects section",
                });
            }
        }
        KeyCode::BackTab => {
            if app.active_panel == ActivePanel::Canvas {
                app.editor.select_prev_field();
                if let Some(idx) = app.editor.selected_field {
                    let field = &app.editor.map.fields[idx];
                    app.editor.cursor_pos = field.pos;
                }
            }
        }
        
        // Execute selected sidebar action or insert object
        KeyCode::Enter => {
            if app.active_panel == ActivePanel::Sidebar {
                match app.sidebar_section {
                    SidebarSection::Actions => {
                        if let Some(selected_idx) = app.sidebar_actions_selected {
                            let actions = SidebarAction::all();
                            if selected_idx < actions.len() {
                                match actions[selected_idx] {
                                    SidebarAction::Edit => {
                                        if app.editor.selected_field.is_some() {
                                            app.mode = AppMode::Properties;
                                            app.property_index = 0;
                                        }
                                    }
                                    SidebarAction::Delete => {
                                        if app.editor.selected_field.is_some() {
                                            app.mode = AppMode::Confirm;
                                            app.confirm_action = ConfirmAction::DeleteField;
                                        }
                                    }
                                    SidebarAction::Move => {
                                        if let Some(idx) = app.editor.selected_field {
                                            app.editor.drag_start = Some(app.editor.map.fields[idx].pos);
                                            app.editor.mode = EditorMode::MoveField;
                                            app.set_message("Move field - arrows to move, Enter to drop");
                                        }
                                    }
                                    SidebarAction::Resize => {
                                        if let Some(idx) = app.editor.selected_field {
                                            app.editor.drag_start = Some((app.editor.map.fields[idx].pos.0, app.editor.map.fields[idx].pos.1 + app.editor.map.fields[idx].length - 1));
                                            app.editor.mode = EditorMode::ResizeField { direction: ResizeDirection::Right };
                                            app.set_message("Resize field - Left/Right to resize");
                                        }
                                    }
                                    SidebarAction::Color => {
                                        if app.editor.selected_field.is_some() {
                                            app.mode = AppMode::ColorPicker;
                                            app.selected_color = None;
                                        }
                                    }
                                    SidebarAction::Attributes => {
                                        if app.editor.selected_field.is_some() {
                                            app.mode = AppMode::AttributePicker;
                                            app.selected_attribute = None;
                                        }
                                    }
                                    SidebarAction::AddField => {
                                        app.editor.add_field_at_cursor(10);
                                        app.set_message("Added field");
                                        app.show_validation_status();
                                    }
                                    SidebarAction::AddLongField => {
                                        app.editor.add_field_at_cursor(20);
                                        app.set_message("Added long field");
                                        app.show_validation_status();
                                    }
                                    SidebarAction::MapType => {
                                        app.mode = AppMode::MapTypePicker;
                                        app.set_message("Select map type");
                                    }
                                    SidebarAction::PreviewBms => {
                                        app.show_bms_text = !app.show_bms_text;
                                        app.set_message(if app.show_bms_text {
                                            "BMS text preview ON"
                                        } else {
                                            "BMS text preview OFF"
                                        });
                                    }
                                }
                            }
                        }
                    }
                    SidebarSection::Objects => {
                        if let Some(selected_idx) = app.sidebar_objects_selected {
                            let objects = InsertableObject::all();
                            if selected_idx < objects.len() {
                                let obj = objects[selected_idx];
                                app.pending_object = Some(obj);
                                app.pending_position = app.editor.cursor_pos;
                                app.mode = AppMode::InsertPosition;
                                app.set_message("Set position with arrows, Enter to confirm");
                            }
                        }
                    }
                }
            }
        }
        
        // Single-letter shortcuts (kept for workflow compatibility)
        // Navigation and special keys
        KeyCode::Char('?') => app.mode = AppMode::Help,
        KeyCode::Char(' ') => app.mode = AppMode::Normal,
        
        // Legacy field add (a/A)
        KeyCode::Char('a') => {
            app.editor.add_field_at_cursor(10);
            app.set_message("Added field [a]");
            app.show_validation_status();
        }
        KeyCode::Char('A') => {
            app.editor.add_field_at_cursor(20);
            app.set_message("Added long field [A]");
            app.show_validation_status();
        }
        
        // New map commands
        KeyCode::Char('n') => {
            app.editor.new_map("NEWMAP", "DEFAULT", (24, 80));
            app.current_file = None;
            app.set_message("New map created");
        }
        KeyCode::Char('N') => {
            let default_map = create_default_map("TEMPLATE", "DEFAULT");
            app.editor = BmsEditor::from_map(default_map);
            app.current_file = None;
            app.set_message("Template map loaded");
        }
        
        // Properties
        KeyCode::Char('e') => {
            if app.editor.selected_field.is_some() {
                app.mode = AppMode::Properties;
                app.property_index = 0;
            }
        }
        KeyCode::Char('C') => {
            if let Some(idx) = app.editor.selected_field {
                app.mode = AppMode::ColorPicker;
                app.selected_color = app.editor.map.fields[idx].color.clone();
            }
        }
        KeyCode::Char('t') => {
            if app.editor.selected_field.is_some() {
                app.mode = AppMode::AttributePicker;
                app.selected_attribute = None;
            }
        }
        
        // Clipboard
        KeyCode::Char('c') => {
            let count = app.editor.selected_count();
            app.editor.copy_selected_fields();
            app.set_message(&format!("Copied {} field(s)", count));
        }
        KeyCode::Char('x') => {
            if app.editor.cut_selected().is_some() {
                app.set_message("Cut");
            }
        }
        KeyCode::Char('v') => {
            if let Some(_first_idx) = app.editor.paste_at_cursor() {
                let count = app.editor.clipboard_count();
                app.set_message(&format!("Pasted {} field(s)", count));
                app.show_validation_status();
            }
        }
        
        // Generate COBOL (legacy)
        KeyCode::Char('g') => {
            let cobol = generate_cobol(&app.editor.map);
            let path = app.current_file.as_ref()
                .map(|p| p.with_extension("cbl"))
                .unwrap_or_else(|| PathBuf::from("output.cbl"));
            if let Err(e) = fs::write(&path, cobol) {
                app.set_message(&format!("Failed: {}", e));
            } else {
                app.set_message(&format!("Generated: {}", path.display()));
            }
        }
        
        // Scroll with capital J/K
        KeyCode::Char('J') => app.scroll_down(),
        KeyCode::Char('K') => app.scroll_up(),
        
        // Exit
        KeyCode::Esc => {
            if app.is_modified() {
                app.mode = AppMode::Confirm;
                app.confirm_action = ConfirmAction::QuitWithoutSave;
            } else {
                app.exit = true;
            }
        }
        
        // Other letters are inert in Edit mode
        // Use Ctrl-based shortcuts for field operations (Ctrl+D, Ctrl+M, Ctrl+R)
        
        _ => {}
    }
}

fn handle_properties_mode(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Esc => app.mode = AppMode::Edit,
        KeyCode::Up => {
            if app.property_index > 0 {
                app.property_index -= 1;
            }
        }
        KeyCode::Down => {
            app.property_index += 1;
        }
        KeyCode::Char('+') | KeyCode::Right => {
            if let Some(idx) = app.editor.selected_field {
                match app.property_index {
                    0 => app.editor.map.fields[idx].pos.1 += 1, // Column
                    1 => app.editor.map.fields[idx].pos.0 += 1, // Row
                    2 => app.editor.map.fields[idx].length += 1, // Length
                    3 => { // Color
                        app.mode = AppMode::ColorPicker;
                        app.selected_color = app.editor.map.fields[idx].color.clone();
                        return;
                    }
                    4 => { // Attributes
                        app.mode = AppMode::AttributePicker;
                        return;
                    }
                    5 => { // INITIAL - open text input
                        let initial = app.editor.map.fields[idx].initial.clone().unwrap_or_default();
                        app.start_text_input("Enter INITIAL value:", &initial, TextInputAction::SetFieldInitial);
                        return;
                    }
                    6 => { // PIC - open text input
                        let pic = app.editor.map.fields[idx].pic.clone().unwrap_or_default();
                        app.start_text_input("Enter PIC value:", &pic, TextInputAction::SetFieldPic);
                        return;
                    }
                    7 => { // Name - open text input
                        let name = app.editor.map.fields[idx].name.clone();
                        app.start_text_input("Enter field name:", &name, TextInputAction::SetFieldName);
                        return;
                    }
                    _ => {}
                }
            }
        }
        KeyCode::Char('-') | KeyCode::Left => {
            if let Some(idx) = app.editor.selected_field {
                match app.property_index {
                    0 => { // Column
                        if app.editor.map.fields[idx].pos.1 > 1 {
                            app.editor.map.fields[idx].pos.1 -= 1;
                        }
                    }
                    1 => { // Row
                        if app.editor.map.fields[idx].pos.0 > 1 {
                            app.editor.map.fields[idx].pos.0 -= 1;
                        }
                    }
                    2 => { // Length
                        if app.editor.map.fields[idx].length > 1 {
                            app.editor.map.fields[idx].length -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        KeyCode::Enter => {
            if let Some(idx) = app.editor.selected_field {
                match app.property_index {
                    5 => { // INITIAL - open text input
                        let initial = app.editor.map.fields[idx].initial.clone().unwrap_or_default();
                        app.start_text_input("Enter INITIAL value:", &initial, TextInputAction::SetFieldInitial);
                        return;
                    }
                    6 => { // PIC - open text input
                        let pic = app.editor.map.fields[idx].pic.clone().unwrap_or_default();
                        app.start_text_input("Enter PIC value:", &pic, TextInputAction::SetFieldPic);
                        return;
                    }
                    7 => { // Name - open text input
                        let name = app.editor.map.fields[idx].name.clone();
                        app.start_text_input("Enter field name:", &name, TextInputAction::SetFieldName);
                        return;
                    }
                    _ => app.mode = AppMode::Edit,
                }
            } else {
                app.mode = AppMode::Edit;
            }
        }
        _ => {}
    }
}

fn handle_insert_position_mode(app: &mut App, key: event::KeyEvent) {
    // Handle Enter for confirmation (terminal doesn't support Shift+Enter detection)
    if key.code == KeyCode::Enter {
        let obj = if let Some(obj) = app.pending_object.take() {
            Some(obj)
        } else if app.active_panel == ActivePanel::Sidebar {
            // Fallback: try to get object from sidebar selection
            app.sidebar_objects_selected.and_then(|idx| {
                InsertableObject::all().get(idx).cloned()
            })
        } else {
            None
        };
        
        if let Some(obj) = obj {
            // Check if position is valid before inserting
            let field_length = obj.default_length();
            if !app.is_valid_field_position(app.pending_position, field_length) {
                app.set_message(&format!("Cannot insert: Invalid position ({},{}) for {}", 
                    app.pending_position.0, app.pending_position.1, obj.display()));
                // Keep pending_object for retry
                app.pending_object = Some(obj);
                return;
            }
            
            let field = obj.create_field(app.pending_position);
            app.editor.map.fields.push(field);
            app.mode = AppMode::Edit;
            app.pending_object = None;
            app.sidebar_objects_selected = None;
            app.active_panel = ActivePanel::Canvas;
            app.set_message(&format!("Inserted {}", obj.display()));
        } else {
            app.set_message("No object selected!");
        }
        return;
    }
    
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.pending_object = None;
            app.sidebar_objects_selected = None;
        }
        KeyCode::Up => {
            if app.pending_position.0 > 1 {
                app.pending_position.0 -= 1;
            }
        }
        KeyCode::Down => {
            app.pending_position.0 += 1;
        }
        KeyCode::Left => {
            if app.pending_position.1 > 1 {
                app.pending_position.1 -= 1;
            }
        }
        KeyCode::Right => {
            app.pending_position.1 += 1;
        }
        _ => {}
    }
}

fn handle_edit_properties_mode(app: &mut App, key: event::KeyEvent) {
    // Handle Enter for saving
    if key.code == KeyCode::Enter {
        if let Some(field) = app.edit_properties_field.take() {
            if let Some(idx) = app.editor.selected_field {
                app.editor.map.fields[idx] = field;
                app.mode = AppMode::Edit;
                app.set_message("Properties saved");
            }
        }
        return;
    }
    
    if let Some(field) = app.edit_properties_field.as_mut() {
        match key.code {
            KeyCode::Esc => {
                app.mode = AppMode::Edit;
                app.edit_properties_field = None;
            }
            KeyCode::Up => {
                if app.edit_properties_index > 0 {
                    app.edit_properties_index -= 1;
                }
            }
            KeyCode::Down => {
                app.edit_properties_index += 1;
            }
            KeyCode::Char('+') | KeyCode::Right => {
                match app.edit_properties_index {
                    0 => field.pos.1 += 1, // Column
                    1 => field.pos.0 += 1, // Row
                    2 => field.length += 1, // Length
                    3 => { // Value/Initial
                        field.initial = Some(field.initial.clone().unwrap_or_default() + "+");
                    }
                    4 => { // Type
                        field.field_type = match field.field_type {
                            FieldType::Field => FieldType::Literal,
                            FieldType::Literal => FieldType::Group,
                            FieldType::Group => FieldType::Map,
                            FieldType::Map => FieldType::Field,
                            _ => field.field_type.clone(),
                        };
                    }
                    _ => {}
                }
            }
            KeyCode::Char('-') | KeyCode::Left => {
                match app.edit_properties_index {
                    0 => {
                        if field.pos.1 > 1 {
                            field.pos.1 -= 1;
                        }
                    }
                    1 => {
                        if field.pos.0 > 1 {
                            field.pos.0 -= 1;
                        }
                    }
                    2 => {
                        if field.length > 1 {
                            field.length -= 1;
                        }
                    }
                    3 => {
                        if let Some(val) = field.initial.as_mut() {
                            val.pop();
                        }
                    }
                    4 => {
                        field.field_type = match field.field_type {
                            FieldType::Field => FieldType::Map,
                            FieldType::Literal => FieldType::Field,
                            FieldType::Group => FieldType::Literal,
                            FieldType::Map => FieldType::Group,
                            _ => field.field_type.clone(),
                        };
                    }
                    _ => {}
                }
            }
            KeyCode::Enter => {
                // Regular Enter exits without saving
                app.mode = AppMode::Edit;
                app.edit_properties_field = None;
            }
            _ => {}
        }
    }
}

fn handle_map_type_picker_mode(app: &mut App, key: event::KeyEvent) {
    use cobol_bms_core::FieldType;
    
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.selected_map_type = None;
        }
        KeyCode::Enter => {
            if let Some(map_type) = app.selected_map_type.clone() {
                app.editor.map.map_type = map_type.clone();
                app.mode = AppMode::Edit;
                app.selected_map_type = None;
                app.set_message(&format!("Map type set to: {:?}", map_type));
            }
        }
        KeyCode::Up => {
            let all_types = get_scrollable_map_types();
            if !all_types.is_empty() {
                let new_selection = if let Some(current) = &app.selected_map_type {
                    if let Some(pos) = all_types.iter().position(|t| t == current) {
                        if pos > 0 {
                            Some(all_types[pos - 1].clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    Some(all_types[all_types.len() - 1].clone())
                };
                if let Some(new_type) = new_selection {
                    app.selected_map_type = Some(new_type);
                }
            }
        }
        KeyCode::Down => {
            let all_types = get_scrollable_map_types();
            if !all_types.is_empty() {
                let new_selection = if let Some(current) = &app.selected_map_type {
                    if let Some(pos) = all_types.iter().position(|t| t == current) {
                        if pos + 1 < all_types.len() {
                            Some(all_types[pos + 1].clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    Some(all_types[0].clone())
                };
                if let Some(new_type) = new_selection {
                    app.selected_map_type = Some(new_type);
                }
            }
        }
        KeyCode::Char('M') => app.selected_map_type = Some(FieldType::Map),
        KeyCode::Char('S') => app.selected_map_type = Some(FieldType::DFHMSD),
        KeyCode::Char('D') => app.selected_map_type = Some(FieldType::DFHMDF),
        KeyCode::Char('I') => app.selected_map_type = Some(FieldType::DFHMDI),
        _ => {}
    }
}

/// Return scrollable map types
fn get_scrollable_map_types() -> &'static [FieldType] {
    use cobol_bms_core::FieldType;
    &[
        FieldType::Map,
        FieldType::DFHMSD,
        FieldType::DFHMDF,
        FieldType::DFHMDI,
    ]
}

fn handle_color_picker_mode(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.selected_color = None;
        }
        KeyCode::Enter => {
            if let Some(color) = app.selected_color.clone() {
                if app.editor.selected_field.is_some() {
                    app.editor.set_selected_field_color(Some(color));
                }
            }
            app.mode = AppMode::Edit;
            app.selected_color = None;
        }
        KeyCode::Char('b') => app.selected_color = Some(BmsColor::Blue),
        KeyCode::Char('g') => app.selected_color = Some(BmsColor::Green),
        KeyCode::Char('r') => app.selected_color = Some(BmsColor::Red),
        KeyCode::Char('y') => app.selected_color = Some(BmsColor::Yellow),
        KeyCode::Char('w') => app.selected_color = Some(BmsColor::White),
        KeyCode::Char('c') => app.selected_color = Some(BmsColor::Cyan),
        KeyCode::Char('m') => app.selected_color = Some(BmsColor::Magenta),
        KeyCode::Char('k') => app.selected_color = Some(BmsColor::Black),
        KeyCode::Char('o') => app.selected_color = Some(BmsColor::Orange),
        KeyCode::Char('p') => app.selected_color = Some(BmsColor::Pink),
        KeyCode::Char(' ') => app.selected_color = None,
        _ => {}
    }
}

fn handle_attribute_picker_mode(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.selected_attribute = None;
        }
        KeyCode::Enter => {
            if let Some(attr) = app.selected_attribute.clone() {
                if app.editor.selected_field.is_some() {
                    app.editor.add_selected_field_attribute(attr);
                }
            }
            app.mode = AppMode::Edit;
            app.selected_attribute = None;
        }
        KeyCode::Char('p') => app.selected_attribute = Some(FieldAttribute::Prot),
        KeyCode::Char('n') => app.selected_attribute = Some(FieldAttribute::Norm),
        KeyCode::Char('u') => app.selected_attribute = Some(FieldAttribute::Num),
        KeyCode::Char('a') => app.selected_attribute = Some(FieldAttribute::Alph),
        KeyCode::Char('l') => app.selected_attribute = Some(FieldAttribute::AlphaNum),
        KeyCode::Char('i') => app.selected_attribute = Some(FieldAttribute::Intens),
        KeyCode::Char('b') => app.selected_attribute = Some(FieldAttribute::Blink),
        KeyCode::Char('v') => app.selected_attribute = Some(FieldAttribute::Reverse),
        KeyCode::Char('d') => app.selected_attribute = Some(FieldAttribute::Dark),
        _ => {}
    }
}

fn handle_save_dialog_mode(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Esc => app.mode = AppMode::Edit,
        KeyCode::Enter => {
            // Validate before saving
            let errors = app.editor.map.validate();
            if !errors.is_empty() {
                app.set_message(&format!("Cannot save: {}", errors.join("; ")));
                return;
            }
            let path = PathBuf::from(&app.save_path);
            match fs::write(&path, app.editor.export_to_bms()) {
                Ok(_) => {
                    app.current_file = Some(path.clone());
                    app.mode = AppMode::Edit;
                    app.set_message(&format!("Saved: {}", path.display()));
                }
                Err(e) => {
                    app.set_message(&format!("Failed: {}", e));
                }
            }
        }
        KeyCode::Backspace => {
            app.save_path.pop();
        }
        KeyCode::Char(c) => {
            app.save_path.push(c);
        }
        _ => {}
    }
}

fn handle_open_dialog_mode(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.open_path.clear();
        }
        KeyCode::Enter => {
            let path = PathBuf::from(&app.open_path);
            if path.exists() {
                match parse_bms_file(path.to_str().unwrap()) {
                    Ok(map) => {
                        app.editor = BmsEditor::from_map(map);
                        app.current_file = Some(path.clone());
                        app.mode = AppMode::Edit;
                        app.open_path.clear();
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
        KeyCode::Backspace => {
            app.open_path.pop();
        }
        KeyCode::Char(c) => {
            app.open_path.push(c);
        }
        _ => {}
    }
}

fn handle_add_object_dialog_mode(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.selected_object_for_add = None;
        }
        KeyCode::Enter => {
            if let Some(obj) = app.selected_object_for_add {
                // Add object at cursor position
                let field = obj.create_field(app.editor.cursor_pos);
                app.editor.map.fields.push(field);
                app.mode = AppMode::Edit;
                app.selected_object_for_add = None;
                app.set_message(&format!("Inserted {}", obj.display()));
                app.show_validation_status();
            }
        }
        KeyCode::Up => {
            let objects = InsertableObject::all();
            if let Some(current_idx) = app.selected_object_for_add.and_then(|obj| {
                objects.iter().position(|&o| o == obj)
            }) {
                if current_idx > 0 {
                    app.selected_object_for_add = Some(objects[current_idx - 1]);
                } else {
                    app.selected_object_for_add = Some(objects[objects.len() - 1]);
                }
            } else if !objects.is_empty() {
                app.selected_object_for_add = Some(objects[objects.len() - 1]);
            }
        }
        KeyCode::Down => {
            let objects = InsertableObject::all();
            if let Some(current_idx) = app.selected_object_for_add.and_then(|obj| {
                objects.iter().position(|&o| o == obj)
            }) {
                if current_idx + 1 < objects.len() {
                    app.selected_object_for_add = Some(objects[current_idx + 1]);
                } else {
                    app.selected_object_for_add = Some(objects[0]);
                }
            } else if !objects.is_empty() {
                app.selected_object_for_add = Some(objects[0]);
            }
        }
        _ => {}
    }
}

fn handle_text_input_mode(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.text_input_prompt.clear();
            app.text_input_value.clear();
            app.text_input_action = None;
        }
        KeyCode::Enter => {
            let value = std::mem::take(&mut app.text_input_value);
            app.apply_text_input(value);
            app.mode = AppMode::Edit;
            app.text_input_prompt.clear();
        }
        KeyCode::Backspace => {
            app.text_input_value.pop();
        }
        KeyCode::Char(c) => {
            app.text_input_value.push(c);
        }
        _ => {}
    }
}

fn handle_help_mode(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app.mode = AppMode::Edit,
        _ => {}
    }
}

fn handle_confirm_mode(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Enter => {
            match app.confirm_action {
                ConfirmAction::QuitWithoutSave => app.exit = true,
                ConfirmAction::DeleteField => {
                    if app.editor.remove_selected_field().is_some() {
                        app.set_message("Field deleted");
                        app.show_validation_status();
                    }
                    app.mode = AppMode::Edit;
                }
                ConfirmAction::ClearMap => {
                    app.editor.map.fields.clear();
                    app.editor.selected_field = None;
                    app.set_message("Map cleared");
                    app.mode = AppMode::Edit;
                }
            }
        }
        KeyCode::Char('n') | KeyCode::Esc => app.mode = AppMode::Edit,
        _ => {}
    }
}

fn handle_normal_mode(app: &mut App, key: event::KeyEvent) {
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
        _ => handle_edit_mode(app, key),
    }
}

// ==================== UI ====================

fn ui(f: &mut Frame, app: &App) {
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
        AppMode::Confirm => " CONFIRM ",
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
        AppMode::Confirm => {
            render_confirm(f, app, content_area);
        }
    }
    
    // Status bar
    render_status_bar(f, app, main_layout[2]);
}

fn render_canvas(f: &mut Frame, app: &App, area: Rect) {
    let canvas_width = area.width.saturating_sub(25);
    let canvas_area = Rect {
        x: area.x,
        y: area.y,
        width: canvas_width,
        height: area.height,
    };
    
    // Draw border
    let canvas_title = match app.active_panel {
        ActivePanel::Canvas => format!(" [>] Canvas ({}x{}) [Ctrl+P:Toggle|Tab:Next|Shift+Tab:Prev|Alt/Ctrl+Arrows:Nav|Ctrl+Shift+P:Preview]", app.editor.map.size.0, app.editor.map.size.1),
        ActivePanel::Sidebar => format!(" Canvas ({}x{}) [Ctrl+P:Toggle|Tab:Next|Shift+Tab:Prev|Alt/Ctrl+Arrows:Nav|Ctrl+Shift+P:Preview]", app.editor.map.size.0, app.editor.map.size.1),
    };
    
    // Couleur du cadre en fonction de l'activation
    let border_color = match app.active_panel {
        ActivePanel::Canvas => TuiColor::Yellow,
        ActivePanel::Sidebar => TuiColor::White,
    };
    
    let canvas_block = Block::default()
        .title(canvas_title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
    f.render_widget(canvas_block, canvas_area);
    
    // Render content based on mode
    let content_area = Rect {
        x: canvas_area.x + 1,
        y: canvas_area.y + 1,
        width: canvas_area.width.saturating_sub(2),
        height: canvas_area.height.saturating_sub(2),
    };
    
    if app.show_bms_text {
        render_bms_text_preview(f, app, content_area);
    } else {
        render_bms_grid(f, app, content_area);
    }
}

fn render_bms_grid(f: &mut Frame, app: &App, area: Rect) {
    let map = &app.editor.map;
    
    // Build list of fields to display (including preview field)
    let mut fields_to_render: Vec<(BmsField, bool)> = map.fields.iter().map(|f| (f.clone(), false)).collect();
    
    // Add preview field for InsertPosition mode
    if let Some(obj) = app.pending_object {
        let preview_field = obj.create_field(app.pending_position);
        fields_to_render.push((preview_field, true));
    }
    
    // Add preview field for EditProperties mode
    if let Some(edit_field) = &app.edit_properties_field {
        fields_to_render.push((edit_field.clone(), true));
    }
    
    // Create a grid based on the visible area
    let visible_rows = area.height as usize;
    let visible_cols = area.width as usize;
    
    let start_row = app.scroll as usize;
    let end_row = (start_row + visible_rows).min(map.size.0 as usize);
    
    for grid_row in start_row..end_row {
        let mut spans = Vec::<Span>::new();
        
        for col in 1..=visible_cols {
            let mut c = ' ';
            let mut style = Style::default();
            let mut is_selected = false;
            
            // Check if any field covers this cell
            for (field, is_preview) in &fields_to_render {
                let (field_row, field_col) = field.pos;
                let field_row = field_row as usize;
                let field_col = field_col as usize;
                let field_end_col = field_col + field.length as usize - 1;
                
                if grid_row + 1 == field_row && col >= field_col && col <= field_end_col {
                    // Determine the character based on field type and position within field
                    let field_start = field_col;
                    let field_end = field_end_col;
                    let is_first_col = col == field_start;
                    let is_last_col = col == field_end;
                    
                    c = if *is_preview {
                        // Preview fields use special characters
                        if is_first_col {
                            '['
                        } else if is_last_col {
                            ']'
                        } else {
                            '-'
                        }
                    } else if is_selected {
                        // Selected fields use filled block for better visibility
                        if is_first_col {
                            '█'
                        } else if is_last_col {
                            '█'
                        } else {
                            '█'
                        }
                    } else {
                        // Regular fields based on type - use horizontal line characters
                        match field.field_type {
                            FieldType::Map => {
                                if is_first_col { '┏' } else if is_last_col { '┓' } else { '━' }
                            }
                            FieldType::Field => {
                                if field.attrb.contains(&FieldAttribute::Prot) {
                                    if is_first_col { '╭' } else if is_last_col { '╮' } else { '─' }
                                } else if field.attrb.contains(&FieldAttribute::Num) {
                                    if is_first_col { '[' } else if is_last_col { ']' } else { '═' }
                                } else if field.attrb.contains(&FieldAttribute::Alph) || field.attrb.contains(&FieldAttribute::AlphaNum) {
                                    if is_first_col { '⟦' } else if is_last_col { '⟧' } else { '─' }
                                } else {
                                    if is_first_col { '┌' } else if is_last_col { '┐' } else { '─' }
                                }
                            }
                            FieldType::Literal => {
                                if is_first_col { '«' } else if is_last_col { '»' } else { '─' }
                            }
                            FieldType::Group => {
                                if is_first_col { '┌' } else if is_last_col { '┐' } else { '─' }
                            }
                            _ => {
                                if is_first_col { '[' } else if is_last_col { ']' } else { '-' }
                            }
                        }
                    };
                    
                    // Check if this field is selected
                    if !is_preview {
                        for &selected_idx in &app.editor.selected_fields {
                            if map.fields.get(selected_idx).map_or(false, |f| f.pos == field.pos) {
                                is_selected = true;
                                break;
                            }
                        }
                        // Also check single selected_field
                        if !is_selected {
                            if let Some(selected_idx) = app.editor.selected_field {
                                if map.fields.get(selected_idx).map_or(false, |f| f.pos == field.pos) {
                                    is_selected = true;
                                }
                            }
                        }
                    }
                    
                    // Use special style for preview fields (blinking or different color)
                    if *is_preview {
                        style = style.fg(TuiColor::Cyan).bg(TuiColor::DarkGray);
                    } else if is_selected {
                        style = style.fg(TuiColor::Black).bg(TuiColor::Yellow);
                    } else {
                        match field.color {
                            Some(BmsColor::Blue) => style = style.fg(TuiColor::Blue),
                            Some(BmsColor::Green) => style = style.fg(TuiColor::Green),
                            Some(BmsColor::Red) => style = style.fg(TuiColor::Red),
                            Some(BmsColor::Yellow) => style = style.fg(TuiColor::Yellow),
                            Some(BmsColor::Cyan) => style = style.fg(TuiColor::Cyan),
                            Some(BmsColor::Magenta) => style = style.fg(TuiColor::Magenta),
                            _ => style = style.fg(TuiColor::White),
                        }
                    }
                    break;
                }
            }
            
            spans.push(Span::styled(c.to_string(), style));
        }
        
        // Add cursor indicator if cursor is on this row
        let cursor_row = app.editor.cursor_pos.0 as usize;
        let cursor_col = app.editor.cursor_pos.1 as usize;
        if grid_row + 1 == cursor_row && cursor_col <= visible_cols {
            // Cursor is on this row and within visible columns
            if cursor_col > 0 && cursor_col <= spans.len() {
                // Replace the character at cursor position with cursor indicator
                if let Some(span) = spans.get_mut(cursor_col - 1) {
                    // Use a different character or style for cursor
                    spans[cursor_col - 1] = Span::styled("▮".to_string(), Style::default().fg(TuiColor::White).bg(TuiColor::Red));
                }
            }
        }
        
        let line = Line::from(spans);
        let paragraph = Paragraph::new(line)
            .block(Block::default().borders(Borders::NONE));
        f.render_widget(paragraph, Rect {
            x: area.x,
            y: area.y + (grid_row as u16 - start_row as u16),
            width: area.width,
            height: 1,
        });
    }
    
    // Add scrollbar if needed
    if map.size.0 > visible_rows as u16 {
        // TODO: Fix scrollbar for current ratatui version
        // let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        //     .begin_symbol(Some("UP"))
        //     .end_symbol(Some("DN"))
        //     .track_symbol(Some("|"))
        //     .thumb_symbol(Some("#"))
        //     .position(app.scroll)
        //     .range(0, map.size.0.saturating_sub(visible_rows as u16));
        // f.render_widget(scrollbar, area);
    }
}

fn render_bms_text_preview(f: &mut Frame, app: &App, area: Rect) {
    // Generate BMS text
    let bms_text = render_bms_text(&app.editor.map);
    
    // Create a scrollable text paragraph
    let text = Text::from(bms_text);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE))
        .scroll((app.scroll as u16, 0));
    
    f.render_widget(paragraph, area);
}

fn render_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let panel_area = Rect {
        x: area.x + area.width - 24,
        y: area.y,
        width: 24,
        height: area.height,
    };
    
    let title = match app.active_panel {
        ActivePanel::Sidebar => " [>] Sidebar [Ctrl+Alt+P:Toggle|Tab:Switch]",
        ActivePanel::Canvas => " Sidebar [Ctrl+Alt+P:Toggle|Tab:Switch]",
    };
    
    // Couleur du cadre en fonction de l'activation
    let border_color = match app.active_panel {
        ActivePanel::Sidebar => TuiColor::Yellow,
        ActivePanel::Canvas => TuiColor::White,
    };
    
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
    f.render_widget(block, panel_area);
    
    let inner = Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    };
    
    // Field info
    let mut lines: Vec<Line> = Vec::new();
    
    if let Some(idx) = app.editor.selected_field {
        let field = &app.editor.map.fields[idx];
        lines.push(Line::from(" Selected Field "));
        lines.push(Line::from(""));
        lines.push(Line::from(format!("Name: {}", field.name)));
        lines.push(Line::from(format!("Pos: ({},{})", field.pos.0, field.pos.1)));
        lines.push(Line::from(format!("Len: {}", field.length)));
        lines.push(Line::from(""));
        
        let mut attrs_line = String::new();
        for attr in &field.attrb {
            attrs_line.push_str(&format!("{:?} ", attr));
        }
        lines.push(Line::from(attrs_line));
        lines.push(Line::from(""));
    } else {
        lines.push(Line::from(" No field selected ".dim()));
        lines.push(Line::from(""));
    }
    
    // Section title based on active section
    let section_title = match app.sidebar_section {
        SidebarSection::Actions => "> Actions ",
        SidebarSection::Objects => "> Objects ",
    };
    lines.push(Line::from(section_title));
    lines.push(Line::from(""));
    
    // Render appropriate section
    match app.sidebar_section {
        SidebarSection::Actions => {
            // Render sidebar actions with selection highlight
            let actions = SidebarAction::all();
            for (i, action) in actions.iter().enumerate() {
                let display_text = action.display();
                let style = if app.active_panel == ActivePanel::Sidebar && app.sidebar_actions_selected == Some(i) {
                    Style::default().fg(TuiColor::Black).bg(TuiColor::Yellow)
                } else {
                    Style::default().fg(TuiColor::White)
                };
                lines.push(Line::from(Span::styled(display_text, style)));
            }
            
            // Additional actions not in SidebarAction enum
            if app.editor.selected_field.is_none() {
                lines.push(Line::from(""));
                lines.push(Line::from("n: New map"));
                lines.push(Line::from("N: Template"));
                lines.push(Line::from("v: Paste"));
                lines.push(Line::from("g: Gen COBOL"));
            }
        }
        SidebarSection::Objects => {
            // Render insertable objects with selection highlight
            let objects = InsertableObject::all();
            for (i, obj) in objects.iter().enumerate() {
                let display_text = obj.display();
                let style = if app.active_panel == ActivePanel::Sidebar && app.sidebar_objects_selected == Some(i) {
                    Style::default().fg(TuiColor::Black).bg(TuiColor::Yellow)
                } else {
                    Style::default().fg(TuiColor::White)
                };
                lines.push(Line::from(Span::styled(display_text, style)));
            }
        }
    }
    
    // Help hints
    lines.push(Line::from(""));
    lines.push(Line::from("Ctrl+P: Toggle Canvas/Sidebar".dim()));
    lines.push(Line::from("Tab: Next field / Switch section".dim()));
    lines.push(Line::from("Shift+Tab: Previous field".dim()));
    lines.push(Line::from("Alt/Ctrl+Up/Down: Fast scroll (5 lines)".dim()));
    lines.push(Line::from("Alt/Ctrl+Left/Right: Prev/Next field".dim()));
    lines.push(Line::from("Ctrl+Shift+P: Toggle preview".dim()));
    
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, inner);
}

fn render_properties_panel(f: &mut Frame, app: &App, area: Rect) {
    let panel_width = area.width.min(35);
    let panel_area = Rect {
        x: area.x + area.width - panel_width,
        y: area.y,
        width: panel_width,
        height: area.height.min(15),
    };
    
    let block = Block::default()
        .title(" Properties [Read-only|Esc:Close] ")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    if let Some(idx) = app.editor.selected_field {
        let field = &app.editor.map.fields[idx];
        let lines = vec![
            Line::from("> Position ".yellow()),
            Line::from(format!("  Row: {} ", field.pos.0)),
            Line::from(format!("  Col: {} ", field.pos.1)),
            Line::from(""),
            Line::from(" Size ".yellow()),
            Line::from(format!("  Length: {} ", field.length)),
            Line::from(""),
            Line::from(" Appearance ".yellow()),
            Line::from(format!("  Color: {:?}", field.color)),
            Line::from(format!("  Attrs: {:?}", field.attrb)),
            Line::from(""),
            Line::from(" Type ".yellow()),
            Line::from(format!("  {:?}", field.field_type)),
        ];
        
        let text = Text::from(lines);
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::NONE));
        f.render_widget(paragraph, Rect {
            x: panel_area.x + 1,
            y: panel_area.y + 1,
            width: panel_area.width.saturating_sub(2),
            height: panel_area.height.saturating_sub(2),
        });
    }
}

fn render_insert_position_dialog(f: &mut Frame, app: &App, area: Rect) {
    let panel_width = 30;
    let panel_area = Rect {
        x: area.x + area.width - panel_width,
        y: area.y,
        width: panel_width,
        height: 10,
    };
    
    let block = Block::default()
        .title(" Insert Position [Arrows:Move|Enter:Confirm|Esc:Cancel|Live Preview]")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let inner = Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    };
    
    let obj_name = app.pending_object.map(|o| o.display()).unwrap_or("Object");
    let (row, col) = app.pending_position;
    
    // Check if position is valid
    let is_valid = if let Some(obj) = app.pending_object {
        app.editor.map.is_valid_field_position(app.pending_position, obj.default_length())
    } else {
        false
    };
    
    let validity_text = if is_valid {
        Line::from("Status: Valid".green())
    } else {
        Line::from("Status: INVALID - will not be inserted".red())
    };
    
    let lines = vec![
        Line::from(format!("Object: {}", obj_name)),
        Line::from(""),
        Line::from("Position:".yellow()),
        Line::from(format!("  Row: {}", row)),
        Line::from(format!("  Col: {}", col)),
        Line::from(""),
        validity_text,
        Line::from(""),
        Line::from("Arrows: Move".dim()),
        Line::from("Enter: Confirm".dim()),
        Line::from("Esc: Cancel".dim()),
    ];
    
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, inner);
}

fn render_edit_properties_panel(f: &mut Frame, app: &App, area: Rect) {
    let panel_width = area.width.min(35);
    let panel_area = Rect {
        x: area.x + area.width - panel_width,
        y: area.y,
        width: panel_width,
        height: area.height.min(18),
    };
    
    let block = Block::default()
        .title(" Edit Properties [Up/Down:Nav|+/-:Modify|Enter:Save|Esc:Cancel|Live Preview]")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let inner = Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    };
    
    if let Some(field) = &app.edit_properties_field {
        let mut lines = vec![
            Line::from("> Position ".yellow()),
            Line::from(format!("  Row: {} ", field.pos.0)),
            Line::from(format!("  Col: {} ", field.pos.1)),
            Line::from(""),
            Line::from("> Size ".yellow()),
            Line::from(format!("  Length: {} ", field.length)),
            Line::from(""),
            Line::from("> Value ".yellow()),
            Line::from(format!("  Initial: {} ", field.initial.as_deref().unwrap_or(""))),
            Line::from(""),
            Line::from("> Type ".yellow()),
            Line::from(format!("  {:?}", field.field_type)),
            Line::from(""),
            Line::from("Up/Down: Navigate".dim()),
            Line::from(r#"+/- : Modify"#.dim()),
            Line::from("Enter: Save".dim()),
            Line::from("Esc: Cancel".dim()),
        ];
        
        // Highlight current property
        if app.edit_properties_index < lines.len() {
            if let Some(line) = lines.get_mut(app.edit_properties_index) {
                *line = Line::from(Span::styled(line.spans[0].content.clone(), Style::default().fg(TuiColor::Black).bg(TuiColor::Yellow)));
            }
        }
        
        let text = Text::from(lines);
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::NONE));
        f.render_widget(paragraph, inner);
    }
}

fn render_map_type_picker(f: &mut Frame, app: &App, area: Rect) {
    use cobol_bms_core::FieldType;
    
    let panel_width = 25;
    let panel_area = Rect {
        x: area.x + area.width - panel_width,
        y: area.y,
        width: panel_width,
        height: 12,
    };
    
    let block = Block::default()
        .title(" Map Type [Up/Down:Nav|M/S/D/I:Select|Enter:Ok|Esc:Cancel]")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let inner = Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    };
    
    let map_types = [
        (FieldType::Map, "Standard MAP", "M"),
        (FieldType::DFHMSD, "Scrollable Data (DFHMSD)", "S"),
        (FieldType::DFHMDF, "Scrollable Formatted (DFHMDF)", "D"),
        (FieldType::DFHMDI, "Scrollable Input (DFHMDI)", "I"),
    ];
    
    let mut lines = vec![Line::from(" Select Map Type ".yellow())];
    for (map_type, name, key) in &map_types {
        let selected_type = app.selected_map_type.as_ref();
        let is_selected = selected_type == Some(map_type);
        let prefix = if is_selected { "> " } else { "  " };
        let style = if is_selected {
            Style::default().fg(TuiColor::Black).bg(TuiColor::Yellow)
        } else {
            Style::default().fg(TuiColor::White)
        };
        lines.push(Line::from(Span::styled(format!("{} {} [{}]", prefix, name, key), style)));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("Enter: Select".dim()));
    lines.push(Line::from("Esc: Cancel".dim()));
    
    // Show current map type
    lines.push(Line::from(""));
    lines.push(Line::from(format!("Current: {:?}", app.editor.map.map_type)).dim());
    
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, inner);
}

fn render_color_picker(f: &mut Frame, app: &App, area: Rect) {
    let panel_width = 28;
    let panel_area = Rect {
        x: area.x + area.width - panel_width,
        y: area.y,
        width: panel_width,
        height: 13,
    };
    
    let block = Block::default()
        .title(" Colors [B/G/R/Y/C/M/W/K/O/P:Select|Space:None|Enter:Apply|Esc:Cancel] ")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let colors = vec![
        (BmsColor::Black, "Black", "K"),
        (BmsColor::Blue, "Blue", "B"),
        (BmsColor::Green, "Green", "G"),
        (BmsColor::Cyan, "Cyan", "C"),
        (BmsColor::Red, "Red", "R"),
        (BmsColor::Magenta, "Magenta", "M"),
        (BmsColor::Yellow, "Yellow", "Y"),
        (BmsColor::White, "White", "W"),
        (BmsColor::Orange, "Orange", "O"),
        (BmsColor::Pink, "Pink", "P"),
    ];
    
    let mut lines = vec![Line::from(" Select: ".yellow())];
    for (color, name, key) in &colors {
        let prefix = if Some(color) == app.selected_color.as_ref() { "> " } else { "  " };
        lines.push(Line::from(format!("{} {} [{}]", prefix, name, key)));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("Space: None".to_string()));
    lines.push(Line::from("Enter: Apply".to_string()));
    
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    });
}

fn render_attribute_picker(f: &mut Frame, app: &App, area: Rect) {
    let panel_width = 30;
    let panel_area = Rect {
        x: area.x + area.width - panel_width,
        y: area.y,
        width: panel_width,
        height: 14,
    };
    
    let block = Block::default()
        .title(" Attributes ")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let attrs = vec![
        (FieldAttribute::Prot, "PROT", "P"),
        (FieldAttribute::Norm, "NORM", "N"),
        (FieldAttribute::Num, "NUM", "U"),
        (FieldAttribute::Alph, "ALPH", "A"),
        (FieldAttribute::AlphaNum, "ALNUM", "L"),
        (FieldAttribute::Intens, "INTENS", "I"),
        (FieldAttribute::Blink, "BLINK", "B"),
        (FieldAttribute::Reverse, "REVERSE", "V"),
        (FieldAttribute::Dark, "DARK", "D"),
    ];
    
    let mut lines = vec![Line::from(" Select: ".yellow())];
    for (attr, name, key) in &attrs {
        let prefix = if Some(attr) == app.selected_attribute.as_ref() { "> " } else { "  " };
        lines.push(Line::from(format!("{} {} [{}]", prefix, name, key)));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("Enter: Add attribute".to_string()));
    
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    });
}

fn render_save_dialog(f: &mut Frame, app: &App, area: Rect) {
    let dialog_width = 40;
    let dialog_height = 5;
    let dialog_area = Rect {
        x: area.x + (area.width.saturating_sub(dialog_width)) / 2,
        y: area.y + (area.height.saturating_sub(dialog_height)) / 2,
        width: dialog_width,
        height: dialog_height,
    };
    
    let block = Block::default()
        .title(" Save File ")
        .borders(Borders::ALL);
    f.render_widget(block, dialog_area);
    
    let inner = Rect {
        x: dialog_area.x + 1,
        y: dialog_area.y + 1,
        width: dialog_area.width.saturating_sub(2),
        height: dialog_area.height.saturating_sub(2),
    };
    
    let prompt = Paragraph::new("File path: ")
        .style(Style::default().fg(TuiColor::Yellow));
    f.render_widget(prompt, Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 });
    
    let path_text = Paragraph::new(app.save_path.as_str())
        .style(Style::default().fg(TuiColor::White));
    f.render_widget(path_text, Rect { x: inner.x, y: inner.y + 1, width: inner.width, height: 1 });
    
    let help = Paragraph::new("Enter: Save | Esc: Cancel")
        .style(Style::default().fg(TuiColor::Cyan));
    f.render_widget(help, Rect { x: inner.x, y: inner.y + 2, width: inner.width, height: 1 });
}

fn render_open_dialog(f: &mut Frame, app: &App, area: Rect) {
    let dialog_width = 40;
    let dialog_height = 5;
    let dialog_area = Rect {
        x: area.x + (area.width.saturating_sub(dialog_width)) / 2,
        y: area.y + (area.height.saturating_sub(dialog_height)) / 2,
        width: dialog_width,
        height: dialog_height,
    };
    
    let block = Block::default()
        .title(" Open File ")
        .borders(Borders::ALL);
    f.render_widget(block, dialog_area);
    
    let inner = Rect {
        x: dialog_area.x + 1,
        y: dialog_area.y + 1,
        width: dialog_area.width.saturating_sub(2),
        height: dialog_area.height.saturating_sub(2),
    };
    
    let prompt = Paragraph::new("File path: ")
        .style(Style::default().fg(TuiColor::Yellow));
    f.render_widget(prompt, Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 });
    
    let path_text = Paragraph::new(app.open_path.as_str())
        .style(Style::default().fg(TuiColor::White));
    f.render_widget(path_text, Rect { x: inner.x, y: inner.y + 1, width: inner.width, height: 1 });
    
    let help = Paragraph::new("Enter: Open | Esc: Cancel")
        .style(Style::default().fg(TuiColor::Cyan));
    f.render_widget(help, Rect { x: inner.x, y: inner.y + 2, width: inner.width, height: 1 });
}

fn render_add_object_dialog(f: &mut Frame, app: &App, area: Rect) {
    let panel_width = 30;
    let panel_area = Rect {
        x: area.x + area.width - panel_width,
        y: area.y,
        width: panel_width,
        height: area.height.min(15),
    };
    
    let block = Block::default()
        .title(" Add Object [Up/Down:Nav|Enter:Select|Esc:Cancel] ")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let inner = Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    };
    
    let objects = InsertableObject::all();
    let mut lines = vec![Line::from(" Select Object Type ".yellow())];
    
    for (_i, obj) in objects.iter().enumerate() {
        let display_text = obj.display();
        let is_selected = app.selected_object_for_add == Some(*obj);
        let prefix = if is_selected { "> " } else { "  " };
        let style = if is_selected {
            Style::default().fg(TuiColor::Black).bg(TuiColor::Yellow)
        } else {
            Style::default().fg(TuiColor::White)
        };
        lines.push(Line::from(Span::styled(format!("{} {}", prefix, display_text), style)));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from("Enter: Select".dim()));
    lines.push(Line::from("Esc: Cancel".dim()));
    
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, inner);
}

fn render_text_input(f: &mut Frame, app: &App, area: Rect) {
    let dialog_width = 50;
    let dialog_height = 5;
    let dialog_area = Rect {
        x: area.x + (area.width.saturating_sub(dialog_width)) / 2,
        y: area.y + (area.height.saturating_sub(dialog_height)) / 2,
        width: dialog_width,
        height: dialog_height,
    };
    
    let block = Block::default()
        .title(" Text Input ")
        .borders(Borders::ALL);
    f.render_widget(block, dialog_area);
    
    let inner = Rect {
        x: dialog_area.x + 1,
        y: dialog_area.y + 1,
        width: dialog_area.width.saturating_sub(2),
        height: dialog_area.height.saturating_sub(2),
    };
    
    let prompt = Paragraph::new(app.text_input_prompt.as_str())
        .style(Style::default().fg(TuiColor::Yellow));
    f.render_widget(prompt, Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 });
    
    let value_text = Paragraph::new(app.text_input_value.as_str())
        .style(Style::default().fg(TuiColor::White));
    f.render_widget(value_text, Rect { x: inner.x, y: inner.y + 1, width: inner.width, height: 1 });
    
    let help = Paragraph::new("Enter: OK | Esc: Cancel | Backspace: Delete")
        .style(Style::default().fg(TuiColor::Cyan));
    f.render_widget(help, Rect { x: inner.x, y: inner.y + 2, width: inner.width, height: 1 });
}

fn render_help(f: &mut Frame, _app: &App, area: Rect) {
    let help_area = area;
    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL);
    f.render_widget(block, help_area);
    
    let inner = Rect {
        x: help_area.x + 1,
        y: help_area.y + 1,
        width: help_area.width.saturating_sub(2),
        height: help_area.height.saturating_sub(2),
    };
    
    let help_text = Text::from(vec![
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
        Line::from("  Ctrl+Shift+P: Toggle preview"),
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
        Line::from("  Ctrl+A: Add object dialog"),
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
        Line::from("  ?: Help"),
        Line::from(""),
        Line::from(" Note: Both legacy (letter) and new (Ctrl+letter) shortcuts work".dim()),
    ]);
    
    let paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, inner);
}

fn render_confirm(f: &mut Frame, app: &App, area: Rect) {
    let dialog_width = 40;
    let dialog_height = 5;
    let dialog_area = Rect {
        x: area.x + (area.width.saturating_sub(dialog_width)) / 2,
        y: area.y + (area.height.saturating_sub(dialog_height)) / 2,
        width: dialog_width,
        height: dialog_height,
    };
    
    let block = Block::default()
        .title(" Confirm ")
        .borders(Borders::ALL);
    f.render_widget(block, dialog_area);
    
    let inner = Rect {
        x: dialog_area.x + 1,
        y: dialog_area.y + 1,
        width: dialog_area.width.saturating_sub(2),
        height: dialog_area.height.saturating_sub(2),
    };
    
    let message = match app.confirm_action {
        ConfirmAction::QuitWithoutSave => "Quit without saving?",
        ConfirmAction::DeleteField => "Delete selected field?",
        ConfirmAction::ClearMap => "Clear all fields?",
    };
    
    let prompt = Paragraph::new(message)
        .style(Style::default().fg(TuiColor::Yellow));
    f.render_widget(prompt, Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 });
    
    let help = Paragraph::new("Y/Enter: Yes | N/Esc: No")
        .style(Style::default().fg(TuiColor::Cyan));
    f.render_widget(help, Rect { x: inner.x, y: inner.y + 1, width: inner.width, height: 1 });
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
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
        AppMode::Confirm => "CONFIRM",
        AppMode::Normal => "PREVIEW",
    };
    
    let mode = Paragraph::new(format!(" MODE: {}", mode_text))
        .style(Style::default().fg(TuiColor::Green).bold())
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(mode, status_layout[0]);
    
    // Message
    let message_text = app.message.as_deref().unwrap_or("");
    let message = Paragraph::new(message_text)
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

#[allow(dead_code)]
fn app_scroll_up(app: &mut App) {
    if app.scroll > 0 {
        app.scroll -= 1;
    }
}

#[allow(dead_code)]
fn app_scroll_down(app: &mut App) {
    app.scroll += 1;
}
