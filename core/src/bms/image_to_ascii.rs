//! Image to ASCII Art conversion module
//!
//! This module provides functionality to convert image files (PNG, JPEG, TIFF, etc.)
//! to ASCII art with color information for use in BMS fields.

use std::path::Path;
use image::ImageReader;
use image::{DynamicImage, Rgba};

use crate::model::{AsciiArt, AsciiArtChar};

/// Convert an image file to ASCII art with color information
/// 
/// Supported formats: PNG, JPEG, TIFF, GIF, BMP, and other formats supported by the image crate
/// 
/// # Arguments
/// * `image_path` - Path to the image file
/// * `target_width` - Desired width in characters
/// * `target_height` - Optional desired height in characters (maintains aspect ratio if None)
/// 
/// # Returns
/// `AsciiArt` struct containing the ASCII art data with color information
pub fn image_to_ascii<P: AsRef<Path>>(
    image_path: P,
    target_width: u32,
    target_height: Option<u32>,
) -> Result<AsciiArt, anyhow::Error> {
    let img = load_image(image_path)?;
    image_to_ascii_internal(&img, target_width, target_height)
}

/// Convert an image file to ASCII art with color information
/// Alias for image_to_ascii for backward compatibility
pub fn image_to_ascii_simple<P: AsRef<Path>>(
    image_path: P,
    target_width: u32,
    target_height: Option<u32>,
) -> Result<AsciiArt, anyhow::Error> {
    image_to_ascii(image_path, target_width, target_height)
}

/// Load an image file using the image crate
fn load_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage, anyhow::Error> {
    let img = ImageReader::open(path)?.decode()?;
    Ok(img)
}

/// Convert a loaded image to ASCII art
fn image_to_ascii_internal(
    img: &DynamicImage,
    target_width: u32,
    target_height: Option<u32>,
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
    
    // Convert each pixel to ASCII character with color
    let mut data = Vec::new();
    
    for y in 0..resized.height() {
        let mut row = Vec::new();
        for x in 0..resized.width() {
            let pixel = resized.get_pixel(x, y);
            let (char, color) = pixel_to_ascii_char(*pixel);
            row.push(AsciiArtChar { character: char, color: Some(color) });
        }
        data.push(row);
    }
    
    Ok(AsciiArt {
        width: target_width as u16,
        height: target_height as u16,
        data,
    })
}

/// Convert a single RGBA pixel to ASCII character and color name
fn pixel_to_ascii_char(pixel: Rgba<u8>) -> (char, String) {
    let Rgba([r, g, b, a]) = pixel;
    
    if a == 0 {
        return (' ', "WHITE".to_string());
    }
    
    // Calculate brightness (perceived luminance)
    let brightness = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
    
    // Map brightness to ASCII characters (darker to lighter)
    let char = match brightness {
        b if b < 0.05 => ' ',
        b if b < 0.10 => '.',
        b if b < 0.15 => ':',
        b if b < 0.20 => ',',
        b if b < 0.25 => ';',
        b if b < 0.30 => 'i',
        b if b < 0.35 => 'l',
        b if b < 0.40 => '!',
        b if b < 0.45 => 'I',
        b if b < 0.50 => '>',
        b if b < 0.55 => '<',
        b if b < 0.60 => '~',
        b if b < 0.65 => '+',
        b if b < 0.70 => '_',
        b if b < 0.75 => '-',
        b if b < 0.80 => '?',
        b if b < 0.85 => '7',
        b if b < 0.90 => 'L',
        b if b < 0.95 => 'F',
        _ => '@',
    };
    
    // Determine color based on RGB values
    let color = rgb_to_color_name(r, g, b);
    
    (char, color)
}

/// Convert RGB values to the nearest BMS color name
fn rgb_to_color_name(r: u8, g: u8, b: u8) -> String {
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

/// Convert image to ASCII art with custom character set
pub fn image_to_ascii_with_charset<P: AsRef<Path>>(
    image_path: P,
    target_width: u32,
    target_height: Option<u32>,
    charset: &str,
) -> Result<AsciiArt, anyhow::Error> {
    let img = load_image(image_path)?;
    image_to_ascii_with_charset_internal(&img, target_width, target_height, charset)
}

/// Convert with custom character set
fn image_to_ascii_with_charset_internal(
    img: &DynamicImage,
    target_width: u32,
    target_height: Option<u32>,
    charset: &str,
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
    
    // Create brightness to character mapping
    let chars: Vec<char> = charset.chars().collect();
    let char_count = chars.len().max(1) as f32;
    
    // Convert each pixel to ASCII character with color
    let mut data = Vec::new();
    
    for y in 0..resized.height() {
        let mut row = Vec::new();
        for x in 0..resized.width() {
            let pixel = resized.get_pixel(x, y);
            let Rgba([r, g, b, a]) = pixel;
            
            if *a == 0 {
                row.push(AsciiArtChar { character: ' ', color: Some("WHITE".to_string()) });
                continue;
            }
            
            // Calculate brightness (perceived luminance)
            let brightness = (0.299 * *r as f32 + 0.587 * *g as f32 + 0.114 * *b as f32) / 255.0;
            
            // Map brightness to character from charset
            let char_index = (brightness * char_count) as usize;
            let char = chars.get(char_index.min(chars.len() - 1)).copied().unwrap_or(' ');
            
            // Determine color
            let color = rgb_to_color_name(*r, *g, *b);
            
            row.push(AsciiArtChar { character: char, color: Some(color) });
        }
        data.push(row);
    }
    
    Ok(AsciiArt {
        width: target_width as u16,
        height: target_height as u16,
        data,
    })
}

/// Create a simple ASCII art from text (for testing and simple cases)
pub fn text_to_ascii(text: &str, color: Option<&str>) -> AsciiArt {
    let lines: Vec<&str> = text.lines().collect();
    let height = lines.len() as u16;
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
    
    let mut data = Vec::new();
    
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            let color_str = color.map(|s| s.to_uppercase()).unwrap_or_else(|| "WHITE".to_string());
            row.push(AsciiArtChar { character: c, color: Some(color_str) });
        }
        // Pad to width
        while row.len() < width as usize {
            row.push(AsciiArtChar { character: ' ', color: Some("WHITE".to_string()) });
        }
        data.push(row);
    }
    
    AsciiArt { width, height, data }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_to_ascii() {
        let text = "HELLO\nWORLD";
        let ascii = text_to_ascii(text, Some("RED"));
        
        assert_eq!(ascii.width, 5);
        assert_eq!(ascii.height, 2);
        assert_eq!(ascii.data.len(), 2);
        assert_eq!(ascii.data[0].len(), 5);
        assert_eq!(ascii.data[1].len(), 5);
    }

    #[test]
    fn test_rgb_to_color_name() {
        assert_eq!(rgb_to_color_name(255, 0, 0), "RED");
        assert_eq!(rgb_to_color_name(0, 255, 0), "GREEN");
        assert_eq!(rgb_to_color_name(0, 0, 255), "BLUE");
        assert_eq!(rgb_to_color_name(255, 255, 0), "YELLOW");
        assert_eq!(rgb_to_color_name(255, 0, 255), "MAGENTA");
        assert_eq!(rgb_to_color_name(0, 255, 255), "CYAN");
        assert_eq!(rgb_to_color_name(0, 0, 0), "BLACK");
        assert_eq!(rgb_to_color_name(255, 255, 255), "WHITE");
    }
}
