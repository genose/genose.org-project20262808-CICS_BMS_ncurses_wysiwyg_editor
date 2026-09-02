//! Utility functions for views
//!
//! This module contains utility functions used across multiple view modules.

use ratatui::style::Color as TuiColor;
use std::path::Path;

use cobol_bms_core::model::Color as BmsColor;

/// Check if the current terminal is running inside VSCode
/// VSCode uses a specific terminal identifier that we can detect
pub fn is_vscode_terminal() -> bool {
    if let Ok(term) = std::env::var("TERM_PROGRAM") {
        term == "vscode" || term.contains("vscode")
    } else {
        false
    }
}

/// Convert BMS color to Ratatui color
pub fn bms_color_to_tui(color: &BmsColor) -> TuiColor {
    use BmsColor::*;
    match color {
        Black => TuiColor::Black,
        Blue => TuiColor::Blue,
        Green => TuiColor::Green,
        Cyan => TuiColor::Cyan,
        Red => TuiColor::Red,
        Magenta => TuiColor::Magenta,
        Yellow => TuiColor::Yellow,
        White => TuiColor::White,
        Turquoise => TuiColor::Cyan,
        Pink => TuiColor::Magenta,
        Orange => TuiColor::Rgb(255, 165, 0),
        Purple => TuiColor::Rgb(128, 0, 128),
        Gray => TuiColor::Gray,
        LightGreen => TuiColor::LightGreen,
        LightBlue => TuiColor::LightBlue,
        LightCyan => TuiColor::LightCyan,
        LightRed => TuiColor::LightRed,
        LightMagenta => TuiColor::LightMagenta,
        LightYellow => TuiColor::LightYellow,
        Neutral => TuiColor::White,
        Custom(_) => TuiColor::White,
        Default => TuiColor::White,
        Unknown(_) => TuiColor::White,
    }
}

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

/// Convert color string to TuiColor for ASCII art rendering
pub fn color_string_to_tui(color_str: &Option<String>) -> TuiColor {
    if let Some(color) = color_str {
        match color.to_uppercase().as_str() {
            "BLACK" => TuiColor::Black,
            "BLUE" => TuiColor::Blue,
            "GREEN" => TuiColor::Green,
            "CYAN" => TuiColor::Cyan,
            "RED" => TuiColor::Red,
            "MAGENTA" => TuiColor::Magenta,
            "YELLOW" => TuiColor::Yellow,
            "WHITE" => TuiColor::White,
            "ORANGE" => TuiColor::Rgb(255, 165, 0),
            "PURPLE" => TuiColor::Rgb(128, 0, 128),
            "PINK" => TuiColor::Magenta,
            "GRAY" | "GREY" => TuiColor::Gray,
            _ => TuiColor::White,
        }
    } else {
        TuiColor::White
    }
}

/// Check if a file is an image file
pub fn is_image_file(filename: &str) -> bool {
    let ext = Path::new(filename)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase());
    
    match ext.as_deref() {
        Some("png") | Some("jpg") | Some("jpeg") | Some("tif") | Some("tiff") |
        Some("gif") | Some("bmp") | Some("webp") | Some("svg") => true,
        _ => false,
    }
}

/// Scan a directory for files, optionally filtering for image files only
pub fn scan_directory_files(directory: &str, image_only: bool) -> Vec<String> {
    let path = Path::new(directory);
    
    if !path.exists() || !path.is_dir() {
        return Vec::new();
    }
    
    let mut files: Vec<String> = std::fs::read_dir(path)
        .ok()
        .map(|entries| {
            entries.filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() {
                    let filename = path.file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.to_string());
                    filename
                } else {
                    None
                }
            }).collect()
        })
        .unwrap_or_default();
    
    // Sort files alphabetically
    files.sort();
    
    if image_only {
        files.into_iter()
            .filter(|f| is_image_file(f))
            .collect()
    } else {
        files
    }
}

/// Get subdirectories in a directory
pub fn scan_directory_dirs(directory: &str) -> Vec<String> {
    let path = Path::new(directory);
    
    if !path.exists() || !path.is_dir() {
        return Vec::new();
    }
    
    std::fs::read_dir(path)
        .ok()
        .map(|entries| {
            entries.filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_dir() {
                    let dirname = path.file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.to_string());
                    dirname
                } else {
                    None
                }
            }).collect()
        })
        .unwrap_or_default()
}

/// Get the minimum height for a field type
pub fn get_min_height(field_type: &crate::FieldType) -> u16 {
    use crate::FieldType::*;
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