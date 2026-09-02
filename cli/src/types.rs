//! Types module
//!
//! This module contains shared types and enums used throughout the application.
//! These types were extracted from main.rs to enable better code organization
//! and to support the extraction of view modules.

use std::fs;

use std::collections::HashMap;

use cobol_bms_core::model::{BmsField, FieldType, FieldAttribute, DecorationType, Justify, DataType, Color as BmsColor};

/// Get next color in sequence
pub fn next_color(current: Option<BmsColor>) -> BmsColor {
    use BmsColor::*;
    match current {
        None => Blue,
        Some(Blue) => Green,
        Some(Green) => Red,
        Some(Red) => Yellow,
        Some(Yellow) => Cyan,
        Some(Cyan) => Magenta,
        Some(Magenta) => White,
        Some(White) => Black,
        Some(Black) => Blue,
        _ => Blue,
    }
}

/// Get previous color in sequence
pub fn prev_color(current: Option<BmsColor>) -> BmsColor {
    use BmsColor::*;
    match current {
        None => Blue,
        Some(Blue) => Black,
        Some(Black) => White,
        Some(White) => Magenta,
        Some(Magenta) => Cyan,
        Some(Cyan) => Yellow,
        Some(Yellow) => Red,
        Some(Red) => Green,
        Some(Green) => Blue,
        _ => Blue,
    }
}

/// Get the minimum height for a field type
pub fn get_min_height(field_type: &FieldType) -> u16 {
    use FieldType::*;
    match field_type {
        Group => 3,  // Fieldset requires 3 minimum
        Map => 1,
        Field => 1,
        Literal => 1,
        Attribute => 1,
        Symbolic => 1,
        // BMS statement types - most don't need height
        DFHMSD | DFHMDF | DFHMDI | 
        DFHMDA | DFHMND | DFHMNT | 
        DFHMDC | DFHMDL => 1,
        // Physical vs Symbolic
        PhysicalMap | SymbolicMap | MapSet => 1,
        // Special field types
        InputOnly | OutputOnly | InputOutput => 1,
        SymbolicMap | PhysicalMap => 1,
        _ => 1,  // Default minimum height
    }
}

/// File filter types for the file browser
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFilter {
    /// Show all files
    AllFiles,
    /// Show only BMS files (.bms)
    BmsFiles,
    /// Show only COBOL files (.cob, .cbl)
    CobolFiles,
    /// Show only text files (.txt)
    TextFiles,
}

impl FileFilter {
    pub fn next(self) -> Self {
        match self {
            FileFilter::AllFiles => FileFilter::BmsFiles,
            FileFilter::BmsFiles => FileFilter::CobolFiles,
            FileFilter::CobolFiles => FileFilter::TextFiles,
            FileFilter::TextFiles => FileFilter::AllFiles,
        }
    }
    
    pub fn display_name(self) -> &'static str {
        match self {
            FileFilter::AllFiles => "All Files",
            FileFilter::BmsFiles => "BMS Files (*.bms)",
            FileFilter::CobolFiles => "COBOL Files (*.cob, *.cbl)",
            FileFilter::TextFiles => "Text Files (*.txt)",
        }
    }
    
    pub fn file_extensions(self) -> Vec<&'static str> {
        match self {
            FileFilter::AllFiles => vec![],
            FileFilter::BmsFiles => vec![".bms"],
            FileFilter::CobolFiles => vec![".cob", ".cbl"],
            FileFilter::TextFiles => vec![".txt"],
        }
    }
    
    pub fn matches(self, filename: &str) -> bool {
        match self {
            FileFilter::AllFiles => true,
            _ => {
                let filename_lower = filename.to_lowercase();
                self.file_extensions().iter().any(|ext| filename_lower.ends_with(ext))
            }
        }
    }
}

/// Scan directory for files with the given filter
pub fn scan_directory_files_with_filter(directory: &str, filter: FileFilter) -> Vec<String> {
    let all_files = scan_directory_files(directory, false);
    all_files.into_iter()
        .filter(|f| filter.matches(f))
        .collect()
}

/// Scan directory for all files (no filter)
pub fn scan_directory_files(directory: &str, show_hidden: bool) -> Vec<String> {
    let path = std::path::Path::new(directory);
    let mut files = Vec::new();
    
    if path.exists() && path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    let is_file = entry.file_type().map(|ft| ft.is_file()).unwrap_or(false);
                    
                    // Skip hidden files if not showing hidden
                    if !show_hidden && file_name.starts_with('.') {
                        continue;
                    }
                    
                    if is_file {
                        files.push(file_name);
                    }
                }
            }
        }
    }
    
    files.sort();
    files
}

/// Types of objects that can be inserted into a BMS map
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InsertableObject {
    AlphanumericField,
    NumericField,
    DateField,
    TimeField,
    BooleanField,
    Literal,
    ProtectedLiteral,
    Fieldset,
    Line,
    AsciiArt,
    Image,
}

impl InsertableObject {
    pub fn all() -> &'static [InsertableObject] {
        &[
            InsertableObject::AlphanumericField,
            InsertableObject::NumericField,
            InsertableObject::DateField,
            InsertableObject::TimeField,
            InsertableObject::BooleanField,
            InsertableObject::Literal,
            InsertableObject::ProtectedLiteral,
            InsertableObject::Fieldset,
            InsertableObject::Line,
            InsertableObject::AsciiArt,
            InsertableObject::Image,
        ]
    }
    
    pub fn display(&self) -> &'static str {
        match self {
            InsertableObject::AlphanumericField => "Alphanumeric Field",
            InsertableObject::NumericField => "Numeric Field",
            InsertableObject::DateField => "Date Field",
            InsertableObject::TimeField => "Time Field",
            InsertableObject::BooleanField => "Boolean Field",
            InsertableObject::Literal => "Literal",
            InsertableObject::ProtectedLiteral => "Protected Literal",
            InsertableObject::Fieldset => "Fieldset",
            InsertableObject::Line => "Horizontal Line",
            InsertableObject::AsciiArt => "ASCII Art",
            InsertableObject::Image => "Import Image",
        }
    }
    
    pub fn default_length(&self) -> u16 {
        match self {
            InsertableObject::AlphanumericField => 20,
            InsertableObject::NumericField => 10,
            InsertableObject::DateField => 8,
            InsertableObject::TimeField => 6,
            InsertableObject::BooleanField => 1,
            InsertableObject::Literal | InsertableObject::ProtectedLiteral => 20,
            InsertableObject::Fieldset => 10,
            InsertableObject::Line | InsertableObject::AsciiArt | InsertableObject::Image => 40,
        }
    }
    
    pub fn create_field(&self, pos: (u16, u16)) -> BmsField {
        let mut field = BmsField::default();
        field.pos = pos;
        field.length = match self {
            InsertableObject::AlphanumericField => 20,
            InsertableObject::NumericField => 10,
            InsertableObject::DateField => 8,
            InsertableObject::TimeField => 6,
            InsertableObject::BooleanField => 1,
            InsertableObject::Literal | InsertableObject::ProtectedLiteral => 20,
            InsertableObject::Fieldset => 10,  // Default length for fieldset
            InsertableObject::Line | InsertableObject::AsciiArt | InsertableObject::Image => 40,
        };
        field.name = match self {
            InsertableObject::AlphanumericField => "ALNUM_FIELD".to_string(),
            InsertableObject::NumericField => "NUM_FIELD".to_string(),
            InsertableObject::DateField => "DATE_FIELD".to_string(),
            InsertableObject::TimeField => "TIME_FIELD".to_string(),
            InsertableObject::BooleanField => "BOOL_FIELD".to_string(),
            InsertableObject::Literal => "LITERAL".to_string(),
            InsertableObject::ProtectedLiteral => "PROT_LITERAL".to_string(),
            InsertableObject::Fieldset => "FIELDSET".to_string(),
            InsertableObject::Line => "HLINE".to_string(),
            InsertableObject::AsciiArt => "ASCII_ART".to_string(),
            InsertableObject::Image => "IMAGE_ART".to_string(),
        };
        field.field_type = match self {
            InsertableObject::Fieldset => FieldType::Group,
            InsertableObject::AsciiArt | InsertableObject::Image => FieldType::Literal, // Treat as literal for ASCII art
            _ => FieldType::Field,
        };
        field.attrb = match self {
            InsertableObject::ProtectedLiteral => vec![FieldAttribute::Prot],
            InsertableObject::NumericField => vec![FieldAttribute::Num],
            InsertableObject::DateField => vec![FieldAttribute::Date],
            InsertableObject::TimeField => vec![FieldAttribute::Time],
            InsertableObject::BooleanField => vec![FieldAttribute::Bool],
            _ => vec![FieldAttribute::Norm],
        };
        field.pic = match self {
            InsertableObject::NumericField => Some("9(10)".to_string()),
            InsertableObject::DateField => Some("X(8)".to_string()),
            InsertableObject::TimeField => Some("X(6)".to_string()),
            InsertableObject::BooleanField => Some("X(1)".to_string()),
            _ => None,
        };
        
        // Set AsciiArt-specific properties
        if matches!(self, InsertableObject::AsciiArt | InsertableObject::Image) {
            field.height = Some(5);  // Default height for ASCII art
        }
        
        // Set Fieldset-specific properties (minimum 3 rows)
        if matches!(self, InsertableObject::Fieldset) {
            field.height = Some(3);  // Use standard height property, minimum 3 for Fieldset
            field.fieldset_decoration = Some(DecorationType::Brackets);  // Default decoration for title
            field.fieldset_border = Some(DecorationType::Dashes);  // Default border for bottom line
            field.fieldset_title_align = Some(Justify::Left);  // Default title alignment: Left
            field.fieldset_title_fill_decoration = None;  // Default: space fill (no decoration)
        }
        
        field
    }
}

/// Object type metadata with ASCII model and properties
#[derive(Debug, Clone)]
pub struct ObjectTypeMetadata {
    /// ASCII art representation of the object (multi-line)
    pub ascii_model: Vec<&'static str>,
    /// Default properties for this object type
    pub default_properties: Vec<PropertyType>,
    /// Minimum height for this object type
    pub min_height: u16,
    /// Default length for this object type
    pub default_length: u16,
}

/// Property types for edit properties panel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyType {
    // Common properties for all field types
    Name,
    FieldType,
    PositionRow,
    PositionCol,
    Length,
    Attributes,
    TextColor,
    BorderColor,
    Initial,
    Pic,
    GrpName,
    
    // Multi-row properties
    Height,
    
    // Fieldset-specific properties
    FieldsetTitle,
    FieldsetDecoration,
    FieldsetBorder,
    FieldsetTitleAlign,
    FieldsetTitleFillDecoration,
    FieldsetTitleColor,
    FieldsetFillTitleColor,
    FieldsetBorderColor,
    FieldsetContentColor,
    
    // ASCII Art properties
    AsciiArt,
    
    // Extended BMS properties
    Justification,
    AutoSkip,
    FieldExit,
    BlankZero,
    Repeat,
    FillChar,
    Format,
    KeyType,
    DataType,
    Occurs,
    DependingOn,
    Redefines,
    SignLeading,
    SignTrailing,
    DecimalPoint,
    Synchronized,
    Usage,
}

impl PropertyType {
    pub fn display_name(&self) -> &'static str {
        match self {
            PropertyType::Name => "Name",
            PropertyType::FieldType => "Type",
            PropertyType::PositionRow => "Row",
            PropertyType::PositionCol => "Col",
            PropertyType::Length => "Length",
            PropertyType::Attributes => "Attributes",
            PropertyType::TextColor => "Text Color",
            PropertyType::BorderColor => "Border Color",
            PropertyType::Initial => "Initial",
            PropertyType::Pic => "PIC",
            PropertyType::GrpName => "Group Name",
            PropertyType::Height => "Height",
            PropertyType::FieldsetTitle => "Fieldset Title",
            PropertyType::FieldsetDecoration => "Decoration",
            PropertyType::FieldsetBorder => "Border",
            PropertyType::FieldsetTitleAlign => "Title Align",
            PropertyType::FieldsetTitleFillDecoration => "Title Fill",
            PropertyType::FieldsetTitleColor => "Title Color",
            PropertyType::FieldsetFillTitleColor => "Fill Title Color",
            PropertyType::FieldsetBorderColor => "Border Color",
            PropertyType::FieldsetContentColor => "Content Color",
            PropertyType::AsciiArt => "ASCII Art",
            PropertyType::Justification => "Justification",
            PropertyType::AutoSkip => "Auto Skip",
            PropertyType::FieldExit => "Field Exit",
            PropertyType::BlankZero => "Blank Zero",
            PropertyType::Repeat => "Repeat",
            PropertyType::FillChar => "Fill Char",
            PropertyType::Format => "Format",
            PropertyType::KeyType => "Key Type",
            PropertyType::DataType => "Data Type",
            PropertyType::Occurs => "Occurs",
            PropertyType::DependingOn => "Depending On",
            PropertyType::Redefines => "Redefines",
            PropertyType::SignLeading => "Sign Leading",
            PropertyType::SignTrailing => "Sign Trailing",
            PropertyType::DecimalPoint => "Decimal Point",
            PropertyType::Synchronized => "Synchronized",
            PropertyType::Usage => "Usage",
        }
    }
    
    /// Get property value as string for display
    pub fn get_value(&self, field: &BmsField) -> String {
        match self {
            PropertyType::Name => field.name.clone(),
            PropertyType::FieldType => format!("{:?}", field.field_type),
            PropertyType::PositionRow => field.pos.0.to_string(),
            PropertyType::PositionCol => field.pos.1.to_string(),
            PropertyType::Length => field.length.to_string(),
            PropertyType::Attributes => {
                if field.attrb.is_empty() {
                    "None".to_string()
                } else {
                    field.attrb.iter().map(|a| format!("{:?}", a)).collect::<Vec<_>>().join(", ")
                }
            },
            PropertyType::TextColor => format!("{:?}", field.text_color),
            PropertyType::BorderColor => format!("{:?}", field.border_color),
            PropertyType::Initial => field.initial.clone().unwrap_or_default(),
            PropertyType::Pic => field.pic.clone().unwrap_or_default(),
            PropertyType::GrpName => field.grp_name.clone().unwrap_or_default(),
            PropertyType::Height => field.height.map_or("None".to_string(), |h| h.to_string()),
            PropertyType::FieldsetTitle => field.fieldset_title.clone().unwrap_or_default(),
            PropertyType::FieldsetDecoration => format!("{:?}", field.fieldset_decoration),
            PropertyType::FieldsetBorder => format!("{:?}", field.fieldset_border),
            PropertyType::FieldsetTitleAlign => format!("{:?}", field.fieldset_title_align),
            PropertyType::FieldsetTitleFillDecoration => format!("{:?}", field.fieldset_title_fill_decoration),
            PropertyType::FieldsetTitleColor => format!("{:?}", field.fieldset_title_color),
            PropertyType::FieldsetFillTitleColor => format!("{:?}", field.fieldset_fill_title_color),
            PropertyType::FieldsetBorderColor => format!("{:?}", field.fieldset_border_color),
            PropertyType::FieldsetContentColor => format!("{:?}", field.fieldset_content_color),
            PropertyType::AsciiArt => {
                if let Some(ascii) = &field.ascii_art {
                    format!("{}x{}", ascii.width, ascii.height)
                } else {
                    "None".to_string()
                }
            },
            PropertyType::Justification => format!("{:?}", field.justification),
            PropertyType::AutoSkip => format!("{:?}", field.autoskip),
            PropertyType::FieldExit => format!("{:?}", field.fieldexit),
            PropertyType::BlankZero => format!("{:?}", field.blank_zero),
            PropertyType::Repeat => field.repeat.map_or("None".to_string(), |r| r.to_string()),
            PropertyType::FillChar => field.fill_char.map_or("None".to_string(), |c| c.to_string()),
            PropertyType::Format => field.format.clone().unwrap_or_default(),
            PropertyType::KeyType => format!("{:?}", field.key_type),
            PropertyType::DataType => format!("{:?}", field.data_type),
            PropertyType::Occurs => field.occurs.map_or("None".to_string(), |o| o.to_string()),
            PropertyType::DependingOn => field.depending_on.clone().unwrap_or_default(),
            PropertyType::Redefines => field.redefines.clone().unwrap_or_default(),
            PropertyType::SignLeading => format!("{:?}", field.sign_leading),
            PropertyType::SignTrailing => format!("{:?}", field.sign_trailing),
            PropertyType::DecimalPoint => format!("{:?}", field.decimal_point),
            PropertyType::Synchronized => format!("{:?}", field.synchronized),
            PropertyType::Usage => field.usage.clone().unwrap_or_default(),
        }
    }
    
    /// Modify property value
    pub fn modify_value(&self, field: &mut BmsField, increase: bool) {
        match self {
            PropertyType::Name => {},
            PropertyType::FieldType => {
                field.field_type = match field.field_type.clone() {
                    FieldType::Field => if increase { FieldType::Literal } else { FieldType::Map },
                    FieldType::Literal => if increase { FieldType::Group } else { FieldType::Field },
                    FieldType::Group => if increase { FieldType::Map } else { FieldType::Literal },
                    FieldType::Map => if increase { FieldType::Field } else { FieldType::Group },
                    other => if increase { FieldType::Field } else { other.clone() },
                };
            },
            PropertyType::PositionRow => {
                if increase { field.pos.0 += 1; } else if field.pos.0 > 1 { field.pos.0 -= 1; }
            },
            PropertyType::PositionCol => {
                if increase { field.pos.1 += 1; } else if field.pos.1 > 1 { field.pos.1 -= 1; }
            },
            PropertyType::Length => {
                if increase { field.length += 1; } else if field.length > 1 { field.length -= 1; }
            },
            PropertyType::Attributes => {
                // Cycle through common attributes
                if !field.attrb.is_empty() {
                    // For now, just add/remove Norm
                    if increase {
                        if !field.attrb.contains(&FieldAttribute::Norm) {
                            field.attrb.push(FieldAttribute::Norm);
                        }
                    } else {
                        field.attrb.retain(|a| a != &FieldAttribute::Norm);
                    }
                } else if increase {
                    field.attrb.push(FieldAttribute::Norm);
                }
            },
            PropertyType::TextColor => {
                field.text_color = Some(if increase {
                    next_color(field.text_color.clone())
                } else {
                    prev_color(field.text_color.clone())
                });
            },
            PropertyType::BorderColor => {
                field.border_color = Some(if increase {
                    next_color(field.border_color.clone())
                } else {
                    prev_color(field.border_color.clone())
                });
            },
            PropertyType::Initial => {
                field.initial = Some(if increase {
                    field.initial.clone().unwrap_or_default() + "+"
                } else {
                    let mut val = field.initial.clone().unwrap_or_default();
                    val.pop();
                    val
                });
            },
            PropertyType::Pic => {
                field.pic = Some(if increase {
                    field.pic.clone().unwrap_or_default() + "X"
                } else {
                    let mut val = field.pic.clone().unwrap_or_default();
                    val.pop();
                    val
                });
            },
            PropertyType::GrpName => {
                field.grp_name = Some(if increase {
                    field.grp_name.clone().unwrap_or_default() + "G"
                } else {
                    let mut val = field.grp_name.clone().unwrap_or_default();
                    val.pop();
                    val
                });
            },
            PropertyType::Height => {
                let min_height = get_min_height(&field.field_type);
                field.height = Some(if increase {
                    field.height.unwrap_or(min_height) + 1
                } else {
                    (field.height.unwrap_or(min_height)).saturating_sub(1).max(min_height)
                });
            },
            PropertyType::FieldsetTitle => {
                field.fieldset_title = Some(if increase {
                    field.fieldset_title.clone().unwrap_or_default() + "T"
                } else {
                    let mut val = field.fieldset_title.clone().unwrap_or_default();
                    val.pop();
                    val
                });
            },

            PropertyType::FieldsetDecoration => {
                field.fieldset_decoration = Some(match field.fieldset_decoration {
                    Some(DecorationType::Brackets) => if increase { DecorationType::Parentheses } else { DecorationType::Equals },
                    Some(DecorationType::Parentheses) => if increase { DecorationType::Plus } else { DecorationType::Brackets },
                    Some(DecorationType::Plus) => if increase { DecorationType::Asterisk } else { DecorationType::Parentheses },
                    Some(DecorationType::Asterisk) => if increase { DecorationType::Hash } else { DecorationType::Plus },
                    Some(DecorationType::Hash) => if increase { DecorationType::Dashes } else { DecorationType::Asterisk },
                    Some(DecorationType::Dashes) => if increase { DecorationType::Equals } else { DecorationType::Hash },
                    Some(DecorationType::Equals) => if increase { DecorationType::Brackets } else { DecorationType::Dashes },
                    None => if increase { DecorationType::Brackets } else { DecorationType::Equals },
                    _ => DecorationType::Brackets,
                });
            },
            PropertyType::FieldsetBorder => {
                field.fieldset_border = Some(match field.fieldset_border {
                    Some(DecorationType::Brackets) => if increase { DecorationType::Parentheses } else { DecorationType::Equals },
                    Some(DecorationType::Parentheses) => if increase { DecorationType::Plus } else { DecorationType::Brackets },
                    Some(DecorationType::Plus) => if increase { DecorationType::Asterisk } else { DecorationType::Parentheses },
                    Some(DecorationType::Asterisk) => if increase { DecorationType::Hash } else { DecorationType::Plus },
                    Some(DecorationType::Hash) => if increase { DecorationType::Dashes } else { DecorationType::Asterisk },
                    Some(DecorationType::Dashes) => if increase { DecorationType::Equals } else { DecorationType::Hash },
                    Some(DecorationType::Equals) => if increase { DecorationType::Brackets } else { DecorationType::Dashes },
                    None => if increase { DecorationType::Dashes } else { DecorationType::Equals },
                    _ => DecorationType::Dashes,
                });
            },
            PropertyType::FieldsetTitleAlign => {
                field.fieldset_title_align = Some(match field.fieldset_title_align {
                    Some(Justify::Left) => if increase { Justify::Center } else { Justify::Right },
                    Some(Justify::Center) => if increase { Justify::Right } else { Justify::Left },
                    Some(Justify::Right) => if increase { Justify::Left } else { Justify::Center },
                    None => if increase { Justify::Left } else { Justify::Right },
                });
            },
            PropertyType::FieldsetTitleFillDecoration => {
                field.fieldset_title_fill_decoration = Some(match field.fieldset_title_fill_decoration {
                    Some(DecorationType::Brackets) => if increase { DecorationType::Parentheses } else { DecorationType::Equals },
                    Some(DecorationType::Parentheses) => if increase { DecorationType::Plus } else { DecorationType::Brackets },
                    Some(DecorationType::Plus) => if increase { DecorationType::Asterisk } else { DecorationType::Parentheses },
                    Some(DecorationType::Asterisk) => if increase { DecorationType::Hash } else { DecorationType::Plus },
                    Some(DecorationType::Hash) => if increase { DecorationType::Dashes } else { DecorationType::Asterisk },
                    Some(DecorationType::Dashes) => if increase { DecorationType::Equals } else { DecorationType::Hash },
                    Some(DecorationType::Equals) => if increase { DecorationType::Brackets } else { DecorationType::Dashes },
                    None => if increase { DecorationType::Brackets } else { DecorationType::Equals },
                    _ => DecorationType::Brackets,
                });
            },
            PropertyType::FieldsetTitleColor => {
                field.fieldset_title_color = Some(if increase {
                    next_color(field.fieldset_title_color.clone())
                } else {
                    prev_color(field.fieldset_title_color.clone())
                });
            },
            PropertyType::FieldsetFillTitleColor => {
                field.fieldset_fill_title_color = Some(if increase {
                    next_color(field.fieldset_fill_title_color.clone())
                } else {
                    prev_color(field.fieldset_fill_title_color.clone())
                });
            },
            PropertyType::FieldsetBorderColor => {
                field.fieldset_border_color = Some(if increase {
                    next_color(field.fieldset_border_color.clone())
                } else {
                    prev_color(field.fieldset_border_color.clone())
                });
            },
            PropertyType::FieldsetContentColor => {
                field.fieldset_content_color = Some(if increase {
                    next_color(field.fieldset_content_color.clone())
                } else {
                    prev_color(field.fieldset_content_color.clone())
                });
            },
            PropertyType::AsciiArt => {
                // Cannot modify ASCII art here - use image import
            },
            PropertyType::Justification => {
                field.justification = Some(match field.justification {
                    Some(Justify::Left) => if increase { Justify::Center } else { Justify::Right },
                    Some(Justify::Center) => if increase { Justify::Right } else { Justify::Left },
                    Some(Justify::Right) => if increase { Justify::Left } else { Justify::Center },
                    None => if increase { Justify::Left } else { Justify::Right },
                });
            },
            PropertyType::AutoSkip => {
                field.autoskip = Some(!field.autoskip.unwrap_or(false));
            },
            PropertyType::FieldExit => {
                field.fieldexit = Some(!field.fieldexit.unwrap_or(false));
            },
            PropertyType::BlankZero => {
                field.blank_zero = Some(!field.blank_zero.unwrap_or(false));
            },
            PropertyType::Repeat => {
                field.repeat = Some(if increase {
                    field.repeat.unwrap_or(1) + 1
                } else {
                    (field.repeat.unwrap_or(1)).saturating_sub(1)
                });
            },
            PropertyType::FillChar => {
                field.fill_char = Some(if increase {
                    if field.fill_char.unwrap_or(' ') == ' ' { '0' } else { ' ' }
                } else {
                    if field.fill_char.unwrap_or('0') == '0' { ' ' } else { '0' }
                });
            },
            PropertyType::Format => {
                field.format = Some(if increase {
                    field.format.clone().unwrap_or_default() + "F"
                } else {
                    let mut val = field.format.clone().unwrap_or_default();
                    val.pop();
                    val
                });
            },
            PropertyType::KeyType => {},
            PropertyType::DataType => {
                field.data_type = Some(match field.data_type {
                    Some(DataType::Alphanumeric) => if increase { DataType::Numeric } else { DataType::Group },
                    Some(DataType::Numeric) => if increase { DataType::Date } else { DataType::Alphanumeric },
                    Some(DataType::Date) => if increase { DataType::Time } else { DataType::Numeric },
                    Some(DataType::Time) => if increase { DataType::Boolean } else { DataType::Date },
                    Some(DataType::Boolean) => if increase { DataType::Group } else { DataType::Time },
                    Some(DataType::Group) => if increase { DataType::Alphanumeric } else { DataType::Boolean },
                    None | Some(_) => if increase { DataType::Alphanumeric } else { DataType::Group },
                });
            },
            PropertyType::Occurs => {
                field.occurs = Some(if increase {
                    field.occurs.unwrap_or(1) + 1
                } else {
                    (field.occurs.unwrap_or(1)).saturating_sub(1)
                });
            },
            PropertyType::DependingOn => {
                field.depending_on = Some(if increase {
                    field.depending_on.clone().unwrap_or_default() + "D"
                } else {
                    let mut val = field.depending_on.clone().unwrap_or_default();
                    val.pop();
                    val
                });
            },
            PropertyType::Redefines => {
                field.redefines = Some(if increase {
                    field.redefines.clone().unwrap_or_default() + "R"
                } else {
                    let mut val = field.redefines.clone().unwrap_or_default();
                    val.pop();
                    val
                });
            },
            PropertyType::SignLeading => {
                field.sign_leading = Some(!field.sign_leading.unwrap_or(false));
            },
            PropertyType::SignTrailing => {
                field.sign_trailing = Some(!field.sign_trailing.unwrap_or(false));
            },
            PropertyType::DecimalPoint => {
                field.decimal_point = Some(!field.decimal_point.unwrap_or(false));
            },
            PropertyType::Synchronized => {
                field.synchronized = Some(!field.synchronized.unwrap_or(false));
            },
            PropertyType::Usage => {
                field.usage = Some(if increase {
                    field.usage.clone().unwrap_or_default() + "U"
                } else {
                    let mut val = field.usage.clone().unwrap_or_default();
                    val.pop();
                    val
                });
            },
        }
    }
}

/// Internal dictionary of object types with their ASCII models and properties
pub fn get_object_type_metadata() -> HashMap<InsertableObject, ObjectTypeMetadata> {
    let mut map = HashMap::new();
    
    // Alphanumeric Field: single line text field
    map.insert(InsertableObject::AlphanumericField, ObjectTypeMetadata {
        ascii_model: vec!["[ALNUM]"],
        default_properties: vec![
            PropertyType::Name, PropertyType::PositionRow, PropertyType::PositionCol,
            PropertyType::Length, PropertyType::Attributes, PropertyType::TextColor,
            PropertyType::Initial, PropertyType::Pic,
        ],
        min_height: 1,
        default_length: 10,
    });
    
    // Numeric Field: single line numeric field
    map.insert(InsertableObject::NumericField, ObjectTypeMetadata {
        ascii_model: vec!["[NUM]"],
        default_properties: vec![
            PropertyType::Name, PropertyType::PositionRow, PropertyType::PositionCol,
            PropertyType::Length, PropertyType::Attributes, PropertyType::TextColor,
            PropertyType::Initial, PropertyType::Pic,
        ],
        min_height: 1,
        default_length: 10,
    });
    
    // Date Field: single line date field
    map.insert(InsertableObject::DateField, ObjectTypeMetadata {
        ascii_model: vec!["[DATE]"],
        default_properties: vec![
            PropertyType::Name, PropertyType::PositionRow, PropertyType::PositionCol,
            PropertyType::Length, PropertyType::Attributes, PropertyType::TextColor,
            PropertyType::Initial, PropertyType::Pic,
        ],
        min_height: 1,
        default_length: 8,
    });
    
    // Time Field: single line time field
    map.insert(InsertableObject::TimeField, ObjectTypeMetadata {
        ascii_model: vec!["[TIME]"],
        default_properties: vec![
            PropertyType::Name, PropertyType::PositionRow, PropertyType::PositionCol,
            PropertyType::Length, PropertyType::Attributes, PropertyType::TextColor,
            PropertyType::Initial, PropertyType::Pic,
        ],
        min_height: 1,
        default_length: 6,
    });
    
    // Boolean Field: single line boolean field
    map.insert(InsertableObject::BooleanField, ObjectTypeMetadata {
        ascii_model: vec!["[BOOL]"],
        default_properties: vec![
            PropertyType::Name, PropertyType::PositionRow, PropertyType::PositionCol,
            PropertyType::Length, PropertyType::Attributes, PropertyType::TextColor,
            PropertyType::Initial,
        ],
        min_height: 1,
        default_length: 1,
    });
    
    // Literal: single line literal text
    map.insert(InsertableObject::Literal, ObjectTypeMetadata {
        ascii_model: vec!["[LITERAL]"],
        default_properties: vec![
            PropertyType::Name, PropertyType::PositionRow, PropertyType::PositionCol,
            PropertyType::Length, PropertyType::Attributes, PropertyType::TextColor,
            PropertyType::Initial,
        ],
        min_height: 1,
        default_length: 10,
    });
    
    // Protected Literal: single line protected literal
    map.insert(InsertableObject::ProtectedLiteral, ObjectTypeMetadata {
        ascii_model: vec!["[PROT LIT]"],
        default_properties: vec![
            PropertyType::Name, PropertyType::PositionRow, PropertyType::PositionCol,
            PropertyType::Length, PropertyType::Attributes, PropertyType::TextColor,
            PropertyType::Initial,
        ],
        min_height: 1,
        default_length: 10,
    });
    
    // Fieldset/Group: multi-line with title and border
    map.insert(InsertableObject::Fieldset, ObjectTypeMetadata {
        ascii_model: vec![
            "╭────────────╮",
            "│  FIELDSET   │",
            "╰────────────╯",
        ],
        default_properties: vec![
            PropertyType::Name, PropertyType::FieldType, PropertyType::PositionRow, PropertyType::PositionCol,
            PropertyType::Length, PropertyType::Height,
            PropertyType::FieldsetTitle,
            PropertyType::FieldsetDecoration, PropertyType::FieldsetBorder,
            PropertyType::FieldsetTitleAlign, PropertyType::FieldsetTitleFillDecoration,
            PropertyType::TextColor, PropertyType::BorderColor,
            PropertyType::FieldsetTitleColor, PropertyType::FieldsetFillTitleColor,
            PropertyType::FieldsetBorderColor, PropertyType::FieldsetContentColor,
        ],
        min_height: 3,
        default_length: 20,
    });
    
    // Horizontal Line: single line separator
    map.insert(InsertableObject::Line, ObjectTypeMetadata {
        ascii_model: vec!["──────────"],
        default_properties: vec![
            PropertyType::Name, PropertyType::PositionRow, PropertyType::PositionCol,
            PropertyType::Length, PropertyType::TextColor,
        ],
        min_height: 1,
        default_length: 80,
    });
    
    // ASCII Art: multi-line art
    map.insert(InsertableObject::AsciiArt, ObjectTypeMetadata {
        ascii_model: vec![
            "  ___  ",
            " /   \\ ",
            "|     |",
            " \\___/ ",
        ],
        default_properties: vec![
            PropertyType::Name, PropertyType::PositionRow, PropertyType::PositionCol,
            PropertyType::Length, PropertyType::Height,
            PropertyType::TextColor, PropertyType::AsciiArt,
        ],
        min_height: 1,
        default_length: 10,
    });
    
    // Image: import image as ASCII art
    map.insert(InsertableObject::Image, ObjectTypeMetadata {
        ascii_model: vec![
            "  .------.",
            "  | IMAGE |",
            "  '------'",
        ],
        default_properties: vec![
            PropertyType::Name, PropertyType::PositionRow, PropertyType::PositionCol,
            PropertyType::Length, PropertyType::Height,
            PropertyType::TextColor, PropertyType::AsciiArt,
        ],
        min_height: 1,
        default_length: 10,
    });

    map
}

/// Get InsertableObject from a BmsField
pub fn get_insertable_object_from_field(field: &BmsField) -> Option<InsertableObject> {
    // Determine which InsertableObject this field corresponds to
    if matches!(field.field_type, FieldType::Group) && field.height.is_some() {
        Some(InsertableObject::Fieldset)
    } else if field.ascii_art.is_some() {
        if field.name == "IMAGE_ART" {
            Some(InsertableObject::Image)
        } else {
            Some(InsertableObject::AsciiArt)
        }
    } else if field.attrb.contains(&FieldAttribute::Prot) && field.initial.is_some() {
        Some(InsertableObject::ProtectedLiteral)
    } else if field.attrb.contains(&FieldAttribute::Prot) {
        Some(InsertableObject::ProtectedLiteral)
    } else if field.attrb.contains(&FieldAttribute::Num) {
        Some(InsertableObject::NumericField)
    } else if field.attrb.contains(&FieldAttribute::Alph) || field.attrb.contains(&FieldAttribute::AlphaNum) {
        Some(InsertableObject::AlphanumericField)
    } else if field.attrb.contains(&FieldAttribute::Date) {
        Some(InsertableObject::DateField)
    } else if field.attrb.contains(&FieldAttribute::Time) {
        Some(InsertableObject::TimeField)
    } else if field.attrb.contains(&FieldAttribute::Bool) {
        Some(InsertableObject::BooleanField)
    } else if field.pic.is_some() {
        // Fields with PIC are likely literals or formatted fields
        Some(InsertableObject::Literal)
    } else {
        // Default to Literal for simple fields
        Some(InsertableObject::Literal)
    }
}

/// Get the ASCII model for an object type
pub fn get_ascii_model(obj_type: &InsertableObject) -> Vec<&'static str> {
    get_object_type_metadata()
        .get(obj_type)
        .map(|m| m.ascii_model.clone())
        .unwrap_or_else(|| vec!["[UNKNOWN]"])
}

/// Get the minimum height for an object type
pub fn get_object_min_height(obj_type: &InsertableObject) -> u16 {
    get_object_type_metadata()
        .get(obj_type)
        .map(|m| m.min_height)
        .unwrap_or(1)
}

/// Get the list of properties to display for a field, based on its type
/// Uses the internal object type dictionary to determine relevant properties
pub fn get_properties_for_field(field: &BmsField) -> Vec<PropertyType> {
    // Try to get the object type metadata
    if let Some(obj_type) = get_insertable_object_from_field(field) {
        if let Some(metadata) = get_object_type_metadata().get(&obj_type) {
            return metadata.default_properties.clone();
        }
    }
    
    // Fallback to default implementation
    let mut properties = vec![
        PropertyType::Name,
        PropertyType::FieldType,
        PropertyType::PositionRow,
        PropertyType::PositionCol,
        PropertyType::Length,
        PropertyType::Attributes,
        PropertyType::TextColor,
        PropertyType::BorderColor,
        PropertyType::Initial,
        PropertyType::Pic,
        PropertyType::GrpName,
    ];
    
    // Add height for multi-row fields (ASCII Art, Fieldset)
    let is_multi_row = field.height.is_some() || field.ascii_art.is_some();
    if is_multi_row {
        properties.push(PropertyType::Height);
    }
    
    // Add fieldset-specific properties for Group type
    if matches!(field.field_type, FieldType::Group) {
        properties.extend(vec![
            PropertyType::FieldsetTitle,
            PropertyType::FieldsetDecoration,
            PropertyType::FieldsetBorder,
            PropertyType::FieldsetTitleAlign,
            PropertyType::FieldsetTitleFillDecoration,
            PropertyType::FieldsetTitleColor,
            PropertyType::FieldsetFillTitleColor,
            PropertyType::FieldsetBorderColor,
            PropertyType::FieldsetContentColor,
        ]);
    }
    
    // Add ASCII Art specific properties
    if field.ascii_art.is_some() {
        properties.push(PropertyType::AsciiArt);
    }
    
    // Add extended BMS properties
    properties.extend(vec![
        PropertyType::Justification,
        PropertyType::AutoSkip,
        PropertyType::FieldExit,
        PropertyType::BlankZero,
        PropertyType::Repeat,
        PropertyType::FillChar,
        PropertyType::Format,
        PropertyType::KeyType,
        PropertyType::DataType,
        PropertyType::Occurs,
        PropertyType::DependingOn,
        PropertyType::Redefines,
        PropertyType::SignLeading,
        PropertyType::SignTrailing,
        PropertyType::DecimalPoint,
        PropertyType::Synchronized,
        PropertyType::Usage,
    ]);
    
    properties
}

/// Check if a property should start a new group
pub fn is_property_group_start(index: usize, properties: &[PropertyType]) -> bool {
    if index == 0 {
        return false;
    }
    
    let current = &properties[index];
    let prev = &properties[index - 1];
    
    // Define property groups
    let current_group = match current {
        PropertyType::Name | PropertyType::FieldType | PropertyType::PositionRow | 
        PropertyType::PositionCol | PropertyType::Length | PropertyType::Attributes => 0,
        PropertyType::TextColor | PropertyType::BorderColor => 1,
        PropertyType::Initial | PropertyType::Pic | PropertyType::GrpName => 2,
        PropertyType::Height => 3,
        PropertyType::FieldsetTitle | PropertyType::FieldsetDecoration | 
        PropertyType::FieldsetBorder | PropertyType::FieldsetTitleAlign | 
        PropertyType::FieldsetTitleFillDecoration | PropertyType::FieldsetTitleColor | 
        PropertyType::FieldsetFillTitleColor | PropertyType::FieldsetBorderColor | 
        PropertyType::FieldsetContentColor => 4,
        PropertyType::AsciiArt => 5,
        PropertyType::Justification | PropertyType::AutoSkip | PropertyType::FieldExit | 
        PropertyType::BlankZero | PropertyType::Repeat | PropertyType::FillChar | 
        PropertyType::Format | PropertyType::KeyType | PropertyType::DataType => 6,
        PropertyType::Occurs | PropertyType::DependingOn | PropertyType::Redefines | 
        PropertyType::SignLeading | PropertyType::SignTrailing | PropertyType::DecimalPoint | 
        PropertyType::Synchronized | PropertyType::Usage => 7,
    };
    
    let prev_group = match prev {
        PropertyType::Name | PropertyType::FieldType | PropertyType::PositionRow | 
        PropertyType::PositionCol | PropertyType::Length | PropertyType::Attributes => 0,
        PropertyType::TextColor | PropertyType::BorderColor => 1,
        PropertyType::Initial | PropertyType::Pic | PropertyType::GrpName => 2,
        PropertyType::Height => 3,
        PropertyType::FieldsetTitle | PropertyType::FieldsetDecoration | 
        PropertyType::FieldsetBorder | PropertyType::FieldsetTitleAlign | 
        PropertyType::FieldsetTitleFillDecoration | PropertyType::FieldsetTitleColor | 
        PropertyType::FieldsetFillTitleColor | PropertyType::FieldsetBorderColor | 
        PropertyType::FieldsetContentColor => 4,
        PropertyType::AsciiArt => 5,
        PropertyType::Justification | PropertyType::AutoSkip | PropertyType::FieldExit | 
        PropertyType::BlankZero | PropertyType::Repeat | PropertyType::FillChar | 
        PropertyType::Format | PropertyType::KeyType | PropertyType::DataType => 6,
        PropertyType::Occurs | PropertyType::DependingOn | PropertyType::Redefines | 
        PropertyType::SignLeading | PropertyType::SignTrailing | PropertyType::DecimalPoint | 
        PropertyType::Synchronized | PropertyType::Usage => 7,
    };
    
    current_group != prev_group
}