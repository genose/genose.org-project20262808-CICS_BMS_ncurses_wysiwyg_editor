//! Object definitions for BMS fields - mirrors Lua OBJECTS_DEFINITIONS structure
//!
//! This module provides the comprehensive object definitions that mirror the Lua
//! OBJECTS_DEFINITIONS table, including all property definitions, default values,
//! and available options for each field type.

use super::{
    field_types::BmsFieldType,
    types::{Color, BorderStyle, TextAlign, VerticalAlign, FillChar, TextStyle, BorderCharSet},
    properties::Property,
    defaults::FieldObjectDefaults,
    render::FieldObject,
};
use std::collections::HashMap;

/// Main OBJECTS_DEFINITIONS structure that contains all property definitions
/// Mirrors the Lua OBJECTS_DEFINITIONS table
#[derive(Debug, Clone)]
pub struct ObjectDefinitions {
    // Property definitions by property name
    pub properties: HashMap<String, PropertyDefinition>,
    
    // Default values for each field type
    pub defaults: HashMap<BmsFieldType, FieldTypeDefaults>,
    
    // GUI field type mappings
    pub gui_field_types: HashMap<String, GuiFieldType>,
    
    // Available options for various properties
    pub available_options: AvailableOptions,
}

/// Property definition structure
#[derive(Debug, Clone)]
pub struct PropertyDefinition {
    pub name: String,
    pub gui_field_type: Option<GuiFieldType>,
    pub gui_field_name: Option<String>,
    pub collapsed: bool,
    pub collapsable: bool,
    pub description: Option<String>,
    pub category: PropertyCategory,
    
    // Property type information
    pub property_type: PropertyType,
    
    // Default values per field type
    pub defaults: HashMap<BmsFieldType, PropertyValue>,
    
    // Available values for enum properties
    pub available_values: Option<Vec<PropertyValue>>,
    
    // For constrained properties
    pub constraints: Option<PropertyConstraints>,
}

/// Property value that can hold different types
#[derive(Debug, Clone)]
pub enum PropertyValue {
    String(String),
    Number(i32),
    Float(f32),
    Boolean(bool),
    Color(Color),
    BorderStyle(BorderStyle),
    TextAlign(TextAlign),
    VerticalAlign(VerticalAlign),
    FillChar(FillChar),
    TextStyle(TextStyle),
    BorderCharSet(BorderCharSet),
    // Add more types as needed
}

impl From<String> for PropertyValue {
    fn from(s: String) -> Self {
        PropertyValue::String(s)
    }
}

impl From<&str> for PropertyValue {
    fn from(s: &str) -> Self {
        PropertyValue::String(s.to_string())
    }
}

impl From<i32> for PropertyValue {
    fn from(n: i32) -> Self {
        PropertyValue::Number(n)
    }
}

impl From<u16> for PropertyValue {
    fn from(n: u16) -> Self {
        PropertyValue::Number(n as i32)
    }
}

impl From<bool> for PropertyValue {
    fn from(b: bool) -> Self {
        PropertyValue::Boolean(b)
    }
}

impl From<Color> for PropertyValue {
    fn from(c: Color) -> Self {
        PropertyValue::Color(c)
    }
}

impl From<BorderStyle> for PropertyValue {
    fn from(bs: BorderStyle) -> Self {
        PropertyValue::BorderStyle(bs)
    }
}

impl From<TextAlign> for PropertyValue {
    fn from(ta: TextAlign) -> Self {
        PropertyValue::TextAlign(ta)
    }
}

impl From<VerticalAlign> for PropertyValue {
    fn from(va: VerticalAlign) -> Self {
        PropertyValue::VerticalAlign(va)
    }
}

impl From<FillChar> for PropertyValue {
    fn from(fc: FillChar) -> Self {
        PropertyValue::FillChar(fc)
    }
}

impl From<TextStyle> for PropertyValue {
    fn from(ts: TextStyle) -> Self {
        PropertyValue::TextStyle(ts)
    }
}

impl From<BorderCharSet> for PropertyValue {
    fn from(bcs: BorderCharSet) -> Self {
        PropertyValue::BorderCharSet(bcs)
    }
}

/// Property type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    String,
    Number,
    Boolean,
    Enum,
    Color,
    BorderStyle,
    TextAlign,
    VerticalAlign,
    FillChar,
    TextStyle,
    Position,
    Size,
    BorderCharSet,
    // Add more types as needed
}

/// Property constraints
#[derive(Debug, Clone, Default)]
pub struct PropertyConstraints {
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub min_length: Option<u16>,
    pub max_length: Option<u16>,
    pub pattern: Option<String>,
}

/// Property category for UI organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyCategory {
    Dimensions,
    Colors,
    Font,
    Style,
    Alignment,
    Position,
    Borders,
    Fill,
    Markers,
    Attributes,
    Values,
    Children,
    Visual,
    Other,
}

/// GUI field type for rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuiFieldType {
    SelectWithLabelString,
    SelectWithLabelNumeric,
    ListTextOrNumWithLabelField,
    CheckboxWithLabelField,
    TextWithLabelField,
    TextField,
    LiteralField,
    ProtectedLiteralField,
    BooleanField,
    ImageField,
    LineField,
    FieldsetField,
}

impl GuiFieldType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GuiFieldType::SelectWithLabelString => "gui_select_with_label_string",
            GuiFieldType::SelectWithLabelNumeric => "gui_select_with_label_numeric",
            GuiFieldType::ListTextOrNumWithLabelField => "gui_list_textornum_with_label_field",
            GuiFieldType::CheckboxWithLabelField => "gui_checkbox_with_label_field",
            GuiFieldType::TextWithLabelField => "gui_text_with_label_field",
            GuiFieldType::TextField => "gui_text_field",
            GuiFieldType::LiteralField => "gui_literal_field",
            GuiFieldType::ProtectedLiteralField => "gui_protected_literal_field",
            GuiFieldType::BooleanField => "gui_boolean_field",
            GuiFieldType::ImageField => "gui_image_field",
            GuiFieldType::LineField => "gui_line_field",
            GuiFieldType::FieldsetField => "gui_fieldset_field",
        }
    }
}

/// Available options for various property types
#[derive(Debug, Clone)]
pub struct AvailableOptions {
    pub colors: Vec<Color>,
    pub border_styles: Vec<BorderStyle>,
    pub text_aligns: Vec<TextAlign>,
    pub vertical_aligns: Vec<VerticalAlign>,
    pub fill_chars: Vec<FillChar>,
    pub text_styles: Vec<TextStyle>,
    pub boolean_values: Vec<bool>,
    // Add more as needed
}

impl AvailableOptions {
    pub fn new() -> Self {
        Self {
            colors: vec![
                Color::Default, Color::Black, Color::Red, Color::Green,
                Color::Yellow, Color::Blue, Color::Magenta, Color::Cyan, Color::White,
                Color::Gray, Color::Turquoise, Color::Pink, Color::Orange, Color::Purple,
            ],
            border_styles: vec![
                BorderStyle::None, BorderStyle::Single, BorderStyle::Double,
                BorderStyle::Solid, BorderStyle::Dashed, BorderStyle::Dotted,
            ],
            text_aligns: vec![TextAlign::Left, TextAlign::Center, TextAlign::Right],
            vertical_aligns: vec![VerticalAlign::Top, VerticalAlign::Middle, VerticalAlign::Bottom],
            fill_chars: vec![
                FillChar::Space, FillChar::Dash, FillChar::Equal, FillChar::Underscore,
                FillChar::Dot, FillChar::Asterisk, FillChar::Pipe, FillChar::Exclamation,
                FillChar::Plus, FillChar::Question, FillChar::LessThan, FillChar::GreaterThan,
            ],
            text_styles: vec![
                TextStyle::Default, TextStyle::Bold, TextStyle::Italic,
                TextStyle::Underline, TextStyle::StrikeThrough, TextStyle::Blink,
                TextStyle::Reverse,
            ],
            boolean_values: vec![true, false],
        }
    }
}

/// Default values for a field type
#[derive(Debug, Clone)]
pub struct FieldTypeDefaults {
    pub properties: HashMap<String, PropertyValue>,
}

impl FieldTypeDefaults {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
    
    pub fn with_property(mut self, name: &str, value: PropertyValue) -> Self {
        self.properties.insert(name.to_string(), value);
        self
    }
}

/// Main OBJECTS_DEFINITIONS implementation
impl ObjectDefinitions {
    /// Create new ObjectDefinitions with all default property definitions
    pub fn new() -> Self {
        let mut definitions = Self {
            properties: HashMap::new(),
            defaults: HashMap::new(),
            gui_field_types: HashMap::new(),
            available_options: AvailableOptions::new(),
        };
        
        // Initialize all property definitions
        definitions.initialize_property_definitions();
        definitions.initialize_gui_field_types();
        definitions.initialize_field_type_defaults();
        
        definitions
    }
    
    /// Initialize all property definitions
    fn initialize_property_definitions(&mut self) {
        // Field name property
        self.add_property("field_name", 
            PropertyDefinition {
                name: "field_name".to_string(),
                gui_field_type: Some(GuiFieldType::TextWithLabelField),
                gui_field_name: Some("Name".to_string()),
                collapsed: false,
                collapsable: true,
                description: Some("Name of the field".to_string()),
                category: PropertyCategory::Values,
                property_type: PropertyType::String,
                defaults: HashMap::new(),
                available_values: None,
                constraints: Some(PropertyConstraints {
                    min_length: Some(1),
                    max_length: Some(64),
                    ..Default::default()
                }),
            }
        );
        
        // Field type property
        self.add_property("field_type",
            PropertyDefinition {
                name: "field_type".to_string(),
                gui_field_type: Some(GuiFieldType::SelectWithLabelString),
                gui_field_name: Some("Type".to_string()),
                collapsed: false,
                collapsable: true,
                description: Some("Type of the field".to_string()),
                category: PropertyCategory::Values,
                property_type: PropertyType::Enum,
                defaults: HashMap::new(),
                available_values: Some(vec![
                    PropertyValue::String("FieldTextORNumeric".to_string()),
                    PropertyValue::String("Literal".to_string()),
                    PropertyValue::String("ProtectedLiteral".to_string()),
                    PropertyValue::String("BooleanField".to_string()),
                    PropertyValue::String("ImageAsciiArt".to_string()),
                    PropertyValue::String("Line".to_string()),
                    PropertyValue::String("Fieldset".to_string()),
                ]),
                constraints: None,
            }
        );
        
        // Dimensions properties
        self.add_numeric_property("field_height", "Height", PropertyCategory::Dimensions, 1, 80, Some(3));
        self.add_numeric_property("field_width", "Width", PropertyCategory::Dimensions, 1, 255, Some(10));
        self.add_numeric_property("field_min_height", "Min Height", PropertyCategory::Dimensions, 1, 80, None);
        self.add_numeric_property("field_max_height", "Max Height", PropertyCategory::Dimensions, 1, 80, None);
        self.add_numeric_property("field_width_min", "Min Width", PropertyCategory::Dimensions, 1, 255, None);
        self.add_numeric_property("field_width_max", "Max Width", PropertyCategory::Dimensions, 1, 255, None);
        
        // Color properties
        self.add_color_property("field_border_color", "Border Color", PropertyCategory::Colors);
        self.add_color_property("field_title_color", "Title Color", PropertyCategory::Colors);
        self.add_color_property("field_text_color", "Text Color", PropertyCategory::Colors);
        self.add_color_property("field_footer_color", "Footer Color", PropertyCategory::Colors);
        self.add_color_property("field_avail_footer_color", "Available Footer Color", PropertyCategory::Colors);
        
        // Style properties
        self.add_style_property("field_style", "Style", PropertyCategory::Style);
        self.add_enum_property("field_avail_style", "Available Styles", PropertyCategory::Style, 
            self.available_options.text_styles.clone().into_iter().map(PropertyValue::from).collect()
        );
        
        // Alignment properties
        self.add_enum_property("field_text_align", "Text Align", PropertyCategory::Alignment,
            self.available_options.text_aligns.clone().into_iter().map(PropertyValue::from).collect()
        );
        self.add_enum_property("field_title_align", "Title Align", PropertyCategory::Alignment,
            self.available_options.text_aligns.clone().into_iter().map(PropertyValue::from).collect()
        );
        self.add_enum_property("field_vertical_align", "Vertical Align", PropertyCategory::Alignment,
            self.available_options.vertical_aligns.clone().into_iter().map(PropertyValue::from).collect()
        );
        
        // Position properties
        self.add_property("field_pos",
            PropertyDefinition {
                name: "field_pos".to_string(),
                gui_field_type: Some(GuiFieldType::TextWithLabelField),
                gui_field_name: Some("Position".to_string()),
                collapsed: false,
                collapsable: true,
                description: Some("Position (row, col) of the field".to_string()),
                category: PropertyCategory::Position,
                property_type: PropertyType::Position,
                defaults: HashMap::new(),
                available_values: None,
                constraints: None,
            }
        );
        
        // Border properties
        self.add_enum_property("field_border_style", "Border Style", PropertyCategory::Borders,
            self.available_options.border_styles.clone().into_iter().map(PropertyValue::from).collect()
        );
        
        self.add_property("field_avail_border_chars",
            PropertyDefinition {
                name: "field_avail_border_chars".to_string(),
                gui_field_type: None,
                gui_field_name: None,
                collapsed: true,
                collapsable: true,
                description: Some("Available border characters".to_string()),
                category: PropertyCategory::Borders,
                property_type: PropertyType::BorderCharSet,
                defaults: HashMap::new(),
                available_values: None,
                constraints: None,
            }
        );
        
        // Fill properties
        self.add_enum_property("field_fill_char", "Fill Char", PropertyCategory::Fill,
            self.available_options.fill_chars.clone().into_iter().map(PropertyValue::from).collect()
        );
        self.add_enum_property("field_title_fill_char", "Title Fill Char", PropertyCategory::Fill,
            self.available_options.fill_chars.clone().into_iter().map(PropertyValue::from).collect()
        );
        self.add_enum_property("field_footer_fill_char", "Footer Fill Char", PropertyCategory::Fill,
            self.available_options.fill_chars.clone().into_iter().map(PropertyValue::from).collect()
        );
        
        // Vertical margin
        self.add_enum_property("field_vertical_margin", "Vertical Margin", PropertyCategory::Borders,
            vec![
                PropertyValue::Number(0),  // None
                PropertyValue::Number(1),  // Small
                PropertyValue::Number(2),  // Medium
                PropertyValue::Number(3),  // Large
            ]
        );
        
        // Markers
        self.add_marker_property("field_required_marker", "Required Marker", PropertyCategory::Markers);
        self.add_marker_property("field_error_marker", "Error Marker", PropertyCategory::Markers);
        self.add_marker_property("field_avail_required_marker", "Available Required Marker", PropertyCategory::Markers);
        self.add_marker_property("field_avail_error_marker", "Available Error Marker", PropertyCategory::Markers);
        self.add_marker_property("field_footer_required_marker", "Footer Required Marker", PropertyCategory::Markers);
        self.add_marker_property("field_footer_error_marker", "Footer Error Marker", PropertyCategory::Markers);
        
        // Prefix/Suffix
        self.add_property("field_title_prefix",
            PropertyDefinition {
                name: "field_title_prefix".to_string(),
                gui_field_type: None,
                gui_field_name: None,
                collapsed: true,
                collapsable: true,
                description: Some("Title prefix configuration".to_string()),
                category: PropertyCategory::Other,
                property_type: PropertyType::String,
                defaults: HashMap::new(),
                available_values: None,
                constraints: None,
            }
        );
        
        self.add_property("field_title_suffix",
            PropertyDefinition {
                name: "field_title_suffix".to_string(),
                gui_field_type: None,
                gui_field_name: None,
                collapsed: true,
                collapsable: true,
                description: Some("Title suffix configuration".to_string()),
                category: PropertyCategory::Other,
                property_type: PropertyType::String,
                defaults: HashMap::new(),
                available_values: None,
                constraints: None,
            }
        );
        
        // Footer
        self.add_property("field_footer",
            PropertyDefinition {
                name: "field_footer".to_string(),
                gui_field_type: None,
                gui_field_name: None,
                collapsed: true,
                collapsable: true,
                description: Some("Footer configuration".to_string()),
                category: PropertyCategory::Other,
                property_type: PropertyType::String,
                defaults: HashMap::new(),
                available_values: None,
                constraints: None,
            }
        );
        
        // Attributes
        self.add_property("field_attrb",
            PropertyDefinition {
                name: "field_attrb".to_string(),
                gui_field_type: None,
                gui_field_name: None,
                collapsed: true,
                collapsable: true,
                description: Some("Field attributes (required, error states)".to_string()),
                category: PropertyCategory::Attributes,
                property_type: PropertyType::Boolean,
                defaults: HashMap::new(),
                available_values: None,
                constraints: None,
            }
        );
        
        // Initial value
        self.add_property("field_initial",
            PropertyDefinition {
                name: "field_initial".to_string(),
                gui_field_type: Some(GuiFieldType::TextWithLabelField),
                gui_field_name: Some("Initial Value".to_string()),
                collapsed: false,
                collapsable: true,
                description: Some("Initial value of the field".to_string()),
                category: PropertyCategory::Values,
                property_type: PropertyType::String,
                defaults: HashMap::new(),
                available_values: None,
                constraints: None,
            }
        );
        
        // Visual representation
        self.add_property("visual_representation",
            PropertyDefinition {
                name: "visual_representation".to_string(),
                gui_field_type: None,
                gui_field_name: None,
                collapsed: true,
                collapsable: false,
                description: Some("Visual representation templates".to_string()),
                category: PropertyCategory::Visual,
                property_type: PropertyType::String,
                defaults: HashMap::new(),
                available_values: None,
                constraints: None,
            }
        );
        
        // Children
        self.add_property("field_children",
            PropertyDefinition {
                name: "field_children".to_string(),
                gui_field_type: None,
                gui_field_name: None,
                collapsed: true,
                collapsable: false,
                description: Some("Child fields for container types".to_string()),
                category: PropertyCategory::Children,
                property_type: PropertyType::String,
                defaults: HashMap::new(),
                available_values: None,
                constraints: None,
            }
        );
    }
    
    /// Add a property definition
    fn add_property(&mut self, name: &str, definition: PropertyDefinition) {
        self.properties.insert(name.to_string(), definition);
    }
    
    /// Helper to add numeric property
    fn add_numeric_property(&mut self, name: &str, gui_name: &str, category: PropertyCategory, min: i32, max: i32, default: Option<i32>) {
        let mut definition = PropertyDefinition {
            name: name.to_string(),
            gui_field_type: Some(GuiFieldType::TextWithLabelField),
            gui_field_name: Some(gui_name.to_string()),
            collapsed: false,
            collapsable: true,
            description: Some(format!("{} of the field", gui_name.to_lowercase())),
            category,
            property_type: PropertyType::Number,
            defaults: HashMap::new(),
            available_values: None,
            constraints: Some(PropertyConstraints {
                min: Some(min),
                max: Some(max),
                ..Default::default()
            }),
        };
        
        // Set defaults for each field type based on the Lua OBJECTS_DEFINITIONS_DEFAULTS
        if let Some(default_val) = default {
            for field_type in self.get_all_field_types() {
                definition.defaults.insert(field_type, PropertyValue::Number(default_val));
            }
        }
        
        self.add_property(name, definition);
    }
    
    /// Helper to add color property
    fn add_color_property(&mut self, name: &str, gui_name: &str, category: PropertyCategory) {
        let definition = PropertyDefinition {
            name: name.to_string(),
            gui_field_type: Some(GuiFieldType::SelectWithLabelString),
            gui_field_name: Some(gui_name.to_string()),
            collapsed: false,
            collapsable: true,
            description: Some(format!("{} for the field", gui_name.to_lowercase())),
            category,
            property_type: PropertyType::Color,
            defaults: self.get_color_defaults(name),
            available_values: Some(self.available_options.colors.clone().into_iter().map(PropertyValue::from).collect()),
            constraints: None,
        };
        
        self.add_property(name, definition);
    }
    
    /// Helper to add style property
    fn add_style_property(&mut self, name: &str, gui_name: &str, category: PropertyCategory) {
        let definition = PropertyDefinition {
            name: name.to_string(),
            gui_field_type: Some(GuiFieldType::SelectWithLabelString),
            gui_field_name: Some(gui_name.to_string()),
            collapsed: false,
            collapsable: true,
            description: Some(format!("{} for the field", gui_name.to_lowercase())),
            category,
            property_type: PropertyType::TextStyle,
            defaults: self.get_style_defaults(name),
            available_values: Some(self.available_options.text_styles.clone().into_iter().map(PropertyValue::from).collect()),
            constraints: None,
        };
        
        self.add_property(name, definition);
    }
    
    /// Helper to add enum property
    fn add_enum_property(&mut self, name: &str, gui_name: &str, category: PropertyCategory, available_values: Vec<PropertyValue>) {
        let definition = PropertyDefinition {
            name: name.to_string(),
            gui_field_type: Some(GuiFieldType::SelectWithLabelString),
            gui_field_name: Some(gui_name.to_string()),
            collapsed: false,
            collapsable: true,
            description: Some(format!("{} for the field", gui_name.to_lowercase())),
            category,
            property_type: PropertyType::Enum,
            defaults: HashMap::new(),
            available_values: Some(available_values),
            constraints: None,
        };
        
        self.add_property(name, definition);
    }
    
    /// Helper to add marker property
    fn add_marker_property(&mut self, name: &str, gui_name: &str, category: PropertyCategory) {
        let definition = PropertyDefinition {
            name: name.to_string(),
            gui_field_type: Some(GuiFieldType::SelectWithLabelString),
            gui_field_name: Some(gui_name.to_string()),
            collapsed: false,
            collapsable: true,
            description: Some(format!("{} configuration", gui_name.to_lowercase())),
            category,
            property_type: PropertyType::String,
            defaults: HashMap::new(),
            available_values: Some(vec![
                PropertyValue::String("none".to_string()),
                PropertyValue::String("required".to_string()),
                PropertyValue::String("error".to_string()),
            ]),
            constraints: None,
        };
        
        self.add_property(name, definition);
    }
    
    /// Initialize GUI field type mappings
    fn initialize_gui_field_types(&mut self) {
        self.gui_field_types.insert("gui_select_with_label_string".to_string(), GuiFieldType::SelectWithLabelString);
        self.gui_field_types.insert("gui_select_with_label_numeric".to_string(), GuiFieldType::SelectWithLabelNumeric);
        self.gui_field_types.insert("gui_list_textornum_with_label_field".to_string(), GuiFieldType::ListTextOrNumWithLabelField);
        self.gui_field_types.insert("gui_checkbox_with_label_field".to_string(), GuiFieldType::CheckboxWithLabelField);
        self.gui_field_types.insert("gui_text_with_label_field".to_string(), GuiFieldType::TextWithLabelField);
        self.gui_field_types.insert("gui_text_field".to_string(), GuiFieldType::TextField);
        self.gui_field_types.insert("gui_literal_field".to_string(), GuiFieldType::LiteralField);
        self.gui_field_types.insert("gui_protected_literal_field".to_string(), GuiFieldType::ProtectedLiteralField);
        self.gui_field_types.insert("gui_boolean_field".to_string(), GuiFieldType::BooleanField);
        self.gui_field_types.insert("gui_image_field".to_string(), GuiFieldType::ImageField);
        self.gui_field_types.insert("gui_line_field".to_string(), GuiFieldType::LineField);
        self.gui_field_types.insert("gui_fieldset_field".to_string(), GuiFieldType::FieldsetField);
    }
    
    /// Initialize default values for each field type
    fn initialize_field_type_defaults(&mut self) {
        let field_types = self.get_all_field_types();
        
        for field_type in field_types {
            let mut defaults = FieldTypeDefaults::new();
            
            // Set size defaults based on field type
            let (width, height) = self.get_size_defaults(field_type);
            defaults = defaults.with_property("field_width", PropertyValue::from(width));
            defaults = defaults.with_property("field_height", PropertyValue::from(height));
            
            // Set color defaults based on field type
            let text_color = self.get_text_color_default(field_type);
            defaults = defaults.with_property("field_text_color", PropertyValue::from(text_color));
            
            // Set border style defaults based on field type
            let border_style = self.get_border_style_default(field_type);
            defaults = defaults.with_property("field_border_style", PropertyValue::from(border_style));
            
            self.defaults.insert(field_type.clone(), defaults);
        }
    }
    
    /// Get all BMS field types
    fn get_all_field_types(&self) -> Vec<BmsFieldType> {
        vec![
            BmsFieldType::FieldTextORNumeric,
            BmsFieldType::Literal,
            BmsFieldType::ProtectedLiteral,
            BmsFieldType::BooleanField,
            BmsFieldType::ImageAsciiArt,
            BmsFieldType::Line,
            BmsFieldType::Fieldset,
            BmsFieldType::Group,
        ]
    }
    
    /// Get size defaults for a field type (from Lua OBJECTS_DEFINITIONS_DEFAULTS)
    fn get_size_defaults(&self, field_type: BmsFieldType) -> (u16, u16) {
        match field_type {
            BmsFieldType::FieldTextORNumeric | BmsFieldType::BooleanField => (10, 3),
            BmsFieldType::Literal | BmsFieldType::ProtectedLiteral => (20, 3),
            BmsFieldType::ImageAsciiArt => (40, 5),
            BmsFieldType::Line => (40, 1),
            BmsFieldType::Fieldset | BmsFieldType::Group => (40, 3),
        }
    }
    
    /// Get text color defaults for a field type
    fn get_text_color_default(&self, field_type: BmsFieldType) -> Color {
        match field_type {
            BmsFieldType::FieldTextORNumeric => Color::Yellow,
            BmsFieldType::Literal => Color::Green,
            BmsFieldType::ProtectedLiteral => Color::White,
            BmsFieldType::BooleanField => Color::Green,
            BmsFieldType::ImageAsciiArt => Color::Default,
            BmsFieldType::Line => Color::Blue,
            BmsFieldType::Fieldset | BmsFieldType::Group => Color::Default,
        }
    }
    
    /// Get border style defaults for a field type
    fn get_border_style_default(&self, field_type: BmsFieldType) -> BorderStyle {
        match field_type {
            BmsFieldType::Fieldset | BmsFieldType::Group => BorderStyle::Double,
            BmsFieldType::Line => BorderStyle::None,
            _ => BorderStyle::None,
        }
    }
    
    /// Get color defaults for a property based on field type
    fn get_color_defaults(&self, property_name: &str) -> HashMap<BmsFieldType, PropertyValue> {
        let mut defaults = HashMap::new();
        
        for field_type in self.get_all_field_types() {
            let color = match property_name {
                "field_text_color" => self.get_text_color_default(field_type.clone()),
                "field_border_color" => Color::Blue,
                "field_title_color" => Color::White,
                "field_footer_color" => Color::White,
                _ => Color::Default,
            };
            defaults.insert(field_type.clone(), PropertyValue::from(color));
        }
        
        defaults
    }
    
    /// Get style defaults for a property based on field type
    fn get_style_defaults(&self, property_name: &str) -> HashMap<BmsFieldType, PropertyValue> {
        let mut defaults = HashMap::new();
        
        for field_type in self.get_all_field_types() {
            let style = match property_name {
                "field_style" => match field_type {
                    BmsFieldType::FieldTextORNumeric | BmsFieldType::BooleanField => TextStyle::Default,
                    BmsFieldType::Literal | BmsFieldType::ProtectedLiteral => TextStyle::Bold,
                    BmsFieldType::ImageAsciiArt => TextStyle::Default,
                    BmsFieldType::Line => TextStyle::Underline,
                    BmsFieldType::Fieldset | BmsFieldType::Group => TextStyle::Default,
                },
                _ => TextStyle::Default,
            };
            defaults.insert(field_type.clone(), PropertyValue::from(style));
        }
        
        defaults
    }
    
    /// Get a property definition by name
    pub fn get_property(&self, name: &str) -> Option<&PropertyDefinition> {
        self.properties.get(name)
    }
    
    /// Get default values for a field type
    pub fn get_defaults(&self, field_type: BmsFieldType) -> Option<&FieldTypeDefaults> {
        self.defaults.get(&field_type)
    }
    
    /// Get GUI field type by name
    pub fn get_gui_field_type(&self, name: &str) -> Option<GuiFieldType> {
        self.gui_field_types.get(name).cloned()
    }
    
    /// Get all properties in a specific category
    pub fn get_properties_by_category(&self, category: PropertyCategory) -> Vec<&PropertyDefinition> {
        self.properties.values()
            .filter(|p| p.category == category)
            .collect()
    }
    
    /// Get available values for a property
    pub fn get_available_values(&self, property_name: &str) -> Option<&Vec<PropertyValue>> {
        self.get_property(property_name)
            .and_then(|p| p.available_values.as_ref())
    }
    
    /// Create a new field object with defaults for a specific type
    pub fn create_field_object(&self, field_type: BmsFieldType) -> FieldObject {
        let defaults = self.get_object_defaults(field_type.clone());
        let mut obj = crate::bms::render::new_field_object(field_type, defaults);
        
        // Set default property values based on field type
        if let Some(type_defaults) = self.get_defaults(field_type.clone()) {
            for (prop_name, prop_value) in &type_defaults.properties {
                self.set_property_value(&mut obj, prop_name, prop_value.clone());
            }
        }
        
        obj
    }
    
    /// Get FieldObjectDefaults for a field type
    pub fn get_object_defaults(&self, field_type: BmsFieldType) -> FieldObjectDefaults {
        let size_defaults = crate::bms::defaults::FieldSizeDefaults::new();
        let color_defaults = crate::bms::defaults::FieldColorDefaults::new();
        let align_defaults = crate::bms::defaults::TextAlignDefaults::new();
        let style_defaults = crate::bms::defaults::FieldStyleDefaults::new();
        let border_defaults = crate::bms::defaults::BorderDefaults::new();
        
        FieldObjectDefaults {
            width: size_defaults.get_width(field_type.clone()),
            height: size_defaults.get_height(field_type.clone()),
            min_width: size_defaults.get_min_width(field_type.clone()),
            max_width: size_defaults.get_max_width(field_type.clone()),
            min_height: size_defaults.get_min_height(field_type.clone()),
            max_height: size_defaults.get_max_height(field_type.clone()),
            border_style: border_defaults.get_default_border_style(field_type.clone()),
            text_color: color_defaults.get_text_color(field_type.clone()),
            border_color: color_defaults.get_border_color(field_type.clone()),
            title_color: color_defaults.get_title_color(field_type.clone()),
            text_align: align_defaults.get_default_text_align(field_type.clone()),
            title_align: align_defaults.get_default_title_align(field_type.clone()),
            text_style: style_defaults.get_default_style(field_type.clone()),
            fill_char: FillChar::Space,
            field_type: field_type.clone(),
        }
    }
    
    /// Set a property value on a field object
    fn set_property_value(&self, obj: &mut super::render::FieldObject, prop_name: &str, value: PropertyValue) {
        match value {
            PropertyValue::String(s) => {
                obj.properties.insert(prop_name.to_string(), Box::new(Property::new(s)));
            }
            PropertyValue::Number(n) => {
                if n >= 0 {
                    obj.properties.insert(prop_name.to_string(), Box::new(Property::new(n as u16)));
                } else {
                    obj.properties.insert(prop_name.to_string(), Box::new(Property::new(n as i32)));
                }
            }
            PropertyValue::Float(f) => {
                obj.properties.insert(prop_name.to_string(), Box::new(Property::new(f)));
            }
            PropertyValue::Boolean(b) => {
                obj.properties.insert(prop_name.to_string(), Box::new(Property::new(b)));
            }
            PropertyValue::Color(c) => {
                obj.properties.insert(prop_name.to_string(), Box::new(Property::new(c)));
            }
            PropertyValue::BorderStyle(bs) => {
                obj.properties.insert(prop_name.to_string(), Box::new(Property::new(bs)));
            }
            PropertyValue::TextAlign(ta) => {
                obj.properties.insert(prop_name.to_string(), Box::new(Property::new(ta)));
            }
            PropertyValue::VerticalAlign(va) => {
                obj.properties.insert(prop_name.to_string(), Box::new(Property::new(va)));
            }
            PropertyValue::FillChar(fc) => {
                obj.properties.insert(prop_name.to_string(), Box::new(Property::new(fc)));
            }
            PropertyValue::TextStyle(ts) => {
                obj.properties.insert(prop_name.to_string(), Box::new(Property::new(ts)));
            }
            PropertyValue::BorderCharSet(bcs) => {
                obj.properties.insert(prop_name.to_string(), Box::new(Property::new(bcs)));
            }
        }
    }
}

impl Default for ObjectDefinitions {
    fn default() -> Self {
        Self::new()
    }
}

// Global OBJECTS_DEFINITIONS instance for easy access
lazy_static::lazy_static! {
    pub static ref OBJECTS_DEFINITIONS: ObjectDefinitions = ObjectDefinitions::new();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_object_definitions_creation() {
        let defs = ObjectDefinitions::new();
        assert!(!defs.properties.is_empty());
        assert!(defs.get_property("field_name").is_some());
        assert!(defs.get_property("field_type").is_some());
    }
    
    #[test]
    fn test_property_categories() {
        let defs = ObjectDefinitions::new();
        let dimensions = defs.get_properties_by_category(PropertyCategory::Dimensions);
        assert!(!dimensions.is_empty());
        
        let colors = defs.get_properties_by_category(PropertyCategory::Colors);
        assert!(!colors.is_empty());
    }
    
    #[test]
    fn test_field_object_creation() {
        let defs = ObjectDefinitions::new();
        let field_obj = defs.create_field_object(BmsFieldType::FieldTextORNumeric);
        
        assert!(field_obj.field_type.is_some());
        assert!(!field_obj.properties.is_empty());
    }
    
    #[test]
    fn test_lazy_static_instance() {
        // This just ensures the lazy static compiles
        assert!(!OBJECTS_DEFINITIONS.properties.is_empty());
    }
}