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
    event::{self, Event, KeyCode, KeyModifiers},
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
    /// Mode color picker
    ColorPicker,
    /// Mode attribute picker
    AttributePicker,
    /// Mode save dialog
    SaveDialog,
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
    // Pour le mode confirm
    confirm_action: ConfirmAction,
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
            property_index: 0,
            pending_object: None,
            pending_position: (0, 0),
            edit_properties_field: None,
            edit_properties_index: 0,
            selected_color: None,
            selected_attribute: None,
            save_path: String::new(),
            confirm_action: ConfirmAction::QuitWithoutSave,
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
    execute!(stdout, EnterAlternateScreen)?;
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
            if let Event::Key(key) = event::read()? {
                handle_input(&mut app, key);
            }
        }
        
        app.tick_message();
    }
    
    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn handle_input(app: &mut App, key: event::KeyEvent) {
    // Handle Ctrl keys in all modes
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('c') => {
                if app.mode == AppMode::Edit {
                    app.editor.copy_selected();
                    app.set_message("Copied");
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
            _ => {}
        }
    }
    
    match app.mode {
        AppMode::Edit => handle_edit_mode(app, key),
        AppMode::Properties => handle_properties_mode(app, key),
        AppMode::InsertPosition => handle_insert_position_mode(app, key),
        AppMode::EditProperties => handle_edit_properties_mode(app, key),
        AppMode::ColorPicker => handle_color_picker_mode(app, key),
        AppMode::AttributePicker => handle_attribute_picker_mode(app, key),
        AppMode::SaveDialog => handle_save_dialog_mode(app, key),
        AppMode::Help => handle_help_mode(app, key),
        AppMode::Confirm => handle_confirm_mode(app, key),
        AppMode::Normal => handle_normal_mode(app, key),
    }
}

fn handle_edit_mode(app: &mut App, key: event::KeyEvent) {
    // Handle Shift+Enter for special actions
    if key.modifiers.contains(KeyModifiers::SHIFT) && key.code == KeyCode::Enter {
        if app.active_panel == ActivePanel::Sidebar && app.sidebar_section == SidebarSection::Objects {
            // Shift+Enter in Objects sidebar: insert object at cursor position
            if let Some(obj) = app.pending_object.take() {
                if let Some(pos_idx) = app.sidebar_objects_selected {
                    let objects = InsertableObject::all();
                    if pos_idx < objects.len() {
                        let field = obj.create_field(app.pending_position);
                        app.editor.map.fields.push(field);
                        app.mode = AppMode::Edit;
                        app.set_message(&format!("Inserted {}", obj.display()));
                    }
                }
            }
        } else if app.active_panel == ActivePanel::Canvas {
            // Shift+Enter on selected field in Canvas: open EditProperties
            if let Some(idx) = app.editor.selected_field {
                let field = app.editor.map.fields[idx].clone();
                app.edit_properties_field = Some(field);
                app.edit_properties_index = 0;
                app.mode = AppMode::EditProperties;
                app.set_message("Edit properties - Shift+Enter to save");
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
        
        // Field selection / Section switching
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
        // Toggle between Canvas and Sidebar navigation
        KeyCode::BackTab => {
            app.active_panel.toggle();
            app.sidebar_actions_selected = None;
            app.sidebar_objects_selected = None;
            app.set_message(match app.active_panel {
                ActivePanel::Canvas => "Canvas mode",
                ActivePanel::Sidebar => "Sidebar mode",
            });
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
                                    }
                                    SidebarAction::AddLongField => {
                                        app.editor.add_field_at_cursor(20);
                                        app.set_message("Added long field");
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
                                app.set_message("Set position with arrows, Shift+Enter to confirm");
                            }
                        }
                    }
                }
            }
        }
        
        // Field manipulation
        KeyCode::Char('a') => {
            app.editor.add_field_at_cursor(10);
            app.set_message("Added field");
        }
        KeyCode::Char('A') => {
            app.editor.add_field_at_cursor(20);
            app.set_message("Added long field");
        }
        KeyCode::Char('d') => {
            if app.editor.selected_field.is_some() {
                app.mode = AppMode::Confirm;
                app.confirm_action = ConfirmAction::DeleteField;
            }
        }
        KeyCode::Char('m') => {
            if let Some(idx) = app.editor.selected_field {
                app.editor.drag_start = Some(app.editor.map.fields[idx].pos);
                app.editor.mode = EditorMode::MoveField;
                app.set_message("Move field - arrows to move, Enter to drop");
            }
        }
        KeyCode::Char('r') => {
            if let Some(idx) = app.editor.selected_field {
                app.editor.drag_start = Some((app.editor.map.fields[idx].pos.0, app.editor.map.fields[idx].pos.1 + app.editor.map.fields[idx].length - 1));
                app.editor.mode = EditorMode::ResizeField { direction: ResizeDirection::Right };
                app.set_message("Resize field - Left/Right to resize");
            }
        }
        
        // Properties
        KeyCode::Char('e') => {
            if app.editor.selected_field.is_some() {
                app.mode = AppMode::Properties;
                app.property_index = 0;
            }
        }
        
        // Clipboard
        KeyCode::Char('c') => {
            app.editor.copy_selected();
            app.set_message("Copied");
        }
        KeyCode::Char('x') => {
            if app.editor.cut_selected().is_some() {
                app.set_message("Cut");
            }
        }
        KeyCode::Char('v') => {
            if app.editor.paste_at_cursor().is_some() {
                app.set_message("Pasted");
            }
        }
        
        // Color picker
        KeyCode::Char('C') => {
            if let Some(idx) = app.editor.selected_field {
                app.mode = AppMode::ColorPicker;
                app.selected_color = app.editor.map.fields[idx].color.clone();
            }
        }
        
        // Attribute picker
        KeyCode::Char('t') => {
            if app.editor.selected_field.is_some() {
                app.mode = AppMode::AttributePicker;
                app.selected_attribute = None;
            }
        }
        
        // New map
        KeyCode::Char('n') => {
            app.editor.new_map("NEWMAP", "DEFAULT", (24, 80));
            app.current_file = None;
            app.set_message("New map created");
        }
        
        // Default map
        KeyCode::Char('N') => {
            let default_map = create_default_map("TEMPLATE", "DEFAULT");
            app.editor = BmsEditor::from_map(default_map);
            app.current_file = None;
            app.set_message("Template map loaded");
        }
        
        // Scroll
        KeyCode::Char('J') => app.scroll_down(),
        KeyCode::Char('K') => app.scroll_up(),
        
        // Help
        KeyCode::Char('?') => app.mode = AppMode::Help,
        
        // Generate COBOL
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
        
        // Mode normal (preview only)
        KeyCode::Char(' ') => app.mode = AppMode::Normal,
        
        // Exit
        KeyCode::Esc => {
            if app.is_modified() {
                app.mode = AppMode::Confirm;
                app.confirm_action = ConfirmAction::QuitWithoutSave;
            } else {
                app.exit = true;
            }
        }
        
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
        KeyCode::Enter => app.mode = AppMode::Edit,
        _ => {}
    }
}

fn handle_insert_position_mode(app: &mut App, key: event::KeyEvent) {
    // Handle Shift+Enter for confirmation
    if key.modifiers.contains(KeyModifiers::SHIFT) && key.code == KeyCode::Enter {
        if let Some(obj) = app.pending_object.take() {
            let field = obj.create_field(app.pending_position);
            app.editor.map.fields.push(field);
            app.mode = AppMode::Edit;
            app.set_message(&format!("Inserted {}", obj.display()));
        }
        return;
    }
    
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::Edit;
            app.pending_object = None;
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
        KeyCode::Enter => {
            // Regular Enter just stays in mode (allows position adjustment)
        }
        _ => {}
    }
}

fn handle_edit_properties_mode(app: &mut App, key: event::KeyEvent) {
    // Handle Shift+Enter for saving
    if key.modifiers.contains(KeyModifiers::SHIFT) && key.code == KeyCode::Enter {
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
        AppMode::ColorPicker => " COLOR PICKER ",
        AppMode::AttributePicker => " ATTRIBUTES ",
        AppMode::SaveDialog => " SAVE FILE ",
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
        ActivePanel::Canvas => format!(" [>] Canvas ({}x{}) ", app.editor.map.size.0, app.editor.map.size.1),
        ActivePanel::Sidebar => format!(" Canvas ({}x{}) ", app.editor.map.size.0, app.editor.map.size.1),
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
    
    // Render grid
    let grid_area = Rect {
        x: canvas_area.x + 1,
        y: canvas_area.y + 1,
        width: canvas_area.width.saturating_sub(2),
        height: canvas_area.height.saturating_sub(2),
    };
    
    render_bms_grid(f, app, grid_area);
}

fn render_bms_grid(f: &mut Frame, app: &App, area: Rect) {
    let map = &app.editor.map;
    
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
            
            // Check if any field covers this cell
            for (idx, field) in map.fields.iter().enumerate() {
                let (field_row, field_col) = field.pos;
                let field_row = field_row as usize;
                let field_col = field_col as usize;
                let field_end_col = field_col + field.length as usize - 1;
                
                if grid_row + 1 == field_row && col >= field_col && col <= field_end_col {
                    c = match field.field_type {
                        FieldType::Map => '#',
                        FieldType::Field => {
                            if field.attrb.contains(&FieldAttribute::Prot) {
                                'P'
                            } else if field.attrb.contains(&FieldAttribute::Num) {
                                '0'
                            } else {
                                'F'
                            }
                        }
                        FieldType::Literal => 'L',
                        FieldType::Group => 'G',
                        _ => 'X',
                    };
                    
                    if Some(idx) == app.editor.selected_field {
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

fn render_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let panel_area = Rect {
        x: area.x + area.width - 24,
        y: area.y,
        width: 24,
        height: area.height,
    };
    
    let title = match app.active_panel {
        ActivePanel::Sidebar => " [>] Sidebar ",
        ActivePanel::Canvas => " Sidebar ",
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
    lines.push(Line::from("Shift+Tab: Toggle Canvas/Sidebar".dim()));
    lines.push(Line::from("Tab: Switch Actions/Objects".dim()));
    
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
        .title(" Properties ")
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
        .title(" Insert Position ")
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
    
    let lines = vec![
        Line::from(format!("Object: {}", obj_name)),
        Line::from(""),
        Line::from("Position:".yellow()),
        Line::from(format!("  Row: {}", row)),
        Line::from(format!("  Col: {}", col)),
        Line::from(""),
        Line::from("Arrows: Move".dim()),
        Line::from("Shift+Enter: Confirm".dim()),
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
        .title(" Edit Properties ")
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
            Line::from("Shift+Enter: Save".dim()),
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

fn render_color_picker(f: &mut Frame, app: &App, area: Rect) {
    let panel_width = 28;
    let panel_area = Rect {
        x: area.x + area.width - panel_width,
        y: area.y,
        width: panel_width,
        height: 13,
    };
    
    let block = Block::default()
        .title(" Colors ")
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
        Line::from("  j/k/Down/Up: Move cursor"),
        Line::from("  h/l/Left/Right: Move cursor"),
        Line::from("  Tab/Shift+Tab: Next/Prev field"),
        Line::from(""),
        Line::from(" Field Ops: ".yellow()),
        Line::from("  a: Add field (10)"),
        Line::from("  A: Add field (20)"),
        Line::from("  d: Delete field"),
        Line::from("  m: Move field"),
        Line::from("  r: Resize field"),
        Line::from(""),
        Line::from(" Properties: ".yellow()),
        Line::from("  e: Edit properties"),
        Line::from("  C: Change color"),
        Line::from("  t: Change attributes"),
        Line::from(""),
        Line::from(" Clipboard: ".yellow()),
        Line::from("  Ctrl+C: Copy"),
        Line::from("  x: Cut"),
        Line::from("  v: Paste"),
        Line::from(""),
        Line::from(" File: ".yellow()),
        Line::from("  n: New map"),
        Line::from("  N: Template"),
        Line::from("  Ctrl+S: Save"),
        Line::from("  g: Generate COBOL"),
        Line::from(""),
        Line::from(" Undo/Redo: ".yellow()),
        Line::from("  Ctrl+Z: Undo"),
        Line::from("  Ctrl+Y: Redo"),
        Line::from(""),
        Line::from(" Other: ".yellow()),
        Line::from("  ?: Help"),
        Line::from("  Ctrl+Q: Quit"),
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
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(area);
    
    // Mode
    let mode_text = match app.mode {
        AppMode::Edit => "EDIT",
        AppMode::Properties => "PROPERTIES",
        AppMode::InsertPosition => "INSERT_POS",
        AppMode::EditProperties => "EDIT_PROPS",
        AppMode::ColorPicker => "COLOR",
        AppMode::AttributePicker => "ATTRS",
        AppMode::SaveDialog => "SAVE",
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
    
    // File info
    let file_info = if let Some(ref path) = app.current_file {
        format!(" {} ", path.file_name().unwrap_or_default().to_string_lossy())
    } else {
        " NEW MAP ".to_string()
    };
    
    let modified = if app.is_modified() { "[MODIFIED]" } else { "" };
    let file = Paragraph::new(format!("{}{}", file_info, modified))
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
