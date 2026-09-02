//! ASCII Art functionality for BMS fields
//!
//! This module provides comprehensive ASCII art generation and manipulation
//! capabilities that integrate with the BMS field system and type-safe
//! color management.

use super::{
    model::{AsciiArt, AsciiArtChar},
    types::Color,
};
use std::path::Path;
use image::{DynamicImage, Rgba};

/// ASCII Art configuration for custom generation
#[derive(Debug, Clone)]
pub struct AsciiArtConfig {
    /// Character set to use for ASCII art (darker to lighter)
    pub charset: String,
    /// Use color information
    pub use_color: bool,
    /// Invert brightness mapping
    pub invert_brightness: bool,
    /// Color mapping function
    pub color_mapper: ColorMapper,
}

impl Default for AsciiArtConfig {
    fn default() -> Self {
        Self {
            charset: Self::default_charset(),
            use_color: true,
            invert_brightness: false,
            color_mapper: ColorMapper::BmsColors,
        }
    }
}

impl AsciiArtConfig {
    /// Default character set for ASCII art (darker to lighter)
    pub fn default_charset() -> String {
        " .:,;i!I><~+_?7LFO@".to_string()
    }
    
    /// High contrast character set
    pub fn high_contrast_charset() -> String {
        " .-:=+*#%@".to_string()
    }
    
    /// Low contrast character set
    pub fn low_contrast_charset() -> String {
        " .,:;+i1tfLCG08&".to_string()
    }
    
    /// Create config with custom charset
    pub fn with_charset(charset: &str) -> Self {
        Self {
            charset: charset.to_string(),
            use_color: true,
            invert_brightness: false,
            color_mapper: ColorMapper::BmsColors,
        }
    }
    
    /// Create config without color
    pub fn without_color() -> Self {
        Self {
            charset: Self::default_charset(),
            use_color: false,
            invert_brightness: false,
            color_mapper: ColorMapper::None,
        }
    }
}

/// Color mapping strategy for ASCII art
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMapper {
    /// Use BMS-compatible colors only
    BmsColors,
    /// Use all available colors
    AllColors,
    /// Use grayscale
    Grayscale,
    /// Use no colors (monochrome)
    None,
}

/// RGB to Color conversion using our type-safe Color enum
/// This provides better integration with the BMS type system
pub fn rgb_to_bms_color(r: u8, g: u8, b: u8, a: u8) -> Color {
    if a == 0 {
        return Color::Default;
    }
    
    let r_f = r as f32 / 255.0;
    let g_f = g as f32 / 255.0;
    let b_f = b as f32 / 255.0;
    
    // Check for grayscale/black/white
    if (r as i32 - g as i32).abs() < 30 && (g as i32 - b as i32).abs() < 30 {
        if r < 30 {
            return Color::Black;
        } else if r > 220 {
            return Color::White;
        } else {
            return Color::Gray;
        }
    }
    
    // Check for primary colors
    if r_f > 0.8 && g_f < 0.3 && b_f < 0.3 {
        return Color::Red;
    }
    if g_f > 0.8 && r_f < 0.3 && b_f < 0.3 {
        return Color::Green;
    }
    if b_f > 0.8 && r_f < 0.3 && g_f < 0.3 {
        return Color::Blue;
    }
    
    // Check for secondary colors
    if r_f > 0.7 && g_f > 0.7 && b_f < 0.3 {
        return Color::Yellow;
    }
    if r_f > 0.7 && b_f > 0.7 && g_f < 0.3 {
        return Color::Magenta;
    }
    if g_f > 0.7 && b_f > 0.7 && r_f < 0.3 {
        return Color::Cyan;
    }
    
    // Check for other BMS colors
    if r_f > 0.6 && g_f > 0.4 && g_f < 0.7 && b_f < 0.3 {
        return Color::Orange;
    }
    if r_f > 0.7 && g_f < 0.4 && b_f > 0.4 && b_f < 0.7 {
        return Color::Pink;
    }
    if r_f > 0.5 && g_f < 0.4 && b_f > 0.5 {
        return Color::Purple;
    }
    
    // Default to closest BMS color
    closest_bms_color(r, g, b)
}

/// Find the closest BMS color using Euclidean distance in RGB space
fn closest_bms_color(r: u8, g: u8, b: u8) -> Color {
    let bms_colors = [
        (Color::Red, (255, 0, 0)),
        (Color::Green, (0, 255, 0)),
        (Color::Blue, (0, 0, 255)),
        (Color::Yellow, (255, 255, 0)),
        (Color::Cyan, (0, 255, 255)),
        (Color::Magenta, (255, 0, 255)),
        (Color::White, (255, 255, 255)),
        (Color::Black, (0, 0, 0)),
        (Color::Gray, (128, 128, 128)),
        (Color::Turquoise, (64, 224, 208)),
        (Color::Pink, (255, 192, 203)),
        (Color::Orange, (255, 165, 0)),
        (Color::Purple, (128, 0, 128)),
    ];
    
    let _target = (r as i32, g as i32, b as i32);
    
    bms_colors.iter()
        .map(|(color, (cr, cg, cb))| {
            let dr = (r as i32 - *cr as i32).pow(2);
            let dg = (g as i32 - *cg as i32).pow(2);
            let db = (b as i32 - *cb as i32).pow(2);
            (color, dr + dg + db)
        })
        .min_by_key(|(_, distance)| *distance)
        .map(|(color, _)| color.clone())
        .unwrap_or(Color::Default)
}

/// Convert string color name to our Color enum
pub fn color_name_to_color(color_name: &str) -> Color {
    match color_name.to_uppercase().as_str() {
        "RED" => Color::Red,
        "GREEN" => Color::Green,
        "BLUE" => Color::Blue,
        "YELLOW" => Color::Yellow,
        "CYAN" => Color::Cyan,
        "MAGENTA" => Color::Magenta,
        "WHITE" => Color::White,
        "BLACK" => Color::Black,
        "GRAY" | "GREY" => Color::Gray,
        "TURQUOISE" => Color::Turquoise,
        "PINK" => Color::Pink,
        "ORANGE" => Color::Orange,
        "PURPLE" => Color::Purple,
        "DEFAULT" => Color::Default,
        _ => Color::Default,
    }
}

/// Convert our Color enum to string for serialization
pub fn color_to_string(color: Color) -> String {
    color.as_str().to_uppercase()
}

/// Enhanced ASCII art generation with type-safe colors
pub fn create_ascii_art_from_image<P: AsRef<Path>>(
    image_path: P,
    target_width: u32,
    target_height: Option<u32>,
    config: AsciiArtConfig,
) -> Result<AsciiArt, anyhow::Error> {
    let img = load_image(image_path)?;
    create_ascii_art_from_dynamic_image(&img, target_width, target_height, &config)
}

/// Create ASCII art from a dynamic image
pub fn create_ascii_art_from_dynamic_image(
    img: &DynamicImage,
    target_width: u32,
    target_height: Option<u32>,
    config: &AsciiArtConfig,
) -> Result<AsciiArt, anyhow::Error> {
    // Convert to RGB/RGBA for processing
    let rgba_img = img.to_rgba8();
    let width = rgba_img.width();
    let height = rgba_img.height();
    
    // Calculate aspect-ratio-preserved dimensions
    let aspect_ratio = width as f32 / height as f32;
    let target_width = target_width.max(1);
    let target_height = target_height.unwrap_or((target_width as f32 / aspect_ratio) as u32).max(1);
    
    // Resize to target dimensions
    let resized = image::imageops::resize(
        &rgba_img,
        target_width,
        target_height,
        image::imageops::FilterType::Lanczos3,
    );
    
    let chars: Vec<char> = config.charset.chars().collect();
    let char_count = chars.len().max(1) as f32;
    
    // Convert each pixel to ASCII character with color
    let mut data = Vec::new();
    
    for y in 0..resized.height() {
        let mut row = Vec::new();
        for x in 0..resized.width() {
            let pixel = resized.get_pixel(x, y);
            let Rgba([r, g, b, a]) = pixel;
            
            // Calculate brightness (perceived luminance)
            let brightness = (0.299 * *r as f32 + 0.587 * *g as f32 + 0.114 * *b as f32) / 255.0;
            
            // Invert brightness if configured
            let brightness = if config.invert_brightness {
                1.0 - brightness
            } else {
                brightness
            };
            
            // Map brightness to character from charset
            let char_index = (brightness * char_count) as usize;
            let char = chars.get(char_index.min(chars.len() - 1)).copied().unwrap_or(' ');
            
            // Determine color based on configuration
            let color = if config.use_color {
                match config.color_mapper {
                    ColorMapper::BmsColors => {
                        let color = rgb_to_bms_color(*r, *g, *b, *a);
                        Some(color_to_string(color))
                    }
                    ColorMapper::AllColors => {
                        // Use the string-based color name from the original function
                        Some(rgb_to_color_name(*r, *g, *b))
                    }
                    ColorMapper::Grayscale => {
                        let gray_value = (brightness * 255.0) as u8;
                        if gray_value < 30 {
                            Some("BLACK".to_string())
                        } else if gray_value > 220 {
                            Some("WHITE".to_string())
                        } else {
                            Some("GRAY".to_string())
                        }
                    }
                    ColorMapper::None => None,
                }
            } else {
                None
            };
            
            row.push(AsciiArtChar { character: char, color });
        }
        data.push(row);
    }
    
    Ok(AsciiArt {
        width: target_width as u16,
        height: target_height as u16,
        data,
    })
}

/// Load an image file using the image crate
fn load_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage, anyhow::Error> {
    let img = image::ImageReader::open(path)?.decode()?;
    Ok(img)
}

/// Convert RGB values to color name string (original function from image_to_ascii.rs)
pub fn rgb_to_color_name(r: u8, g: u8, b: u8) -> String {
    // Simple threshold-based color detection
    let r_f = r as f32 / 255.0;
    let g_f = g as f32 / 255.0;
    let b_f = b as f32 / 255.0;
    
    // Check for grayscale/black/white
    if (r as i32 - g as i32).abs() < 30 && (g as i32 - b as i32).abs() < 30 {
        if r < 30 {
            return "BLACK".to_string();
        } else if r > 220 {
            return "WHITE".to_string();
        } else {
            return "GRAY".to_string();
        }
    }
    
    // Check for primary colors
    if r_f > 0.8 && g_f < 0.3 && b_f < 0.3 {
        return "RED".to_string();
    }
    if g_f > 0.8 && r_f < 0.3 && b_f < 0.3 {
        return "GREEN".to_string();
    }
    if b_f > 0.8 && r_f < 0.3 && g_f < 0.3 {
        return "BLUE".to_string();
    }
    
    // Check for secondary colors
    if r_f > 0.7 && g_f > 0.7 && b_f < 0.3 {
        return "YELLOW".to_string();
    }
    if r_f > 0.7 && b_f > 0.7 && g_f < 0.3 {
        return "MAGENTA".to_string();
    }
    if g_f > 0.7 && b_f > 0.7 && r_f < 0.3 {
        return "CYAN".to_string();
    }
    
    // Check for other colors
    if r_f > 0.6 && g_f > 0.4 && g_f < 0.7 && b_f < 0.3 {
        return "ORANGE".to_string();
    }
    if r_f > 0.7 && g_f < 0.4 && b_f > 0.4 && b_f < 0.7 {
        return "PINK".to_string();
    }
    if r_f > 0.5 && g_f < 0.4 && b_f > 0.5 {
        return "PURPLE".to_string();
    }
    
    // Default to closest match
    let distances = [
        ("RED", (r_f - 1.0).powi(2) + g_f.powi(2) + b_f.powi(2)),
        ("GREEN", r_f.powi(2) + (g_f - 1.0).powi(2) + b_f.powi(2)),
        ("BLUE", r_f.powi(2) + g_f.powi(2) + (b_f - 1.0).powi(2)),
        ("YELLOW", (r_f - 1.0).powi(2) + (g_f - 1.0).powi(2) + b_f.powi(2)),
        ("CYAN", r_f.powi(2) + (g_f - 1.0).powi(2) + (b_f - 1.0).powi(2)),
        ("MAGENTA", (r_f - 1.0).powi(2) + g_f.powi(2) + (b_f - 1.0).powi(2)),
        ("WHITE", (r_f - 1.0).powi(2) + (g_f - 1.0).powi(2) + (b_f - 1.0).powi(2)),
        ("BLACK", r_f.powi(2) + g_f.powi(2) + b_f.powi(2)),
    ];
    
    distances
        .into_iter()
        .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
        .map(|(name, _)| name.to_string())
        .unwrap_or_else(|| "WHITE".to_string())
}

/// ASCII Art templates for common patterns
pub struct AsciiArtTemplates;

impl AsciiArtTemplates {
    /// Create a simple box
    pub fn create_box(width: u16, height: u16, border_char: char, fill_char: char, color: Option<&str>) -> AsciiArt {
        let mut data = Vec::new();
        let color_str = color.map(|s| s.to_uppercase()).unwrap_or_else(|| "WHITE".to_string());
        
        // Top border
        let mut top_row = Vec::new();
        for _ in 0..width {
            top_row.push(AsciiArtChar { character: border_char, color: Some(color_str.clone()) });
        }
        data.push(top_row);
        
        // Middle rows
        for _ in 1..height-1 {
            let mut row = Vec::new();
            row.push(AsciiArtChar { character: border_char, color: Some(color_str.clone()) });
            for _ in 1..width-1 {
                row.push(AsciiArtChar { character: fill_char, color: Some(color_str.clone()) });
            }
            row.push(AsciiArtChar { character: border_char, color: Some(color_str.clone()) });
            data.push(row);
        }
        
        // Bottom border (if height > 1)
        if height > 1 {
            let mut bottom_row = Vec::new();
            for _ in 0..width {
                bottom_row.push(AsciiArtChar { character: border_char, color: Some(color_str.clone()) });
            }
            data.push(bottom_row);
        }
        
        AsciiArt { width, height, data }
    }
    
    /// Create a simple line
    pub fn line(length: u16, line_char: char, color: Option<&str>) -> AsciiArt {
        let mut data = Vec::new();
        let color_str = color.map(|s| s.to_uppercase()).unwrap_or_else(|| "WHITE".to_string());
        
        let mut row = Vec::new();
        for _ in 0..length {
            row.push(AsciiArtChar { character: line_char, color: Some(color_str.clone()) });
        }
        data.push(row);
        
        AsciiArt { width: length, height: 1, data }
    }
    
    /// Create centered text
    pub fn centered_text(text: &str, width: u16, color: Option<&str>) -> AsciiArt {
        let mut data = Vec::new();
        let color_str = color.map(|s| s.to_uppercase()).unwrap_or_else(|| "WHITE".to_string());
        
        let mut row = Vec::new();
        let text_len = text.len() as u16;
        let padding = (width.saturating_sub(text_len)) / 2;
        
        // Left padding
        for _ in 0..padding {
            row.push(AsciiArtChar { character: ' ', color: Some(color_str.clone()) });
        }
        
        // Text
        for c in text.chars() {
            row.push(AsciiArtChar { character: c, color: Some(color_str.clone()) });
        }
        
        // Right padding
        let remaining = width.saturating_sub(padding + text_len);
        for _ in 0..remaining {
            row.push(AsciiArtChar { character: ' ', color: Some(color_str.clone()) });
        }
        
        data.push(row);
        
        AsciiArt { width, height: 1, data }
    }
    
    /// Create a title bar
    pub fn title_bar(title: &str, width: u16, fill_char: char, color: Option<&str>) -> AsciiArt {
        let color_str = color.map(|s| s.to_uppercase()).unwrap_or_else(|| "WHITE".to_string());
        let mut data = Vec::new();
        
        // Create the title bar
        let mut row = Vec::new();
        
        // Left border
        row.push(AsciiArtChar { character: '╔', color: Some(color_str.clone()) });
        
        // Fill and title
        let available_width = width.saturating_sub(2); // Subtract borders
        let title_len = title.len() as u16;
        let left_padding = (available_width.saturating_sub(title_len)) / 2;
        
        // Left fill
        for _ in 0..left_padding {
            row.push(AsciiArtChar { character: fill_char, color: Some(color_str.clone()) });
        }
        
        // Title
        for c in title.chars() {
            row.push(AsciiArtChar { character: c, color: Some(color_str.clone()) });
        }
        
        // Right fill
        let right_padding = available_width.saturating_sub(left_padding + title_len);
        for _ in 0..right_padding {
            row.push(AsciiArtChar { character: fill_char, color: Some(color_str.clone()) });
        }
        
        // Right border
        row.push(AsciiArtChar { character: '╗', color: Some(color_str.clone()) });
        
        data.push(row);
        
        AsciiArt { width, height: 1, data }
    }
}

/// ASCII Art utility functions
pub struct AsciiArtUtils;

impl AsciiArtUtils {
    /// Flip ASCII art horizontally
    pub fn flip_horizontal(ascii_art: &AsciiArt) -> AsciiArt {
        let mut data = Vec::new();
        
        for row in &ascii_art.data {
            let mut new_row: Vec<AsciiArtChar> = row.iter().rev().cloned().collect();
            // Ensure the row maintains the correct width
            while new_row.len() < ascii_art.width as usize {
                new_row.push(AsciiArtChar { character: ' ', color: None });
            }
            data.push(new_row);
        }
        
        AsciiArt {
            width: ascii_art.width,
            height: ascii_art.height,
            data,
        }
    }
    
    /// Flip ASCII art vertically
    pub fn flip_vertical(ascii_art: &AsciiArt) -> AsciiArt {
        let data: Vec<Vec<AsciiArtChar>> = ascii_art.data.iter().rev().cloned().collect();
        
        AsciiArt {
            width: ascii_art.width,
            height: ascii_art.height,
            data,
        }
    }
    
    /// Rotate ASCII art 90 degrees clockwise
    pub fn rotate_90_clockwise(ascii_art: &AsciiArt) -> AsciiArt {
        let mut data = Vec::new();
        
        for x in 0..ascii_art.width {
            let mut row = Vec::new();
            for y in (0..ascii_art.height).rev() {
                if y < ascii_art.data.len() as u16 && x < ascii_art.data[y as usize].len() as u16 {
                    row.push(ascii_art.data[y as usize][x as usize].clone());
                } else {
                    row.push(AsciiArtChar { character: ' ', color: None });
                }
            }
            data.push(row);
        }
        
        AsciiArt {
            width: ascii_art.height,
            height: ascii_art.width,
            data,
        }
    }
    
    /// Convert ASCII art to plain text (without colors)
    pub fn to_plain_text(ascii_art: &AsciiArt) -> String {
        let mut result = String::new();
        
        for row in &ascii_art.data {
            for char_data in row {
                result.push(char_data.character);
            }
            result.push('\n');
        }
        
        // Remove trailing newline if not empty
        if !result.is_empty() {
            result.pop();
        }
        
        result
    }
    
    /// Convert ASCII art to colored text (with ANSI color codes)
    pub fn to_colored_text(ascii_art: &AsciiArt) -> String {
        let mut result = String::new();
        
        for row in &ascii_art.data {
            for char_data in row {
                if let Some(color_str) = &char_data.color {
                    let ansi_code = AsciiArtUtils::color_name_to_ansi(color_str);
                    result.push_str(&ansi_code);
                    result.push(char_data.character);
                    // Reset to default
                    result.push_str("\x1b[0m");
                } else {
                    result.push(char_data.character);
                }
            }
            result.push('\n');
        }
        
        // Remove trailing newline if not empty
        if !result.is_empty() {
            result.pop();
        }
        
        result
    }
    
    /// Convert color name to ANSI escape code
    pub fn color_name_to_ansi(color_name: &str) -> String {
        match color_name.to_uppercase().as_str() {
            "BLACK" => "\x1b[30m",
            "RED" => "\x1b[31m",
            "GREEN" => "\x1b[32m",
            "YELLOW" => "\x1b[33m",
            "BLUE" => "\x1b[34m",
            "MAGENTA" => "\x1b[35m",
            "CYAN" => "\x1b[36m",
            "WHITE" => "\x1b[37m",
            "GRAY" | "GREY" => "\x1b[90m",
            "BRIGHT_RED" => "\x1b[91m",
            "BRIGHT_GREEN" => "\x1b[92m",
            "BRIGHT_YELLOW" => "\x1b[93m",
            "BRIGHT_BLUE" => "\x1b[94m",
            "BRIGHT_MAGENTA" => "\x1b[95m",
            "BRIGHT_CYAN" => "\x1b[96m",
            "BRIGHT_WHITE" => "\x1b[97m",
            _ => "\x1b[0m", // Default/Reset
        }.to_string()
    }
}

/// Wrapper for the original image_to_ascii functions that integrates with our type system
pub mod legacy {
    use super::*;
    use crate::bms::image_to_ascii as original_image_to_ascii;
    
    /// Convert an image file to ASCII art with color information
    /// This is a wrapper around the original image_to_ascii_simple function
    pub fn image_to_ascii<P: AsRef<Path>>(
        image_path: P,
        target_width: u32,
        target_height: Option<u32>,
    ) -> Result<AsciiArt, anyhow::Error> {
        original_image_to_ascii::image_to_ascii_simple(image_path, target_width, target_height)
    }
    
    /// Convert an image file to ASCII art with color information (alias)
    pub fn image_to_ascii_simple<P: AsRef<Path>>(
        image_path: P,
        target_width: u32,
        target_height: Option<u32>,
    ) -> Result<AsciiArt, anyhow::Error> {
        original_image_to_ascii::image_to_ascii_simple(image_path, target_width, target_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_color_conversions() {
        assert_eq!(color_name_to_color("RED"), Color::Red);
        assert_eq!(color_name_to_color("GREEN"), Color::Green);
        assert_eq!(color_name_to_color("BLUE"), Color::Blue);
        assert_eq!(color_name_to_color("unknown"), Color::Default);
        
        assert_eq!(color_to_string(Color::Red), "RED");
        assert_eq!(color_to_string(Color::Green), "GREEN");
    }
    
    #[test]
    fn test_rgb_to_bms_color() {
        assert_eq!(rgb_to_bms_color(255, 0, 0, 255), Color::Red);
        assert_eq!(rgb_to_bms_color(0, 255, 0, 255), Color::Green);
        assert_eq!(rgb_to_bms_color(0, 0, 255, 255), Color::Blue);
        assert_eq!(rgb_to_bms_color(255, 255, 255, 255), Color::White);
        assert_eq!(rgb_to_bms_color(0, 0, 0, 255), Color::Black);
    }
    
    #[test]
    fn test_ascii_art_config() {
        let config = AsciiArtConfig::default();
        assert!(config.use_color);
        assert!(!config.invert_brightness);
        assert_eq!(config.color_mapper, ColorMapper::BmsColors);
        
        let custom_config = AsciiArtConfig::with_charset(".:-=+*#%@");
        assert_eq!(custom_config.charset, ".:-=+*#%@");
    }
    
    #[test]
    fn test_ascii_art_templates() {
        let box_art = AsciiArtTemplates::create_box(10, 5, '#', ' ', Some("RED"));
        assert_eq!(box_art.width, 10);
        assert_eq!(box_art.height, 5);
        
        let line_art = AsciiArtTemplates::line(20, '=', Some("BLUE"));
        assert_eq!(line_art.width, 20);
        assert_eq!(line_art.height, 1);
        
        let text_art = AsciiArtTemplates::centered_text("Hello", 10, Some("GREEN"));
        assert_eq!(text_art.width, 10);
        assert_eq!(text_art.height, 1);
    }
    
    #[test]
    fn test_ascii_art_utils() {
        let original = AsciiArtTemplates::centered_text("Test", 10, Some("WHITE"));
        let flipped = AsciiArtUtils::flip_horizontal(&original);
        assert_eq!(flipped.width, original.width);
        assert_eq!(flipped.height, original.height);
        
        let rotated = AsciiArtUtils::rotate_90_clockwise(&original);
        assert_eq!(rotated.width, original.height);
        assert_eq!(rotated.height, original.width);
    }
}