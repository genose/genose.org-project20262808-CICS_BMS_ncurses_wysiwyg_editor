//! Types module
//!
//! This module contains shared types and enums used throughout the application.
//! These types were extracted from main.rs to enable better code organization
//! and to support the extraction of view modules.

use std::fs;

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