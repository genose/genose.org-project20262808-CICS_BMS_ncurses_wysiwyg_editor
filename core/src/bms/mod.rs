pub mod model;
pub mod parser;
pub mod generator;
pub mod editor;
pub mod templates;
pub mod binary_parser;
pub mod image_to_ascii;

// New property-based modules that mirror Lua OBJECTS_DEFINITIONS structure
pub mod properties;
pub mod field_types;
// pub mod types; // Temporarily disabled due to encoding issues
// pub mod defaults; // Temporarily disabled due to dependency on types
// pub mod field; // Temporarily disabled due to dependencies
