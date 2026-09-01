//! Type definitions for BMS fields - mirrors Lua OBJECTS_DEFINITIONS structure
//!
//! This module provides all the basic types used in BMS field properties,
//! organized to mirror the structure in Lua's OBJECTS_DEFINITIONS.

use serde::{Serialize, Deserialize};
use std::fmt;

// ============================================================================
// POSITION
// ============================================================================

/// Position in the BMS screen (row, col, rowend, colend)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub row: u16,
    pub col: u16,
    pub rowend: u16,
    pub colend: u16,
}

impl Position {
    pub fn new(row: u16, col: u16) -> Self {
        Self { row, col, rowend: 0, colend: 0 }
    }

    pub fn with_end(row: u16, col: u16, rowend: u16, colend: u16) -> Self {
        Self { row, col, rowend, colend }
    }

    pub fn width(&self) -> u16 {
        if self.colend > self.col {
            self.colend - self.col + 1
        } else {
            0
        }
    }

    pub fn height(&self) -> u16 {
        if self.rowend > self.row {
            self.rowend - self.row + 1
        } else {
            0
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.rowend > 0 || self.colend > 0 {
            write!(f, "({},{})->({},{})", self.row, self.col, self.rowend, self.colend)
        } else {
            write!(f, "({},{})", self.row, self.col)
        }
    }
}

// ============================================================================
// COLORS
// ============================================================================

/// Color enumeration for BMS fields
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    LightGreen,
    LightBlue,
    LightCyan,
    LightRed,
    LightMagenta,
    LightYellow,
    Turquoise,
    Pink,
    Orange,
    Purple,
    Neutral,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::Default => "default",
            Color::Black => "black",
            Color::Red => "red",
            Color::Green => "green",
            Color::Yellow => "yellow",
            Color::Blue => "blue",
            Color::Magenta => "magenta",
            Color::Cyan => "cyan",
            Color::White => "white",
            Color::Gray => "gray",
            Color::LightGreen => "light_green",
            Color::LightBlue => "light_blue",
            Color::LightCyan => "light_cyan",
            Color::LightRed => "light_red",
            Color::LightMagenta => "light_magenta",
            Color::LightYellow => "light_yellow",
            Color::Turquoise => "turquoise",
            Color::Pink => "pink",
            Color::Orange => "orange",
            Color::Purple => "purple",
            Color::Neutral => "neutral",
            Color::BrightBlack => "bright_black",
            Color::BrightRed => "bright_red",
            Color::BrightGreen => "bright_green",
            Color::BrightYellow => "bright_yellow",
            Color::BrightBlue => "bright_blue",
            Color::BrightMagenta => "bright_magenta",
            Color::BrightCyan => "bright_cyan",
            Color::BrightWhite => "bright_white",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "default" => Some(Color::Default),
            "black" => Some(Color::Black),
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "yellow" => Some(Color::Yellow),
            "blue" => Some(Color::Blue),
            "magenta" => Some(Color::Magenta),
            "cyan" => Some(Color::Cyan),
            "white" => Some(Color::White),
            "gray" | "grey" => Some(Color::Gray),
            "light_green" | "lightgreen" => Some(Color::LightGreen),
            "light_blue" | "lightblue" => Some(Color::LightBlue),
            "light_cyan" | "lightcyan" => Some(Color::LightCyan),
            "light_red" | "lightred" => Some(Color::LightRed),
            "light_magenta" | "lightmagenta" => Some(Color::LightMagenta),
            "light_yellow" | "lightyellow" => Some(Color::LightYellow),
            "turquoise" => Some(Color::Turquoise),
            "pink" => Some(Color::Pink),
            "orange" => Some(Color::Orange),
            "purple" => Some(Color::Purple),
            "neutral" => Some(Color::Neutral),
            "bright_black" | "brightblack" => Some(Color::BrightBlack),
            "bright_red" | "brightred" => Some(Color::BrightRed),
            "bright_green" | "brightgreen" => Some(Color::BrightGreen),
            "bright_yellow" | "brightyellow" => Some(Color::BrightYellow),
            "bright_blue" | "brightblue" => Some(Color::BrightBlue),
            "bright_magenta" | "brightmagenta" => Some(Color::BrightMagenta),
            "bright_cyan" | "brightcyan" => Some(Color::BrightCyan),
            "bright_white" | "brightwhite" => Some(Color::BrightWhite),
            _ => None,
        }
    }

    pub fn is_bms_color(&self) -> bool {
        matches!(
            self,
            Color::Default | Color::White | Color::Green | Color::Yellow | Color::Blue | Color::Cyan | Color::Red
        )
    }
}

// Conversion from legacy model::Color to types::Color
impl From<crate::bms::model::Color> for Color {
    fn from(legacy_color: crate::bms::model::Color) -> Self {
        match legacy_color {
            crate::bms::model::Color::Default => Color::Default,
            crate::bms::model::Color::Black => Color::Black,
            crate::bms::model::Color::Blue => Color::Blue,
            crate::bms::model::Color::Green => Color::Green,
            crate::bms::model::Color::Cyan => Color::Cyan,
            crate::bms::model::Color::Red => Color::Red,
            crate::bms::model::Color::Magenta => Color::Magenta,
            crate::bms::model::Color::Yellow => Color::Yellow,
            crate::bms::model::Color::White => Color::White,
            crate::bms::model::Color::Turquoise => Color::Turquoise,
            crate::bms::model::Color::Pink => Color::Pink,
            crate::bms::model::Color::Orange => Color::Orange,
            crate::bms::model::Color::Purple => Color::Purple,
            crate::bms::model::Color::Gray => Color::Gray,
            crate::bms::model::Color::LightGreen => Color::LightGreen,
            crate::bms::model::Color::LightBlue => Color::LightBlue,
            crate::bms::model::Color::LightCyan => Color::LightCyan,
            crate::bms::model::Color::LightRed => Color::LightRed,
            crate::bms::model::Color::LightMagenta => Color::LightMagenta,
            crate::bms::model::Color::LightYellow => Color::LightYellow,
            crate::bms::model::Color::Neutral => Color::Neutral,
            crate::bms::model::Color::Custom(_) => Color::Default, // Map custom to default
            crate::bms::model::Color::Unknown(_) => Color::Default, // Map unknown to default
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Default
    }
}

// ============================================================================
// TEXT ALIGNMENT
// ============================================================================

/// Text alignment for BMS fields
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl TextAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextAlign::Left => "left",
            TextAlign::Center => "center",
            TextAlign::Right => "right",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "left" | "l" => Some(TextAlign::Left),
            "center" | "c" => Some(TextAlign::Center),
            "right" | "r" => Some(TextAlign::Right),
            _ => None,
        }
    }
}

impl fmt::Display for TextAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}

// ============================================================================
// VERTICAL ALIGNMENT
// ============================================================================

/// Vertical alignment for BMS fields
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

impl VerticalAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            VerticalAlign::Top => "top",
            VerticalAlign::Middle => "middle",
            VerticalAlign::Bottom => "bottom",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "top" | "t" => Some(VerticalAlign::Top),
            "middle" | "m" => Some(VerticalAlign::Middle),
            "bottom" | "b" => Some(VerticalAlign::Bottom),
            _ => None,
        }
    }
}

impl fmt::Display for VerticalAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for VerticalAlign {
    fn default() -> Self {
        VerticalAlign::Top
    }
}

// ============================================================================
// VERTICAL MARGIN
// ============================================================================

/// Vertical margin values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerticalMargin {
    None = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
}

impl VerticalMargin {
    pub fn value(&self) -> u16 {
        *self as u16
    }

    pub fn from_u16(n: u16) -> Self {
        match n {
            0 => VerticalMargin::None,
            1 => VerticalMargin::Small,
            2 => VerticalMargin::Medium,
            _ => VerticalMargin::Large,
        }
    }
}

impl fmt::Display for VerticalMargin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

// ============================================================================
// BORDER STYLE
// ============================================================================

/// Border style for BMS fields
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorderStyle {
    None,
    Single,
    Double,
    Solid,
    Dashed,
    Dotted,
}

impl BorderStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            BorderStyle::None => "none",
            BorderStyle::Single => "single",
            BorderStyle::Double => "double",
            BorderStyle::Solid => "solid",
            BorderStyle::Dashed => "dashed",
            BorderStyle::Dotted => "dotted",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "none" | "no" | "" => Some(BorderStyle::None),
            "single" | "s" => Some(BorderStyle::Single),
            "double" | "d" => Some(BorderStyle::Double),
            "solid" => Some(BorderStyle::Solid),
            "dashed" | "dash" => Some(BorderStyle::Dashed),
            "dotted" | "dot" => Some(BorderStyle::Dotted),
            _ => None,
        }
    }
}

impl fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle::None
    }
}

// ============================================================================
// BORDER CHARACTERS
// ============================================================================

/// Border character sets for different border styles
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorderCharSet {
    pub top_left: String,
    pub top: String,
    pub top_right: String,
    pub left: String,
    pub right: String,
    pub bottom_left: String,
    pub bottom: String,
    pub bottom_right: String,
}

impl BorderCharSet {
    pub fn none() -> Self {
        Self {
            top_left: String::new(),
            top: String::new(),
            top_right: String::new(),
            left: String::new(),
            right: String::new(),
            bottom_left: String::new(),
            bottom: String::new(),
            bottom_right: String::new(),
        }
    }

    pub fn single() -> Self {
        Self {
            top_left: "┌".to_string(),
            top: "─".to_string(),
            top_right: "┐".to_string(),
            left: "│".to_string(),
            right: "│".to_string(),
            bottom_left: "└".to_string(),
            bottom: "─".to_string(),
            bottom_right: "┘".to_string(),
        }
    }

    pub fn double() -> Self {
        Self {
            top_left: "╔".to_string(),
            top: "═".to_string(),
            top_right: "╗".to_string(),
            left: "║".to_string(),
            right: "║".to_string(),
            bottom_left: "╚".to_string(),
            bottom: "═".to_string(),
            bottom_right: "╝".to_string(),
        }
    }

    pub fn dashed() -> Self {
        Self {
            top_left: "+ ".to_string(),
            top: "-".to_string(),
            top_right: "+".to_string(),
            left: "|".to_string(),
            right: "|".to_string(),
            bottom_left: "+".to_string(),
            bottom: "-".to_string(),
            bottom_right: "+".to_string(),
        }
    }

    /// Get border character set for a specific border style
    pub fn for_style(style: BorderStyle) -> Self {
        match style {
            BorderStyle::Single => Self::single(),
            BorderStyle::Double => Self::double(),
            BorderStyle::Dashed => Self::dashed(),
            BorderStyle::None | BorderStyle::Solid | BorderStyle::Dotted => Self::none(),
        }
    }
}

/// Available border characters for each border style
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorderChars {
    pub single: BorderCharSet,
    pub double: BorderCharSet,
    pub dashed: BorderCharSet,
    pub none: BorderCharSet,
}

impl BorderChars {
    pub fn new() -> Self {
        Self {
            single: BorderCharSet::single(),
            double: BorderCharSet::double(),
            dashed: BorderCharSet::dashed(),
            none: BorderCharSet::none(),
        }
    }

    pub fn get(&self, style: BorderStyle) -> &BorderCharSet {
        match style {
            BorderStyle::Single => &self.single,
            BorderStyle::Double => &self.double,
            BorderStyle::Dashed => &self.dashed,
            BorderStyle::Solid | BorderStyle::Dotted | BorderStyle::None => &self.none,
        }
    }
}

// ============================================================================
// FILL CHARACTERS
// ============================================================================

/// Fill character types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FillChar {
    Space,
    Dash,
    Equal,
    Underscore,
    Dot,
    Asterisk,
    Pipe,
    Exclamation,
    Plus,
    Question,
    LessThan,
    GreaterThan,
    Tilde,
    Hash,
    Percent,
    Ampersand,
    At,
    Caret,
    Dollar,
    Semicolon,
    Colon,
    Comma,
    Period,
    Slash,
    Backslash,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    EqualSign,
    DoubleQuote,
    SingleQuote,
}

impl FillChar {
    pub fn char(&self) -> char {
        match self {
            FillChar::Space => ' ',
            FillChar::Dash => '─',
            FillChar::Equal => '=',
            FillChar::Underscore => '_',
            FillChar::Dot => '.',
            FillChar::Asterisk => '*',
            FillChar::Pipe => '|',
            FillChar::Exclamation => '!',
            FillChar::Plus => '+',
            FillChar::Question => '?',
            FillChar::LessThan => '<',
            FillChar::GreaterThan => '>',
            FillChar::Tilde => '~',
            FillChar::Hash => '#',
            FillChar::Percent => '%',
            FillChar::Ampersand => '&',
            FillChar::At => '@',
            FillChar::Caret => '^',
            FillChar::Dollar => '$',
            FillChar::Semicolon => ';',
            FillChar::Colon => ':',
            FillChar::Comma => ',',
            FillChar::Period => '.',
            FillChar::Slash => '/',
            FillChar::Backslash => '\\',
            FillChar::LeftBracket => '[',
            FillChar::RightBracket => ']',
            FillChar::LeftBrace => '{',
            FillChar::RightBrace => '}',
            FillChar::LeftParenthesis => '(',
            FillChar::RightParenthesis => ')',
            FillChar::EqualSign => '=',
            FillChar::DoubleQuote => '"',
            FillChar::SingleQuote => '\'',
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            ' ' => FillChar::Space,
            '─' => FillChar::Dash,
            '=' => FillChar::Equal,
            '_' => FillChar::Underscore,
            '.' => FillChar::Dot,
            '*' => FillChar::Asterisk,
            '|' => FillChar::Pipe,
            '!' => FillChar::Exclamation,
            '+' => FillChar::Plus,
            '?' => FillChar::Question,
            '<' => FillChar::LessThan,
            '>' => FillChar::GreaterThan,
            '~' => FillChar::Tilde,
            '#' => FillChar::Hash,
            '%' => FillChar::Percent,
            '&' => FillChar::Ampersand,
            '@' => FillChar::At,
            '^' => FillChar::Caret,
            '$' => FillChar::Dollar,
            ';' => FillChar::Semicolon,
            ':' => FillChar::Colon,
            ',' => FillChar::Comma,
            '/' => FillChar::Slash,
            '\\' => FillChar::Backslash,
            '[' => FillChar::LeftBracket,
            ']' => FillChar::RightBracket,
            '{' => FillChar::LeftBrace,
            '}' => FillChar::RightBrace,
            '(' => FillChar::LeftParenthesis,
            ')' => FillChar::RightParenthesis,
            '"' => FillChar::DoubleQuote,
            '\'' => FillChar::SingleQuote,
            _ => FillChar::Space,
        }
    }
}

impl fmt::Display for FillChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char())
    }
}

// ============================================================================
// TEXT STYLE
// ============================================================================

/// Text style for BMS fields
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextStyle {
    Default,
    Bold,
    Italic,
    Underline,
    StrikeThrough,
    Blink,
    Reverse,
}

impl TextStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextStyle::Default => "default",
            TextStyle::Bold => "bold",
            TextStyle::Italic => "italic",
            TextStyle::Underline => "underline",
            TextStyle::StrikeThrough => "strikethrough",
            TextStyle::Blink => "blink",
            TextStyle::Reverse => "reverse",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "default" | "d" | "" => Some(TextStyle::Default),
            "bold" | "b" => Some(TextStyle::Bold),
            "italic" | "i" => Some(TextStyle::Italic),
            "underline" | "u" => Some(TextStyle::Underline),
            "strikethrough" | "strike" | "s" => Some(TextStyle::StrikeThrough),
            "blink" | "bl" => Some(TextStyle::Blink),
            "reverse" | "r" => Some(TextStyle::Reverse),
            _ => None,
        }
    }

    pub fn exported_value(&self) -> u32 {
        match self {
            TextStyle::Default => 0,
            TextStyle::Bold => 1,
            TextStyle::Italic => 2,
            TextStyle::Underline => 4,
            TextStyle::StrikeThrough => 8,
            TextStyle::Blink => 16,
            TextStyle::Reverse => 32,
        }
    }
}

impl fmt::Display for TextStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle::Default
    }
}

// ============================================================================
// MARKERS
// ============================================================================

/// Marker for required/error fields
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Marker {
    pub enabled: bool,
    pub marker: String,
}

impl Marker {
    /// Create a required field marker
    pub fn required() -> Self {
        Self {
            enabled: true,
            marker: " *".to_string(),
        }
    }

    /// Create an error field marker
    pub fn error() -> Self {
        Self {
            enabled: true,
            marker: Self::error_string(),
        }
    }

    /// Create error marker string programmatically to avoid escaping issues
    fn error_string() -> String {
        let mut s = String::new();
        s.push(' ');
        s.push('/');
        s.push('!');
        s.push('\\');  // This is the backslash character
        s.push(' ');
        s
    }

    /// Create a disabled/no marker
    pub fn none() -> Self {
        Self {
            enabled: false,
            marker: String::new(),
        }
    }

    /// Check if marker should be displayed
    pub fn should_display(&self) -> bool {
        self.enabled && !self.marker.is_empty()
    }
}

// ============================================================================
// PREFIX/SUFFIX
// ============================================================================

/// Prefix/Suffix configuration for field titles and footers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrefixSuffix {
    pub enabled: bool,
    pub color: Color,
    pub prefix_char: Option<FillChar>,
    pub required: Option<Marker>,
    pub errors: Option<Marker>,
}

impl Default for PrefixSuffix {
    fn default() -> Self {
        Self {
            enabled: false,
            color: Color::Default,
            prefix_char: None,
            required: None,
            errors: None,
        }
    }
}

impl PrefixSuffix {
    pub fn none() -> Self {
        Self::default()
    }
}

// ============================================================================
// FOOTER
// ============================================================================

/// Footer configuration for BMS fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Footer {
    pub color: Color,
    pub align: TextAlign,
    pub fill_marker: FillChar,
    pub title: String,
    pub required_marker: Option<Marker>,
    pub error_marker: Option<Marker>,
}

impl Default for Footer {
    fn default() -> Self {
        Self {
            color: Color::Default,
            align: TextAlign::Center,
            fill_marker: FillChar::Space,
            title: String::new(),
            required_marker: None,
            error_marker: None,
        }
    }
}

impl Footer {
    pub fn none() -> Self {
        Self::default()
    }
}

// ============================================================================
// DECORATION TYPE
// ============================================================================

/// Decoration type for field styling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DecorationType {
    Normal,
    Required,
    Error,
    Selected,
    Highlighted,
}

impl Default for DecorationType {
    fn default() -> Self {
        DecorationType::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.row, 5);
        assert_eq!(pos.col, 10);
        assert_eq!(pos.rowend, 0);
        assert_eq!(pos.colend, 0);
        
        let pos2 = Position::with_end(1, 1, 3, 10);
        assert_eq!(pos2.width(), 10);
        assert_eq!(pos2.height(), 3);
    }

    #[test]
    fn test_color() {
        assert_eq!(Color::Red.as_str(), "red");
        assert!(matches!(Color::from_str("RED"), Some(Color::Red)));
        assert!(matches!(Color::from_str("red"), Some(Color::Red)));
        assert!(Color::Red.is_bms_color());
        assert!(!Color::BrightMagenta.is_bms_color());
    }

    #[test]
    fn test_text_align() {
        assert_eq!(TextAlign::Left.as_str(), "left");
        assert!(matches!(TextAlign::from_str("left"), Some(TextAlign::Left)));
        assert!(matches!(TextAlign::from_str("l"), Some(TextAlign::Left)));
    }

    #[test]
    fn test_fill_char() {
        assert_eq!(FillChar::Dash.char(), '─');
        assert_eq!(FillChar::from_char('*'), FillChar::Asterisk);
        assert_eq!(FillChar::Backslash.char(), '\\');
    }

    #[test]
    fn test_marker() {
        let req = Marker::required();
        assert!(req.enabled);
        assert_eq!(req.marker, " *");
        
        let err = Marker::error();
        assert!(err.enabled);
        // Check that the error marker contains the expected characters
        assert!(err.marker.contains('/'));
        assert!(err.marker.contains('!'));
        assert!(err.marker.contains('\\'));
        
        let none = Marker::none();
        assert!(!none.enabled);
        assert!(none.marker.is_empty());
    }

    #[test]
    fn test_border_style() {
        assert_eq!(BorderStyle::Single.as_str(), "single");
        assert!(matches!(BorderStyle::from_str("single"), Some(BorderStyle::Single)));
    }

    #[test]
    fn test_vertical_margin() {
        assert_eq!(VerticalMargin::None.value(), 0);
        assert_eq!(VerticalMargin::Small.value(), 1);
        assert_eq!(VerticalMargin::from_u16(2), VerticalMargin::Medium);
    }
}