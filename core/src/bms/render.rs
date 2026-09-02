//! Rendering functionality for BMS fields - mirrors Lua OBJECTS-DEFINITIONS rendering functions
//!
//! This module provides rendering functions that mirror the Lua OBJECTS-DEFINITIONS.lua
//! rendering logic, including border handling, title building, footer building,
//! and various field type renderers.

use super::{
    field_types::BmsFieldType,
    types::{BorderStyle, BorderCharSet, FillChar, TextAlign, Marker, PrefixSuffix, Footer},
    properties::Property,
    defaults::FieldObjectDefaults,
};
use std::collections::HashMap;
use serde_json;

/// Get a property from an object, handling the initial/edited structure
/// Mirrors Lua: local function get_property(obj, prop_name)
pub fn get_property<T: Clone + 'static>(obj: &FieldObject, prop_name: &str) -> Option<T> {
    // Try to get from the object's properties
    obj.properties.get(prop_name).and_then(|prop| {
        prop.downcast_ref::<Property<T>>().map(|p| p.get())
    })
}

/// Get property with default fallback
pub fn get_property_with_default<T: Clone + Default + 'static>(obj: &FieldObject, prop_name: &str) -> T {
    get_property::<T>(obj, prop_name).unwrap_or_default()
}

/// Get border characters for an object
/// Mirrors Lua: local function get_border_chars(obj)
pub fn get_border_chars(obj: &FieldObject) -> BorderCharSet {
    let border_style = get_property::<BorderStyle>(obj, "field_border_style")
        .unwrap_or_else(|| obj.defaults.border_style.clone());
    
    let border_chars = get_property::<BorderChars>(obj, "field_border_chars");
    
    if let Some(border_chars) = border_chars {
        return border_chars.get(border_style.clone());
    }
    
    // Use default border chars for the style
    BorderCharSet::for_style(border_style)
}

/// Get required marker for an object
/// Mirrors Lua: local function get_required_marker(obj)
pub fn get_required_marker(obj: &FieldObject) -> String {
    let required_marker = get_property::<Marker>(obj, "field_required_marker")
        .or_else(|| get_property::<Marker>(obj, "field_avail_required_marker"));
    
    if let Some(marker) = required_marker {
        if marker.should_display() {
            return marker.marker;
        }
    }
    
    String::new()
}

/// Get error marker for an object
/// Mirrors Lua: local function get_error_marker(obj)
pub fn get_error_marker(obj: &FieldObject) -> String {
    let error_marker = get_property::<Marker>(obj, "field_error_marker")
        .or_else(|| get_property::<Marker>(obj, "field_avail_error_marker"));
    
    if let Some(marker) = error_marker {
        if marker.should_display() {
            return marker.marker;
        }
    }
    
    String::new()
}

/// Build title string for an object
/// Mirrors Lua: local function build_title(obj)
pub fn build_title(obj: &FieldObject) -> String {
    let name = get_property::<String>(obj, "field_name");
    let initial = get_property::<String>(obj, "field_initial");
    
    let title = name.unwrap_or_else(|| initial.unwrap_or_default());
    
    if title.is_empty() {
        return String::new();
    }
    
    // Get prefix and suffix configurations
    let title_prefix = get_property::<PrefixSuffix>(obj, "field_title_prefix");
    let title_suffix = get_property::<PrefixSuffix>(obj, "field_title_suffix");
    let attrb = get_property::<FieldAttributes>(obj, "field_attrb");
    
    // Build prefix
    let mut prefix = String::new();
    if let Some(prefix_config) = title_prefix {
        if prefix_config.enabled {
            let mut marker = String::new();
            
            // Check if we should show required or error marker
            if let Some(ref attrb) = attrb {
                if attrb.field_required && prefix_config.required.is_some() {
                    marker = get_required_marker(obj);
                }
                if marker.is_empty() && attrb.field_has_error && prefix_config.errors.is_some() {
                    marker = get_error_marker(obj);
                }
            }
            
            if !marker.is_empty() {
                prefix = marker;
            } else if let Some(fill_char) = prefix_config.prefix_char {
                prefix = fill_char.char().to_string();
            }
        }
    }
    
    // Build suffix
    let mut suffix = String::new();
    if let Some(suffix_config) = title_suffix {
        if suffix_config.enabled {
            let mut marker = String::new();
            
            // Check if we should show required or error marker
            if let Some(ref attrb) = attrb {
                if attrb.field_required && suffix_config.required.is_some() {
                    marker = get_required_marker(obj);
                }
                if marker.is_empty() && attrb.field_has_error && suffix_config.errors.is_some() {
                    marker = get_error_marker(obj);
                }
            }
            
            if !marker.is_empty() {
                suffix = marker;
            } else if let Some(fill_char) = suffix_config.prefix_char {
                suffix = fill_char.char().to_string();
            }
        }
    }
    
    format!("{}{}{}", prefix, title, suffix)
}

/// Build footer line for an object
/// Mirrors Lua: local function build_footer(obj, width)
pub fn build_footer(obj: &FieldObject, width: u16) -> String {
    let footer_config = get_property::<Footer>(obj, "field_footer");
    
    if footer_config.is_none() {
        return String::new();
    }
    
    let footer = footer_config.unwrap();
    
    // Handle fill_marker
    let fill_marker = footer.fill_marker.char();
    
    // Handle title
    let title = footer.title;
    
    // Handle align
    let align = footer.align;
    
    // Handle color (not used in string output, but available for future)
    let _color = footer.color;
    
    // Check if we should show required/error markers
    let attrb = get_property::<FieldAttributes>(obj, "field_attrb");
    let show_required = attrb.as_ref().map_or(false, |a| a.field_required);
    let show_error = attrb.as_ref().map_or(false, |a| a.field_has_error);
    
    // Get markers
    let required_marker = footer.required_marker
        .map(|m| if m.should_display() { m.marker } else { String::new() })
        .unwrap_or_default();
    
    let error_marker = footer.error_marker
        .map(|m| if m.should_display() { m.marker } else { String::new() })
        .unwrap_or_default();
    
    // Only show footer if title is non-empty or there are markers to display
    if title.is_empty() && !show_required && !show_error {
        return String::new();
    }
    
    // Build footer content parts
    let mut marker_content = String::new();
    if show_required && !required_marker.is_empty() {
        marker_content.push_str(&required_marker);
    }
    if show_error && !error_marker.is_empty() {
        marker_content.push_str(&error_marker);
    }
    
    // Check if title + markers fit
    let total_length = title.len() + marker_content.len();
    let display_title = if total_length > width as usize {
        let available_for_title = width as usize - marker_content.len();
        if available_for_title > 0 {
            title.chars().take(available_for_title).collect()
        } else {
            String::new()
        }
    } else {
        title
    };
    
    let content = format!("{}{}", display_title, marker_content);
    
    // If no content after adding markers, return empty
    if content.is_empty() {
        return String::new();
    }
    
    // Apply alignment
    let content_length = content.len();
    let padding = width as usize - content_length;
    
    if padding > 0 {
        match align {
            TextAlign::Left => {
                format!("{}{}", content, fill_marker.to_string().repeat(padding))
            }
            TextAlign::Right => {
                format!("{}{}", fill_marker.to_string().repeat(padding), content)
            }
            TextAlign::Center => {
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                format!(
                    "{}{}{}", 
                    fill_marker.to_string().repeat(left_pad),
                    content,
                    fill_marker.to_string().repeat(right_pad)
                )
            }
        }
    } else {
        // If still too long after truncating title, do a final truncation
        content.chars().take(width as usize).collect()
    }
}

/// Render a simple bordered field
/// Mirrors Lua: function render_bordered_field(obj, custom_content)
pub fn render_bordered_field(obj: &FieldObject, custom_content: Option<String>) -> String {
    let height = get_property::<u16>(obj, "field_height")
        .unwrap_or_else(|| obj.defaults.height);
    let width = get_property::<u16>(obj, "field_width")
        .unwrap_or_else(|| obj.defaults.width);
    let border_style = get_property::<BorderStyle>(obj, "field_border_style")
        .unwrap_or(BorderStyle::None);
    let border_chars = get_border_chars(obj);
    
    let fill_char_raw = get_property::<FillChar>(obj, "field_fill_char");
    let fill_marker = fill_char_raw.map_or(' ', |fc| fc.char());
    
    let obj_type = get_property::<BmsFieldType>(obj, "field_type")
        .unwrap_or(BmsFieldType::FieldTextORNumeric);
    
    // Determine content
    let content = if let Some(custom) = custom_content {
        custom
    } else if matches!(obj_type, BmsFieldType::BooleanField) {
        // For BooleanField, show [X] or [ ] based on initial_value
        let initial_value = get_property::<bool>(obj, "field_initial")
            .or_else(|| get_property::<bool>(obj, "initial_value"));
        if initial_value.unwrap_or(false) {
            "[X]".to_string()
        } else {
            "[ ]".to_string()
        }
    } else {
        let initial_value = get_property::<String>(obj, "field_initial")
            .or_else(|| get_property::<String>(obj, "initial_value"));
        initial_value.unwrap_or_default()
    };
    
    let mut lines = Vec::new();
    
    // If no border, just return content centered
    if matches!(border_style, BorderStyle::None) {
        if height >= 1 {
            let line = if !content.is_empty() && content.len() <= width as usize {
                let padding = (width as usize - content.len()) / 2;
                format!(
                    "{}{}{}",
                    " ".repeat(padding),
                    content,
                    " ".repeat(width as usize - padding - content.len())
                )
            } else {
                if content.len() > width as usize {
                    content.chars().take(width as usize).collect()
                } else {
                    content
                }
            };
            lines.push(line);
        }
        return lines.join("\n");
    }
    
    // Top border with title if height >= 1
    if height >= 1 {
        let title = build_title(obj);
        let title_fill = get_property::<FillChar>(obj, "field_title_fill_char")
            .map_or(FillChar::Space, |fc| fc);
        let fill_char_str = title_fill.char();
        
        if !title.is_empty() {
            // Create title line with border
            let mut title_str = title;
            let title_len = title_str.len();
            let content_width = width as usize;
            
            if title_len > content_width {
                title_str = title_str.chars().take(content_width).collect();
            }
            
            // Get title alignment
            let title_align = get_property::<TextAlign>(obj, "field_title_align")
                .unwrap_or(TextAlign::Center);
            
            let padding = content_width - title_str.len();
            let (left_fill, right_fill) = match title_align {
                TextAlign::Left => (0, padding),
                TextAlign::Right => (padding, 0),
                TextAlign::Center => {
                    let left = padding / 2;
                    (left, padding - left)
                }
            };
            
            title_str = format!(
                "{}{}{}",
                fill_char_str.to_string().repeat(left_fill),
                title_str,
                fill_char_str.to_string().repeat(right_fill)
            );
            
            lines.push(format!(
                "{}{}{}",
                border_chars.top_left,
                title_str,
                border_chars.top_right
            ));
        } else {
            // No title, just border
            lines.push(format!(
                "{}{}{}",
                border_chars.top_left,
                border_chars.top.repeat(width as usize),
                border_chars.top_right
            ));
        }
    }
    
    // Content area
    let content_lines: Vec<&str> = if !content.is_empty() {
        content.lines().collect()
    } else {
        vec![""]
    };
    
    // Center content vertically
    let content_height = content_lines.len();
    let content_start = if height > 2 {
        ((height - 1 - content_height as u16) / 2) + 1
    } else {
        1
    };
    
    // Get text alignment
    let text_align = get_property::<TextAlign>(obj, "field_text_align")
        .unwrap_or(TextAlign::Left);
    
    for i in 1..height {
        if i >= content_start && (i - content_start) < content_height as u16 {
            let content_line = content_lines[(i - content_start) as usize];
            let padding = width as usize - content_line.len();
            
            let (left_pad, right_pad) = if padding > 0 {
                match text_align {
                    TextAlign::Left => (0, padding),
                    TextAlign::Right => (padding, 0),
                    TextAlign::Center => {
                        let left = padding / 2;
                        (left, padding - left)
                    }
                }
            } else {
                (0, 0)
            };
            
            let mut content_line = format!(
                "{}{}{}",
                " ".repeat(left_pad),
                content_line,
                " ".repeat(right_pad)
            );
            
            // Truncate if too long
            if content_line.len() > width as usize {
                content_line = content_line.chars().take(width as usize).collect();
            }
            
            // Pad with fill_marker if still too short
            content_line = format!("{}{}", content_line, fill_marker.to_string().repeat(width as usize - content_line.len()));
            
            lines.push(format!(
                "{}{}{}",
                border_chars.left,
                content_line,
                border_chars.right
            ));
        } else {
            lines.push(format!(
                "{}{}{}",
                border_chars.left,
                fill_marker.to_string().repeat(width as usize),
                border_chars.right
            ));
        }
    }
    
    // Bottom border or footer
    if height >= 2 {
        let footer = build_footer(obj, width);
        if !footer.is_empty() {
            // Footer line
            lines.push(format!(
                "{}{}{}",
                border_chars.bottom_left,
                footer,
                border_chars.bottom_right
            ));
        } else {
            // Regular bottom border
            let bottom_line = format!(
                "{}{}{}",
                border_chars.bottom_left,
                border_chars.bottom.repeat(width as usize),
                border_chars.bottom_right
            );
            lines.push(bottom_line);
        }
    }
    
    lines.join("\n")
}

/// Render a Line (horizontal line)
/// Mirrors Lua: function render_line(obj)
pub fn render_line(obj: &FieldObject) -> String {
    let width = get_property::<u16>(obj, "field_width")
        .unwrap_or_else(|| obj.defaults.width);
    let border_style = get_property::<BorderStyle>(obj, "field_border_style")
        .unwrap_or(BorderStyle::None);
    let border_chars = get_border_chars(obj);
    let line_char = border_chars.top; // Use top border char for horizontal line
    
    if matches!(border_style, BorderStyle::None) {
        "-".repeat(width as usize)
    } else {
        line_char.repeat(width as usize)
    }
}

/// Render a Fieldset (container with title)
/// Mirrors Lua: function render_fieldset(obj)
pub fn render_fieldset(obj: &FieldObject) -> String {
    let height = get_property::<u16>(obj, "field_height")
        .unwrap_or_else(|| obj.defaults.height);
    let width = get_property::<u16>(obj, "field_width")
        .unwrap_or_else(|| obj.defaults.width);
    let border_chars = get_border_chars(obj);
    
    let fill_char_raw = get_property::<FillChar>(obj, "field_fill_char");
    let fill_marker = fill_char_raw.map_or(' ', |fc| fc.char());
    
    let mut lines = Vec::new();
    
    // Top border with title
    let title_fill = get_property::<FillChar>(obj, "field_title_fill_char")
        .map_or(FillChar::Space, |fc| fc);
    let fill_char_str = title_fill.char();
    
    if height >= 1 {
        let title = build_title(obj);
        
        if !title.is_empty() {
            let mut title_str = title;
            let title_len = title_str.len();
            let content_width = width as usize;
            
            if title_len > content_width {
                title_str = title_str.chars().take(content_width).collect();
            }
            
            // Get title alignment
            let title_align = get_property::<TextAlign>(obj, "field_title_align")
                .unwrap_or(TextAlign::Center);
            
            let padding = content_width - title_str.len();
            let (left_fill, right_fill) = match title_align {
                TextAlign::Left => (0, padding),
                TextAlign::Right => (padding, 0),
                TextAlign::Center => {
                    let left = padding / 2;
                    (left, padding - left)
                }
            };
            
            title_str = format!(
                "{}{}{}",
                fill_char_str.to_string().repeat(left_fill),
                title_str,
                fill_char_str.to_string().repeat(right_fill)
            );
            
            let top_line = format!(
                "{}{}{}",
                border_chars.top_left,
                title_str,
                border_chars.top_right
            );
            lines.push(top_line);
        } else {
            // No title, just border
            let top_line = format!(
                "{}{}{}",
                border_chars.top_left,
                border_chars.top.repeat(width as usize),
                border_chars.top_right
            );
            lines.push(top_line);
        }
    }
    
    // Content area
    for _ in 1..height - 1 {
        lines.push(format!(
            "{}{}{}",
            border_chars.left,
            fill_marker.to_string().repeat(width as usize),
            border_chars.right
        ));
    }
    
    // Bottom border or footer
    if height >= 2 {
        let footer = build_footer(obj, width);
        if !footer.is_empty() {
            // Footer line
            lines.push(format!(
                "{}{}{}",
                border_chars.bottom_left,
                footer,
                border_chars.bottom_right
            ));
        } else {
            // Regular bottom border
            let bottom_line = format!(
                "{}{}{}",
                border_chars.bottom_left,
                border_chars.bottom.repeat(width as usize),
                border_chars.bottom_right
            );
            lines.push(bottom_line);
        }
    }
    
    lines.join("\n")
}

/// Main object renderer
/// Mirrors Lua: function render_object(obj)
pub fn render_object(obj: &FieldObject) -> String {
    if obj.field_type.is_none() {
        return "[Invalid Object]".to_string();
    }
    
    let obj_type = obj.field_type.as_ref().unwrap();
    
    match obj_type {
        BmsFieldType::FieldTextORNumeric | 
        BmsFieldType::Literal | 
        BmsFieldType::ProtectedLiteral | 
        BmsFieldType::BooleanField => {
            render_bordered_field(obj, None)
        }
        BmsFieldType::ImageAsciiArt => {
            // For ImageAsciiArt, we'd need custom content
            render_bordered_field(obj, None)
        }
        BmsFieldType::Line => {
            render_line(obj)
        }
        BmsFieldType::Fieldset | BmsFieldType::Group => {
            render_fieldset(obj)
        }
    }
}

// ============================================================================
// OBJECT CONSTRUCTOR AND UTILITY FUNCTIONS
// ============================================================================

/// Field object structure that mirrors Lua OBJECTS_DEFINITIONS objects
#[derive(Debug)]
pub struct FieldObject {
    pub field_type: Option<BmsFieldType>,
    pub field_name: Option<String>,
    pub defaults: FieldObjectDefaults,
    pub properties: HashMap<String, Box<dyn std::any::Any>>,  // Using Any for heterogeneous property types
}

/// Field attributes structure
#[derive(Debug, Clone)]
pub struct FieldAttributes {
    pub field_required: bool,
    pub field_has_error: bool,
    // Other attributes as needed
}

/// Border characters for different styles
#[derive(Debug, Clone)]
pub struct BorderChars {
    pub single: BorderCharSet,
    pub double: BorderCharSet,
    pub dashed: BorderCharSet,
    pub none: BorderCharSet,
}

impl BorderChars {
    pub fn get(&self, style: BorderStyle) -> BorderCharSet {
        match style {
            BorderStyle::Single => self.single.clone(),
            BorderStyle::Double => self.double.clone(),
            BorderStyle::Dashed => self.dashed.clone(),
            _ => self.none.clone(),
        }
    }
}

impl Default for BorderChars {
    fn default() -> Self {
        Self {
            single: BorderCharSet::single(),
            double: BorderCharSet::double(),
            dashed: BorderCharSet::dashed(),
            none: BorderCharSet::none(),
        }
    }
}

// ============================================================================
// GUI PROPERTY EXTRACTION (mirroring Lua functionality)
// ============================================================================

/// GUI property information for property editing
#[derive(Debug, Clone)]
pub struct GuiProperty {
    pub name: String,
    pub gui_name: String,
    pub category: String,
    pub gui_type: String,
    pub control_type: String,
    pub default: Option<String>,
    pub min_max: Option<(u16, u16)>,
    pub available_values: Option<Vec<String>>,
    pub read_only: bool,
    pub hint: String,
}

/// Get GUI properties for a specific field type
/// Mirrors Lua: OBJECTS_DEFINITIONS.get_gui_properties(obj_type)
pub fn get_gui_properties(obj_type: BmsFieldType) -> Vec<GuiProperty> {
    let properties = vec![
        // Dimensions
        create_gui_property("field_height", "Height", "dimensions", "text", "text", None, Some((1, 80)), None, false, "Field height"),
        create_gui_property("field_width", "Width", "dimensions", "text", "text", None, Some((1, 255)), None, false, "Field width"),
        create_gui_property("field_min_height", "Min Height", "dimensions", "text", "text", None, Some((1, 80)), None, false, "Minimum field height"),
        create_gui_property("field_max_height", "Max Height", "dimensions", "text", "text", None, Some((1, 80)), None, false, "Maximum field height"),
        
        // Colors
        create_gui_property("field_border_color", "Border Color", "colors", "select", "select", None, None, Some(get_color_values()), false, "Border color"),
        create_gui_property("field_title_color", "Title Color", "colors", "select", "select", None, None, Some(get_color_values()), false, "Title color"),
        create_gui_property("field_text_color", "Text Color", "colors", "select", "select", None, None, Some(get_color_values()), false, "Text color"),
        
        // Style
        create_gui_property("field_style", "Style", "style", "select", "select", None, None, Some(get_style_values()), false, "Text style"),
        
        // Alignment
        create_gui_property("field_text_align", "Text Align", "alignment", "select", "select", None, None, Some(get_align_values()), false, "Text alignment"),
        create_gui_property("field_title_align", "Title Align", "alignment", "select", "select", None, None, Some(get_align_values()), false, "Title alignment"),
        
        // Borders
        create_gui_property("field_border_style", "Border Style", "borders", "select", "select", None, None, Some(get_border_style_values()), false, "Border style"),
        create_gui_property("field_fill_char", "Fill Char", "fill", "select", "select", None, None, Some(get_fill_char_values()), false, "Fill character"),
        
        // Markers
        create_gui_property("field_required_marker", "Required Marker", "markers", "select", "select", None, None, Some(get_marker_values()), false, "Required field marker"),
        create_gui_property("field_error_marker", "Error Marker", "markers", "select", "select", None, None, Some(get_marker_values()), false, "Error field marker"),
    ];
    
    // Filter properties based on field type
    properties.into_iter()
        .filter(|prop| is_property_relevant_for_type(&prop.name, &obj_type))
        .collect()
}

fn create_gui_property(
    name: &str, 
    gui_name: &str, 
    category: &str, 
    gui_type: &str, 
    control_type: &str, 
    default: Option<String>, 
    min_max: Option<(u16, u16)>, 
    available_values: Option<Vec<String>>, 
    read_only: bool, 
    hint: &str
) -> GuiProperty {
    GuiProperty {
        name: name.to_string(),
        gui_name: gui_name.to_string(),
        category: category.to_string(),
        gui_type: gui_type.to_string(),
        control_type: control_type.to_string(),
        default,
        min_max,
        available_values,
        read_only,
        hint: hint.to_string(),
    }
}

fn is_property_relevant_for_type(property_name: &str, obj_type: &BmsFieldType) -> bool {
    // Some properties are not relevant for certain field types
    match property_name {
        "field_height" | "field_width" => true,  // Always relevant
        "field_border_color" | "field_title_color" | "field_text_color" => {
            // Colors are relevant for most types except maybe Line
            !matches!(obj_type, BmsFieldType::Line)
        }
        "field_style" => {
            // Style is relevant for text-based fields
            matches!(obj_type, BmsFieldType::FieldTextORNumeric | BmsFieldType::Literal | BmsFieldType::ProtectedLiteral)
        }
        "field_border_style" | "field_fill_char" => {
            // Border and fill are relevant for container types
            matches!(obj_type, BmsFieldType::Fieldset | BmsFieldType::Group | BmsFieldType::FieldTextORNumeric | BmsFieldType::Literal | BmsFieldType::ProtectedLiteral | BmsFieldType::BooleanField)
        }
        _ => true,  // Default to relevant
    }
}

fn get_color_values() -> Vec<String> {
    vec!["Default", "Black", "Red", "Green", "Yellow", "Blue", "Magenta", "Cyan", "White"]
        .into_iter().map(String::from).collect()
}

fn get_style_values() -> Vec<String> {
    vec!["Default", "Bold", "Italic", "Underline", "Blink", "Reverse"]
        .into_iter().map(String::from).collect()
}

fn get_align_values() -> Vec<String> {
    vec!["Left", "Center", "Right"]
        .into_iter().map(String::from).collect()
}

fn get_border_style_values() -> Vec<String> {
    vec!["None", "Single", "Double", "Dashed", "Dotted"]
        .into_iter().map(String::from).collect()
}

fn get_fill_char_values() -> Vec<String> {
    vec!["Space", "Dash", "Equal", "Underscore", "Dot", "Asterisk", "Pipe"]
        .into_iter().map(String::from).collect()
}

fn get_marker_values() -> Vec<String> {
    vec!["None", "Required", "Error"]
        .into_iter().map(String::from).collect()
}

/// Get menu items for ncurses-based UI
/// Mirrors Lua: OBJECTS_DEFINITIONS.get_ncurses_menu_items(obj_type)
pub fn get_ncurses_menu_items(obj_type: BmsFieldType) -> Vec<GuiProperty> {
    get_gui_properties(obj_type)
}

/// Export object to JSON representation
/// Mirrors Lua: OBJECTS_DEFINITIONS.export_to_json(obj_type)
pub fn export_to_json(obj: &FieldObject) -> serde_json::Value {
    use serde_json::json;
    
    let mut result = json!({
        "field_type": obj.field_type.map(|t| t.short_name()),
        "field_name": obj.field_name,
        "properties": {}
    });
    
    // Add properties to the JSON
    for (name, prop) in &obj.properties {
        // For now, just add string representations
        // In a full implementation, we'd handle each property type specifically
        if let Some(prop_typed) = prop.downcast_ref::<Property<String>>() {
            result["properties"][name] = json!(prop_typed.get());
        } else if let Some(prop_typed) = prop.downcast_ref::<Property<u16>>() {
            result["properties"][name] = json!(prop_typed.get());
        } else if let Some(prop_typed) = prop.downcast_ref::<Property<bool>>() {
            result["properties"][name] = json!(prop_typed.get());
        }
        // Add other types as needed...
    }
    
    result
}

// ============================================================================
// OBJECT CONSTRUCTOR
// ============================================================================

/// Create a new FieldObject
/// Mirrors Lua: OBJECTS_DEFINITIONS.new(obj_type, overrides)
pub fn new_field_object(obj_type: BmsFieldType, defaults: FieldObjectDefaults) -> FieldObject {
    let mut obj = FieldObject {
        field_type: Some(obj_type),
        field_name: None,
        defaults,
        properties: HashMap::new(),
    };
    
    // Initialize default properties based on type
    initialize_default_properties(&mut obj);
    
    obj
}

fn initialize_default_properties(obj: &mut FieldObject) {
    if let Some(_) = obj.field_type.as_ref() {
        // Set default properties based on field type
        obj.properties.insert(
            "field_height".to_string(),
            Box::new(Property::new(obj.defaults.height)),
        );
        obj.properties.insert(
            "field_width".to_string(),
            Box::new(Property::new(obj.defaults.width)),
        );
        obj.properties.insert(
            "field_border_style".to_string(),
            Box::new(Property::new(obj.defaults.border_style.clone())),
        );
        // Add more default properties as needed...
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_border_chars() {
        let border_chars = BorderCharSet::for_style(BorderStyle::Single);
        assert_eq!(border_chars.top_left, "┌");
        assert_eq!(border_chars.top, "─");
        
        let border_chars = BorderCharSet::for_style(BorderStyle::Double);
        assert_eq!(border_chars.top_left, "╔");
        assert_eq!(border_chars.top, "═");
    }
    
    #[test]
    fn test_get_color_values() {
        let colors = get_color_values();
        assert!(colors.contains(&"Red".to_string()));
        assert!(colors.contains(&"Blue".to_string()));
    }
    
    #[test]
    fn test_gui_properties() {
        let properties = get_gui_properties(BmsFieldType::FieldTextORNumeric);
        assert!(!properties.is_empty());
        
        // Should have dimensions properties
        let has_height = properties.iter().any(|p| p.name == "field_height");
        let has_width = properties.iter().any(|p| p.name == "field_width");
        assert!(has_height);
        assert!(has_width);
    }
}