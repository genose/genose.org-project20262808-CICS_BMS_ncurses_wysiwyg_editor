//! Combo key handling system for BMS ncurses editor
//!
//! This module provides a structured system for handling keyboard combinations
//! (Ctrl, Alt, Shift modifiers) with proper state management and action dispatching.
//!
//! # Features:
//! - **Combo key detection**: Handles Ctrl, Alt, Shift modifier combinations
//! - **Context-aware**: Key bindings can be context-specific (edit mode, properties mode, etc.)
//! - **Fallback handling**: Provides alternative bindings for terminals that don't capture certain combinations
//! - **VSCode compatibility**: Handles limitations of VSCode embedded terminals
//! - **Sequential key support**: Supports leader key patterns (e.g., space + key sequences)

use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};
use std::collections::{HashMap, VecDeque};
use std::time::{Instant, Duration};

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
    ShowComboKeyHelp,
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
            ComboAction::ShowComboKeyHelp => "Show Combo Key Help",
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

/// Terminal type for capability detection
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalType {
    /// Standard terminal with full key support
    Standard,
    /// VSCode embedded terminal (limited key support)
    VSCode,
    /// Windows Terminal
    WindowsTerminal,
    /// iTerm2 on macOS
    ITerm2,
    /// Other/Unknown terminal
    Unknown,
}

impl Default for TerminalType {
    fn default() -> Self {
        TerminalType::Unknown
    }
}

impl TerminalType {
    /// Detect the current terminal type
    pub fn detect() -> Self {
        let term = std::env::var("TERM").unwrap_or_default();
        let vscode = std::env::var("VSCODE_INJECTION").is_ok();
        
        if vscode || term.contains("vscode") || term.contains("xterm-256color") && is_likely_vscode() {
            TerminalType::VSCode
        } else if term.contains("windows") || term.contains("wt") || term.contains("Windows Terminal") {
            TerminalType::WindowsTerminal
        } else if term.contains("iterm") || term.contains("iTerm") {
            TerminalType::ITerm2
        } else if term.is_empty() || term == "dumb" {
            TerminalType::Unknown
        } else {
            TerminalType::Standard
        }
    }
    
    /// Check if this terminal has limitations
    pub fn has_limitations(&self) -> bool {
        matches!(self, TerminalType::VSCode | TerminalType::Unknown)
    }
    
    /// Check if Alt combinations work reliably
    pub fn supports_alt_combinations(&self) -> bool {
        !matches!(self, TerminalType::VSCode)
    }
    
    /// Check if Shift+modifier combinations work reliably
    pub fn supports_shift_combinations(&self) -> bool {
        !matches!(self, TerminalType::VSCode | TerminalType::Unknown)
    }
    
    /// Get string representation of terminal type
    pub fn as_str(&self) -> &'static str {
        match self {
            TerminalType::Standard => "Standard",
            TerminalType::VSCode => "VSCode",
            TerminalType::WindowsTerminal => "Windows Terminal",
            TerminalType::ITerm2 => "iTerm2",
            TerminalType::Unknown => "Unknown",
        }
    }
}

/// Check if we're likely running in VSCode
fn is_likely_vscode() -> bool {
    // Check for VSCode-specific environment variables
    std::env::var("VSCODE_PID").is_ok() ||
    std::env::var("VSCODE_IPC_HOOK").is_ok() ||
    std::env::var("VSCODE_CWD").is_ok()
}

/// Key sequence for leader key patterns (e.g., Space + key)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeySequence {
    pub keys: Vec<KeyCode>,
    pub timeout: Duration,
}

impl KeySequence {
    pub fn new(keys: Vec<KeyCode>, timeout: Duration) -> Self {
        Self { keys, timeout }
    }
    
    pub fn is_complete(&self, input_keys: &[KeyCode]) -> bool {
        if input_keys.len() < self.keys.len() {
            return false;
        }
        self.keys == input_keys[..self.keys.len()]
    }
    
    pub fn matches_partial(&self, input_keys: &[KeyCode]) -> bool {
        if input_keys.is_empty() {
            return true;
        }
        input_keys.starts_with(&self.keys[..input_keys.len().min(self.keys.len())])
    }
}

/// Combo key manager for handling key bindings
#[derive(Debug, Default)]
pub struct ComboKeyManager {
    bindings: HashMap<ComboKey, ComboKeyBinding>,
    context_stack: Vec<ComboContext>,
    terminal_type: TerminalType,
    fallback_bindings: HashMap<ComboAction, Vec<ComboKey>>,
    leader_key: Option<KeyCode>,
    pending_sequence: VecDeque<KeyCode>,
    sequence_timeout: Duration,
    last_key_time: Option<Instant>,
}

impl ComboKeyManager {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            context_stack: Vec::new(),
            terminal_type: TerminalType::detect(),
            fallback_bindings: HashMap::new(),
            leader_key: None,
            pending_sequence: VecDeque::new(),
            sequence_timeout: Duration::from_millis(1000), // 1 second timeout for sequences
            last_key_time: None,
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
    
    /// Handle a key event and return the corresponding action if any (internal simple version)
    fn handle_key_simple(&self, event: &KeyEvent) -> Option<ComboAction> {
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
    
    /// Set the terminal type manually (for testing or override)
    pub fn set_terminal_type(&mut self, terminal_type: TerminalType) {
        self.terminal_type = terminal_type;
    }
    
    /// Get the current terminal type
    pub fn terminal_type(&self) -> TerminalType {
        self.terminal_type.clone()
    }
    
    /// Set the leader key for sequential commands
    pub fn set_leader_key(&mut self, key_code: KeyCode) {
        self.leader_key = Some(key_code);
    }
    
    /// Set timeout for key sequences
    pub fn set_sequence_timeout(&mut self, timeout: Duration) {
        self.sequence_timeout = timeout;
    }
    
    /// Register fallback bindings for an action (alternative key combos)
    pub fn register_fallbacks(&mut self, action: ComboAction, fallback_keys: Vec<ComboKey>) {
        self.fallback_bindings.entry(action).or_default().extend(fallback_keys);
    }
    
    /// Reset any pending key sequences (call this on mode change)
    pub fn reset_pending_sequences(&mut self) {
        self.pending_sequence.clear();
        self.last_key_time = None;
    }
    
    /// Handle a key event with enhanced logic for sequences and fallbacks
    pub fn handle_key(&mut self, event: &KeyEvent) -> Option<ComboAction> {
        let now = Instant::now();
        
        // Check if we have a pending sequence that timed out
        if let Some(last_time) = self.last_key_time {
            if now.duration_since(last_time) > self.sequence_timeout {
                self.pending_sequence.clear();
            }
        }
        self.last_key_time = Some(now);
        
        // Check if this key starts or continues a sequence
        if let Some(leader) = self.leader_key {
            if event.code == leader && event.modifiers.is_empty() {
                // Leader key pressed - start a new sequence
                self.pending_sequence.clear();
                self.pending_sequence.push_back(event.code);
                return None; // Don't execute action yet, wait for next key
            }
            
            if !self.pending_sequence.is_empty() {
                // We're in a sequence - add this key
                self.pending_sequence.push_back(event.code);
                
                // Check if this completes any sequence
                // For now, we'll handle this in the main input loop
                return None;
            }
        }
        
        // Try primary bindings first
        if let Some(action) = self.handle_key_simple(event) {
            return Some(action);
        }
        
        // Try fallback bindings for this terminal
        if self.terminal_type.has_limitations() {
            return self.try_fallback_bindings(event);
        }
        
        None
    }
    

    
    /// Try fallback bindings when primary doesn't match
    fn try_fallback_bindings(&self, event: &KeyEvent) -> Option<ComboAction> {
        // Check for terminal-specific fallbacks
        match self.terminal_type {
            TerminalType::VSCode => self.try_vscode_fallbacks(event),
            TerminalType::WindowsTerminal => self.try_windows_fallbacks(event),
            _ => None,
        }
    }
    
    /// VSCode-specific fallback bindings
    fn try_vscode_fallbacks(&self, event: &KeyEvent) -> Option<ComboAction> {
        // In VSCode, many key combinations are intercepted or don't work properly
        // So we need to use combinations that VSCode doesn't capture
        
        // VSCode captures: Ctrl+C, Ctrl+V, Ctrl+S, Ctrl+O, Ctrl+Q, Ctrl+Z, Ctrl+Y, Ctrl+A, etc.
        // So we need to use alternatives that VSCode doesn't intercept
        
        match (event.modifiers, event.code) {
            // VSCode alternative for panel toggle (Ctrl+P is safe)
            (KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Char('p')) => {
                self.find_action_for(ComboAction::TogglePanel)
            }
            // VSCode: Ctrl+Alt+Space instead of Ctrl+Space for preview
            (KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Char(' ')) => {
                self.find_action_for(ComboAction::TogglePreview)
            }
            // VSCode: Ctrl+Alt+Arrows instead of Alt+Arrows for fast scroll
            (KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Up) => {
                self.find_action_for(ComboAction::FastScrollUp)
            }
            (KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Down) => {
                self.find_action_for(ComboAction::FastScrollDown)
            }
            (KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Left) => {
                self.find_action_for(ComboAction::FastScrollLeft)
            }
            (KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Right) => {
                self.find_action_for(ComboAction::FastScrollRight)
            }
            // VSCode intercepts Ctrl+C, so use Ctrl+Shift+C for copy
            (KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('c')) => {
                self.find_action_for(ComboAction::CopyField)
            }
            // VSCode intercepts Ctrl+V, so use Ctrl+Shift+V for paste
            (KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('v')) => {
                self.find_action_for(ComboAction::PasteField)
            }
            // VSCode intercepts Ctrl+S, so use Ctrl+Shift+S for save
            (KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('s')) => {
                self.find_action_for(ComboAction::SaveMap)
            }
            // VSCode intercepts Ctrl+O, so use Ctrl+Shift+O for open
            (KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('o')) => {
                self.find_action_for(ComboAction::OpenMap)
            }
            // VSCode intercepts Ctrl+Z, so use Ctrl+Shift+Z for undo
            (KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('z')) => {
                self.find_action_for(ComboAction::Undo)
            }
            // VSCode intercepts Ctrl+Y, so use Ctrl+Shift+Y for redo
            (KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('y')) => {
                self.find_action_for(ComboAction::Redo)
            }
            _ => None,
        }
    }
    
    /// Windows Terminal-specific fallback bindings
    fn try_windows_fallbacks(&self, event: &KeyEvent) -> Option<ComboAction> {
        // Windows sometimes has different behavior for Alt combinations
        match (event.modifiers, event.code) {
            // Windows: Use Ctrl+Alt for some Alt-only combos
            (KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Char(c)) => {
                // Try to find the corresponding Alt-only binding
                let alt_event = KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: KeyModifiers::ALT,
                    kind: event.kind,
                    state: event.state,
                };
                self.handle_key_simple(&alt_event)
            }
            _ => None,
        }
    }
    
    /// Find a binding for a specific action (used for fallbacks)
    fn find_action_for(&self, action: ComboAction) -> Option<ComboAction> {
        for binding in self.bindings.values() {
            if binding.action == action && self.is_context_active(&binding.context) {
                return Some(action);
            }
        }
        None
    }
    
    /// Get the current pending sequence
    pub fn get_pending_sequence(&self) -> Vec<KeyCode> {
        self.pending_sequence.iter().cloned().collect()
    }
    
    /// Check if we're waiting for a sequence to complete
    pub fn is_waiting_for_sequence(&self) -> bool {
        !self.pending_sequence.is_empty()
    }

    /// Get VSCode-specific bindings that avoid keys captured by VSCode
    pub fn vscode_bindings() -> Vec<ComboKeyBinding> {
        // VSCode intercepts: Ctrl+C, Ctrl+V, Ctrl+S, Ctrl+O, Ctrl+Q, Ctrl+Z, Ctrl+Y, Ctrl+A, Ctrl+N, Ctrl+G, Ctrl+F
        // Safe keys: Ctrl+P, Ctrl+Space, Ctrl+H, Ctrl+M, Ctrl+R, Ctrl+D, Ctrl+L, Ctrl+K, Ctrl+J, etc.
        // Also safe: Alt+Shift combinations, Ctrl+Alt combinations (for non-intercepted keys)
        vec![
            // Panel switching - Ctrl+P is safe
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('p')),
                ComboAction::TogglePanel,
                "Toggle Canvas/Sidebar",
                ComboContext::Global,
            ),
            
            // Preview toggle - Ctrl+Space is safe
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char(' ')),
                ComboAction::TogglePreview,
                "Toggle BMS/Grid Preview",
                ComboContext::EditMode,
            ),
            
            // Help - Ctrl+H is safe
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('h')),
                ComboAction::ToggleHelp,
                "Toggle Help",
                ComboContext::Global,
            ),
            
            // Combo key help - Ctrl+Shift+H
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('h')),
                ComboAction::ShowComboKeyHelp,
                "Show Combo Key Help",
                ComboContext::Global,
            ),
            
            // Navigation - use keys VSCode doesn't intercept
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
            
            // Fast scrolling - use Ctrl+Shift+Arrows (Ctrl+Arrows might be safe in VSCode)
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Up),
                ComboAction::FastScrollUp,
                "Fast Scroll Up",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Down),
                ComboAction::FastScrollDown,
                "Fast Scroll Down",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Left),
                ComboAction::FastScrollLeft,
                "Fast Scroll Left",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Right),
                ComboAction::FastScrollRight,
                "Fast Scroll Right",
                ComboContext::EditMode,
            ),
            
            // Editing operations - use Ctrl+Shift for VSCode
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('c')),
                ComboAction::CopyField,
                "Copy Field(s)",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('v')),
                ComboAction::PasteField,
                "Paste Field(s)",
                ComboContext::EditMode,
            ),
            
            // Undo/Redo - use Ctrl+Shift
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('z')),
                ComboAction::Undo,
                "Undo",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('y')),
                ComboAction::Redo,
                "Redo",
                ComboContext::EditMode,
            ),
            
            // File operations - use Ctrl+Shift
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('s')),
                ComboAction::SaveMap,
                "Save Map",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('o')),
                ComboAction::OpenMap,
                "Open Map",
                ComboContext::EditMode,
            ),
            
            // Grid operations
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('g')),
                ComboAction::ToggleGridSnap,
                "Toggle Grid Snap",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('l')),
                ComboAction::AlignToGrid,
                "Align to Grid",
                ComboContext::EditMode,
            ),
            
            // Validation - use Ctrl+Shift+V (VSCode uses Ctrl+Shift+V for paste, but we already have that for paste)
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('k')),
                ComboAction::ValidateMap,
                "Validate Map",
                ComboContext::EditMode,
            ),
            
            // Field operations
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
            
            // Object operations
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::NONE, KeyCode::Char('a')),
                ComboAction::AddObject,
                "Add Object",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('m')),
                ComboAction::MoveObject,
                "Move Field",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('r')),
                ComboAction::ResizeObject,
                "Resize Field",
                ComboContext::EditMode,
            ),
            
            // Generate COBOL
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('j')),
                ComboAction::GenerateCobol,
                "Generate COBOL",
                ComboContext::EditMode,
            ),
            
            // Exit
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('q')),
                ComboAction::ExitApplication,
                "Exit Application",
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
            
            // Leader key sequences (Space + key) - these are handled separately
        ]
    }
    
    /// Get default bindings for the BMS editor
        let mut bindings = vec![
            // Panel switching
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL, KeyCode::Char('p')),
                ComboAction::TogglePanel,
                "Toggle Canvas/Sidebar",
                ComboContext::Global,
            ),
            // VSCode fallback for panel toggle
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Char('p')),
                ComboAction::TogglePanel,
                "Toggle Canvas/Sidebar (VSCode)",
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
            // Combo key help
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::SHIFT, KeyCode::Char('h')),
                ComboAction::ShowComboKeyHelp,
                "Show Combo Key Help",
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
            
            // VSCode-specific fallbacks (these will be filtered by terminal type)
            // Alt combinations often don't work in VSCode, so we provide Ctrl+Alt alternatives
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Up),
                ComboAction::FastScrollUp,
                "Fast Scroll Up (VSCode)",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Down),
                ComboAction::FastScrollDown,
                "Fast Scroll Down (VSCode)",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Left),
                ComboAction::FastScrollLeft,
                "Fast Scroll Left (VSCode)",
                ComboContext::EditMode,
            ),
            ComboKeyBinding::new(
                ComboKey::new(KeyModifiers::CONTROL | KeyModifiers::ALT, KeyCode::Right),
                ComboAction::FastScrollRight,
                "Fast Scroll Right (VSCode)",
                ComboContext::EditMode,
            ),
        ];
        bindings
    }
    
    /// Get leader key sequences for VSCode compatibility
    pub fn default_leader_sequences() -> Vec<(KeyCode, Vec<(KeyCode, ComboAction)>)> {
        vec![
            // Space as leader key
            (KeyCode::Char(' '), vec![
                (KeyCode::Char('p'), ComboAction::TogglePanel),
                (KeyCode::Char('s'), ComboAction::SaveMap),
                (KeyCode::Char('o'), ComboAction::OpenMap),
                (KeyCode::Char('n'), ComboAction::NewMap),
                (KeyCode::Char('g'), ComboAction::GenerateCobol),
                (KeyCode::Char('h'), ComboAction::ToggleHelp),
                (KeyCode::Char('q'), ComboAction::ExitApplication),
                (KeyCode::Char('c'), ComboAction::CopyField),
                (KeyCode::Char('v'), ComboAction::PasteField),
                (KeyCode::Char('z'), ComboAction::Undo),
                (KeyCode::Char('y'), ComboAction::Redo),
            ]),
            
            // Alternative leader key: Backslash
            (KeyCode::Char('\\'), vec![
                (KeyCode::Char('p'), ComboAction::TogglePreview),
                (KeyCode::Char('e'), ComboAction::ShowFieldProperties),
                (KeyCode::Char('d'), ComboAction::DeleteObject),
            ]),
        ]
    }
    
    /// Handle leader key sequence
    pub fn handle_leader_sequence(&mut self, key_code: KeyCode) -> Option<ComboAction> {
        if !self.is_waiting_for_sequence() {
            return None;
        }
        
        let sequence = self.get_pending_sequence();
        let mut all_keys = sequence.clone();
        all_keys.push(key_code);
        
        // Check if this completes any known sequence
        let leader_sequences = Self::default_leader_sequences();
        
        for (leader, mappings) in leader_sequences {
            if !all_keys.is_empty() && all_keys[0] == leader {
                // This sequence starts with our leader key
                if all_keys.len() == 2 {
                    // We have leader + one more key
                    for (key, action) in mappings {
                        if all_keys[1] == key {
                            self.pending_sequence.clear();
                            return Some(action);
                        }
                    }
                }
            }
        }
        
        // If no sequence matched, clear and return None
        self.pending_sequence.clear();
        None
    }
    
    /// Get all available actions for the current context
    pub fn get_available_actions(&self) -> Vec<ComboAction> {
        let mut actions = Vec::new();
        
        for binding in self.bindings.values() {
            if self.is_context_active(&binding.context) && !actions.contains(&binding.action) {
                actions.push(binding.action.clone());
            }
        }
        
        actions
    }
    
    /// Check if a specific action is available in the current context
    pub fn is_action_available(&self, action: &ComboAction) -> bool {
        for binding in self.bindings.values() {
            if binding.action == *action && self.is_context_active(&binding.context) {
                return true;
            }
        }
        false
    }
}