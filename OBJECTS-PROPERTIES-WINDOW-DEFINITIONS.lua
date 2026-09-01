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
-- the peroperties window is rendered according to the field's type and its properties, with the appropriate visual representation for each field type, and the adapted values of the existing fields in the properties window are updated accordingly in .initial and .edited,
-- graphicly rendered as a "Collapsible Object Fieldset" marked by [+] or [-] (using height.min and accorded content height) for each TYPE OBJECT, with the available fields and their default properties, to be used in the properties window
-- This file defines how the properties window should be structured and rendered for each type of object, using the available fields and their properties.
-- rendred like following the "Collapsible Object Fieldset" pattern, with the available fields and their default properties,
-- to be used in the properties window
-- rendering a value depends of gui_field_type, and the rendering is done according to the field's type and its properties,
-- with the appropriate visual representation for each field type, and the adapted values of the existing fields in the properties window are updated accordingly in .initial and .edited,
-- "Window Properties" always list properties from .default with selected values from order of precedence: .edited | .initial | .default
-- mechanism of insert object is "Add Field", showing "Window Properties" with the available fields from .default for the selected object type, and the existing values of the Object.
-- from action "Add Field" initialize the field's value from .default, and replicate value to .initial and .edited, by replacing the value of the field.
-- from action "Edit Field" on seleted Object, show value from .initial, and replicate value to .edited, by replacing the value of the field.
-- for selected object, the properties window will display the fields according to the order of precedence: .edited | .initial | .default
-- ***********************************************************
-- | ========================= | Object Properties (mode/action: Add Field | Edit Field) | ==================================  |
-- | | ==[+] ======== | (marked field N <name>) field_(name) | ========================================  |     |
-- | | .default[TYPE].(props key P <name>) : [ .... props values (Select String/ Select Numeric) .... |v] |    |     |
-- | | .default[TYPE].(props key P <name>) : [ .... props values (Text String/ Text Numeric) .... ] |    |     |
-- | | .default[TYPE].(props key P <name>) : [ .... props values (boolean) .... ] |    |     |
-- | | ... P props fields ...
-- | | ===========================================================  | |
-- |  ...N marked fields ...
-- |  ==============================================================  |
-- ***********************************************************
-- | ========================= | Object Properties (mode/action: Add Field | Edit Field) | ==================================  |
-- | | ==[-] ======== | (marked field N <name>) field_(name) | ==============  |     |
-- | | ****************** [ COLLAPSED ] ********************     | |
-- | | ===========================================================  | |
-- |  ...N marked fields ...
-- | ===========================================================  | |
-- ***********************************************************
-- Check if OBJECTS_DEFINITIONS is loaded
if not OBJECTS_DEFINITIONS then
    dofile("OBJECTS-DEFINITIONS.lua")
end

-- ***********************************************************
-- the window properties used to edit the properties of the selected object,
-- with a list of available fields associated to the selected object type, 
-- and a list of existing values of the selected object, to edit or delete them
-- ***********************************************************
-- using OBJECTS_DEFINITIONS, we define the properties window for each TYPE OBJECT,
-- with the available marked fields, and the existing values of the selected object, to edit or delete them
-- the values of the existing fields are adapted according to the selected object's current state and the available fields for its type, to allow editing or deleting them
-- showing the adapted values of the existing fields in the properties window
-- ===========================================================
-- exemple for an OBJECTS possessing a set of available fields, with their default properties, to be used in the properties window
-- here the following exemple show one of the rendering of the properties window for an OBJECTS of type "Field",
-- with the available fields and their default properties, to be used in the properties window
-- the order for rendering value fields in the properties window is .default, .initial, .edited, to allow the user to edit the properties of the selected object, and to show the adapted values of the existing fields in the properties window
-- by selecting the value of the field, the user can edit the properties of the selected object, and the adapted values of the existing fields in the properties window are updated accordingly in .initial and .edited,
-- to allow the user to edit the properties of the selected object, and to show the adapted values of the existing fields in the properties window
-- ***********************************************************
-- GUI Render : 
-- each rendering is based on introspection of the object's fields and their current values having gui_field_type, and the rendering is done according to the field's type and its properties, with the appropriate visual representation for each field type, and the adapted values of the existing fields in the properties window are updated accordingly in .initial and .edited,
-- to allow the user to edit the properties of the selected object, and to show the adapted
--  field_*+gui_field_type, will render in HTML like phylosophy, with the appropriate visual representation for each field type, and the adapted values of the existing fields in the properties window are updated accordingly in .initial and .edited,
-- like as follow for an property "border_style" form "field_border_style", with the appropriate visual representation for each field type, and the adapted values of the existing fields in the properties window are updated accordingly in .initial and .edited,

-- ***********************************************************
test_gui_object = OBJECTS_DEFINITIONS.new(OBJECTS_DEFINITIONS.field_type.enum.Field) -- Crée un objet de type Field, avec .default pointant vers.default[TYPE] et .initial = nil, .edited = nil
-- .default is already defined for the field_border_style of the test_gui_object by .new() constructor, but we can re-assign it to the default value for the specific type of the test_gui_object
-- using the already defined : test_gui_object.field_border_style.gui_field_type for rendering the border style field in the properties window
myPropsToShow = test_gui_object.field_border_style -- Récupère les attributs de border_style du champ
-- using decoration (Fieldset) section is rendered like this in the properties window:
-- ======= | border style | ========
-- | kprops : [ ....kvprops... |v] |
-- | kprops : [ ....kvprops... |v] |
-- =================================
-- **********************************************************

-- =============================================================================
-- PROPERTIES WINDOW RENDERING FUNCTIONS
-- =============================================================================

-- Property category definitions for grouping
local PROPERTY_CATEGORIES = {
    { name = "Dimensions", props = {"field_height", "field_width", "field_min_height", "field_max_height", "field_width_min", "field_width_max"} },
    { name = "Colors", props = {"field_border_color", "field_title_color", "field_text_color", "field_footer_color"} },
    { name = "Borders", props = {"field_border_style", "field_border"} },
    { name = "Alignment", props = {"field_text_align", "field_title_align", "field_footer_align", "field_vertical_align"} },
    { name = "Fill & Style", props = {"field_fill_char", "field_title_fill_char", "field_footer_fill_char", "field_style"} },
    { name = "Markers", props = {"field_required_marker", "field_error_marker", "field_title_prefix", "field_title_suffix"} },
    { name = "Value & Content", props = {"field_initial", "field_name", "field_type"} },
    { name = "Position", props = {"field_pos", "field_avail_pos"} },
    { name = "Attributes", props = {"field_attrb"} }
}

-- Helper to recursively extract a string value from a property table
local function extract_string_value(prop)
    if type(prop) == "string" then return prop end
    if type(prop) == "number" then return tostring(prop) end
    if type(prop) == "boolean" then return prop and "true" or "false" end
    if type(prop) ~= "table" then return "[" .. type(prop) .. "]" end
    
    -- Check for common value fields
    if prop.marker then return extract_string_value(prop.marker) end
    if prop.value then return extract_string_value(prop.value) end
    if prop.initial_value then return extract_string_value(prop.initial_value) end
    if prop.style then return extract_string_value(prop.style) end
    if prop.color then return extract_string_value(prop.color) end
    if prop.fill_char then return extract_string_value(prop.fill_char) end
    
    -- Check for initial/edited
    if prop.initial then return extract_string_value(prop.initial) end
    if prop.edited then return extract_string_value(prop.edited) end
    
    -- Check for enum
    if prop.enum then
        for _, v in pairs(prop.enum) do
            local val = extract_string_value(v)
            if type(val) == "string" and val ~= "[" then return val end
        end
    end
    
    -- Look for any string in the table
    for _, v in pairs(prop) do
        if type(v) == "string" then return v end
        if type(v) == "number" then return tostring(v) end
        if type(v) == "boolean" then return v and "true" or "false" end
    end
    
    -- If we have nested tables, try to extract from them
    for _, v in pairs(prop) do
        if type(v) == "table" then
            local result = extract_string_value(v)
            if result and result ~= "[table]" then return result end
        end
    end
    
    return "[table]"
end

-- Get the GUI-friendly value of a property
local function get_property_value(obj, prop_name)
    local prop = obj[prop_name]
    if not prop then return "nil" end
    
    return extract_string_value(prop)
end

-- Get category for a property
local function get_property_category(prop_name)
    for _, cat in ipairs(PROPERTY_CATEGORIES) do
        for _, p in ipairs(cat.props) do
            if p == prop_name then return cat.name end
        end
    end
    return "Other"
end

-- Render a single property line
local function render_property_line(prop_name, value, width, gui_type)
    local display_name = prop_name:gsub("[_%%]", " "):gsub("^(.)", function(c) return c:upper() end)
    local control_type = "text"
    if gui_type then
        if gui_type:find("select") then control_type = "[Select|v]"
        elseif gui_type:find("checkbox") then control_type = "[ ]"
        elseif gui_type:find("text") then control_type = "[____]"
        end
    end
    
    local line = string.format("  %-25s : %s %-15s", display_name, control_type, tostring(value))
    return line
end

-- Render a category section
local function render_category_section(category_name, props, width)
    local lines = {}
    local border = "═"
    local left = "║"
    local right = "║"
    local corner_tl = "╔"
    local corner_tr = "╗"
    local corner_bl = "╚"
    local corner_br = "╝"
    
    -- Category header
    local header = string.format(" %s %s %s ", corner_tl, category_name, corner_tr)
    table.insert(lines, header)
    
    -- Properties
    for _, prop_info in ipairs(props) do
        local line = render_property_line(prop_info.name, prop_info.value, width, prop_info.gui_type)
        table.insert(lines, left .. line .. right)
    end
    
    -- Category footer
    local footer = corner_bl .. string.rep(border, width) .. corner_br
    table.insert(lines, footer)
    
    return table.concat(lines, "\n")
end

-- Render a collapsible section header
local function render_collapsible_header(obj, section_name, is_expanded)
    local marker = is_expanded and "[-]" or "[+]"
    local header = string.format(" %s %s %s ", marker, section_name, is_expanded and "▼" or "▲")
    return header
end

-- Main function: Render properties window for an object
function render_properties_window(obj, options)
    options = options or {}
    local width = options.width or 80
    local show_categories = options.show_categories ~= false
    local expanded = options.expanded ~= false
    
    local lines = {}
    local obj_type = obj.field_type and (obj.field_type.initial or obj.field_type.edited) or "Unknown"
    
    -- Window title
    local title = string.format(" Object Properties (%s) ", obj_type)
    local title_line = "╔" .. string.rep("═", width) .. "╗"
    local name_line = "║" .. string.format("%-" .. (width-2) .. "s", title) .. "║"
    table.insert(lines, title_line)
    table.insert(lines, name_line)
    
    -- Collect properties by category
    local categorized = {}
    for prop_name, prop_def in pairs(OBJECTS_DEFINITIONS) do
        if type(prop_def) == "table" and obj[prop_name] then
            local cat_name = get_property_category(prop_name)
            if not categorized[cat_name] then
                categorized[cat_name] = {}
            end
            
            local gui_type = prop_def.gui_field_type or "unknown"
            local value = get_property_value(obj, prop_name)
            
            table.insert(categorized[cat_name], {
                name = prop_name,
                value = value,
                gui_type = gui_type
            })
        end
    end
    
    -- Sort categories
    local category_order = {"Dimensions", "Value & Content", "Colors", "Borders", "Fill & Style", "Alignment", "Markers", "Position", "Attributes", "Other"}
    
    -- Render categories
    for _, cat_name in ipairs(category_order) do
        if categorized[cat_name] and #categorized[cat_name] > 0 then
            local section = render_category_section(cat_name, categorized[cat_name], width)
            table.insert(lines, section)
        end
    end
    
    -- Window footer
    local footer_line = "╚" .. string.rep("═", width) .. "╝"
    table.insert(lines, footer_line)
    
    return table.concat(lines, "\n")
end

-- Render a nested Fieldset-based properties window
function render_properties_fieldset(obj, options)
    options = options or {}
    local width = options.width or 80
    local is_expanded = options.expanded ~= false
    
    local obj_type = obj.field_type and (obj.field_type.initial or obj.field_type.edited) or "Unknown"
    local obj_name = obj.field_name and (obj.field_name.initial or obj.field_name.edited) or "Unnamed"
    
    -- Create a main container Fieldset
    local main_fieldset = OBJECTS_DEFINITIONS.new("Fieldset", {
        field_name = { initial = "Properties: " .. obj_name },
        field_width = { initial = width },
        field_height = { initial = 20 },
        field_border_style = { initial = "double" }
    })
    
    -- Collect properties into groups
    local property_groups = {
        {
            name = "Core Properties",
            props = {"field_type", "field_name", "field_width", "field_height"}
        },
        {
            name = "Appearance",
            props = {"field_border_style", "field_border_color", "field_text_color", "field_fill_char"}
        },
        {
            name = "Text & Titles",
            props = {"field_text_align", "field_title_align", "field_title_prefix", "field_title_suffix"}
        },
        {
            name = "Validation",
            props = {"field_attrb", "field_required_marker", "field_error_marker"}
        }
    }
    
    -- Build content for the main fieldset
    local content_lines = {}
    
    for _, group in ipairs(property_groups) do
        -- Group header with collapsible marker
        local marker = is_expanded and "[-]" or "[+]"
        local group_header = string.format("  %s %s", marker, group.name)
        table.insert(content_lines, group_header)
        
        if is_expanded then
            -- Group content
            for _, prop_name in ipairs(group.props) do
                if obj[prop_name] then
                    local prop_def = OBJECTS_DEFINITIONS[prop_name]
                    local gui_type = prop_def and prop_def.gui_field_type or "text"
                    local value = get_property_value(obj, prop_name)
                    local display_name = prop_name:gsub("[_%%]", " ")
                    local line = string.format("    %-20s : %s", display_name, tostring(value))
                    table.insert(content_lines, line)
                end
            end
            table.insert(content_lines, "")
        else
            -- Collapsed - just show placeholder
            local collapsed_line = string.format("  %s [ %d properties hidden ]", string.rep(".", 30), #group.props)
            table.insert(content_lines, collapsed_line)
        end
    end
    
    -- Render the main fieldset with custom content
    -- We need to override the render to include our custom content
    local border_chars = {
        top_left = "╔",
        top = "═",
        top_right = "╗",
        left = "║",
        right = "║",
        bottom_left = "╚",
        bottom = "═",
        bottom_right = "╝"
    }
    
    local lines = {}
    local total_width = width
    local title = "Properties: " .. obj_name
    
    -- Top border with title
    local title_padding = math.floor((total_width - #title - 2) / 2)
    local top_line = border_chars.top_left .. string.rep(border_chars.top, total_width) .. border_chars.top_right
    local title_line = border_chars.left .. string.rep(" ", title_padding) .. title .. string.rep(" ", total_width - title_padding - #title - 2) .. border_chars.right
    
    table.insert(lines, top_line)
    table.insert(lines, title_line)
    
    -- Content area
    for _, line in ipairs(content_lines) do
        local padded = border_chars.left .. line .. string.rep(" ", total_width - #line - 2) .. border_chars.right
        table.insert(lines, padded)
    end
    
    -- Bottom border
    local bottom_line = border_chars.bottom_left .. string.rep(border_chars.bottom, total_width) .. border_chars.bottom_right
    table.insert(lines, bottom_line)
    
    return table.concat(lines, "\n")
end

-- =============================================================================
-- DEMO: Render properties window for example objects
-- =============================================================================

print("\n" .. string.rep("=", 80))
print("  PROPERTIES WINDOW DEMO")
print(string.rep("=", 80) .. "\n")

-- Create example objects of different types
local field_obj = OBJECTS_DEFINITIONS.new("Field", {
    field_name = { initial = "Username" },
    field_width = { initial = 40 },
    field_height = { initial = 3 },
    field_border_style = { initial = "single" },
    field_attrb = { initial = { field_required = true, field_has_error = false } },
    field_required_marker = { initial = { marker = " *" } },
    field_title_suffix = { initial = { enabled = true, required = true, suffix_char = " " } }
})

local fieldset_obj = OBJECTS_DEFINITIONS.new("Fieldset", {
    field_name = { initial = "Personal Info" },
    field_width = { initial = 60 },
    field_height = { initial = 8 },
    field_border_style = { initial = "double" }
})

-- Render properties window for Field object
print("\n1. Properties Window (Text-Based):")
print("-" .. string.rep("-", 78))
print(render_properties_window(field_obj, { width = 78, show_categories = true }))

print("\n2. Properties Window as Nested Fieldset (Collapsible):")
print("-" .. string.rep("-", 78))
print(render_properties_fieldset(field_obj, { width = 78, expanded = true }))

print("\n3. Properties Window as Nested Fieldset (Collapsed):")
print("-" .. string.rep("-", 78))
print(render_properties_fieldset(field_obj, { width = 78, expanded = false }))

print("\n4. Properties Window for Fieldset object:")
print("-" .. string.rep("-", 78))
print(render_properties_window(fieldset_obj, { width = 78 }))

print("\n" .. string.rep("=", 80))
print("  DEMO COMPLETE")
print(string.rep("=", 80) .. "\n")
