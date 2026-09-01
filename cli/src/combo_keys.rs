//! Combo key handling system for BMS ncurses editor
//!
//! This module provides a structured system for handling keyboard combinations
//! (Ctrl, Alt, Shift modifiers) with proper state management and action dispatching.

use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};
use std::collections::HashMap;

/// Represents a keyboard combination (modifier + key)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComboKey {
    pub modifiers: KeyModifiers,
    pub key_code: KeyCode,
}

impl ComboKey {
    /// Create a new combo key
    pub fn new(modifiers: KeyModifiers, key_code: KeyCode) -> Self {
        Self { modifiers, key_code }
    }
    
    /// Create from a key event
    pub fn from_event(event: &KeyEvent) -> Self {
        Self {
            modifiers: event.modifiers,
            key_code: event.code,
        }
    }
    
    /// Check if this combo key matches the given event
    pub fn matches(&self, event: &KeyEvent) -> bool {
        self.modifiers == event.modifiers && self.key_code == event.code
    }
    
    /// Get a string representation
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        if self.modifiers.contains(KeyModifiers::CONTROL) { parts.push("Ctrl"); }
        if self.modifiers.contains(KeyModifiers::ALT) { parts.push("Alt"); }
        if self.modifiers.contains(KeyModifiers::SHIFT) { parts.push("Shift"); }
        
        let key_str = match self.key_code {
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
            _ => format!("{:?}", self.key_code),
        };
        
        if parts.is_empty() {
            key_str
        } else {
            format!("{} + {}", parts.join("+"), key_str)
        }
    }
}

/// Action types that can be triggered by combo keys
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComboAction {
    // View switching
    TogglePanel,
    SwitchToCanvas,
    SwitchToSidebar,
    TogglePreview,
    ToggleHelp,
    
    // Navigation
    NextField,
    PreviousField,
    NextPage,
    PreviousPage,
    FastScrollUp,
    FastScrollDown,
    FastScrollLeft,
    FastScrollRight,
    NextSection,
    PreviousSection,
    
    // Editing
    EnterEditMode,
    ExitEditMode,
    CopyField,
    PasteField,
    Undo,
    Redo,
    
    // Field operations
    ShowProperties,
    ShowFieldProperties,
    ToggleGridSnap,
    AlignToGrid,
    
    // File operations
    NewMap,
    SaveMap,
    OpenMap,
    GenerateCobol,
    ValidateMap,
    
    // Object operations
    AddObject,
    InsertObject,
    DeleteObject,
    MoveObject,
    ResizeObject,
    
    // Color and attributes
    ShowColorPicker,
    ShowAttributePicker,
    
    // Text input
    StartTextInput,
    ConfirmInput,
    CancelInput,
    
    // Sidebar operations
    SwitchToActions,
    SwitchToObjects,
    SelectObject,
    
    // Misc
    ToggleDebug,
    ExitApplication,
    ShowAbout,
}

impl ComboAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComboAction::TogglePanel => "Toggle Canvas/Sidebar",
            ComboAction::SwitchToCanvas => "Switch to Canvas",
            ComboAction::SwitchToSidebar => "Switch to Sidebar",
            ComboAction::TogglePreview => "Toggle BMS/Grid Preview",
            ComboAction::ToggleHelp => "Toggle Help",
            ComboAction::NextField => "Next Field",
            ComboAction::PreviousField => "Previous Field",
            ComboAction::NextPage => "Next Page",
            ComboAction::PreviousPage => "Previous Page",
            ComboAction::FastScrollUp => "Fast Scroll Up",
            ComboAction::FastScrollDown => "Fast Scroll Down",
            ComboAction::FastScrollLeft => "Fast Scroll Left",
            ComboAction::FastScrollRight => "Fast Scroll Right",
            ComboAction::NextSection => "Next Section",
            ComboAction::PreviousSection => "Previous Section",
            ComboAction::EnterEditMode => "Enter Edit Mode",
            ComboAction::ExitEditMode => "Exit Edit Mode",
            ComboAction::CopyField => "Copy Field",
            ComboAction::PasteField => "Paste Field",
            ComboAction::Undo => "Undo",
            ComboAction::Redo => "Redo",
            ComboAction::ShowProperties => "Show Properties",
            ComboAction::ShowFieldProperties => "Show Field Properties",
            ComboAction::ToggleGridSnap => "Toggle Grid Snap",
            ComboAction::AlignToGrid => "Align to Grid",
            ComboAction::NewMap => "New Map",
            ComboAction::SaveMap => "Save Map",
            ComboAction::OpenMap => "Open Map",
            ComboAction::GenerateCobol => "Generate COBOL",
            ComboAction::ValidateMap => "Validate Map",
            ComboAction::AddObject => "Add Object",
            ComboAction::InsertObject => "Insert Object",
            ComboAction::DeleteObject => "Delete Object",
            ComboAction::MoveObject => "Move Object",
            ComboAction::ResizeObject => "Resize Object",
            ComboAction::ShowColorPicker => "Show Color Picker",
            ComboAction::ShowAttributePicker => "Show Attribute Picker",
            ComboAction::StartTextInput => "Start Text Input",
            ComboAction::ConfirmInput => "Confirm Input",
            ComboAction::CancelInput => "Cancel Input",
            ComboAction::SwitchToActions => "Switch to Actions",
            ComboAction::SwitchToObjects => "Switch to Objects",
            ComboAction::SelectObject => "Select Object",
            ComboAction::ToggleDebug => "Toggle Debug",
            ComboAction::ExitApplication => "Exit Application",
            ComboAction::ShowAbout => "Show About",
        }
    }
}

/// Combo key binding configuration
#[derive(Debug, Clone)]
pub struct ComboKeyBinding {
    pub combo_key: ComboKey,
    pub action: ComboAction,
    pub description: String,
    pub context: ComboContext,
}

impl ComboKeyBinding {
    pub fn new(combo_key: ComboKey, action: ComboAction, description: &str, context: ComboContext) -> Self {
        Self {
            combo_key,
            action,
            description: description.to_string(),
            context,
        }
    }
    
    pub fn matches(&self, event: &KeyEvent) -> bool {
        self.combo_key.matches(event)
    }
}

/// Context in which a combo key is valid
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComboContext {
    /// Always available
    Global,
    /// Only in edit mode
    EditMode,
    /// Only in properties mode
    PropertiesMode,
    /// Only in insert position mode
    InsertPositionMode,
    /// Only when a field is selected
    FieldSelected,
    /// Only when multiple fields are selected
    MultipleFieldsSelected,
    /// Only in dialog modes
    DialogMode,
    /// Only in color picker mode
    ColorPickerMode,
    /// Only in attribute picker mode
    AttributePickerMode,
    /// Only in text input mode
    TextInputMode,
    /// Only in canvas panel
    CanvasPanel,
    /// Only in sidebar panel
    SidebarPanel,
}

impl ComboContext {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComboContext::Global => "Global",
            ComboContext::EditMode => "Edit Mode",
            ComboContext::PropertiesMode => "Properties Mode",
            ComboContext::InsertPositionMode => "Insert Position Mode",
            ComboContext::FieldSelected => "Field Selected",
            ComboContext::MultipleFieldsSelected => "Multiple Fields Selected",
            ComboContext::DialogMode => "Dialog Mode",
            ComboContext::ColorPickerMode => "Color Picker Mode",
            ComboContext::AttributePickerMode => "Attribute Picker Mode",
            ComboContext::TextInputMode => "Text Input Mode",
            ComboContext::CanvasPanel => "Canvas Panel",
            ComboContext::SidebarPanel => "Sidebar Panel",
        }
    }
}

/// Combo key manager for handling key bindings
#[derive(Debug, Default)]
pub struct ComboKeyManager {
    bindings: HashMap<ComboKey, ComboKeyBinding>,
    context_stack: Vec<ComboContext>,
}

impl ComboKeyManager {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            context_stack: Vec::new(),
        }
    }
    
    /// Register a new key binding
    pub fn register_binding(&mut self, binding: ComboKeyBinding) {
        self.bindings.insert(binding.combo_key.clone(), binding);
    }
    
    /// Register multiple bindings at once
    pub fn register_bindings(&mut self, bindings: Vec<ComboKeyBinding>) {
        for binding in bindings {
            self.register_binding(binding);
        }
    }
    
    /// Push a context onto the stack (enables bindings for that context)
    pub fn push_context(&mut self, context: ComboContext) {
        self.context_stack.push(context);
    }
    
    /// Pop a context from the stack
    pub fn pop_context(&mut self) {
        self.context_stack.pop();
    }
    
    /// Check if a context is active
    pub fn is_context_active(&self, context: &ComboContext) -> bool {
        self.context_stack.contains(context)
    }
    
    /// Clear all contexts
    pub fn clear_contexts(&mut self) {
        self.context_stack.clear();
    }
    
    /// Handle a key event and return the corresponding action if any
    pub fn handle_key(&self, event: &KeyEvent) -> Option<ComboAction> {
        for (combo_key, binding) in &self.bindings {
            if combo_key.matches(event) && self.is_context_active(&binding.context) {
                return Some(binding.action.clone());
            }
        }
        None
    }
    
    /// Get all bindings for the current contexts
    pub fn get_active_bindings(&self) -> Vec<&ComboKeyBinding> {
        self.bindings.values()
            .filter(|binding| self.is_context_active(&binding.context))
            .collect()
    }
    
    /// Get help text for all active bindings
    pub fn get_help_text(&self) -> Vec<String> {
        let mut lines = Vec::new();
        
        // Group bindings by context
        use std::collections::HashMap;
        let mut by_context: HashMap<ComboContext, Vec<&ComboKeyBinding>> = HashMap::new();
        for binding in self.get_active_bindings() {
            by_context.entry(binding.context.clone()).or_default().push(binding);
        }
        
        // Add global bindings first
        if let Some(global_bindings) = by_context.remove(&ComboContext::Global) {
            lines.push("Global:".to_string());
            for binding in global_bindings {
                lines.push(format!("  {}: {}", binding.combo_key.to_string(), binding.description));
            }
        }
        
        // Add other contexts
        for (context, bindings) in by_context {
            lines.push(format!("{}:", context.as_str()));
            for binding in bindings {
                lines.push(format!("  {}: {}", binding.combo_key.to_string(), binding.description));
            }
        }
        
        lines
    }
    
    /// Get default bindings for the BMS editor
    pub fn default_bindings() -> Vec<ComboKeyBinding> {
        vec![
            // Panel switching
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('p')),
                ComboAction::TogglePanel,
                "Toggle Canvas/Sidebar",
                ComboContext::Global,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Char('p')),
                ComboAction::TogglePanel,
                "Toggle Canvas/Sidebar (alternate)",
                ComboContext::Global,
            ),
            
            // Preview toggle
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char(' ')),
                ComboAction::TogglePreview,
                "Toggle BMS/Grid Preview",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('p')),
                ComboAction::TogglePreview,
                "Toggle BMS/Grid Preview (alternate)",
                ComboContext::EditMode,
            ),
            
            // Help
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('h')),
                ComboAction::ToggleHelp,
                "Toggle Help",
                ComboContext::Global,
            ),
            
            // Navigation
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::NONE, KeyCode::Tab),
                ComboAction::NextField,
                "Next Field",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::SHIFT, KeyCode::BackTab),
                ComboAction::PreviousField,
                "Previous Field",
                ComboContext::EditMode,
            ),
            
            // Section switching (for sidebar)
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::NONE, KeyCode::Tab),
                ComboAction::NextSection,
                "Next Section",
                ComboContext::SidebarPanel,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::SHIFT, KeyCode::BackTab),
                ComboAction::PreviousSection,
                "Previous Section",
                ComboContext::SidebarPanel,
            ),
            
            // Fast scrolling
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::ALT, KeyCode::Up),
                ComboAction::FastScrollUp,
                "Fast Scroll Up",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::ALT, KeyCode::Down),
                ComboAction::FastScrollDown,
                "Fast Scroll Down",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::ALT, KeyCode::Left),
                ComboAction::FastScrollLeft,
                "Fast Scroll Left",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::ALT, KeyCode::Right),
                ComboAction::FastScrollRight,
                "Fast Scroll Right",
                ComboContext::EditMode,
            ),
            
            // Also support Ctrl for fast scrolling
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Up),
                ComboAction::FastScrollUp,
                "Fast Scroll Up (Ctrl)",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Down),
                ComboAction::FastScrollDown,
                "Fast Scroll Down (Ctrl)",
                ComboContext::EditMode,
            ),
            
            // Editing
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::NONE, KeyCode::Char('e')),
                ComboAction::ShowFieldProperties,
                "Show Field Properties",
                ComboContext::FieldSelected,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::NONE, KeyCode::Char('d')),
                ComboAction::DeleteObject,
                "Delete Field",
                ComboContext::FieldSelected,
            ),
            
            // File operations
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('n')),
                ComboAction::NewMap,
                "New Map",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('s')),
                ComboAction::SaveMap,
                "Save Map",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('o')),
                ComboAction::OpenMap,
                "Open Map",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('g')),
                ComboAction::GenerateCobol,
                "Generate COBOL",
                ComboContext::EditMode,
            ),
            
            // Copy/Paste
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('c')),
                ComboAction::CopyField,
                "Copy Field(s)",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('v')),
                ComboAction::PasteField,
                "Paste Field(s)",
                ComboContext::EditMode,
            ),
            
            // Undo/Redo
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('z')),
                ComboAction::Undo,
                "Undo",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('y')),
                ComboAction::Redo,
                "Redo",
                ComboContext::EditMode,
            ),
            
            // Grid snap
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('g')),
                ComboAction::ToggleGridSnap,
                "Toggle Grid Snap",
                ComboContext::EditMode,
            ),
            
            // Validation
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('v')),
                ComboAction::ValidateMap,
                "Validate Map",
                ComboContext::EditMode,
            ),
            
            // Exit
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('q')),
                ComboAction::ExitApplication,
                "Exit Application",
                ComboContext::Global,
            ),
            
            // Force exit without save
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Esc),
                ComboAction::ExitApplication,
                "Force Exit (no save)",
                ComboContext::Global,
            ),
            
            // Sidebar navigation
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::NONE, KeyCode::Char('a')),
                ComboAction::SwitchToActions,
                "Switch to Actions section",
                ComboContext::SidebarPanel,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::NONE, KeyCode::Char('o')),
                ComboAction::SwitchToObjects,
                "Switch to Objects section",
                ComboContext::SidebarPanel,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::NONE, KeyCode::Enter),
                ComboAction::SelectObject,
                "Select Object",
                ComboContext::SidebarPanel,
            ),
        ]
    }
}