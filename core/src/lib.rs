pub mod bms;

pub use bms::model::{BmsField, BmsMap, BmsMapSet, FieldType, FieldAttribute, Color};
pub use bms::parser::{parse_bms, parse_bms_file, BmsParseError};
pub use bms::generator::{generate_cobol, render_bms_text, render_bms_html};
pub use bms::editor::{BmsEditor, EditHistory, EditOperation, EditorMode, CursorDirection, ResizeDirection, create_default_map, create_preset_fields};
pub use bms::templates::{BmsTemplate, get_all_templates, get_template_by_name, get_template_names, TEMPLATE_MENU, TEMPLATE_DATA_ENTRY_FORM, TEMPLATE_LIST, TEMPLATE_CONFIRMATION, TEMPLATE_ERROR, TEMPLATE_LOGIN, TEMPLATE_SEARCH, TEMPLATE_DASHBOARD};

// Re-export templates module for direct access
pub use bms::templates;
