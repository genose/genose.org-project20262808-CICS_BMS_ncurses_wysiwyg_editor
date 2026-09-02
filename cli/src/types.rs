//! Types module
//!
//! This module contains shared types and enums used throughout the application.
//! These types were extracted from main.rs to enable better code organization
//! and to support the extraction of view modules.

use std::fs;

use cobol_bms_core::model::{BmsField, FieldType, FieldAttribute, DecorationType, Justify};

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