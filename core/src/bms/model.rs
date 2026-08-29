use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};

/// Justification type for BMS fields
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Justify {
    Left,
    Right,
    Center,
}

impl fmt::Display for Justify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Justify::Left => write!(f, "LEFT"),
            Justify::Right => write!(f, "RIGHT"),
            Justify::Center => write!(f, "CENTER"),
        }
    }
}

impl Justify {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "LEFT" => Some(Justify::Left),
            "RIGHT" => Some(Justify::Right),
            "CENTER" => Some(Justify::Center),
            _ => None,
        }
    }
}

/// Special key types for BMS fields
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    FunctionKey(u8),
    PAKey(u8),
    ClearKey,
    EnterKey,
    TabKey,
    BackTabKey,
    Unknown(String),
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyType::FunctionKey(n) => write!(f, "PF{}", n),
            KeyType::PAKey(n) => write!(f, "PA{}", n),
            KeyType::ClearKey => write!(f, "CLEAR"),
            KeyType::EnterKey => write!(f, "ENTER"),
            KeyType::TabKey => write!(f, "TAB"),
            KeyType::BackTabKey => write!(f, "BACKTAB"),
            KeyType::Unknown(s) => write!(f, "{}", s),
        }
    }
}

/// Data type for COBOL fields
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    Alphanumeric,
    Numeric,
    NumericSigned,
    NumericPacked,
    NumericBinary,
    NumericFloat,
    Date,
    Time,
    DateTime,
    Boolean,
    Character,
    Group,
    Filler,
    OccursDependingOn,
    Redefines,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Alphanumeric => write!(f, "ALPHANUMERIC"),
            DataType::Numeric => write!(f, "NUMERIC"),
            DataType::NumericSigned => write!(f, "NUMERIC_SIGNED"),
            DataType::NumericPacked => write!(f, "PACKED_DECIMAL"),
            DataType::NumericBinary => write!(f, "BINARY"),
            DataType::NumericFloat => write!(f, "FLOAT"),
            DataType::Date => write!(f, "DATE"),
            DataType::Time => write!(f, "TIME"),
            DataType::DateTime => write!(f, "DATETIME"),
            DataType::Boolean => write!(f, "BOOLEAN"),
            DataType::Character => write!(f, "CHARACTER"),
            DataType::Group => write!(f, "GROUP"),
            DataType::Filler => write!(f, "FILLER"),
            DataType::OccursDependingOn => write!(f, "OCCURS_DEPENDING_ON"),
            DataType::Redefines => write!(f, "REDEFINES"),
        }
    }
}

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
    pub pic: Option<String>,
    pub grp_name: Option<String>,
    
    // Extended CICS/BMS support
    pub justification: Option<Justify>,
    pub autoskip: Option<bool>,
    pub fieldexit: Option<bool>,
    pub blank_zero: Option<bool>,
    pub repeat: Option<u16>,
    pub fill_char: Option<char>,
    pub format: Option<String>,
    pub key_type: Option<KeyType>,
    pub data_type: Option<DataType>,
    pub occurs: Option<u16>,
    pub depending_on: Option<String>,
    pub redefines: Option<String>,
    pub sign_leading: Option<bool>,
    pub sign_trailing: Option<bool>,
    pub decimal_point: Option<bool>,
    pub synchronized: Option<bool>,
    pub usage: Option<String>,
}

/// Represents a BMS map (DFHMSD)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BmsMap {
    pub name: String,
    pub mapset: String,
    pub size: (u16, u16),
    pub language: Option<String>,
    pub map_type: FieldType,
    pub fields: Vec<BmsField>,
    pub physical: bool,
    pub symbolic: bool,
    pub terminal: Option<String>,
    pub cursor_pos: Option<(u16, u16)>,
    pub erase: Option<bool>,
    pub freekb: Option<bool>,
    pub alarm: Option<bool>,
    pub timetag: Option<bool>,
}

/// Represents a BMS mapset (collection of maps)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmsMapSet {
    pub name: String,
    pub maps: HashMap<String, BmsMap>,
}

/// Field type (from DFHMND TYPE=)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FieldType {
    #[default]
    Map,
    Field,
    Literal,
    Group,
    Attribute,
    Symbolic,
    
    // BMS statement types
    DFHMSD,
    DFHMDF,
    DFHMDI,
    DFHMDA,
    DFHMND,
    DFHMNT,
    DFHMDC,
    DFHMDL,
    
    // Physical vs Symbolic
    PhysicalMap,
    SymbolicMap,
    MapSet,
    
    // Special field types
    InputOnly,
    OutputOnly,
    InputOutput,
    Hidden,
    
    // Data types
    Alphanumeric,
    Numeric,
    Date,
    Time,
    Boolean,
    
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
            FieldType::DFHMSD => write!(f, "DFHMSD"),
            FieldType::DFHMDF => write!(f, "DFHMDF"),
            FieldType::DFHMDI => write!(f, "DFHMDI"),
            FieldType::DFHMDA => write!(f, "DFHMDA"),
            FieldType::DFHMND => write!(f, "DFHMND"),
            FieldType::DFHMNT => write!(f, "DFHMNT"),
            FieldType::DFHMDC => write!(f, "DFHMDC"),
            FieldType::DFHMDL => write!(f, "DFHMDL"),
            FieldType::PhysicalMap => write!(f, "PHYSICAL"),
            FieldType::SymbolicMap => write!(f, "SYMBOLIC"),
            FieldType::MapSet => write!(f, "MAPSET"),
            FieldType::InputOnly => write!(f, "INPUT"),
            FieldType::OutputOnly => write!(f, "OUTPUT"),
            FieldType::InputOutput => write!(f, "INOUT"),
            FieldType::Hidden => write!(f, "HIDDEN"),
            FieldType::Alphanumeric => write!(f, "ALNUM"),
            FieldType::Numeric => write!(f, "NUM"),
            FieldType::Date => write!(f, "DATE"),
            FieldType::Time => write!(f, "TIME"),
            FieldType::Boolean => write!(f, "BOOL"),
            FieldType::Unknown(s) => write!(f, "{}", s),
        }
    }
}

/// Field attributes (from ATTRB=)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldAttribute {
    // Protection
    Prot,
    Unprot,
    
    // Intensity
    Norm,
    Intens,
    Dark,
    
    // Data type attributes
    Num,
    Alph,
    AlphaNum,
    Bool,
    Date,
    Time,
    Float,
    Signed,
    Packed,
    Binary,
    
    // Display attributes
    Blink,
    Reverse,
    Underline,
    
    // Justification
    Left,
    Right,
    Center,
    
    // Special handling
    AutoSkip,
    NoAutoSkip,
    FieldExit,
    NoFieldExit,
    
    // Blank/Zero handling
    Blank,
    Zero,
    
    // Repeat/Fill
    Repeat,
    Fill,
    
    // Terminal control
    Erase,
    Reset,
    Set,
    
    // Function keys
    FunctionKey(u8),
    PAKey(u8),
    Clear,
    
    Unknown(String),
}

impl fmt::Display for FieldAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldAttribute::Prot => write!(f, "PROT"),
            FieldAttribute::Unprot => write!(f, "UNPROT"),
            FieldAttribute::Norm => write!(f, "NORM"),
            FieldAttribute::Intens => write!(f, "INTENS"),
            FieldAttribute::Dark => write!(f, "DARK"),
            FieldAttribute::Num => write!(f, "NUM"),
            FieldAttribute::Alph => write!(f, "ALPH"),
            FieldAttribute::AlphaNum => write!(f, "ALNUM"),
            FieldAttribute::Bool => write!(f, "BOOL"),
            FieldAttribute::Date => write!(f, "DATE"),
            FieldAttribute::Time => write!(f, "TIME"),
            FieldAttribute::Float => write!(f, "FLOAT"),
            FieldAttribute::Signed => write!(f, "SIGN"),
            FieldAttribute::Packed => write!(f, "PACKED"),
            FieldAttribute::Binary => write!(f, "BINARY"),
            FieldAttribute::Blink => write!(f, "BLINK"),
            FieldAttribute::Reverse => write!(f, "REVERSE"),
            FieldAttribute::Underline => write!(f, "UNDERLINE"),
            FieldAttribute::Left => write!(f, "LEFT"),
            FieldAttribute::Right => write!(f, "RIGHT"),
            FieldAttribute::Center => write!(f, "CENTER"),
            FieldAttribute::AutoSkip => write!(f, "AUTOSKIP"),
            FieldAttribute::NoAutoSkip => write!(f, "NOAUTOSKIP"),
            FieldAttribute::FieldExit => write!(f, "FIELDEXIT"),
            FieldAttribute::NoFieldExit => write!(f, "NOFIELDEXIT"),
            FieldAttribute::Blank => write!(f, "BLANK"),
            FieldAttribute::Zero => write!(f, "ZERO"),
            FieldAttribute::Repeat => write!(f, "REPEAT"),
            FieldAttribute::Fill => write!(f, "FILL"),
            FieldAttribute::Erase => write!(f, "ERASE"),
            FieldAttribute::Reset => write!(f, "RESET"),
            FieldAttribute::Set => write!(f, "SET"),
            FieldAttribute::FunctionKey(n) => write!(f, "PF{}", n),
            FieldAttribute::PAKey(n) => write!(f, "PA{}", n),
            FieldAttribute::Clear => write!(f, "CLEAR"),
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
    LightBlue,
    LightCyan,
    LightRed,
    LightMagenta,
    LightYellow,
    Neutral,
    Custom(u16),
    Default,
    Unknown(String),
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
            Color::LightBlue => write!(f, "LIGHTBLUE"),
            Color::LightCyan => write!(f, "LIGHTCYAN"),
            Color::LightRed => write!(f, "LIGHTRED"),
            Color::LightMagenta => write!(f, "LIGHTMAGENTA"),
            Color::LightYellow => write!(f, "LIGHTYELLOW"),
            Color::Neutral => write!(f, "NEUTRAL"),
            Color::Custom(code) => write!(f, "COLOR({})", code),
            Color::Default => write!(f, "DEFAULT"),
            Color::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Color {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "BLACK" => Color::Black,
            "BLUE" => Color::Blue,
            "GREEN" => Color::Green,
            "CYAN" => Color::Cyan,
            "RED" => Color::Red,
            "MAGENTA" => Color::Magenta,
            "YELLOW" => Color::Yellow,
            "WHITE" => Color::White,
            "TURQUOISE" => Color::Turquoise,
            "PINK" => Color::Pink,
            "ORANGE" => Color::Orange,
            "PURPLE" => Color::Purple,
            "GRAY" | "GREY" => Color::Gray,
            "LIGHTGREEN" => Color::LightGreen,
            "LIGHTBLUE" => Color::LightBlue,
            "LIGHTCYAN" => Color::LightCyan,
            "LIGHTRED" => Color::LightRed,
            "LIGHTMAGENTA" => Color::LightMagenta,
            "LIGHTYELLOW" => Color::LightYellow,
            "NEUTRAL" => Color::Neutral,
            "DEFAULT" => Color::Default,
            s if s.starts_with("COLOR(") && s.ends_with(")") => {
                if let Ok(code) = s.trim_start_matches("COLOR(").trim_end_matches(")").parse() {
                    Color::Custom(code)
                } else {
                    Color::Unknown(s.to_string())
                }
            }
            _ => Color::Unknown(s.to_string()),
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
            justification: None,
            autoskip: None,
            fieldexit: None,
            blank_zero: None,
            repeat: None,
            fill_char: None,
            format: None,
            key_type: None,
            data_type: None,
            occurs: None,
            depending_on: None,
            redefines: None,
            sign_leading: None,
            sign_trailing: None,
            decimal_point: None,
            synchronized: None,
            usage: None,
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
            map_type: FieldType::Map,
            fields: vec![],
            physical: true,
            symbolic: false,
            terminal: None,
            cursor_pos: None,
            erase: None,
            freekb: None,
            alarm: None,
            timetag: None,
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
    
    /// Valider que la map est correcte
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        // Verifier les dimensions de la map
        if self.size.0 == 0 || self.size.1 == 0 {
            errors.push("Map size must be greater than 0".to_string());
        }
        
        // Verifier chaque champ
        for (i, field) in self.fields.iter().enumerate() {
            // Verifier la position
            if field.pos.0 == 0 || field.pos.1 == 0 {
                errors.push(format!("Field {}: Position must be greater than 0", i));
            }
            
            // Verifier que la position est dans les limites de la map
            if field.pos.0 > self.size.0 {
                errors.push(format!("Field {}: Row {} exceeds map height {}", i, field.pos.0, self.size.0));
            }
            if field.pos.1 > self.size.1 {
                errors.push(format!("Field {}: Column {} exceeds map width {}", i, field.pos.1, self.size.1));
            }
            
            // Verifier la longueur
            if field.length == 0 {
                errors.push(format!("Field {}: Length must be greater than 0", i));
            }
            
            // Verifier que le champ ne depasse pas les limites
            if field.pos.1 + field.length - 1 > self.size.1 {
                errors.push(format!("Field {}: Field extends beyond column {}", i, self.size.1));
            }
        }
        
        // Verifier les chevauchements de champs
        for i in 0..self.fields.len() {
            for j in i + 1..self.fields.len() {
                let f1 = &self.fields[i];
                let f2 = &self.fields[j];
                
                // Verifier si les champs se chevauchent
                if f1.pos.0 == f2.pos.0 { // Memes ligne
                    let f1_start = f1.pos.1;
                    let f1_end = f1.pos.1 + f1.length - 1;
                    let f2_start = f2.pos.1;
                    let f2_end = f2.pos.1 + f2.length - 1;
                    
                    if f1_start <= f2_end && f2_start <= f1_end {
                        errors.push(format!("Fields {} and {} overlap at row {}", i, j, f1.pos.0));
                    }
                }
            }
        }
        
        errors
    }
    
    /// Verifier si une position est valide pour un champ de longueur donnee
    pub fn is_valid_field_position(&self, pos: (u16, u16), length: u16) -> bool {
        if pos.0 == 0 || pos.1 == 0 || length == 0 {
            return false;
        }
        if pos.0 > self.size.0 || pos.1 > self.size.1 {
            return false;
        }
        if pos.1 + length - 1 > self.size.1 {
            return false;
        }
        
        // Verifier les chevauchements avec les champs existants
        let new_start = pos.1;
        let new_end = pos.1 + length - 1;
        
        for field in &self.fields {
            if field.pos.0 == pos.0 { // Memes ligne
                let f_start = field.pos.1;
                let f_end = field.pos.1 + field.length - 1;
                
                if new_start <= f_end && f_start <= new_end {
                    return false;
                }
            }
        }
        
        true
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
