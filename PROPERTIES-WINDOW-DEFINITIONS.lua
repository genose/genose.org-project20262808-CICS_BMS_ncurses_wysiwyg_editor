-- ===========================================================
-- PROPERTIES-WINDOW-DEFINITIONS.lua
-- Properties window definitions
-- ===========================================================
-- A Object type embedded in the editor window, to edit the properties of the selected object, with a list of available fields to add, and a list of existing fields to edit or delete
-- each field has a type, a name, a value, and a set of properties (default, initial, edited) to manage the state of the field in the GUI
-- each field has a set of properties (default, initial, edited) to manage the state of the field in the GUI
-- each TYPE OBJECT has a set of available fields, each field has a type, a name, a value, and a set of properties (default, initial, edited) to manage the state of the field in the GUI
-- to render the properties window, we use a template visual, combining the properties of the fields according to the conditions
-- here exemples of available fields for each TYPE OBJECT, with their default properties, to be used in the properties window

-- Check if OBJECTS_DEFINITIONS is loaded
if not OBJECTS_DEFINITIONS then
    dofile("OBJECTS-DEFINITIONS.lua")
end

-- Example field_border definition
local field_border = {
    Field = {style = OBJECTS_DEFINITIONS.field_avail_border_style.default.Field, chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default.Field}
}

-- ===========================================================
-- GUI Render notes:
-- border style : [ ... |v]
-- border chars : [ ... |v]
-- ===========================================================