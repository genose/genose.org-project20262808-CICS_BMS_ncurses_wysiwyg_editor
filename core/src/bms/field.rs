//! BMS Field structure - mirrors Lua OBJECTS_DEFINITIONS.new() output
//!
//! This module provides the main BmsField struct that mirrors the structure
//! created by Lua's OBJECTS_DEFINITIONS.new() function. Each field has properties
//! with initial and edited states for efficient memory usage and undo/redo support.

use super::properties::{Property, ConstrainedProperty, EnumProperty};
use super::field_types::{BmsFieldType, GuiFieldType};
use super::types::{
    Color, BorderStyle, TextAlign, VerticalAlign, FillChar, TextStyle, 
    Position, BorderCharSet, BorderChars, Marker, PrefixSuffix, Footer, DecorationType, VerticalMargin
};
use super::defaults::BmsDefaults;
use serde::{Serialize, Deserialize};
use std::fmt;

// ============================================================================
// FIELD INITIAL VALUE
// ============================================================================

/// Initial value for a field - mirrors Lua field_initial
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldInitialValue {
    /// Text value (for FieldTextORNumeric, Literal, ProtectedLiteral)
    Text(String),
    /// Boolean value (for BooleanField)
    Boolean(bool),
    /// Numeric value (for numeric fields)
    Numeric(i64),
    /// ASCII art data (for ImageAsciiArt)
    AsciiArt(AsciiArtData),
    /// No initial value
    None,
}

/// ASCII Art data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AsciiArtData {
    /// Lines of ASCII art
    pub ascii_code: Vec<String>,
    /// Optional file path
    pub file_path: Option<String>,
}

impl FieldInitialValue {
    /// Create default initial value for a field type
    pub fn default_for(field_type: BmsFieldType) -> Self {
        match field_type {
            BmsFieldType::BooleanField => FieldInitialValue::Boolean(false),
            BmsFieldType::ImageAsciiArt => FieldInitialValue::AsciiArt(AsciiArtData {
                ascii_code: Vec::new(),
                file_path: None,
            }),
            BmsFieldType::Fieldset | BmsFieldType::Group => FieldInitialValue::Text("title".to_string()),
            _ => FieldInitialValue::Text("text".to_string()),
        }
    }

    /// Get string representation
    pub fn to_string(&self) -> Option<String> {
        match self {
            FieldInitialValue::Text(s) => Some(s.clone()),
            FieldInitialValue::Boolean(b) => Some(if *b { "[X]".to_string() } else { "[ ]".to_string() }),
            FieldInitialValue::Numeric(n) => Some(n.to_string()),
            FieldInitialValue::AsciiArt(a) => Some(a.ascii_code.join("\n")),
            FieldInitialValue::None => None,
        }
    }

    /// Get display representation
    pub fn display(&self) -> String {
        match self {
            FieldInitialValue::Text(s) => s.clone(),
            FieldInitialValue::Boolean(b) => if *b { "[X]".to_string() } else { "[ ]".to_string() },
            FieldInitialValue::Numeric(n) => n.to_string(),
            FieldInitialValue::AsciiArt(a) => a.ascii_code.join("\n"),
            FieldInitialValue::None => String::new(),
        }
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        match self {
            FieldInitialValue::Text(s) => s.is_empty(),
            FieldInitialValue::Boolean(_) => false,
            FieldInitialValue::Numeric(n) => *n == 0,
            FieldInitialValue::AsciiArt(a) => a.ascii_code.is_empty(),
            FieldInitialValue::None => true,
        }
    }
}

impl Default for FieldInitialValue {
    fn default() -> Self {
        FieldInitialValue::Text("text".to_string())
    }
}

// ============================================================================
// FIELD ATTRIBUTES
// ============================================================================

/// Field attributes - mirrors Lua field_attrb
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldAttributes {
    /// Whether field is in edit mode
    pub field_in_edit_mode: bool,
    /// Whether field is visible
    pub field_visible: bool,
    /// Whether field is required
    pub field_required: bool,
    /// Whether field has error
    pub field_has_error: bool,
    /// Whether field is read-only
    pub field_readonly: bool,
    /// Whether field is enabled
    pub field_enabled: bool,
    /// Whether field is focused
    pub field_focused: bool,
    /// Whether field is selected
    pub field_selected: bool,
    /// Whether field is highlighted
    pub field_highlighted: bool,
    /// Whether field is hidden
    pub field_hidden: bool,
    /// Whether field is protected
    pub field_protected: bool,
    /// Whether field is numeric
    pub field_numeric: bool,
}

impl FieldAttributes {
    /// Create default attributes for a field type
    pub fn default_for(field_type: BmsFieldType) -> Self {
        match field_type {
            BmsFieldType::Literal | BmsFieldType::ProtectedLiteral | BmsFieldType::Line => Self {
                field_readonly: true,
                field_protected: field_type == BmsFieldType::ProtectedLiteral,
                ..Default::default()
            },
            BmsFieldType::BooleanField => Self {
                field_readonly: false,
                field_protected: false,
                field_numeric: false,
                ..Default::default()
            },
            _ => Self::default(),
        }
    }

    /// Check if field is editable
    pub fn is_editable(&self) -> bool {
        self.field_visible && !self.field_readonly && !self.field_protected && self.field_enabled
    }

    /// Check if field is protected from modification
    pub fn is_protected(&self) -> bool {
        self.field_protected || self.field_readonly
    }

    /// Check if field can receive focus
    pub fn can_focus(&self) -> bool {
        self.field_visible && self.field_enabled && !self.field_hidden
    }
}

impl Default for FieldAttributes {
    fn default() -> Self {
        Self {
            field_in_edit_mode: false,
            field_visible: true,
            field_required: false,
            field_has_error: false,
            field_readonly: false,
            field_enabled: true,
            field_focused: false,
            field_selected: false,
            field_highlighted: false,
            field_hidden: false,
            field_protected: false,
            field_numeric: false,
        }
    }
}

// ============================================================================
// FIELD BORDER CONFIGURATION
// ============================================================================

/// Field border configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldBorder {
    /// Border style
    pub style: BorderStyle,
    /// Border character set
    pub chars: BorderCharSet,
}

impl FieldBorder {
    pub fn new(style: BorderStyle, chars: BorderCharSet) -> Self {
        Self { style, chars }
    }

    pub fn none() -> Self {
        Self {
            style: BorderStyle::None,
            chars: BorderCharSet::none(),
        }
    }

    pub fn single() -> Self {
        Self {
            style: BorderStyle::Single,
            chars: BorderCharSet::single(),
        }
    }

    pub fn double() -> Self {
        Self {
            style: BorderStyle::Double,
            chars: BorderCharSet::double(),
        }
    }

    pub fn for_style(style: BorderStyle) -> Self {
        Self {
            style,
            chars: BorderCharSet::for_style(style),
        }
    }
}

impl Default for FieldBorder {
    fn default() -> Self {
        Self::none()
    }
}

// ============================================================================
// VISUAL REPRESENTATION
// ============================================================================

/// Visual representation type for fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VisualRepresentation {
    /// Function-based rendering
    Function,
    /// Template-based rendering with lines
    Template(Vec<String>),
    /// Custom rendering
    Custom,
}

impl VisualRepresentation {
    /// Get default visual representation for a field type
    pub fn default_for(field_type: BmsFieldType) -> Self {
        // For now, use simple function representation
        // In the future, this can be enhanced with actual rendering functions
        VisualRepresentation::Function
    }
}

impl Default for VisualRepresentation {
    fn default() -> Self {
        VisualRepresentation::Function
    }
}

// ============================================================================
// MAIN FIELD STRUCTURE
// ============================================================================

/// A BMS field with structured properties
/// 
/// This struct mirrors the structure created by Lua's OBJECTS_DEFINITIONS.new()
/// function. Each property has initial and edited states for:
/// - Memory efficiency (only edited values consume extra space)
/// - Undo/redo support (can revert to initial value)
/// - Default values (initial provides sensible defaults)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmsField {
    // ========================================================================
    // IDENTIFICATION PROPERTIES
    // ========================================================================
    
    /// Field name - mirrors Lua field_name
    pub field_name: Property<String>,
    /// Field type - mirrors Lua field_type
    pub field_type: Property<BmsFieldType>,
    
    // ========================================================================
    // SIZE PROPERTIES
    // ========================================================================
    
    /// Field height (rows) - mirrors Lua field_height
    pub field_height: ConstrainedProperty<u16>,
    /// Field width (columns) - mirrors Lua field_width
    pub field_width: ConstrainedProperty<u16>,
    /// Minimum height - mirrors Lua field_min_height
    pub field_min_height: Property<u16>,
    /// Maximum height - mirrors Lua field_max_height
    pub field_max_height: Property<u16>,
    /// Minimum width - mirrors Lua field_min_width
    pub field_min_width: Property<u16>,
    /// Maximum width - mirrors Lua field_max_width
    pub field_max_width: Property<u16>,
    
    // ========================================================================
    // POSITION PROPERTIES
    // ========================================================================
    
    /// Field position - mirrors Lua field_pos
    pub field_pos: Property<Position>,
    
    // ========================================================================
    // COLOR PROPERTIES
    // ========================================================================
    
    /// Available colors - mirrors Lua field_avail_color
    pub field_avail_color: Property<Vec<Color>>,
    /// Border color - mirrors Lua field_border_color
    pub field_border_color: EnumProperty<Color>,
    /// Title color - mirrors Lua field_title_color
    pub field_title_color: EnumProperty<Color>,
    /// Text color - mirrors Lua field_text_color
    pub field_text_color: EnumProperty<Color>,
    /// Available footer color - mirrors Lua field_avail_footer_color
    pub field_avail_footer_color: Property<Vec<Color>>,
    /// Footer color - mirrors Lua field_footer_color
    pub field_footer_color: EnumProperty<Color>,
    
    // ========================================================================
    // FONT PROPERTIES
    // ========================================================================
    
    /// Available font family - mirrors Lua field_avail_font_family
    pub field_avail_font_family: Property<Vec<String>>,
    /// Font family - mirrors Lua field_font_family
    pub field_font_family: Property<String>,
    
    // ========================================================================
    // STYLE PROPERTIES
    // ========================================================================
    
    /// Available styles - mirrors Lua field_avail_style
    pub field_avail_style: Property<Vec<TextStyle>>,
    /// Field style - mirrors Lua field_style
    pub field_style: EnumProperty<TextStyle>,
    
    // ========================================================================
    // ALIGNMENT PROPERTIES
    // ========================================================================
    
    /// Available text alignment - mirrors Lua field_avail_text_align
    pub field_avail_text_align: Property<Vec<TextAlign>>,
    /// Text alignment - mirrors Lua field_text_align
    pub field_text_align: EnumProperty<TextAlign>,
    /// Title alignment - mirrors Lua field_title_align
    pub field_title_align: EnumProperty<TextAlign>,
    /// Vertical alignment - mirrors Lua field_vertical_align
    pub field_vertical_align: EnumProperty<VerticalAlign>,
    /// Footer alignment - mirrors Lua field_footer_align
    pub field_footer_align: EnumProperty<TextAlign>,
    
    // ========================================================================
    // BORDER PROPERTIES
    // ========================================================================
    
    /// Available border characters - mirrors Lua field_avail_border_chars
    pub field_avail_border_chars: Property<BorderChars>,
    /// Available border style - mirrors Lua field_avail_border_style
    pub field_avail_border_style: Property<Vec<BorderStyle>>,
    /// Border configuration - mirrors Lua field_border
    pub field_border: Property<FieldBorder>,
    /// Border style - mirrors Lua field_border_style
    pub field_border_style: EnumProperty<BorderStyle>,
    
    // ========================================================================
    // FILL CHARACTER PROPERTIES
    // ========================================================================
    
    /// Title fill character - mirrors Lua field_title_fill_char
    pub field_title_fill_char: EnumProperty<FillChar>,
    /// Fill character - mirrors Lua field_fill_char
    pub field_fill_char: EnumProperty<FillChar>,
    /// Footer fill character - mirrors Lua field_footer_fill_char
    pub field_footer_fill_char: EnumProperty<FillChar>,
    
    // ========================================================================
    // MARKER PROPERTIES
    // ========================================================================
    
    /// Available required marker - mirrors Lua field_avail_required_marker
    pub field_avail_required_marker: Property<Vec<Marker>>,
    /// Required marker - mirrors Lua field_required_marker
    pub field_required_marker: Property<Marker>,
    /// Footer required marker - mirrors Lua field_footer_required_marker
    pub field_footer_required_marker: Property<Marker>,
    /// Available error marker - mirrors Lua field_avail_error_marker
    pub field_avail_error_marker: Property<Vec<Marker>>,
    /// Error marker - mirrors Lua field_error_marker
    pub field_error_marker: Property<Marker>,
    /// Footer error marker - mirrors Lua field_footer_error_marker
    pub field_footer_error_marker: Property<Marker>,
    
    // ========================================================================
    // PREFIX/SUFFIX PROPERTIES
    // ========================================================================
    
    /// Title prefix - mirrors Lua field_title_prefix
    pub field_title_prefix: Property<PrefixSuffix>,
    /// Title suffix - mirrors Lua field_title_suffix
    pub field_title_suffix: Property<PrefixSuffix>,
    
    // ========================================================================
    // FOOTER PROPERTIES
    // ========================================================================
    
    /// Footer title - mirrors Lua field_footer_title
    pub field_footer_title: Property<String>,
    /// Footer configuration - mirrors Lua field_footer
    pub field_footer: Property<Footer>,
    
    // ========================================================================
    // CHILDREN PROPERTIES (for Fieldset/Group)
    // ========================================================================
    
    /// Child fields - mirrors Lua field_children
    pub field_children: Property<Vec<BmsField>>,
    
    // ========================================================================
    // ATTRIBUTE PROPERTIES
    // ========================================================================
    
    /// Field attributes - mirrors Lua field_attrb
    pub field_attrb: Property<FieldAttributes>,
    
    // ========================================================================
    // VALUE PROPERTIES
    // ========================================================================
    
    /// Initial value - mirrors Lua field_initial
    pub field_initial: Property<FieldInitialValue>,
    
    // ========================================================================
    // VERTICAL MARGIN PROPERTIES
    // ========================================================================
    
    /// Vertical margin - mirrors Lua field_vertical_margin
    pub field_vertical_margin: EnumProperty<VerticalMargin>,
    
    // ========================================================================
    // VISUAL REPRESENTATION
    // ========================================================================
    
    /// Visual representation - mirrors Lua visual_representation
    pub visual_representation: Property<VisualRepresentation>,
}

impl BmsField {
    /// Create a new field with defaults for the given type
    /// 
    /// This mirrors Lua's OBJECTS_DEFINITIONS.new() constructor.
    /// It initializes all properties with appropriate defaults for the field type.
    pub fn new(field_type: BmsFieldType, defaults: &BmsDefaults) -> Self {
        // Get size defaults for this field type
        let size = &defaults.size;
        
        // Create the field with all properties initialized
        Self {
            // Identification
            field_name: Property::new(field_type.display_name().to_string()),
            field_type: Property::new(field_type),
            
            // Size properties
            field_height: size.height_property(field_type),
            field_width: size.width_property(field_type),
            field_min_height: Property::new(size.get_min_height(field_type)),
            field_max_height: Property::new(size.get_max_height(field_type)),
            field_min_width: Property::new(size.get_min_width(field_type)),
            field_max_width: Property::new(size.get_max_width(field_type)),
            
            // Position
            field_pos: Property::new(Position::new(1, 1)),
            
            // Color properties
            field_avail_color: defaults.colors.available_colors_property(),
            field_border_color: defaults.colors.border_color_property(field_type),
            field_title_color: defaults.colors.title_color_property(field_type),
            field_text_color: defaults.colors.text_color_property(field_type),
            field_avail_footer_color: Property::new(defaults.colors.get_avail_footer_colors(field_type).clone()),
            field_footer_color: defaults.colors.footer_color_property(field_type),
            
            // Font properties
            field_avail_font_family: Property::new(vec!["default".to_string()]),
            field_font_family: Property::new("default".to_string()),
            
            // Style properties
            field_avail_style: defaults.styles.available_styles_property(),
            field_style: defaults.styles.style_property(field_type),
            
            // Alignment properties
            field_avail_text_align: Property::new(vec![TextAlign::Left, TextAlign::Center, TextAlign::Right]),
            field_text_align: defaults.align.text_align_property(field_type),
            field_title_align: defaults.align.title_align_property(field_type),
            field_vertical_align: defaults.vertical_align.vertical_align_property(field_type),
            field_footer_align: defaults.align.footer_align_property(field_type),
            
            // Border properties
            field_avail_border_chars: Property::new(BorderChars::new()),
            field_avail_border_style: Property::new(defaults.border.get_available_border_styles(field_type).clone()),
            field_border: Property::new(FieldBorder::for_style(defaults.border.get_default_border_style(field_type))),
            field_border_style: defaults.border.border_style_property(field_type),
            
            // Fill character properties
            field_title_fill_char: defaults.fill.title_fill_char_property(field_type),
            field_fill_char: defaults.fill.fill_char_property(field_type),
            field_footer_fill_char: defaults.fill.footer_fill_char_property(field_type),
            
            // Marker properties
            field_avail_required_marker: Property::new(vec![Marker::required(), Marker::none()]),
            field_required_marker: Property::new(Marker::none()),
            field_footer_required_marker: Property::new(Marker::none()),
            field_avail_error_marker: Property::new(vec![Marker::error(), Marker::none()]),
            field_error_marker: Property::new(Marker::none()),
            field_footer_error_marker: Property::new(Marker::none()),
            
            // Prefix/Suffix properties
            field_title_prefix: Property::new(PrefixSuffix::none()),
            field_title_suffix: Property::new(PrefixSuffix::none()),
            
            // Footer properties
            field_footer_title: Property::new(String::new()),
            field_footer: Property::new(Footer::none()),
            
            // Children properties
            field_children: Property::new(Vec::new()),
            
            // Attribute properties
            field_attrb: Property::new(FieldAttributes::default_for(field_type)),
            
            // Value properties
            field_initial: Property::new(FieldInitialValue::default_for(field_type)),
            
            // Vertical margin
            field_vertical_margin: defaults.vertical_margin.vertical_margin_property(field_type),
            
            // Visual representation
            visual_representation: Property::new(VisualRepresentation::default_for(field_type)),
        }
    }
    
    /// Create a new field with overrides for specific properties
    /// 
    /// This allows creating a field with custom initial values for specific properties.
    pub fn new_with_overrides(
        field_type: BmsFieldType,
        defaults: &BmsDefaults,
        overrides: &FieldOverrides
    ) -> Self {
        let mut field = Self::new(field_type, defaults);
        field.apply_overrides(overrides);
        field
    }
    
    /// Apply overrides to field properties
    pub fn apply_overrides(&mut self, overrides: &FieldOverrides) {
        // Apply each override
        if let Some(name) = &overrides.name {
            self.field_name.set(name.clone());
        }
        if let Some(pos) = overrides.pos {
            self.field_pos.set(pos);
        }
        if let Some(width) = overrides.width {
            let _ = self.field_width.set(width);
        }
        if let Some(height) = overrides.height {
            let _ = self.field_height.set(height);
        }
        if let Some(text_color) = overrides.text_color {
            let _ = self.field_text_color.set(text_color);
        }
        if let Some(initial) = &overrides.initial {
            self.field_initial.set(initial.clone());
        }
        // Add more overrides as needed
    }
    
    // ========================================================================
    // PROPERTY ACCESS HELPERS
    // ========================================================================
    
    /// Get field type
    pub fn get_field_type(&self) -> BmsFieldType {
        self.field_type.get()
    }
    
    /// Get field name
    pub fn get_field_name(&self) -> String {
        self.field_name.get()
    }
    
    /// Get position
    pub fn get_pos(&self) -> Position {
        self.field_pos.get()
    }
    
    /// Get width
    pub fn get_width(&self) -> u16 {
        self.field_width.get()
    }
    
    /// Get height
    pub fn get_height(&self) -> u16 {
        self.field_height.get()
    }
    
    /// Get text color
    pub fn get_text_color(&self) -> Color {
        self.field_text_color.get()
    }
    
    /// Get border style
    pub fn get_border_style(&self) -> BorderStyle {
        self.field_border_style.get()
    }
    
    /// Get text alignment
    pub fn get_text_align(&self) -> TextAlign {
        self.field_text_align.get()
    }
    
    /// Get initial value
    pub fn get_initial(&self) -> FieldInitialValue {
        self.field_initial.get()
    }
    
    /// Get attributes
    pub fn get_attrb(&self) -> FieldAttributes {
        self.field_attrb.get()
    }
    
    /// Get GUI field type for rendering
    pub fn get_gui_field_type(&self) -> GuiFieldType {
        self.get_field_type().gui_field_type()
    }
    
    // ========================================================================
    // PROPERTY MODIFICATION HELPERS
    // ========================================================================
    
    /// Set field name
    pub fn set_field_name(&mut self, name: impl Into<String>) {
        self.field_name.set(name.into());
    }
    
    /// Set position
    pub fn set_pos(&mut self, pos: Position) {
        self.field_pos.set(pos);
    }
    
    /// Set width
    pub fn set_width(&mut self, width: u16) -> Result<(), String> {
        self.field_width.set(width)
    }
    
    /// Set height
    pub fn set_height(&mut self, height: u16) -> Result<(), String> {
        self.field_height.set(height)
    }
    
    /// Set text color
    pub fn set_text_color(&mut self, color: Color) -> Result<(), String> {
        self.field_text_color.set(color)
    }
    
    /// Set border style
    pub fn set_border_style(&mut self, style: BorderStyle) -> Result<(), String> {
        self.field_border_style.set(style)
    }
    
    /// Set text alignment
    pub fn set_text_align(&mut self, align: TextAlign) -> Result<(), String> {
        self.field_text_align.set(align)
    }
    
    /// Set initial value
    pub fn set_initial(&mut self, value: FieldInitialValue) {
        self.field_initial.set(value);
    }
    
    /// Set required
    pub fn set_required(&mut self, required: bool) {
        let mut attrs = self.get_attrb();
        attrs.field_required = required;
        self.field_attrb.set(attrs);
    }
    
    /// Set has error
    pub fn set_has_error(&mut self, has_error: bool) {
        let mut attrs = self.get_attrb();
        attrs.field_has_error = has_error;
        self.field_attrb.set(attrs);
    }
    
    // ========================================================================
    // UTILITY METHODS
    // ========================================================================
    
    /// Check if field is editable
    pub fn is_editable(&self) -> bool {
        self.get_attrb().is_editable()
    }
    
    /// Check if field is protected
    pub fn is_protected(&self) -> bool {
        self.get_attrb().is_protected()
    }
    
    /// Check if field can have children (Fieldset/Group)
    pub fn can_have_children(&self) -> bool {
        self.get_field_type().can_have_children()
    }
    
    /// Check if any property has been edited
    pub fn has_edits(&self) -> bool {
        // Check all properties for edited state
        // This is a simplified check - in practice you might want to check
        // specific properties or have a more sophisticated change tracking system
        
        // For now, check a few key properties
        if self.field_name.is_edited() { return true; }
        if self.field_pos.is_edited() { return true; }
        if self.field_width.is_edited() { return true; }
        if self.field_height.is_edited() { return true; }
        if self.field_text_color.is_edited() { return true; }
        if self.field_border_style.is_edited() { return true; }
        if self.field_text_align.is_edited() { return true; }
        if self.field_initial.is_edited() { return true; }
        if self.field_attrb.is_edited() { return true; }
        
        false
    }
    
    /// Reset all properties to initial values
    pub fn reset(&mut self) {
        // Reset all properties - this is a simplified version
        // In a full implementation, you'd iterate through all properties
        self.field_name.reset();
        self.field_pos.reset();
        self.field_width.reset();
        self.field_height.reset();
        self.field_text_color.reset();
        self.field_border_style.reset();
        self.field_text_align.reset();
        self.field_initial.reset();
        self.field_attrb.reset();
        // ... reset all other properties
    }
    
    /// Get a reference to the children
    pub fn children(&self) -> &Vec<BmsField> {
        &self.field_children.get()
    }
    
    /// Get mutable reference to children
    pub fn children_mut(&mut self) -> &mut Vec<BmsField> {
        &mut self.field_children.edited.get_or_insert_with(|| self.field_children.initial.clone())
    }
    
    /// Add a child field (for Fieldset/Group)
    pub fn add_child(&mut self, child: BmsField) {
        let mut children = self.field_children.get();
        children.push(child);
        self.field_children.set(children);
    }
    
    /// Remove a child field by index
    pub fn remove_child(&mut self, index: usize) -> Option<BmsField> {
        let mut children = self.field_children.get();
        if index < children.len() {
            let child = children.remove(index);
            self.field_children.set(children);
            Some(child)
        } else {
            None
        }
    }
    
    /// Get display value for rendering
    pub fn get_display_value(&self) -> String {
        match self.get_initial() {
            FieldInitialValue::Text(s) => s,
            FieldInitialValue::Boolean(b) => if b { "[X]".to_string() } else { "[ ]".to_string() },
            FieldInitialValue::Numeric(n) => n.to_string(),
            FieldInitialValue::AsciiArt(a) => a.ascii_code.join("\n"),
            FieldInitialValue::None => String::new(),
        }
    }
    
    /// Get title for rendering (name or initial value)
    pub fn get_title(&self) -> String {
        let name = self.get_field_name();
        if !name.is_empty() && name != self.get_field_type().display_name() {
            return name;
        }
        
        // For Fieldset, use initial value as title
        if self.get_field_type() == BmsFieldType::Fieldset || self.get_field_type() == BmsFieldType::Group {
            if let FieldInitialValue::Text(s) = self.get_initial() {
                if !s.is_empty() {
                    return s;
                }
            }
        }
        
        self.get_field_type().display_name().to_string()
    }
}

impl Default for BmsField {
    fn default() -> Self {
        // Create with default field type
        Self::new(BmsFieldType::default(), &BmsDefaults::default())
    }
}

impl fmt::Display for BmsField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BmsField({} at {} size {}x{} color={})",
            self.get_field_name(),
            self.get_pos(),
            self.get_width(),
            self.get_height(),
            self.get_text_color()
        )
    }
}

// ============================================================================
// FIELD OVER RIDES
// ============================================================================

/// Overrides for field creation
#[derive(Debug, Clone, Default)]
pub struct FieldOverrides {
    pub name: Option<String>,
    pub field_type: Option<BmsFieldType>,
    pub pos: Option<Position>,
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub text_color: Option<Color>,
    pub border_color: Option<Color>,
    pub title_color: Option<Color>,
    pub border_style: Option<BorderStyle>,
    pub text_align: Option<TextAlign>,
    pub initial: Option<FieldInitialValue>,
    pub required: Option<bool>,
    pub readonly: Option<bool>,
    pub protected: Option<bool>,
    pub numeric: Option<bool>,
    pub children: Option<Vec<BmsField>>,
}

impl FieldOverrides {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    pub fn with_pos(mut self, pos: Position) -> Self {
        self.pos = Some(pos);
        self
    }
    
    pub fn with_width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }
    
    pub fn with_height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }
    
    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }
    
    pub fn with_initial(mut self, initial: FieldInitialValue) -> Self {
        self.initial = Some(initial);
        self
    }
}

// ============================================================================
// CONVERSION FROM LEGACY
// ============================================================================

impl From<&crate::bms::model::BmsField> for BmsField {
    fn from(legacy: &crate::bms::model::BmsField) -> Self {
        let defaults = BmsDefaults::default();
        let mut field = BmsField::new(BmsFieldType::FieldTextORNumeric, &defaults);
        
        // Convert from legacy field
        field.field_name.set(legacy.name.clone());
        field.field_pos.set(Position::new(legacy.pos.0, legacy.pos.1));
        field.field_width.set(legacy.length).ok();
        
        // Set text color if present
        if let Some(color) = &legacy.text_color {
            field.field_text_color.set(*color).ok();
        }
        
        // Set initial value if present
        if let Some(initial) = &legacy.initial {
            field.field_initial.set(FieldInitialValue::Text(initial.clone()));
        }
        
        // Convert attributes
        let mut attrs = FieldAttributes::default();
        for attr in &legacy.attrb {
            match attr {
                crate::bms::model::FieldAttribute::Prot => attrs.field_protected = true,
                crate::bms::model::FieldAttribute::Unprot => attrs.field_protected = false,
                crate::bms::model::FieldAttribute::Norm => {}
                crate::bms::model::FieldAttribute::Intens => {}
                crate::bms::model::FieldAttribute::Dark => {}
                crate::bms::model::FieldAttribute::Num => attrs.field_numeric = true,
                crate::bms::model::FieldAttribute::Alph => {}
                crate::bms::model::FieldAttribute::AlphaNum => {}
                crate::bms::model::FieldAttribute::Bool => {}
                crate::bms::model::FieldAttribute::Date => {}
                crate::bms::model::FieldAttribute::Time => {}
                crate::bms::model::FieldAttribute::Float => {}
                crate::bms::model::FieldAttribute::Signed => {}
                crate::bms::model::FieldAttribute::Packed => {}
                crate::bms::model::FieldAttribute::Binary => {}
                crate::bms::model::FieldAttribute::Blink => {}
                crate::bms::model::FieldAttribute::Reverse => {}
                crate::bms::model::FieldAttribute::Underline => {}
                crate::bms::model::FieldAttribute::Left => attrs.field_text_align = Some(TextAlign::Left),
                crate::bms::model::FieldAttribute::Right => attrs.field_text_align = Some(TextAlign::Right),
                crate::bms::model::FieldAttribute::Center => attrs.field_text_align = Some(TextAlign::Center),
                _ => {}
            }
        }
        
        // Set readonly based on attributes
        attrs.field_readonly = legacy.attrb.contains(&crate::bms::model::FieldAttribute::Prot);
        
        field.field_attrb.set(attrs);
        
        field
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_field() {
        let defaults = BmsDefaults::default();
        let field = BmsField::new(BmsFieldType::FieldTextORNumeric, &defaults);
        
        assert_eq!(field.get_field_type(), BmsFieldType::FieldTextORNumeric);
        assert_eq!(field.get_width(), 10); // Default width for FieldTextORNumeric
        assert_eq!(field.get_height(), 3); // Default height
        assert_eq!(field.get_pos(), Position::new(1, 1));
    }

    #[test]
    fn test_field_properties() {
        let defaults = BmsDefaults::default();
        let mut field = BmsField::new(BmsFieldType::Literal, &defaults);
        
        assert_eq!(field.get_field_type(), BmsFieldType::Literal);
        assert_eq!(field.get_width(), 20); // Default width for Literal
        
        field.set_width(30).unwrap();
        assert_eq!(field.get_width(), 30);
        
        field.set_pos(Position::new(5, 10));
        assert_eq!(field.get_pos(), Position::new(5, 10));
    }

    #[test]
    fn test_field_defaults() {
        let defaults = BmsDefaults::default();
        
        // Test FieldTextORNumeric defaults
        let field = BmsField::new(BmsFieldType::FieldTextORNumeric, &defaults);
        assert_eq!(field.get_width(), 10);
        assert_eq!(field.get_height(), 3);
        
        // Test Literal defaults
        let literal = BmsField::new(BmsFieldType::Literal, &defaults);
        assert_eq!(literal.get_width(), 20);
        
        // Test Fieldset defaults
        let fieldset = BmsField::new(BmsFieldType::Fieldset, &defaults);
        assert_eq!(fieldset.get_height(), 3);
    }

    #[test]
    fn test_field_can_have_children() {
        let defaults = BmsDefaults::default();
        
        let fieldset = BmsField::new(BmsFieldType::Fieldset, &defaults);
        assert!(fieldset.can_have_children());
        
        let field = BmsField::new(BmsFieldType::FieldTextORNumeric, &defaults);
        assert!(!field.can_have_children());
    }

    #[test]
    fn test_field_overrides() {
        let defaults = BmsDefaults::default();
        let overrides = FieldOverrides::new()
            .with_name("MyField")
            .with_pos(Position::new(2, 5))
            .with_width(15)
            .with_text_color(Color::Red);
        
        let field = BmsField::new_with_overrides(BmsFieldType::FieldTextORNumeric, &defaults, &overrides);
        
        assert_eq!(field.get_field_name(), "MyField");
        assert_eq!(field.get_pos(), Position::new(2, 5));
        assert_eq!(field.get_width(), 15);
    }
}
