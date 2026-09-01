//! Field type definitions for BMS - mirrors Lua OBJECTS_DEFINITIONS_GUI_TYPE and objects_types
//!
//! This module defines all the field types used in BMS maps, including:
//! - Field types (FieldTextORNumeric, Literal, ProtectedLiteral, etc.)
//! - GUI field types for rendering
//! - Display names and metadata for each type

use serde::{Serialize, Deserialize};
use std::fmt;

/// GUI field type for rendering - mirrors Lua OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GuiFieldType {
    /// Select box with label, string values
    SelectWithLabelString,
    /// Select box with label, numeric values
    SelectWithLabelNumeric,
    /// List field with label, text or numeric values
    ListTextOrNumWithLabelField,
    /// Checkbox with label field
    CheckboxWithLabelField,
    /// Text input with label field
    TextWithLabelField,
    /// Plain text field
    TextField,
    /// Literal text field (read-only)
    LiteralField,
    /// Protected literal field (read-only, protected)
    ProtectedLiteralField,
    /// Boolean field (checkbox)
    BooleanField,
    /// Image/ASCII art field
    ImageField,
    /// Horizontal line separator
    LineField,
    /// Fieldset/group container
    FieldsetField,
}

impl GuiFieldType {
    /// Get string representation
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

impl fmt::Display for GuiFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// BMS Field type - mirrors Lua objects_types.field_type.enum
/// These are the main field types that can appear in a BMS map
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BmsFieldType {
    /// Editable text or numeric field (main input field type)
    #[default]
    FieldTextORNumeric,
    /// Static text display (read-only)
    Literal,
    /// Protected static text (read-only, cannot be modified by user)
    ProtectedLiteral,
    /// Boolean/checkbox field
    BooleanField,
    /// ASCII art or image placeholder
    ImageAsciiArt,
    /// Horizontal line separator
    Line,
    /// Container/group of fields with border and title
    Fieldset,
    /// Group of related fields (synonym for Fieldset)
    Group,
}

impl BmsFieldType {
    /// Get display name - mirrors Lua field_name.enum
    pub fn display_name(&self) -> &'static str {
        match self {
            BmsFieldType::FieldTextORNumeric => "Field Text or Numeric",
            BmsFieldType::Literal => "Literal Text",
            BmsFieldType::ProtectedLiteral => "Protected Literal",
            BmsFieldType::BooleanField => "Boolean Field",
            BmsFieldType::ImageAsciiArt => "ImageAsciiArt / Ascii Art",
            BmsFieldType::Line => "Line Separator",
            BmsFieldType::Fieldset => "Fieldset Group",
            BmsFieldType::Group => "Group",
        }
    }

    /// Get short name for code generation
    pub fn short_name(&self) -> &'static str {
        match self {
            BmsFieldType::FieldTextORNumeric => "Field",
            BmsFieldType::Literal => "Literal",
            BmsFieldType::ProtectedLiteral => "Protected",
            BmsFieldType::BooleanField => "Boolean",
            BmsFieldType::ImageAsciiArt => "Image",
            BmsFieldType::Line => "Line",
            BmsFieldType::Fieldset => "Fieldset",
            BmsFieldType::Group => "Group",
        }
    }

    /// Get the corresponding GUI field type for rendering
    pub fn gui_field_type(&self) -> GuiFieldType {
        match self {
            BmsFieldType::FieldTextORNumeric | 
            BmsFieldType::Literal | 
            BmsFieldType::ProtectedLiteral => GuiFieldType::TextWithLabelField,
            BmsFieldType::BooleanField => GuiFieldType::CheckboxWithLabelField,
            BmsFieldType::ImageAsciiArt => GuiFieldType::ImageField,
            BmsFieldType::Line => GuiFieldType::LineField,
            BmsFieldType::Fieldset | BmsFieldType::Group => GuiFieldType::FieldsetField,
        }
    }

    /// Check if this field type can have children (Fieldset/Group)
    pub fn can_have_children(&self) -> bool {
        matches!(self, BmsFieldType::Fieldset | BmsFieldType::Group)
    }

    /// Check if this field type is editable by user
    pub fn is_editable(&self) -> bool {
        !matches!(
            self,
            BmsFieldType::Literal | 
            BmsFieldType::ProtectedLiteral | 
            BmsFieldType::Line | 
            BmsFieldType::ImageAsciiArt
        )
    }

    /// Check if this field type is read-only
    pub fn is_readonly(&self) -> bool {
        matches!(
            self,
            BmsFieldType::Literal | 
            BmsFieldType::ProtectedLiteral | 
            BmsFieldType::Line | 
            BmsFieldType::ImageAsciiArt
        )
    }

    /// Get default height for this field type
    pub fn default_height(&self) -> u16 {
        match self {
            BmsFieldType::FieldTextORNumeric | 
            BmsFieldType::Literal | 
            BmsFieldType::ProtectedLiteral | 
            BmsFieldType::BooleanField => 3,
            BmsFieldType::ImageAsciiArt => 5,
            BmsFieldType::Line => 1,
            BmsFieldType::Fieldset | BmsFieldType::Group => 3,
        }
    }

    /// Get default width for this field type
    pub fn default_width(&self) -> u16 {
        match self {
            BmsFieldType::FieldTextORNumeric | BmsFieldType::BooleanField => 10,
            BmsFieldType::Literal | BmsFieldType::ProtectedLiteral => 20,
            BmsFieldType::ImageAsciiArt | BmsFieldType::Line => 40,
            BmsFieldType::Fieldset | BmsFieldType::Group => 40,
        }
    }

    /// Get minimum height for this field type
    pub fn min_height(&self) -> u16 {
        match self {
            BmsFieldType::BooleanField | BmsFieldType::Line => 1,
            _ => 1,
        }
    }

    /// Get maximum height for this field type
    pub fn max_height(&self) -> u16 {
        match self {
            BmsFieldType::BooleanField => 3,
            BmsFieldType::Line => 1,
            _ => 80,
        }
    }

    /// Get minimum width for this field type
    pub fn min_width(&self) -> u16 {
        match self {
            BmsFieldType::FieldTextORNumeric | BmsFieldType::BooleanField => 5,
            BmsFieldType::Literal | BmsFieldType::ProtectedLiteral => 10,
            BmsFieldType::ImageAsciiArt | BmsFieldType::Line => 20,
            BmsFieldType::Fieldset | BmsFieldType::Group => 20,
        }
    }

    /// Get maximum width for this field type
    pub fn max_width(&self) -> u16 {
        255 // All types max at 255 for BMS
    }
}

impl fmt::Display for BmsFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_name())
    }
}

impl From<&str> for BmsFieldType {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "FIELD" | "FIELDTEXTORNUMERIC" | "TEXT" | "NUMERIC" | "INPUT" => BmsFieldType::FieldTextORNumeric,
            "LITERAL" => BmsFieldType::Literal,
            "PROTECTED" | "PROTECTEDLITERAL" => BmsFieldType::ProtectedLiteral,
            "BOOLEAN" | "CHECKBOX" | "BOOL" => BmsFieldType::BooleanField,
            "IMAGE" | "IMAGEASCIIART" | "ASCII" | "ART" => BmsFieldType::ImageAsciiArt,
            "LINE" | "SEPARATOR" | "HLINE" | "VLINE" => BmsFieldType::Line,
            "FIELDSET" | "GROUP" | "CONTAINER" => BmsFieldType::Fieldset,
            _ => BmsFieldType::default(),
        }
    }
}

/// Extended field type that includes BMS statement types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExtendedFieldType {
    /// Main field types (same as BmsFieldType)
    Field(BmsFieldType),
    /// BMS statement types
    DFHMSD,
    DFHMDF,
    DFHMDI,
    DFHMDA,
    DFHMND,
    DFHMNT,
    DFHMDC,
    DFHMDL,
    /// Map type variants
    PhysicalMap,
    SymbolicMap,
    MapSet,
    /// Special field types
    InputOnly,
    OutputOnly,
    InputOutput,
    Hidden,
    /// Data type variants
    Alphanumeric,
    Numeric,
    Date,
    Time,
    Boolean,
    /// Unknown type with string representation
    Unknown(String),
}

impl ExtendedFieldType {
    pub fn as_str(&self) -> &str {
        match self {
            ExtendedFieldType::Field(f) => f.short_name(),
            ExtendedFieldType::DFHMSD => "DFHMSD",
            ExtendedFieldType::DFHMDF => "DFHMDF",
            ExtendedFieldType::DFHMDI => "DFHMDI",
            ExtendedFieldType::DFHMDA => "DFHMDA",
            ExtendedFieldType::DFHMND => "DFHMND",
            ExtendedFieldType::DFHMNT => "DFHMNT",
            ExtendedFieldType::DFHMDC => "DFHMDC",
            ExtendedFieldType::DFHMDL => "DFHMDL",
            ExtendedFieldType::PhysicalMap => "PHYSICAL",
            ExtendedFieldType::SymbolicMap => "SYMBOLIC",
            ExtendedFieldType::MapSet => "MAPSET",
            ExtendedFieldType::InputOnly => "INPUT",
            ExtendedFieldType::OutputOnly => "OUTPUT",
            ExtendedFieldType::InputOutput => "INOUT",
            ExtendedFieldType::Hidden => "HIDDEN",
            ExtendedFieldType::Alphanumeric => "ALNUM",
            ExtendedFieldType::Numeric => "NUM",
            ExtendedFieldType::Date => "DATE",
            ExtendedFieldType::Time => "TIME",
            ExtendedFieldType::Boolean => "BOOL",
            ExtendedFieldType::Unknown(s) => s,
        }
    }
}

impl From<BmsFieldType> for ExtendedFieldType {
    fn from(field_type: BmsFieldType) -> Self {
        ExtendedFieldType::Field(field_type)
    }
}

impl Default for ExtendedFieldType {
    fn default() -> Self {
        ExtendedFieldType::Field(BmsFieldType::default())
    }
}

impl fmt::Display for ExtendedFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Field category for organization in UI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FieldCategory {
    /// Input fields (text, numeric, boolean)
    Input,
    /// Display fields (literal, protected)
    Display,
    /// Container fields (fieldset, group)
    Container,
    /// Separator fields (line)
    Separator,
    /// Image/ASCII art fields
    Image,
    /// Special/system fields
    Special,
}

impl BmsFieldType {
    /// Get category for UI organization
    pub fn category(&self) -> FieldCategory {
        match self {
            BmsFieldType::FieldTextORNumeric | BmsFieldType::BooleanField => FieldCategory::Input,
            BmsFieldType::Literal | BmsFieldType::ProtectedLiteral => FieldCategory::Display,
            BmsFieldType::Fieldset | BmsFieldType::Group => FieldCategory::Container,
            BmsFieldType::Line => FieldCategory::Separator,
            BmsFieldType::ImageAsciiArt => FieldCategory::Image,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_type_display_names() {
        assert_eq!(BmsFieldType::FieldTextORNumeric.display_name(), "Field Text or Numeric");
        assert_eq!(BmsFieldType::Literal.display_name(), "Literal Text");
        assert_eq!(BmsFieldType::ProtectedLiteral.display_name(), "Protected Literal");
        assert_eq!(BmsFieldType::BooleanField.display_name(), "Boolean Field");
        assert_eq!(BmsFieldType::ImageAsciiArt.display_name(), "ImageAsciiArt / Ascii Art");
        assert_eq!(BmsFieldType::Line.display_name(), "Line Separator");
        assert_eq!(BmsFieldType::Fieldset.display_name(), "Fieldset Group");
    }

    #[test]
    fn test_field_type_gui_mapping() {
        assert!(matches!(
            BmsFieldType::FieldTextORNumeric.gui_field_type(),
            GuiFieldType::TextWithLabelField
        ));
        assert!(matches!(
            BmsFieldType::BooleanField.gui_field_type(),
            GuiFieldType::CheckboxWithLabelField
        ));
        assert!(matches!(
            BmsFieldType::ImageAsciiArt.gui_field_type(),
            GuiFieldType::ImageField
        ));
        assert!(matches!(
            BmsFieldType::Line.gui_field_type(),
            GuiFieldType::LineField
        ));
    }

    #[test]
    fn test_field_type_defaults() {
        assert_eq!(BmsFieldType::FieldTextORNumeric.default_height(), 3);
        assert_eq!(BmsFieldType::FieldTextORNumeric.default_width(), 10);
        assert_eq!(BmsFieldType::Literal.default_width(), 20);
        assert_eq!(BmsFieldType::Line.default_height(), 1);
        assert_eq!(BmsFieldType::ImageAsciiArt.default_height(), 5);
    }

    #[test]
    fn test_field_type_can_have_children() {
        assert!(BmsFieldType::Fieldset.can_have_children());
        assert!(BmsFieldType::Group.can_have_children());
        assert!(!BmsFieldType::FieldTextORNumeric.can_have_children());
        assert!(!BmsFieldType::Literal.can_have_children());
    }

    #[test]
    fn test_field_type_from_str() {
        assert!(matches!(
            BmsFieldType::from("FIELD"),
            BmsFieldType::FieldTextORNumeric
        ));
        assert!(matches!(
            BmsFieldType::from("LITERAL"),
            BmsFieldType::Literal
        ));
        assert!(matches!(
            BmsFieldType::from("PROTECTED"),
            BmsFieldType::ProtectedLiteral
        ));
        assert!(matches!(
            BmsFieldType::from("BOOLEAN"),
            BmsFieldType::BooleanField
        ));
    }
}
