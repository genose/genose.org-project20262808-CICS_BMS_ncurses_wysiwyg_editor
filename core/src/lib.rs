pub mod bms;

pub use bms::model::{BmsField, BmsMap, BmsMapSet, FieldType, FieldAttribute, Color};
pub use bms::parser::{parse_bms, parse_bms_file, BmsParseError};
pub use bms::generator::{generate_cobol, render_bms_text, render_bms_html};
pub use bms::editor::{BmsEditor, EditHistory, EditOperation, EditorMode, CursorDirection, ResizeDirection, create_default_map, create_preset_fields};
pub use bms::templates::{BmsTemplate, get_all_templates, get_template_by_name, get_template_names, create_editor_from_template};
pub use bms::binary_parser::{parse_bms_binary, parse_bms_binary_from_bytes, detect_bms_file_type, is_bms_binary_file, BmsBinaryParseError};

// Re-export modules for direct access
pub use bms::model;
pub use bms::templates;
pub use bms::binary_parser;
