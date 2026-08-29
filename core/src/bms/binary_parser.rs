//! Binary parser for BMS load modules
//!
//! This module provides functionality to parse BMS load modules (compiled binary format).
//! BMS load modules have a specific binary structure that contains the map definition
//! in a compiled form.

use crate::bms::model::*;
use std::fs;
use std::io::Read;
use thiserror::Error;

/// Error type for binary BMS parsing
#[derive(Error, Debug)]
pub enum BmsBinaryParseError {
    #[error("Failed to read binary file: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid BMS load module format: {0}")]
    FormatError(String),
    
    #[error("Unsupported BMS version: {0}")]
    VersionError(u8),
}

/// Magic number for BMS load modules (EBCDIC encoding of 'BMS')
/// In EBCDIC: B=0xC2, M=0xD4, S=0xE2
const BMS_MAGIC_EBCDIC: [u8; 3] = [0xC2, 0xD4, 0xE2];

/// Magic number for BMS load modules (ASCII encoding - some variants)
const BMS_MAGIC_ASCII: [u8; 3] = [b'B', b'M', b'S'];

/// BMS load module header structure
#[derive(Debug)]
struct BmsLoadHeader {
    /// Magic number identifying this as a BMS load module
    magic: [u8; 3],
    /// Version of the BMS format
    version: u8,
    /// Length of the header
    header_length: u16,
    /// Total length of the load module
    total_length: u32,
    /// Map name (8 bytes, EBCDIC or ASCII)
    map_name: [u8; 8],
    /// Mapset name (8 bytes)
    mapset_name: [u8; 8],
    /// Map type flags
    map_type: u8,
    /// Reserved bytes
    reserved: [u8; 3],
}

impl BmsLoadHeader {
    fn from_bytes(data: &[u8]) -> Result<Self, BmsBinaryParseError> {
        if data.len() < 28 {
            return Err(BmsBinaryParseError::FormatError(
                "Header too short".to_string()
            ));
        }
        
        let magic: [u8; 3] = [data[0], data[1], data[2]];
        
        // Check magic number (EBCDIC or ASCII)
        if magic != BMS_MAGIC_EBCDIC && magic != BMS_MAGIC_ASCII {
            return Err(BmsBinaryParseError::FormatError(
                "Invalid magic number".to_string()
            ));
        }
        
        let version = data[3];
        if version != 0x01 && version != 0x02 {
            return Err(BmsBinaryParseError::VersionError(version));
        }
        
        // Note: BMS load modules use big-endian (EBCDIC) or little-endian (ASCII) encoding
        // For simplicity, we'll assume big-endian for now
        let header_length = u16::from_be_bytes([data[4], data[5]]);
        let total_length = u32::from_be_bytes([data[6], data[7], data[8], data[9]]);
        
        let mut map_name = [0u8; 8];
        map_name.copy_from_slice(&data[10..18]);
        
        let mut mapset_name = [0u8; 8];
        mapset_name.copy_from_slice(&data[18..26]);
        
        let map_type = data[26];
        let reserved = [data[27], data[28], data[29]];
        
        Ok(Self {
            magic,
            version,
            header_length,
            total_length,
            map_name,
            mapset_name,
            map_type,
            reserved,
        })
    }
    
    /// Decode map name from EBCDIC or ASCII
    fn decode_map_name(&self) -> String {
        decode_ebcdic_or_ascii(&self.map_name)
    }
    
    /// Decode mapset name from EBCDIC or ASCII
    fn decode_mapset_name(&self) -> String {
        decode_ebcdic_or_ascii(&self.mapset_name)
    }
}

/// Decode a byte array that could be EBCDIC or ASCII
fn decode_ebcdic_or_ascii(bytes: &[u8]) -> String {
    // Check if all bytes are valid ASCII (excluding nulls)
    let all_ascii = bytes.iter().all(|&b| b == 0 || (b >= 32 && b <= 126));
    
    if all_ascii {
        // Filter out null bytes and convert to string
        return bytes.iter()
            .filter(|&&b| b != 0)
            .map(|&b| b as char)
            .collect::<String>()
            .trim()
            .to_string();
    }
    
    // Try EBCDIC conversion
    bytes.iter()
        .map(|&b| ebcdic_to_ascii(b))
        .filter(|&c| c != '\x00' && !c.is_control())
        .collect::<String>()
        .trim()
        .to_string()
}

/// Convert EBCDIC byte to ASCII character
/// This is a simplified EBCDIC to ASCII mapping
fn ebcdic_to_ascii(byte: u8) -> char {
    // EBCDIC to ASCII mapping table (simplified)
    // This covers the basic alphanumeric characters
    match byte {
        // A-Z in EBCDIC (0xC1-0xE9)
        0xC1 => 'A', 0xC2 => 'B', 0xC3 => 'C', 0xC4 => 'D', 0xC5 => 'E',
        0xC6 => 'F', 0xC7 => 'G', 0xC8 => 'H', 0xC9 => 'I', 0xD1 => 'J',
        0xD2 => 'K', 0xD3 => 'L', 0xD4 => 'M', 0xD5 => 'N', 0xD6 => 'O',
        0xD7 => 'P', 0xD8 => 'Q', 0xD9 => 'R', 0xE2 => 'S', 0xE3 => 'T',
        0xE4 => 'U', 0xE5 => 'V', 0xE6 => 'W', 0xE7 => 'X', 0xE8 => 'Y',
        0xE9 => 'Z',
        
        // a-z in EBCDIC (0x81-0xA9)
        0x81 => 'a', 0x82 => 'b', 0x83 => 'c', 0x84 => 'd', 0x85 => 'e',
        0x86 => 'f', 0x87 => 'g', 0x88 => 'h', 0x89 => 'i', 0x91 => 'j',
        0x92 => 'k', 0x93 => 'l', 0x94 => 'm', 0x95 => 'n', 0x96 => 'o',
        0x97 => 'p', 0x98 => 'q', 0x99 => 'r', 0xA2 => 's', 0xA3 => 't',
        0xA4 => 'u', 0xA5 => 'v', 0xA6 => 'w', 0xA7 => 'x', 0xA8 => 'y',
        0xA9 => 'z',
        
        // 0-9 in EBCDIC (0xF0-0xF9)
        0xF0 => '0', 0xF1 => '1', 0xF2 => '2', 0xF3 => '3', 0xF4 => '4',
        0xF5 => '5', 0xF6 => '6', 0xF7 => '7', 0xF8 => '8', 0xF9 => '9',
        
        // Space
        0x40 => ' ',
        
        // Default: return space
        _ => ' '
    }
}

/// Field descriptor in binary format
#[derive(Debug)]
struct BmsFieldDescriptor {
    /// Field position (row, col) - 1-indexed
    pos: (u16, u16),
    /// Field length
    length: u16,
    /// Field attributes (bit flags)
    attributes: u16,
    /// Field color
    color: u8,
    /// Field type / flags
    field_type: u8,
    /// Initial value length
    initial_len: u16,
    /// Initial value (variable length)
    initial: Vec<u8>,
}

impl BmsFieldDescriptor {
    fn from_bytes(data: &[u8], offset: usize) -> Result<(Self, usize), BmsBinaryParseError> {
        if offset + 12 > data.len() {
            return Err(BmsBinaryParseError::FormatError(
                "Field descriptor too short".to_string()
            ));
        }
        
        // Position is 2 bytes each (row, col) - big endian
        let row = u16::from_be_bytes([data[offset], data[offset + 1]]);
        let col = u16::from_be_bytes([data[offset + 2], data[offset + 3]]);
        
        // Length is 2 bytes
        let length = u16::from_be_bytes([data[offset + 4], data[offset + 5]]);
        
        // Attributes are 2 bytes of bit flags
        let attributes = u16::from_be_bytes([data[offset + 6], data[offset + 7]]);
        
        // Color is 1 byte
        let color = data[offset + 8];
        
        // Field type is 1 byte
        let field_type = data[offset + 9];
        
        // Initial value length is 2 bytes
        let initial_len = u16::from_be_bytes([data[offset + 10], data[offset + 11]]);
        
        // Initial value follows
        let initial_start = offset + 12;
        let initial_end = initial_start + initial_len as usize;
        
        if initial_end > data.len() {
            return Err(BmsBinaryParseError::FormatError(
                "Initial value exceeds data length".to_string()
            ));
        }
        
        let initial = data[initial_start..initial_end].to_vec();
        
        let field = Self {
            pos: (row, col),
            length,
            attributes,
            color,
            field_type,
            initial_len,
            initial,
        };
        
        Ok((field, initial_end))
    }
    
    fn to_bms_field(&self) -> BmsField {
        let mut field = BmsField::default();
        field.pos = self.pos;
        field.length = self.length;
        
        // Convert attributes bit flags to FieldAttribute enum
        field.attrb = self.decode_attributes();
        
        // Convert color byte to Color enum
        field.color = self.decode_color();
        
        // Convert initial value from EBCDIC to ASCII
        if !self.initial.is_empty() {
            field.initial = Some(self.decode_initial());
        }
        
        // Determine field type
        field.field_type = self.decode_field_type();
        
        field
    }
    
    fn decode_attributes(&self) -> Vec<FieldAttribute> {
        let mut attrs = Vec::new();
        
        // Check bit flags in attributes
        if self.attributes & 0x80 != 0 {
            attrs.push(FieldAttribute::Prot);
        }
        if self.attributes & 0x40 != 0 {
            attrs.push(FieldAttribute::Intens);
        }
        if self.attributes & 0x20 != 0 {
            attrs.push(FieldAttribute::Blink);
        }
        if self.attributes & 0x10 != 0 {
            attrs.push(FieldAttribute::Reverse);
        }
        if self.attributes & 0x08 != 0 {
            attrs.push(FieldAttribute::Dark);
        }
        
        // Check for field type attributes
        if self.attributes & 0x04 != 0 {
            attrs.push(FieldAttribute::Norm);
        }
        if self.attributes & 0x02 != 0 {
            attrs.push(FieldAttribute::Num);
        }
        if self.attributes & 0x01 != 0 {
            attrs.push(FieldAttribute::AlphaNum);
        }
        
        // If no specific type attribute, add Norm
        if attrs.iter().all(|a| !matches!(a, FieldAttribute::Norm | FieldAttribute::Num | FieldAttribute::AlphaNum)) {
            attrs.push(FieldAttribute::Norm);
        }
        
        attrs
    }
    
    fn decode_color(&self) -> Option<Color> {
        match self.color {
            0x00 => None, // Default
            0xF0 => Some(Color::Black),
            0xF1 => Some(Color::Blue),
            0xF2 => Some(Color::Green),
            0xF3 => Some(Color::Cyan),
            0xF4 => Some(Color::Red),
            0xF5 => Some(Color::Magenta),
            0xF6 => Some(Color::Yellow),
            0xF7 => Some(Color::White),
            _ => Some(Color::Default),
        }
    }
    
    fn decode_initial(&self) -> String {
        decode_ebcdic_or_ascii(&self.initial)
    }
    
    fn decode_field_type(&self) -> FieldType {
        match self.field_type {
            0x00 => FieldType::Field,
            0x01 => FieldType::InputOnly,
            0x02 => FieldType::OutputOnly,
            _ => FieldType::Field,
        }
    }
}

/// Parse a BMS load module (binary file)
/// 
/// BMS load modules have the following general structure:
/// 1. Header (28-32 bytes) - magic, version, sizes, map name, mapset
/// 2. Map information block - dimensions, flags, etc.
/// 3. Field descriptors - for each field in the map
///    - Position (2 bytes each for row, col)
///    - Length (2 bytes)
///    - Attributes (2 bytes bit flags)
///    - Color (1 byte)
///    - Type (1 byte)
///    - Initial value length (2 bytes)
///    - Initial value (variable length)
/// 4. Literal data - field initial values and literals
pub fn parse_bms_binary(path: &std::path::Path) -> Result<BmsMap, BmsBinaryParseError> {
    let mut file = fs::File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    
    parse_bms_binary_from_bytes(&data)
}

/// Parse a BMS load module from raw bytes
pub fn parse_bms_binary_from_bytes(data: &[u8]) -> Result<BmsMap, BmsBinaryParseError> {
    // Parse header
    let header = BmsLoadHeader::from_bytes(data)?;
    
    let map_name = header.decode_map_name();
    let mapset_name = header.decode_mapset_name();
    
    // Create map with parsed names
    let mut map = BmsMap::new(&map_name, &mapset_name);
    
    // Parse map dimensions
    // In the header, dimensions might be stored at offset 32
    // For now, use default 24x80 and let the field positions determine the actual size
    map.size = (24, 80);
    
    // Find and parse field descriptors
    // The field descriptors start after the header
    // The exact offset can vary, but typically starts at offset 32 or 64
    
    let header_size = header.header_length as usize;
    let mut offset = header_size.min(data.len());
    
    // Parse fields until we reach the end or encounter an invalid descriptor
    while offset + 12 <= data.len() {
        match BmsFieldDescriptor::from_bytes(data, offset) {
            Ok((field_desc, next_offset)) => {
                // Add the field to the map
                let field = field_desc.to_bms_field();
                
                // Update map dimensions based on field positions
                if field.pos.0 > map.size.0 {
                    map.size.0 = field.pos.0;
                }
                if field.pos.1 + field.length - 1 > map.size.1 {
                    map.size.1 = field.pos.1 + field.length - 1;
                }
                
                map.fields.push(field);
                offset = next_offset;
            }
            Err(_) => {
                // Could not parse field descriptor - might be end of fields or data section
                break;
            }
        }
    }
    
    Ok(map)
}

/// Check if a file is likely a BMS load module (binary format)
/// by checking for the BMS magic number at the start
pub fn is_bms_binary_file(data: &[u8]) -> bool {
    if data.len() < 3 {
        return false;
    }
    
    let magic: [u8; 3] = [data[0], data[1], data[2]];
    magic == BMS_MAGIC_EBCDIC || magic == BMS_MAGIC_ASCII
}

/// Detect if a file is a BMS source text file or binary load module
pub fn detect_bms_file_type(path: &std::path::Path) -> Result<String, BmsBinaryParseError> {
    let mut file = fs::File::open(path)?;
    let mut header = [0u8; 32];
    file.read_exact(&mut header)?;
    
    if is_bms_binary_file(&header) {
        Ok("binary".to_string())
    } else {
        Ok("text".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ebcdic_to_ascii() {
        // Test some known EBCDIC values
        assert_eq!(ebcdic_to_ascii(0xC1), 'A');
        assert_eq!(ebcdic_to_ascii(0xC2), 'B');
        assert_eq!(ebcdic_to_ascii(0xF0), '0');
        assert_eq!(ebcdic_to_ascii(0xF1), '1');
    }
    
    #[test]
    fn test_decode_ebcdic_or_ascii() {
        // Test ASCII
        assert_eq!(decode_ebcdic_or_ascii(b"HELLO"), "HELLO");
        
        // Test with null bytes (EBCDIC padding)
        assert_eq!(decode_ebcdic_or_ascii(b"HEL\x00\x00"), "HEL");
    }
    
    #[test]
    fn test_is_bms_binary_file() {
        // Test with ASCII magic
        let ascii_magic = b"BMS";
        assert!(is_bms_binary_file(ascii_magic));
        
        // Test with EBCDIC magic
        let ebcdic_magic = [0xC2u8, 0xD4, 0xE2];
        assert!(is_bms_binary_file(&ebcdic_magic));
        
        // Test with non-BMS data
        let not_bms = b"ABC";
        assert!(!is_bms_binary_file(not_bms));
    }
    
    #[test]
    fn test_header_from_bytes() {
        // Create a minimal valid header
        let mut data = vec![0u8; 32];
        data[0] = b'B';
        data[1] = b'M';
        data[2] = b'S';
        data[3] = 0x01; // Version 1
        data[4] = 0x00; // Header length (big endian)
        data[5] = 0x20; // 32 bytes
        
        // Map name "TESTMAP"
        data[10..18].copy_from_slice(b"TESTMAP ");
        
        let header = BmsLoadHeader::from_bytes(&data).unwrap();
        assert_eq!(header.version, 1);
        assert_eq!(header.header_length, 32);
        assert_eq!(header.decode_map_name(), "TESTMAP");
    }
}
