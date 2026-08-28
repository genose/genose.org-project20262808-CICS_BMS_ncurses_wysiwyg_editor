use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};

/// Represents a single BMS field (DFHMND, DFHMDF, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BmsField {
    pub name: String,
    pub field_type: FieldType,
    pub pos: (u16, u16),      // (line, column) - 1-based
    pub length: u16,
    pub attrb: Vec<FieldAttribute>,
    pub color: Option<Color>,
    pub initial: Option<String>,
    pub pic: Option<String>,   // PIC clause (for numeric fields)
    pub grp_name: Option<String>, // Group name (for DFHMND TYPE=GRP)
}

/// Represents a BMS map (DFHMSD)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmsMap {
    pub name: String,
    pub mapset: String,
    pub size: (u16, u16),     // (lines, columns)
    pub language: Option<String>, // LANG=COBOL/ASM/etc.
    pub fields: Vec<BmsField>,
    pub physical: bool,       // PHYSICAL=YES/NO
}

/// Represents a BMS mapset (collection of maps)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmsMapSet {
    pub name: String,
    pub maps: HashMap<String, BmsMap>,
}

/// Field type (from DFHMND TYPE=)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
    Map,
    Field,
    Literal,
    Group,
    Attribute,
    Symbolic,
    Unknown(String),
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::Map => write!(f, "MAP"),
            FieldType::Field => write!(f, "FIELD"),
            FieldType::Literal => write!(f, "LITERAL"),
            FieldType::Group => write!(f, "GRP"),
            FieldType::Attribute => write!(f, "ATTRB"),
            FieldType::Symbolic => write!(f, "SYMBOLIC"),
            FieldType::Unknown(s) => write!(f, "{}", s),
        }
    }
}

/// Field attributes (from ATTRB=)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldAttribute {
    Prot,    // PROT - Protected
    Norm,    // NORM - Normal
    Num,     // NUM - Numeric
    Alph,    // ALPH - Alphabetic
    AlphaNum, // ALNUM - Alphanumeric
    Bool,    // BOOL - Boolean
    Date,    // DATE
    Time,    // TIME
    Float,   // FLOAT
    Signed,  // SIGN - Signed numeric
    Intens,  // INTENS - High intensity
    Blink,   // BLINK
    Reverse, // REVERSE
    Underline, // UNDERLINE
    Dark,    // DARK
    Unknown(String),
}

impl fmt::Display for FieldAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldAttribute::Prot => write!(f, "PROT"),
            FieldAttribute::Norm => write!(f, "NORM"),
            FieldAttribute::Num => write!(f, "NUM"),
            FieldAttribute::Alph => write!(f, "ALPH"),
            FieldAttribute::AlphaNum => write!(f, "ALNUM"),
            FieldAttribute::Bool => write!(f, "BOOL"),
            FieldAttribute::Date => write!(f, "DATE"),
            FieldAttribute::Time => write!(f, "TIME"),
            FieldAttribute::Float => write!(f, "FLOAT"),
            FieldAttribute::Signed => write!(f, "SIGN"),
            FieldAttribute::Intens => write!(f, "INTENS"),
            FieldAttribute::Blink => write!(f, "BLINK"),
            FieldAttribute::Reverse => write!(f, "REVERSE"),
            FieldAttribute::Underline => write!(f, "UNDERLINE"),
            FieldAttribute::Dark => write!(f, "DARK"),
            FieldAttribute::Unknown(s) => write!(f, "{}", s),
        }
    }
}

/// Color definitions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Yellow,
    White,
    Turquoise,
    Pink,
    Orange,
    Purple,
    Gray,
    LightGreen,
    Custom(u16), // For custom color codes
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Black => write!(f, "BLACK"),
            Color::Blue => write!(f, "BLUE"),
            Color::Green => write!(f, "GREEN"),
            Color::Cyan => write!(f, "CYAN"),
            Color::Red => write!(f, "RED"),
            Color::Magenta => write!(f, "MAGENTA"),
            Color::Yellow => write!(f, "YELLOW"),
            Color::White => write!(f, "WHITE"),
            Color::Turquoise => write!(f, "TURQUOISE"),
            Color::Pink => write!(f, "PINK"),
            Color::Orange => write!(f, "ORANGE"),
            Color::Purple => write!(f, "PURPLE"),
            Color::Gray => write!(f, "GRAY"),
            Color::LightGreen => write!(f, "LIGHTGREEN"),
            Color::Custom(code) => write!(f, "COLOR({})", code),
        }
    }
}

impl Color {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "BLACK" => Some(Color::Black),
            "BLUE" => Some(Color::Blue),
            "GREEN" => Some(Color::Green),
            "CYAN" => Some(Color::Cyan),
            "RED" => Some(Color::Red),
            "MAGENTA" => Some(Color::Magenta),
            "YELLOW" => Some(Color::Yellow),
            "WHITE" => Some(Color::White),
            "TURQUOISE" => Some(Color::Turquoise),
            "PINK" => Some(Color::Pink),
            "ORANGE" => Some(Color::Orange),
            "PURPLE" => Some(Color::Purple),
            "GRAY" | "GREY" => Some(Color::Gray),
            "LIGHTGREEN" => Some(Color::LightGreen),
            _ => None,
        }
    }
}

impl Default for BmsField {
    fn default() -> Self {
        Self {
            name: String::new(),
            field_type: FieldType::Field,
            pos: (1, 1),
            length: 1,
            attrb: vec![],
            color: None,
            initial: None,
            pic: None,
            grp_name: None,
        }
    }
}

impl BmsMap {
    pub fn new(name: &str, mapset: &str) -> Self {
        Self {
            name: name.to_uppercase(),
            mapset: mapset.to_uppercase(),
            size: (24, 80),
            language: Some("COBOL".to_string()),
            fields: vec![],
            physical: true,
        }
    }
    
    /// Exporter la map au format JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// Importer une map depuis du JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl BmsMapSet {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_uppercase(),
            maps: HashMap::new(),
        }
    }
    
    /// Exporter le mapset au format JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// Importer un mapset depuis du JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
