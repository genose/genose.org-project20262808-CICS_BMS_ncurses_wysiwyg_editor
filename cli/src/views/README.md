# Views Module - Directory Structure

This directory is intended to organize the application's UI code into separate, maintainable modules.

## Current State

The views module is being actively extracted from `main.rs`. A new `types.rs` module was created to contain shared types like `FileFilter` that are needed by multiple view modules.

Currently extracted views:

### ✅ Fully Extracted and Integrated:
- `attribute_picker.rs` - Attribute selection dialog
- `color_picker.rs` - Color selection dialog
- `combo_key_help.rs` - Combo key bindings help view
- `confirm.rs` - Confirmation dialog
- `help.rs` - Help view with keyboard shortcuts
- `map_type_picker.rs` - Map type selection dialog
- `open_dialog.rs` - Open file dialog with file browser (NEW)
- `save_dialog.rs` - Save dialog
- `status_bar.rs` - Status bar rendering
- `text_input.rs` - Text input dialog

### 📋 Template/Reference:
- `help.rs` - Template file with improved paging (not yet activated)

### ⏳ Still in main.rs:
- Canvas rendering
- Sidebar
- Properties panel
- Various dialogs (save, open, add_object, image_import)
- Main input handling and mode management

## Intended Structure

```
cli/src/
├── main.rs              # Main application entry point, App struct
├── combo_keys.rs        # Combo key system
└── views/
    ├── mod.rs           # Re-exports all view modules
    ├── canvas.rs        # Main canvas rendering and input handling
    ├── sidebar.rs       # Sidebar rendering and input handling
    ├── properties.rs    # Property panel rendering and input handling
    ├── help.rs          # Help view rendering and input handling
    ├── combo_key_help.rs # Combo key help view
    ├── dialogs/
    │   ├── mod.rs       # Re-exports all dialog modules
    │   ├── save.rs      # Save dialog
    │   ├── open.rs      # Open dialog
    │   ├── confirm.rs   # Confirmation dialog
    │   ├── text_input.rs # Text input dialog
    │   └── ...
    └── status_bar.rs    # Status bar rendering
```

## Migration Plan

To migrate the code from `main.rs` to the views module:

### Step 1: Extract App Struct and Types (DONE - Help view paging fixed)
- The `App` struct and related enums (`AppMode`, `ActivePanel`, etc.) should remain in `main.rs` for now
- Alternatively, create `app.rs` module for these types

### Step 2: Create View Modules
For each view, create a separate file with:
1. All type definitions specific to that view
2. `render_<view>(f: &mut Frame, app: &App, area: Rect)` function
3. `handle_<view>_mode(app: &mut App, key: KeyEvent)` function
4. Any helper functions

### Step 3: Update main.rs
- Remove the extracted functions from main.rs
- Add imports from the views module
- Update function calls to use the module path

## View Identification

The following render and handle functions exist in main.rs and should be extracted:

### Render Functions (30+ found):
- `render_canvas`
- `render_bms_grid`
- `render_bms_text_preview`
- `render_sidebar`
- `render_properties_panel`
- `render_insert_position_dialog` (TO DO - complex dependencies)
- `render_edit_properties_panel` (TO DO - complex dependencies)
- `render_map_type_picker`
- `render_color_picker` (EXTRACTED)
- `render_attribute_picker` (EXTRACTED)
- `render_save_dialog` (EXTRACTED)
- `render_open_dialog` (EXTRACTED)
- `render_add_object_dialog` (TO DO - complex dependencies)
- `render_text_input`
- `render_help` (EXTRACTED)
- `render_combo_key_help`
- `render_confirm`
- `render_image_import_dialog` (TO DO - complex dependencies)
- `render_status_bar`

### Handle Functions (30+ found):
- `handle_input`
- `handle_mouse_input`
- `handle_combo_action`
- `handle_edit_mode`
- `handle_normal_mode`
- `handle_properties_mode`
- `handle_insert_position_mode`
- `handle_edit_properties_mode`
- `handle_map_type_picker_mode`
- `handle_color_picker_mode` (EXTRACTED)
- `handle_attribute_picker_mode` (EXTRACTED)
- `handle_save_dialog_mode` (EXTRACTED)
- `handle_open_dialog_mode` (EXTRACTED)
- `handle_add_object_dialog_mode`
- `handle_text_input_mode`
- `handle_help_mode` (EXTRACTED)
- `handle_combo_key_help_mode`
- `handle_confirm_mode`
- `handle_image_import_mode`

## Example: Help View (Already Partially Fixed)

The help view has been updated with proper paging:
- Bounds checking on scroll position
- Proper PageUp/PageDown behavior
- Fixed End key behavior

The help functions are currently at the end of `main.rs` and can be moved to `views/help.rs` when ready.

## Benefits of This Structure

1. **Better Organization**: Each view is in its own file
2. **Easier Navigation**: No more 5000+ line files
3. **Improved Maintainability**: Changes to one view don't affect others
4. **Better Collaboration**: Multiple developers can work on different views
5. **Clearer Dependencies**: Each view's dependencies are explicit
6. **Easier Testing**: Views can be tested in isolation

## Notes

- The `App` struct is central to all views and contains the application state
- Consider using a state management pattern if the app grows more complex
- Some views may need to share helper functions - create utility modules as needed
