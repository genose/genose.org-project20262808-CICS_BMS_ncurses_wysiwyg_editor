//! Centralized defaults for BMS fields - mirrors Lua OBJECTS_DEFINITIONS_DEFAULTS
//!
//! This module provides all the default values for BMS field properties,
//! organized by field type. It mirrors the structure in Lua's OBJECTS_DEFINITIONS_DEFAULTS.

use super::field_types::BmsFieldType;
use super::types::{Color, BorderStyle, TextAlign, VerticalAlign, FillChar, TextStyle};
use super::properties::{Property, ConstrainedProperty, EnumProperty};
use std::collections::HashMap;

// ============================================================================
// SIZE DEFAULTS
// ============================================================================

/// Size defaults per field type - mirrors Lua field_size
#[derive(Debug, Clone)]
pub struct FieldSizeDefaults {
    pub width: HashMap<BmsFieldType, u16>,
    pub height: HashMap<BmsFieldType, u16>,
    pub min_width: HashMap<BmsFieldType, u16>,
    pub max_width: HashMap<BmsFieldType, u16>,
    pub min_height: HashMap<BmsFieldType, u16>,
    pub max_height: HashMap<BmsFieldType, u16>,
}

impl FieldSizeDefaults {
    /// Create new size defaults
    pub fn new() -> Self {
        let mut width = HashMap::new();
        width.insert(BmsFieldType::FieldTextORNumeric, 10);
        width.insert(BmsFieldType::Literal, 20);
        width.insert(BmsFieldType::ProtectedLiteral, 20);
        width.insert(BmsFieldType::BooleanField, 10);
        width.insert(BmsFieldType::ImageAsciiArt, 40);
        width.insert(BmsFieldType::Line, 40);
        width.insert(BmsFieldType::Fieldset, 40);
        width.insert(BmsFieldType::Group, 40);

        let mut height = HashMap::new();
        height.insert(BmsFieldType::FieldTextORNumeric, 3);
        height.insert(BmsFieldType::Literal, 3);
        height.insert(BmsFieldType::ProtectedLiteral, 3);
        height.insert(BmsFieldType::BooleanField, 3);
        height.insert(BmsFieldType::ImageAsciiArt, 5);
        height.insert(BmsFieldType::Line, 1);
        height.insert(BmsFieldType::Fieldset, 3);
        height.insert(BmsFieldType::Group, 3);

        let mut min_width = HashMap::new();
        min_width.insert(BmsFieldType::FieldTextORNumeric, 5);
        min_width.insert(BmsFieldType::Literal, 10);
        min_width.insert(BmsFieldType::ProtectedLiteral, 10);
        min_width.insert(BmsFieldType::BooleanField, 5);
        min_width.insert(BmsFieldType::ImageAsciiArt, 20);
        min_width.insert(BmsFieldType::Line, 20);
        min_width.insert(BmsFieldType::Fieldset, 20);
        min_width.insert(BmsFieldType::Group, 20);

        let mut max_width = HashMap::new();
        max_width.insert(BmsFieldType::FieldTextORNumeric, 255);
        max_width.insert(BmsFieldType::Literal, 255);
        max_width.insert(BmsFieldType::ProtectedLiteral, 255);
        max_width.insert(BmsFieldType::BooleanField, 255);
        max_width.insert(BmsFieldType::ImageAsciiArt, 255);
        max_width.insert(BmsFieldType::Line, 255);
        max_width.insert(BmsFieldType::Fieldset, 255);
        max_width.insert(BmsFieldType::Group, 255);

        let mut min_height = HashMap::new();
        min_height.insert(BmsFieldType::FieldTextORNumeric, 1);
        min_height.insert(BmsFieldType::Literal, 1);
        min_height.insert(BmsFieldType::ProtectedLiteral, 1);
        min_height.insert(BmsFieldType::BooleanField, 1);
        min_height.insert(BmsFieldType::ImageAsciiArt, 3);
        min_height.insert(BmsFieldType::Line, 1);
        min_height.insert(BmsFieldType::Fieldset, 1);
        min_height.insert(BmsFieldType::Group, 1);

        let mut max_height = HashMap::new();
        max_height.insert(BmsFieldType::FieldTextORNumeric, 80);
        max_height.insert(BmsFieldType::Literal, 80);
        max_height.insert(BmsFieldType::ProtectedLiteral, 80);
        max_height.insert(BmsFieldType::BooleanField, 3);
        max_height.insert(BmsFieldType::ImageAsciiArt, 40);
        max_height.insert(BmsFieldType::Line, 1);
        max_height.insert(BmsFieldType::Fieldset, 80);
        max_height.insert(BmsFieldType::Group, 80);

        Self {
            width,
            height,
            min_width,
            max_width,
            min_height,
            max_height,
        }
    }

    /// Get default width for a field type
    pub fn get_width(&self, field_type: BmsFieldType) -> u16 {
        *self.width.get(&field_type).unwrap_or(&10)
    }

    /// Get default height for a field type
    pub fn get_height(&self, field_type: BmsFieldType) -> u16 {
        *self.height.get(&field_type).unwrap_or(&3)
    }

    /// Get minimum width for a field type
    pub fn get_min_width(&self, field_type: BmsFieldType) -> u16 {
        *self.min_width.get(&field_type).unwrap_or(&1)
    }

    /// Get maximum width for a field type
    pub fn get_max_width(&self, field_type: BmsFieldType) -> u16 {
        *self.max_width.get(&field_type).unwrap_or(&255)
    }

    /// Get minimum height for a field type
    pub fn get_min_height(&self, field_type: BmsFieldType) -> u16 {
        *self.min_height.get(&field_type).unwrap_or(&1)
    }

    /// Get maximum height for a field type
    pub fn get_max_height(&self, field_type: BmsFieldType) -> u16 {
        *self.max_height.get(&field_type).unwrap_or(&80)
    }

    /// Create a constrained width property for a field type
    pub fn width_property(&self, field_type: BmsFieldType) -> ConstrainedProperty<u16> {
        let initial = self.get_width(field_type);
        let min = self.get_min_width(field_type);
        let max = self.get_max_width(field_type);
        ConstrainedProperty::new(initial, min, max)
    }

    /// Create a constrained height property for a field type
    pub fn height_property(&self, field_type: BmsFieldType) -> ConstrainedProperty<u16> {
        let initial = self.get_height(field_type);
        let min = self.get_min_height(field_type);
        let max = self.get_max_height(field_type);
        ConstrainedProperty::new(initial, min, max)
    }
}

impl Default for FieldSizeDefaults {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// COLOR DEFAULTS
// ============================================================================

/// Color defaults per field type - mirrors Lua field_avail_color.default
#[derive(Debug, Clone)]
pub struct FieldColorDefaults {
    /// All available colors
    pub available_colors: Vec<Color>,
    /// Default text color per field type
    pub default_text_color: HashMap<BmsFieldType, Color>,
    /// Default border color per field type
    pub default_border_color: HashMap<BmsFieldType, Color>,
    /// Default title color per field type
    pub default_title_color: HashMap<BmsFieldType, Color>,
    /// Default footer color per field type
    pub default_footer_color: HashMap<BmsFieldType, Color>,
    /// Default available footer color per field type
    pub default_avail_footer_color: HashMap<BmsFieldType, Vec<Color>>,
}

impl FieldColorDefaults {
    pub fn new() -> Self {
        // All available colors for BMS
        let available_colors = vec![
            Color::Default, Color::Black, Color::Red, Color::Green,
            Color::Yellow, Color::Blue, Color::Magenta, Color::Cyan, Color::White,
            Color::Turquoise, Color::Pink, Color::Orange, Color::Purple,
            Color::Gray, Color::LightGreen, Color::LightBlue, Color::LightCyan,
            Color::LightRed, Color::LightMagenta, Color::LightYellow, Color::Neutral,
        ];

        // Default text colors per field type (mirrors Lua field_text_color.default)
        let mut default_text_color = HashMap::new();
        default_text_color.insert(BmsFieldType::FieldTextORNumeric, Color::Yellow);
        default_text_color.insert(BmsFieldType::Literal, Color::Green);
        default_text_color.insert(BmsFieldType::ProtectedLiteral, Color::White);
        default_text_color.insert(BmsFieldType::BooleanField, Color::Green);
        default_text_color.insert(BmsFieldType::ImageAsciiArt, Color::Default);
        default_text_color.insert(BmsFieldType::Line, Color::Blue);
        default_text_color.insert(BmsFieldType::Fieldset, Color::Default);
        default_text_color.insert(BmsFieldType::Group, Color::Default);

        // Default border colors per field type (mirrors Lua field_border_color.default)
        let mut default_border_color = HashMap::new();
        default_border_color.insert(BmsFieldType::FieldTextORNumeric, Color::Default);
        default_border_color.insert(BmsFieldType::Literal, Color::Default);
        default_border_color.insert(BmsFieldType::ProtectedLiteral, Color::Green);
        default_border_color.insert(BmsFieldType::BooleanField, Color::Default);
        default_border_color.insert(BmsFieldType::ImageAsciiArt, Color::Default);
        default_border_color.insert(BmsFieldType::Line, Color::Default);
        default_border_color.insert(BmsFieldType::Fieldset, Color::Blue);
        default_border_color.insert(BmsFieldType::Group, Color::Blue);

        // Default title colors per field type (mirrors Lua field_title_color.default)
        let mut default_title_color = HashMap::new();
        default_title_color.insert(BmsFieldType::FieldTextORNumeric, Color::Default);
        default_title_color.insert(BmsFieldType::Literal, Color::Default);
        default_title_color.insert(BmsFieldType::ProtectedLiteral, Color::White);
        default_title_color.insert(BmsFieldType::BooleanField, Color::Default);
        default_title_color.insert(BmsFieldType::ImageAsciiArt, Color::Default);
        default_title_color.insert(BmsFieldType::Line, Color::Default);
        default_title_color.insert(BmsFieldType::Fieldset, Color::Blue);
        default_title_color.insert(BmsFieldType::Group, Color::Blue);

        // Default footer colors per field type (mirrors Lua field_footer_color.default)
        let mut default_footer_color = HashMap::new();
        default_footer_color.insert(BmsFieldType::FieldTextORNumeric, Color::Default);
        default_footer_color.insert(BmsFieldType::Literal, Color::Default);
        default_footer_color.insert(BmsFieldType::ProtectedLiteral, Color::Default);
        default_footer_color.insert(BmsFieldType::BooleanField, Color::Default);
        default_footer_color.insert(BmsFieldType::ImageAsciiArt, Color::Default);
        default_footer_color.insert(BmsFieldType::Line, Color::Default);
        default_footer_color.insert(BmsFieldType::Fieldset, Color::Default);
        default_footer_color.insert(BmsFieldType::Group, Color::Default);

        // Default available footer colors per field type
        let mut default_avail_footer_color = HashMap::new();
        for field_type in [
            BmsFieldType::FieldTextORNumeric,
            BmsFieldType::Literal,
            BmsFieldType::ProtectedLiteral,
            BmsFieldType::BooleanField,
            BmsFieldType::ImageAsciiArt,
            BmsFieldType::Line,
            BmsFieldType::Fieldset,
            BmsFieldType::Group,
        ] {
            default_avail_footer_color.insert(field_type, available_colors.clone());
        }

        Self {
            available_colors,
            default_text_color,
            default_border_color,
            default_title_color,
            default_footer_color,
            default_avail_footer_color,
        }
    }

    /// Get default text color for a field type
    pub fn get_text_color(&self, field_type: BmsFieldType) -> Color {
        *self.default_text_color.get(&field_type).unwrap_or(&Color::Default)
    }

    /// Get default border color for a field type
    pub fn get_border_color(&self, field_type: BmsFieldType) -> Color {
        *self.default_border_color.get(&field_type).unwrap_or(&Color::Default)
    }

    /// Get default title color for a field type
    pub fn get_title_color(&self, field_type: BmsFieldType) -> Color {
        *self.default_title_color.get(&field_type).unwrap_or(&Color::Default)
    }

    /// Get default footer color for a field type
    pub fn get_footer_color(&self, field_type: BmsFieldType) -> Color {
        *self.default_footer_color.get(&field_type).unwrap_or(&Color::Default)
    }

    /// Get available footer colors for a field type
    pub fn get_avail_footer_colors(&self, field_type: BmsFieldType) -> &Vec<Color> {
        self.default_avail_footer_color.get(&field_type)
            .unwrap_or(&self.available_colors)
    }

    /// Create a text color property for a field type
    pub fn text_color_property(&self, field_type: BmsFieldType) -> EnumProperty<Color> {
        let initial = self.get_text_color(field_type);
        EnumProperty::new(initial, self.available_colors.clone())
    }

    /// Create a border color property for a field type
    pub fn border_color_property(&self, field_type: BmsFieldType) -> EnumProperty<Color> {
        let initial = self.get_border_color(field_type);
        EnumProperty::new(initial, self.available_colors.clone())
    }

    /// Create a title color property for a field type
    pub fn title_color_property(&self, field_type: BmsFieldType) -> EnumProperty<Color> {
        let initial = self.get_title_color(field_type);
        EnumProperty::new(initial, self.available_colors.clone())
    }

    /// Create a footer color property for a field type
    pub fn footer_color_property(&self, field_type: BmsFieldType) -> EnumProperty<Color> {
        let initial = self.get_footer_color(field_type);
        let available = self.get_avail_footer_colors(field_type).clone();
        EnumProperty::new(initial, available)
    }

    /// Create available colors property
    pub fn available_colors_property(&self) -> Property<Vec<Color>> {
        Property::new(self.available_colors.clone())
    }
}

impl Default for FieldColorDefaults {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// STYLE DEFAULTS
// ============================================================================

/// Style defaults per field type - mirrors Lua field_avail_style.default
#[derive(Debug, Clone)]
pub struct FieldStyleDefaults {
    /// All available text styles
    pub available_styles: Vec<TextStyle>,
    /// Default style per field type
    pub default_style: HashMap<BmsFieldType, TextStyle>,
    /// Available styles per field type
    pub available_styles_per_type: HashMap<BmsFieldType, Vec<TextStyle>>,
}

impl FieldStyleDefaults {
    pub fn new() -> Self {
        // All available styles
        let available_styles = vec![
            TextStyle::Default,
            TextStyle::Bold,
            TextStyle::Italic,
            TextStyle::Underline,
            TextStyle::Blink,
            TextStyle::Reverse,
            TextStyle::StrikeThrough,
        ];

        // Default style per field type
        let mut default_style = HashMap::new();
        default_style.insert(BmsFieldType::FieldTextORNumeric, TextStyle::Default);
        default_style.insert(BmsFieldType::Literal, TextStyle::Default);
        default_style.insert(BmsFieldType::ProtectedLiteral, TextStyle::Default);
        default_style.insert(BmsFieldType::BooleanField, TextStyle::Default);
        default_style.insert(BmsFieldType::ImageAsciiArt, TextStyle::Default);
        default_style.insert(BmsFieldType::Line, TextStyle::Underline);
        default_style.insert(BmsFieldType::Fieldset, TextStyle::Default);
        default_style.insert(BmsFieldType::Group, TextStyle::Default);

        // Available styles per field type (mirrors Lua field_avail_style.default)
        let mut available_styles_per_type = HashMap::new();
        
        // Field: All styles available
        available_styles_per_type.insert(
            BmsFieldType::FieldTextORNumeric,
            vec![
                TextStyle::Default, TextStyle::Bold, TextStyle::Italic,
                TextStyle::Underline, TextStyle::Blink, TextStyle::Reverse
            ]
        );
        
        // Literal: All styles
        available_styles_per_type.insert(
            BmsFieldType::Literal,
            vec![
                TextStyle::Default, TextStyle::Bold, TextStyle::Italic,
                TextStyle::Underline, TextStyle::Blink, TextStyle::Reverse
            ]
        );
        
        // ProtectedLiteral: No italic/strikethrough/blink (distracting for read-only)
        available_styles_per_type.insert(
            BmsFieldType::ProtectedLiteral,
            vec![
                TextStyle::Default, TextStyle::Bold, TextStyle::Underline, TextStyle::Reverse
            ]
        );
        
        // BooleanField: No italic/strikethrough; blink for attention
        available_styles_per_type.insert(
            BmsFieldType::BooleanField,
            vec![
                TextStyle::Default, TextStyle::Bold, TextStyle::Underline,
                TextStyle::Blink, TextStyle::Reverse
            ]
        );
        
        // ImageAsciiArt: No italic/strikethrough/blink (distracting for placeholder)
        available_styles_per_type.insert(
            BmsFieldType::ImageAsciiArt,
            vec![
                TextStyle::Default, TextStyle::Bold, TextStyle::Underline, TextStyle::Reverse
            ]
        );
        
        // Line: underline for line effect, strikethrough for broken line
        available_styles_per_type.insert(
            BmsFieldType::Line,
            vec![
                TextStyle::Underline, TextStyle::StrikeThrough,
                TextStyle::Default, TextStyle::Bold, TextStyle::Reverse
            ]
        );
        
        // Fieldset: All styles except strikethrough (not relevant for borders)
        available_styles_per_type.insert(
            BmsFieldType::Fieldset,
            vec![
                TextStyle::Default, TextStyle::Bold, TextStyle::Underline,
                TextStyle::Blink, TextStyle::Reverse
            ]
        );
        
        available_styles_per_type.insert(BmsFieldType::Group, available_styles_per_type[&BmsFieldType::Fieldset].clone());

        Self {
            available_styles,
            default_style,
            available_styles_per_type,
        }
    }

    /// Get default style for a field type
    pub fn get_default_style(&self, field_type: BmsFieldType) -> TextStyle {
        *self.default_style.get(&field_type).unwrap_or(&TextStyle::Default)
    }

    /// Get available styles for a field type
    pub fn get_available_styles(&self, field_type: BmsFieldType) -> &Vec<TextStyle> {
        self.available_styles_per_type.get(&field_type)
            .unwrap_or(&self.available_styles)
    }

    /// Create a style property for a field type
    pub fn style_property(&self, field_type: BmsFieldType) -> EnumProperty<TextStyle> {
        let initial = self.get_default_style(field_type);
        let available = self.get_available_styles(field_type).clone();
        EnumProperty::new(initial, available)
    }

    /// Create an available styles property
    pub fn available_styles_property(&self) -> Property<Vec<TextStyle>> {
        Property::new(self.available_styles.clone())
    }
}

impl Default for FieldStyleDefaults {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ALIGNMENT DEFAULTS
// ============================================================================

/// Alignment defaults per field type
#[derive(Debug, Clone)]
pub struct TextAlignDefaults {
    /// All available text alignments
    pub available_aligns: Vec<TextAlign>,
    /// Default text alignment per field type
    pub default_text_align: HashMap<BmsFieldType, TextAlign>,
    /// Default title alignment per field type
    pub default_title_align: HashMap<BmsFieldType, TextAlign>,
    /// Default footer alignment per field type
    pub default_footer_align: HashMap<BmsFieldType, TextAlign>,
    /// Available text alignments per field type
    pub available_text_aligns_per_type: HashMap<BmsFieldType, Vec<TextAlign>>,
}

impl TextAlignDefaults {
    pub fn new() -> Self {
        let available_aligns = vec![TextAlign::Left, TextAlign::Center, TextAlign::Right];

        // Default text alignment per field type
        let mut default_text_align = HashMap::new();
        default_text_align.insert(BmsFieldType::FieldTextORNumeric, TextAlign::Left);
        default_text_align.insert(BmsFieldType::Literal, TextAlign::Left);
        default_text_align.insert(BmsFieldType::ProtectedLiteral, TextAlign::Left);
        default_text_align.insert(BmsFieldType::BooleanField, TextAlign::Left);
        default_text_align.insert(BmsFieldType::ImageAsciiArt, TextAlign::Left);
        default_text_align.insert(BmsFieldType::Line, TextAlign::Center);
        default_text_align.insert(BmsFieldType::Fieldset, TextAlign::Left);
        default_text_align.insert(BmsFieldType::Group, TextAlign::Left);

        // Default title alignment per field type
        let mut default_title_align = HashMap::new();
        default_title_align.insert(BmsFieldType::FieldTextORNumeric, TextAlign::Left);
        default_title_align.insert(BmsFieldType::Literal, TextAlign::Left);
        default_title_align.insert(BmsFieldType::ProtectedLiteral, TextAlign::Left);
        default_title_align.insert(BmsFieldType::BooleanField, TextAlign::Left);
        default_title_align.insert(BmsFieldType::ImageAsciiArt, TextAlign::Center);
        default_title_align.insert(BmsFieldType::Line, TextAlign::Center);
        default_title_align.insert(BmsFieldType::Fieldset, TextAlign::Center);
        default_title_align.insert(BmsFieldType::Group, TextAlign::Center);

        // Default footer alignment per field type
        let mut default_footer_align = HashMap::new();
        for field_type in [
            BmsFieldType::FieldTextORNumeric,
            BmsFieldType::Literal,
            BmsFieldType::ProtectedLiteral,
            BmsFieldType::BooleanField,
            BmsFieldType::ImageAsciiArt,
            BmsFieldType::Line,
            BmsFieldType::Fieldset,
            BmsFieldType::Group,
        ] {
            default_footer_align.insert(field_type, TextAlign::Center);
        }

        // Available text alignments per field type (all types support all alignments)
        let mut available_text_aligns_per_type = HashMap::new();
        for field_type in [
            BmsFieldType::FieldTextORNumeric,
            BmsFieldType::Literal,
            BmsFieldType::ProtectedLiteral,
            BmsFieldType::BooleanField,
            BmsFieldType::ImageAsciiArt,
            BmsFieldType::Line,
            BmsFieldType::Fieldset,
            BmsFieldType::Group,
        ] {
            available_text_aligns_per_type.insert(field_type, available_aligns.clone());
        }

        Self {
            available_aligns,
            default_text_align,
            default_title_align,
            default_footer_align,
            available_text_aligns_per_type,
        }
    }

    /// Get default text alignment for a field type
    pub fn get_default_text_align(&self, field_type: BmsFieldType) -> TextAlign {
        *self.default_text_align.get(&field_type).unwrap_or(&TextAlign::Left)
    }

    /// Get default title alignment for a field type
    pub fn get_default_title_align(&self, field_type: BmsFieldType) -> TextAlign {
        *self.default_title_align.get(&field_type).unwrap_or(&TextAlign::Left)
    }

    /// Get default footer alignment for a field type
    pub fn get_default_footer_align(&self, field_type: BmsFieldType) -> TextAlign {
        *self.default_footer_align.get(&field_type).unwrap_or(&TextAlign::Center)
    }

    /// Create a text align property for a field type
    pub fn text_align_property(&self, field_type: BmsFieldType) -> EnumProperty<TextAlign> {
        let initial = self.get_default_text_align(field_type);
        EnumProperty::new(initial, self.available_aligns.clone())
    }

    /// Create a title align property for a field type
    pub fn title_align_property(&self, field_type: BmsFieldType) -> EnumProperty<TextAlign> {
        let initial = self.get_default_title_align(field_type);
        EnumProperty::new(initial, self.available_aligns.clone())
    }

    /// Create a footer align property for a field type
    pub fn footer_align_property(&self, field_type: BmsFieldType) -> EnumProperty<TextAlign> {
        let initial = self.get_default_footer_align(field_type);
        EnumProperty::new(initial, self.available_aligns.clone())
    }
}

impl Default for TextAlignDefaults {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// BORDER DEFAULTS
// ============================================================================

/// Border defaults per field type - mirrors Lua field_avail_border_style.default
#[derive(Debug, Clone)]
pub struct BorderDefaults {
    /// All available border styles
    pub available_border_styles: Vec<BorderStyle>,
    /// Default border style per field type
    pub default_border_style: HashMap<BmsFieldType, BorderStyle>,
    /// Available border styles per field type
    pub available_border_styles_per_type: HashMap<BmsFieldType, Vec<BorderStyle>>,
    /// Default border characters
    pub default_border_chars: HashMap<BmsFieldType, BorderCharSet>,
}

use super::types::BorderCharSet;

impl BorderDefaults {
    pub fn new() -> Self {
        let available_border_styles = vec![
            BorderStyle::None,
            BorderStyle::Single,
            BorderStyle::Double,
            BorderStyle::Dashed,
            BorderStyle::Dotted,
            BorderStyle::Solid,
        ];

        // Default border style per field type
        let mut default_border_style = HashMap::new();
        default_border_style.insert(BmsFieldType::FieldTextORNumeric, BorderStyle::None);
        default_border_style.insert(BmsFieldType::Literal, BorderStyle::None);
        default_border_style.insert(BmsFieldType::ProtectedLiteral, BorderStyle::Dashed);
        default_border_style.insert(BmsFieldType::BooleanField, BorderStyle::Single);
        default_border_style.insert(BmsFieldType::ImageAsciiArt, BorderStyle::Double);
        default_border_style.insert(BmsFieldType::Line, BorderStyle::None);
        default_border_style.insert(BmsFieldType::Fieldset, BorderStyle::Double);
        default_border_style.insert(BmsFieldType::Group, BorderStyle::Double);

        // Available border styles per field type (mirrors Lua field_avail_border_style.default)
        let mut available_border_styles_per_type = HashMap::new();
        
        // Field: All border styles
        available_border_styles_per_type.insert(
            BmsFieldType::FieldTextORNumeric,
            vec![
                BorderStyle::None, BorderStyle::Single, BorderStyle::Double, BorderStyle::Dashed
            ]
        );
        
        // Literal: No border by default, but can have single/dashed
        available_border_styles_per_type.insert(
            BmsFieldType::Literal,
            vec![
                BorderStyle::None, BorderStyle::Single, BorderStyle::Dashed
            ]
        );
        
        // ProtectedLiteral: Dashed for protection indication
        available_border_styles_per_type.insert(
            BmsFieldType::ProtectedLiteral,
            vec![
                BorderStyle::Dashed, BorderStyle::Single, BorderStyle::Double, BorderStyle::None
            ]
        );
        
        // BooleanField: Simple borders for checkboxes
        available_border_styles_per_type.insert(
            BmsFieldType::BooleanField,
            vec![
                BorderStyle::Single, BorderStyle::Double, BorderStyle::Dashed
            ]
        );
        
        // ImageAsciiArt: Double border for placeholders
        available_border_styles_per_type.insert(
            BmsFieldType::ImageAsciiArt,
            vec![
                BorderStyle::Double, BorderStyle::Single, BorderStyle::Dashed, BorderStyle::None
            ]
        );
        
        // Line: No border (it's already a line)
        available_border_styles_per_type.insert(
            BmsFieldType::Line,
            vec![BorderStyle::None]
        );
        
        // Fieldset: Double border for containers
        available_border_styles_per_type.insert(
            BmsFieldType::Fieldset,
            vec![
                BorderStyle::Double, BorderStyle::Single, BorderStyle::Dashed
            ]
        );
        
        available_border_styles_per_type.insert(BmsFieldType::Group, available_border_styles_per_type[&BmsFieldType::Fieldset].clone());

        // Default border characters per field type
        let mut default_border_chars = HashMap::new();
        let single_chars = BorderCharSet::single();
        let double_chars = BorderCharSet::double();

        for field_type in [
            BmsFieldType::FieldTextORNumeric,
            BmsFieldType::Literal,
            BmsFieldType::ProtectedLiteral,
            BmsFieldType::BooleanField,
            BmsFieldType::ImageAsciiArt,
            BmsFieldType::Line,
            BmsFieldType::Fieldset,
            BmsFieldType::Group,
        ] {
            // Default to single border characters for most types
            // Fieldset and Group get double by default
            if matches!(field_type, BmsFieldType::Fieldset | BmsFieldType::Group) {
                default_border_chars.insert(field_type, double_chars.clone());
            } else {
                default_border_chars.insert(field_type, single_chars.clone());
            }
        }

        Self {
            available_border_styles,
            default_border_style,
            available_border_styles_per_type,
            default_border_chars,
        }
    }

    /// Get default border style for a field type
    pub fn get_default_border_style(&self, field_type: BmsFieldType) -> BorderStyle {
        *self.default_border_style.get(&field_type).unwrap_or(&BorderStyle::None)
    }

    /// Get available border styles for a field type
    pub fn get_available_border_styles(&self, field_type: BmsFieldType) -> &Vec<BorderStyle> {
        self.available_border_styles_per_type.get(&field_type)
            .unwrap_or(&self.available_border_styles)
    }

    /// Get default border characters for a field type
    pub fn get_default_border_chars(&self, field_type: BmsFieldType) -> BorderCharSet {
        self.default_border_chars.get(&field_type)
            .cloned()
            .unwrap_or_else(BorderCharSet::single)
    }

    /// Create a border style property for a field type
    pub fn border_style_property(&self, field_type: BmsFieldType) -> EnumProperty<BorderStyle> {
        let initial = self.get_default_border_style(field_type);
        let available = self.get_available_border_styles(field_type).clone();
        EnumProperty::new(initial, available)
    }
}

impl Default for BorderDefaults {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// FILL CHARACTER DEFAULTS
// ============================================================================

/// Fill character defaults per field type
#[derive(Debug, Clone)]
pub struct FillCharDefaults {
    /// All available fill characters
    pub available_fill_chars: Vec<FillChar>,
    /// Default fill character per field type
    pub default_fill_char: HashMap<BmsFieldType, FillChar>,
    /// Default title fill character per field type
    pub default_title_fill_char: HashMap<BmsFieldType, FillChar>,
    /// Default footer fill character per field type
    pub default_footer_fill_char: HashMap<BmsFieldType, FillChar>,
    /// Available fill characters per field type
    pub available_fill_chars_per_type: HashMap<BmsFieldType, Vec<FillChar>>,
}

impl FillCharDefaults {
    pub fn new() -> Self {
        // Common fill characters
        let available_fill_chars = vec![
            FillChar::Space,
            FillChar::Dash,
            FillChar::Equal,
            FillChar::Underscore,
            FillChar::Dot,
            FillChar::Asterisk,
            FillChar::Pipe,
        ];

        // Default fill character per field type
        let mut default_fill_char = HashMap::new();
        default_fill_char.insert(BmsFieldType::FieldTextORNumeric, FillChar::Underscore);
        default_fill_char.insert(BmsFieldType::Literal, FillChar::Space);
        default_fill_char.insert(BmsFieldType::ProtectedLiteral, FillChar::Space);
        default_fill_char.insert(BmsFieldType::BooleanField, FillChar::Space);
        default_fill_char.insert(BmsFieldType::ImageAsciiArt, FillChar::Space);
        default_fill_char.insert(BmsFieldType::Line, FillChar::Dash);
        default_fill_char.insert(BmsFieldType::Fieldset, FillChar::Space);
        default_fill_char.insert(BmsFieldType::Group, FillChar::Space);

        // Default title fill character per field type
        let mut default_title_fill_char = HashMap::new();
        default_title_fill_char.insert(BmsFieldType::FieldTextORNumeric, FillChar::Space);
        default_title_fill_char.insert(BmsFieldType::Literal, FillChar::Space);
        default_title_fill_char.insert(BmsFieldType::ProtectedLiteral, FillChar::Space);
        default_title_fill_char.insert(BmsFieldType::BooleanField, FillChar::Space);
        default_title_fill_char.insert(BmsFieldType::ImageAsciiArt, FillChar::Dash);
        default_title_fill_char.insert(BmsFieldType::Line, FillChar::Dash);
        default_title_fill_char.insert(BmsFieldType::Fieldset, FillChar::Dash);
        default_title_fill_char.insert(BmsFieldType::Group, FillChar::Dash);

        // Default footer fill character per field type
        let mut default_footer_fill_char = HashMap::new();
        for field_type in [
            BmsFieldType::FieldTextORNumeric,
            BmsFieldType::Literal,
            BmsFieldType::ProtectedLiteral,
            BmsFieldType::BooleanField,
            BmsFieldType::ImageAsciiArt,
            BmsFieldType::Line,
            BmsFieldType::Fieldset,
            BmsFieldType::Group,
        ] {
            default_footer_fill_char.insert(field_type, FillChar::Space);
        }

        // Available fill characters per field type
        let mut available_fill_chars_per_type = HashMap::new();
        for field_type in [
            BmsFieldType::FieldTextORNumeric,
            BmsFieldType::Literal,
            BmsFieldType::ProtectedLiteral,
            BmsFieldType::BooleanField,
            BmsFieldType::ImageAsciiArt,
            BmsFieldType::Line,
            BmsFieldType::Fieldset,
            BmsFieldType::Group,
        ] {
            available_fill_chars_per_type.insert(field_type, available_fill_chars.clone());
        }

        Self {
            available_fill_chars,
            default_fill_char,
            default_title_fill_char,
            default_footer_fill_char,
            available_fill_chars_per_type,
        }
    }

    /// Get default fill character for a field type
    pub fn get_default_fill_char(&self, field_type: BmsFieldType) -> FillChar {
        *self.default_fill_char.get(&field_type).unwrap_or(&FillChar::Space)
    }

    /// Get default title fill character for a field type
    pub fn get_default_title_fill_char(&self, field_type: BmsFieldType) -> FillChar {
        *self.default_title_fill_char.get(&field_type).unwrap_or(&FillChar::Space)
    }

    /// Get default footer fill character for a field type
    pub fn get_default_footer_fill_char(&self, field_type: BmsFieldType) -> FillChar {
        *self.default_footer_fill_char.get(&field_type).unwrap_or(&FillChar::Space)
    }

    /// Create a fill char property for a field type
    pub fn fill_char_property(&self, field_type: BmsFieldType) -> EnumProperty<FillChar> {
        let initial = self.get_default_fill_char(field_type);
        let available = self.available_fill_chars_per_type.get(&field_type)
            .cloned()
            .unwrap_or_else(|| self.available_fill_chars.clone());
        EnumProperty::new(initial, available)
    }

    /// Create a title fill char property for a field type
    pub fn title_fill_char_property(&self, field_type: BmsFieldType) -> EnumProperty<FillChar> {
        let initial = self.get_default_title_fill_char(field_type);
        let available = self.available_fill_chars_per_type.get(&field_type)
            .cloned()
            .unwrap_or_else(|| self.available_fill_chars.clone());
        EnumProperty::new(initial, available)
    }

    /// Create a footer fill char property for a field type
    pub fn footer_fill_char_property(&self, field_type: BmsFieldType) -> EnumProperty<FillChar> {
        let initial = self.get_default_footer_fill_char(field_type);
        let available = self.available_fill_chars_per_type.get(&field_type)
            .cloned()
            .unwrap_or_else(|| self.available_fill_chars.clone());
        EnumProperty::new(initial, available)
    }
}

impl Default for FillCharDefaults {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// VERTICAL ALIGN DEFAULTS
// ============================================================================

/// Vertical alignment defaults per field type
#[derive(Debug, Clone)]
pub struct VerticalAlignDefaults {
    /// All available vertical alignments
    pub available_vertical_aligns: Vec<VerticalAlign>,
    /// Default vertical alignment per field type
    pub default_vertical_align: HashMap<BmsFieldType, VerticalAlign>,
    /// Available vertical alignments per field type
    pub available_vertical_aligns_per_type: HashMap<BmsFieldType, Vec<VerticalAlign>>,
}

impl VerticalAlignDefaults {
    pub fn new() -> Self {
        let available_vertical_aligns = vec![
            VerticalAlign::Top,
            VerticalAlign::Middle,
            VerticalAlign::Bottom,
        ];

        // Default vertical alignment per field type
        let mut default_vertical_align = HashMap::new();
        default_vertical_align.insert(BmsFieldType::FieldTextORNumeric, VerticalAlign::Top);
        default_vertical_align.insert(BmsFieldType::Literal, VerticalAlign::Top);
        default_vertical_align.insert(BmsFieldType::ProtectedLiteral, VerticalAlign::Top);
        default_vertical_align.insert(BmsFieldType::BooleanField, VerticalAlign::Top);
        default_vertical_align.insert(BmsFieldType::ImageAsciiArt, VerticalAlign::Top);
        default_vertical_align.insert(BmsFieldType::Line, VerticalAlign::Top);
        default_vertical_align.insert(BmsFieldType::Fieldset, VerticalAlign::Top);
        default_vertical_align.insert(BmsFieldType::Group, VerticalAlign::Top);

        // Available vertical alignments per field type
        let mut available_vertical_aligns_per_type = HashMap::new();
        
        // Most fields support all vertical alignments
        for field_type in [
            BmsFieldType::FieldTextORNumeric,
            BmsFieldType::Literal,
            BmsFieldType::ProtectedLiteral,
            BmsFieldType::BooleanField,
        ] {
            available_vertical_aligns_per_type.insert(field_type, available_vertical_aligns.clone());
        }
        
        // ImageAsciiArt and Fieldset support top and bottom
        for field_type in [BmsFieldType::ImageAsciiArt, BmsFieldType::Fieldset, BmsFieldType::Group] {
            available_vertical_aligns_per_type.insert(
                field_type,
                vec![VerticalAlign::Top, VerticalAlign::Bottom]
            );
        }
        
        // Line only supports top
        available_vertical_aligns_per_type.insert(
            BmsFieldType::Line,
            vec![VerticalAlign::Top]
        );

        Self {
            available_vertical_aligns,
            default_vertical_align,
            available_vertical_aligns_per_type,
        }
    }

    /// Get default vertical alignment for a field type
    pub fn get_default_vertical_align(&self, field_type: BmsFieldType) -> VerticalAlign {
        *self.default_vertical_align.get(&field_type).unwrap_or(&VerticalAlign::Top)
    }

    /// Get available vertical alignments for a field type
    pub fn get_available_vertical_aligns(&self, field_type: BmsFieldType) -> &Vec<VerticalAlign> {
        self.available_vertical_aligns_per_type.get(&field_type)
            .unwrap_or(&self.available_vertical_aligns)
    }

    /// Create a vertical align property for a field type
    pub fn vertical_align_property(&self, field_type: BmsFieldType) -> EnumProperty<VerticalAlign> {
        let initial = self.get_default_vertical_align(field_type);
        let available = self.get_available_vertical_aligns(field_type).clone();
        EnumProperty::new(initial, available)
    }
}

impl Default for VerticalAlignDefaults {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// VERTICAL MARGIN DEFAULTS
// ============================================================================

/// Vertical margin defaults per field type
#[derive(Debug, Clone)]
pub struct VerticalMarginDefaults {
    /// All available vertical margins
    pub available_vertical_margins: Vec<VerticalMargin>,
    /// Default vertical margin per field type
    pub default_vertical_margin: HashMap<BmsFieldType, VerticalMargin>,
}

use super::types::VerticalMargin;

impl VerticalMarginDefaults {
    pub fn new() -> Self {
        let available_vertical_margins = vec![
            VerticalMargin::None,
            VerticalMargin::Small,
            VerticalMargin::Medium,
            VerticalMargin::Large,
        ];

        let mut default_vertical_margin = HashMap::new();
        for field_type in [
            BmsFieldType::FieldTextORNumeric,
            BmsFieldType::Literal,
            BmsFieldType::ProtectedLiteral,
            BmsFieldType::BooleanField,
            BmsFieldType::ImageAsciiArt,
            BmsFieldType::Line,
            BmsFieldType::Fieldset,
            BmsFieldType::Group,
        ] {
            default_vertical_margin.insert(field_type, VerticalMargin::None);
        }

        Self {
            available_vertical_margins,
            default_vertical_margin,
        }
    }

    /// Get default vertical margin for a field type
    pub fn get_default_vertical_margin(&self, field_type: BmsFieldType) -> VerticalMargin {
        *self.default_vertical_margin.get(&field_type).unwrap_or(&VerticalMargin::None)
    }

    /// Create a vertical margin property for a field type
    pub fn vertical_margin_property(&self, field_type: BmsFieldType) -> EnumProperty<VerticalMargin> {
        let initial = self.get_default_vertical_margin(field_type);
        EnumProperty::new(initial, self.available_vertical_margins.clone())
    }
}

impl Default for VerticalMarginDefaults {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MAIN DEFAULTS STRUCTURE
// ============================================================================

/// Main collection of all BMS field defaults
/// Mirrors the structure in Lua's OBJECTS_DEFINITIONS_DEFAULTS
#[derive(Debug, Clone, Default)]
pub struct BmsDefaults {
    pub size: FieldSizeDefaults,
    pub colors: FieldColorDefaults,
    pub styles: FieldStyleDefaults,
    pub align: TextAlignDefaults,
    pub border: BorderDefaults,
    pub fill: FillCharDefaults,
    pub vertical_align: VerticalAlignDefaults,
    pub vertical_margin: VerticalMarginDefaults,
}

impl BmsDefaults {
    pub fn new() -> Self {
        Self {
            size: FieldSizeDefaults::new(),
            colors: FieldColorDefaults::new(),
            styles: FieldStyleDefaults::new(),
            align: TextAlignDefaults::new(),
            border: BorderDefaults::new(),
            fill: FillCharDefaults::new(),
            vertical_align: VerticalAlignDefaults::new(),
            vertical_margin: VerticalMarginDefaults::new(),
        }
    }

    /// Get all BMS field types
    pub fn field_types(&self) -> Vec<BmsFieldType> {
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

    /// Get FieldObjectDefaults for a specific field type
    pub fn get_object_defaults(&self, field_type: BmsFieldType) -> FieldObjectDefaults {
        FieldObjectDefaults::new(field_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_defaults() {
        let defaults = FieldSizeDefaults::new();
        assert_eq!(defaults.get_width(BmsFieldType::FieldTextORNumeric), 10);
        assert_eq!(defaults.get_width(BmsFieldType::Literal), 20);
        assert_eq!(defaults.get_height(BmsFieldType::ImageAsciiArt), 5);
        assert_eq!(defaults.get_min_width(BmsFieldType::FieldTextORNumeric), 5);
        assert_eq!(defaults.get_max_height(BmsFieldType::BooleanField), 3);
    }

    #[test]
    fn test_color_defaults() {
        let defaults = FieldColorDefaults::new();
        assert_eq!(defaults.get_text_color(BmsFieldType::FieldTextORNumeric), Color::Yellow);
        assert_eq!(defaults.get_text_color(BmsFieldType::Literal), Color::Green);
        assert!(defaults.available_colors.contains(&Color::Red));
    }

    #[test]
    fn test_style_defaults() {
        let defaults = FieldStyleDefaults::new();
        assert_eq!(defaults.get_default_style(BmsFieldType::Line), TextStyle::Underline);
        let styles = defaults.get_available_styles(BmsFieldType::ProtectedLiteral);
        assert!(styles.contains(&TextStyle::Bold));
        assert!(!styles.contains(&TextStyle::Italic)); // Not available for ProtectedLiteral
    }

    #[test]
    fn test_border_defaults() {
        let defaults = BorderDefaults::new();
        assert_eq!(defaults.get_default_border_style(BmsFieldType::Fieldset), BorderStyle::Double);
        let styles = defaults.get_available_border_styles(BmsFieldType::Line);
        assert_eq!(styles.len(), 1); // Line only supports None
        assert_eq!(styles[0], BorderStyle::None);
    }

    #[test]
    fn test_main_defaults() {
        let defaults = BmsDefaults::new();
        assert!(defaults.field_types().contains(&BmsFieldType::FieldTextORNumeric));
        assert!(defaults.field_types().contains(&BmsFieldType::Fieldset));
    }
}

// ============================================================================
// FIELD OBJECT DEFAULTS
// ============================================================================

/// Default values for a field object - mirrors Lua OBJECTS_DEFINITIONS default values
/// This provides the default values that a field object should have based on its type
#[derive(Debug, Clone)]
pub struct FieldObjectDefaults {
    pub width: u16,
    pub height: u16,
    pub min_width: u16,
    pub max_width: u16,
    pub min_height: u16,
    pub max_height: u16,
    pub border_style: BorderStyle,
    pub text_color: Color,
    pub border_color: Color,
    pub title_color: Color,
    pub text_align: TextAlign,
    pub title_align: TextAlign,
    pub text_style: TextStyle,
    pub fill_char: FillChar,
    pub field_type: BmsFieldType,
}

impl FieldObjectDefaults {
    /// Create new FieldObjectDefaults for a specific field type
    pub fn new(field_type: BmsFieldType) -> Self {
        let size_defaults = FieldSizeDefaults::new();
        let color_defaults = FieldColorDefaults::new();
        let align_defaults = TextAlignDefaults::new();
        let style_defaults = FieldStyleDefaults::new();
        let border_defaults = BorderDefaults::new();
        
        Self {
            width: size_defaults.get_width(field_type),
            height: size_defaults.get_height(field_type),
            min_width: size_defaults.get_min_width(field_type),
            max_width: size_defaults.get_max_width(field_type),
            min_height: size_defaults.get_min_height(field_type),
            max_height: size_defaults.get_max_height(field_type),
            border_style: border_defaults.get_default_border_style(field_type),
            text_color: color_defaults.get_text_color(field_type),
            border_color: color_defaults.get_border_color(field_type),
            title_color: color_defaults.get_title_color(field_type),
            text_align: align_defaults.get_default_text_align(field_type),
            title_align: align_defaults.get_default_title_align(field_type),
            text_style: style_defaults.get_default_style(field_type),
            fill_char: FillChar::Space, // Default fill character
            field_type: field_type.clone(),
        }
    }
}

impl Default for FieldObjectDefaults {
    fn default() -> Self {
        Self::new(BmsFieldType::default())
    }
}
