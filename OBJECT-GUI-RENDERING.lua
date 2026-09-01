-- ***********************************************************
-- Project : CICS BMS ncurses WYSIWYG Editor
-- File    : OBJECT-GUI-RENDERING.lua
-- Designed-by : Sebastien Genose.org
-- Date    : 2024-08-31
-- Description : GUI Rendering module for CICS BMS ncurses WYSIWYG Editor
-- Description : Provides graphical rendering functions for BMS field types
-- Description : Uses position = {row, col, rowend, colend} structure for screen placement
-- Description : Supports complex GUI field types with labels, selections, and rendering properties
-- ***********************************************************
-- ===== DEPENDENCIES =====
dofile('OBJECTS-DEFINITIONS.lua')

-- ===== GUI FIELD TYPES =====
-- from OBJECTS-DEFINITIONS.lua, we have gui_field_type enum for rendering different field types

-- ===== GUI RENDERING CONSTANTS =====
-- for reference, see OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type in OBJECTS-DEFINITIONS.lua
-- use dynamic notation not constant, so all must came from OBJECTS_DEFINITIONS.field_*.enum and OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type or from the object itself from .default
-- local OBJECTS_DEFINITIONS_DEFAULTS = nil; -- deprecated, use OBJECTS_DEFINITIONS and OBJECTS_DEFINITIONS_GUI_TYPE instead

-- ===== HELPER FUNCTIONS =====

-- Display width calculation that treats multi-byte UTF-8 box-drawing characters as width 1
-- This is necessary because Lua's # operator counts bytes, not display width
local function display_width(s)
    if s == nil then
        return 0
    end
    local width = 0
    local i = 1
    while i <= #s do
        local byte = string.byte(s, i)
        -- UTF-8 multi-byte sequence detection
        if byte >= 128 then
            -- Multi-byte character - count as 1 display width
            -- Determine how many bytes this character uses
            if byte >= 240 then
                i = i + 4 -- 4-byte sequence
            elseif byte >= 224 then
                i = i + 3 -- 3-byte sequence
            elseif byte >= 192 then
                i = i + 2 -- 2-byte sequence
            end
            width = width + 1
        else
            -- ASCII character - count as 1 display width
            width = width + 1
            i = i + 1
        end
    end
    return width
end

-- Get the current value for a property (edited if set, otherwise initial)
local function get_gui_property(obj, prop_name)
    local prop = obj[prop_name]
    if not prop then
        return nil
    end
    if type(prop) == "table" then
        if prop.edited ~= nil then
            return prop.edited
        end
        if prop.initial ~= nil then
            return prop.initial
        end
    end
    return prop
end

-- Get position table from object
local function get_position(obj)
    local pos = get_gui_property(obj, "field_pos")
    if pos and type(pos) == "table" then
        return pos.initial or pos.edited or pos
    end
    if obj.field_pos and obj.field_pos.default then
        return obj.field_pos.default
    end
    return OBJECTS_DEFINITIONS_DEFAULTS.field_pos.default
end

-- Helper: Recursively resolve property value through nested tables
-- Handles structures like {initial = {initial = value}} or {style = "single"}
local function resolve_property_value(prop)
    if prop == nil then
        return nil
    end

    -- If it's a simple value (number, string, boolean), return it
    if type(prop) ~= "table" then
        return prop
    end

    -- If it's a table, try to find a value
    -- Priority 1: .initial
    if prop.initial ~= nil then
        return resolve_property_value(prop.initial)
    end
    -- Priority 2: .edited
    if prop.edited ~= nil then
        return resolve_property_value(prop.edited)
    end
    -- Priority 3: .marker (for marker properties)
    if prop.marker ~= nil then
        return resolve_property_value(prop.marker)
    end
    -- Priority 4: known border styles (fallback to checking common style keys)
    local style_priority = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum
    for _, style in pairs(style_priority) do
        if type(style) == "string" and prop[style] ~= nil then
            return resolve_property_value(prop[style])
        end
    end
    -- Priority 5: first string value (recursive)
    for _, v in pairs(prop) do
        if type(v) == "string" then
            return v
        elseif type(v) == "table" then
            local resolved = resolve_property_value(v)
            if type(resolved) == "string" or type(resolved) == "number" or type(resolved) == "boolean" then
                return resolved
            end
        end
    end
    -- Priority 6: first numeric value
    for _, v in pairs(prop) do
        if type(v) == "number" then
            return v
        end
    end
    -- Priority 6: first non-table value
    for _, v in pairs(prop) do
        if type(v) ~= "table" then
            return v
        end
    end

    return prop
end

-- Helper: Extract a simple value from a property that might be a table
-- For tables like {key = "value"}, returns the first value
-- For tables like {min=1, max=10, initial=5}, returns .initial
local function get_gui_simple_value(obj, prop_name, default_value)
    local prop = obj[prop_name]
    if prop == nil then
        return default_value
    end

    -- Check if this is a property definition table (from OBJECTS_DEFINITIONS.new)
    -- If gui_field_name exists, it's a property definition, not a value
    if prop.gui_field_name then
        -- Extract default value for this object's type
        local obj_type = obj.field_type or "Field"
        if type(obj_type) == "table" then
            obj_type = obj_type.initial or obj_type.edited or "Field"
        end
        local default_table = prop.default
        if default_table and type(default_table) == "table" then
            local type_default = default_table[obj_type]
            if type_default and type(type_default) == "table" then
                -- type_default might be a table, try to extract initial value
                if type_default.initial ~= nil then
                    return type_default.initial
                end
                if type_default.edited ~= nil then
                    return type_default.edited
                end
                -- For tables like {space = " ", dash = "-"}, return the first string value
                for _, v in pairs(type_default) do
                    if type(v) == "string" or type(v) == "number" or type(v) == "boolean" then
                        return v
                    end
                end
            elseif type(type_default) ~= nil then
                return type_default
            end
        end
        return default_value
    end

    -- Recursively resolve the property value
    local resolved = resolve_property_value(prop)

    if resolved == nil or resolved == prop then
        return default_value
    end

    return resolved or default_value
end

-- Helper function to check if a value is valid for a given enum
local function is_valid_enum_value(value, enum_table)
    if not value or not enum_table then
        return false
    end
    for k, v in pairs(enum_table) do
        if v == value then
            return true
        end
    end
    return false
end

-- Get text alignment (horizontal) for an object
-- Returns: "left", "center", or "right"
local function get_text_align(obj, default_align)
    local align = get_gui_simple_value(obj, "field_text_align",
        default_align or OBJECTS_DEFINITIONS_DEFAULTS.text_align.default)
    if align and is_valid_enum_value(align, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum) then
        return align
    end
    return default_align or OBJECTS_DEFINITIONS_DEFAULTS.text_align.default
end

-- Get vertical alignment for an object
-- Returns: "top", "middle", or "bottom"
local function get_vertical_align(obj, default_align)
    local align = get_gui_simple_value(obj, "field_vertical_align",
        default_align or OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.default)
    if align and is_valid_enum_value(align, OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum) then
        return align
    end
    return default_align or OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.default
end

-- Get title alignment for an object
-- Returns: "left", "center", or "right"
local function get_title_align(obj, default_align)
    local align = get_gui_simple_value(obj, "field_title_align",
        default_align or OBJECTS_DEFINITIONS_DEFAULTS.text_align.default)
    if align and is_valid_enum_value(align, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum) then
        return align
    end
    return default_align or OBJECTS_DEFINITIONS_DEFAULTS.text_align.default
end

-- Get footer alignment for an object
-- Returns: "left", "center", or "right"
local function get_footer_align(obj, default_align)
    local align = get_gui_simple_value(obj, "field_footer_align",
        default_align or OBJECTS_DEFINITIONS_DEFAULTS.text_align.default)
    if align and is_valid_enum_value(align, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum) then
        return align
    end
    return default_align or OBJECTS_DEFINITIONS_DEFAULTS.text_align.default
end

-- Get color property and return ANSI code
-- Returns: ANSI color code string
local function get_color_code(obj, prop_name, default_color)
    local color_prop = obj[prop_name]
    local fallback = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes.default or "\27[0m"
    -- If property doesn't exist, use default
    if not color_prop then
        return OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes[default_color or "default"] or fallback
    end

    -- Check if user has explicitly set a color (not from defaults)
    if color_prop.initial and type(color_prop.initial) == "string" then
        return OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes[color_prop.initial] or fallback
    end
    if color_prop.edited and type(color_prop.edited) == "string" then
        return OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes[color_prop.edited] or fallback
    end

    -- Resolve the color value
    local color = resolve_property_value(color_prop)

    -- If color is a string and valid, use it
    if type(color) == "string" and OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes[color] then
        return OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes[color]
    end

    -- Handle case where color is a table (from field_text_color.default.Field)
    if type(color) == "table" then
        -- First check if 'default' key exists
        if color.default and type(color.default) == "string" then
            return OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes[color.default] or fallback
        end
        -- Check if any key is "default"
        if color["default"] and type(color["default"]) == "string" then
            return OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes[color["default"]] or fallback
        end
        -- Look for any valid color string
        for k, v in pairs(color) do
            if type(v) == "string" and OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes[v] then
                return OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes[v]
            end
        end
    end

    return fallback
end

-- Get title prefix (can be table with .initial)
local function get_title_prefix(obj)
    local prefix = get_gui_property(obj, "field_title_prefix")
    if prefix then
        if type(prefix) == "table" then
            prefix = resolve_property_value(prefix) or ""
        end
        if type(prefix) == "table" and prefix.prefix_char then
            return resolve_property_value(prefix.prefix_char) or ""
        end
        return prefix or ""
    end
    return ""
end

-- Get title suffix (can be table with .initial)
local function get_title_suffix(obj)
    local suffix = get_gui_property(obj, "field_title_suffix")
    if suffix then
        if type(suffix) == "table" then
            suffix = resolve_property_value(suffix) or ""
        end
        if type(suffix) == "table" and suffix.suffix_char then
            return resolve_property_value(suffix.suffix_char) or ""
        end
        return suffix or ""
    end
    return ""
end

-- Get fill character for a field
local function get_fill_char(obj, default_char)
    local fill_prop = obj.field_fill_char
    -- If property doesn't exist, use default
    if not fill_prop then
        return default_char or OBJECTS_DEFINITIONS_DEFAULTS.default_fill_char
    end

    -- Check if this is a property definition table (from OBJECTS_DEFINITIONS.new)
    -- If gui_field_name exists, it's a property definition, not a value
    if fill_prop.gui_field_name then
        -- Extract default value for this object's type
        local obj_type = obj.field_type or "Field"
        if type(obj_type) == "table" then
            obj_type = obj_type.initial or obj_type.edited or "Field"
        end
        local default_table = fill_prop.default
        if default_table and type(default_table) == "table" then
            local type_default = default_table[obj_type]
            if type_default and type(type_default) == "table" then
                -- type_default is a table of fill characters, use the first one or space
                if type_default.space and type(type_default.space) == "string" then
                    return type_default.space
                end
                for _, v in pairs(type_default) do
                    if type(v) == "string" then
                        return v
                    end
                end
            elseif type(type_default) == "string" then
                return type_default
            end
        end
        return default_char or OBJECTS_DEFINITIONS_DEFAULTS.default_fill_char
    end

    -- Check if user has explicitly set a fill_char (not from defaults)
    -- If fill_prop.initial is a simple string (not a table), use it
    if fill_prop.initial and type(fill_prop.initial) == "string" then
        return fill_prop.initial
    end
    if fill_prop.edited and type(fill_prop.edited) == "string" then
        return fill_prop.edited
    end

    -- If fill_prop.initial is the defaults table, look for space
    if fill_prop.initial and type(fill_prop.initial) == "table" then
        -- Try direct access first
        local initial_table = fill_prop.initial
        if initial_table.space and type(initial_table.space) == "string" then
            return initial_table.space
        end
        if initial_table["space"] and type(initial_table["space"]) == "string" then
            return initial_table["space"]
        end
        -- Look for space character
        for k, v in pairs(initial_table) do
            if type(v) == "string" and v == " " then
                return v
            end
        end
    end

    -- Resolve the fill value as fallback
    local fill = resolve_property_value(fill_prop)

    -- If fill is a string, return it
    if type(fill) == "string" then
        return fill
    end

    -- Handle case where fill is a table
    if type(fill) == "table" then
        -- Try to get the space value
        if fill.space and type(fill.space) == "string" then
            return fill.space
        end
        if fill["space"] and type(fill["space"]) == "string" then
            return fill["space"]
        end
        -- Look for space character
        for k, v in pairs(fill) do
            if type(v) == "string" and v == " " then
                return v
            end
        end
        -- Look for any string value
        for k, v in pairs(fill) do
            if type(v) == "string" then
                return v
            end
        end
    end

    return default_char or OBJECTS_DEFINITIONS_DEFAULTS.default_fill_char
end

-- Get text color for a field
local function get_text_color(obj)
    return get_color_code(obj, "field_text_color")
end

-- Get title color for a field
local function get_title_color(obj)
    return get_color_code(obj, "field_title_color")
end

-- Get border color for a field
local function get_border_color(obj)
    return get_color_code(obj, "field_border_color")
end

-- Get footer color for a field
local function get_footer_color(obj)
    return get_color_code(obj, "field_footer_color")
end

-- Get required marker for a field
local function get_required_marker(obj)
    local marker_prop = obj.field_required_marker
    
    -- If property doesn't exist, use default
    if not marker_prop then
        return OBJECTS_DEFINITIONS_DEFAULTS.required_marker_str
    end
    
    -- Check if this is a property definition table (from OBJECTS_DEFINITIONS.new)
    -- If gui_field_name exists, it's a property definition, not a value
    if marker_prop.gui_field_name then
        -- Extract default value for this object's type
        local obj_type = obj.field_type or "Field"
        if type(obj_type) == "table" then
            obj_type = obj_type.initial or obj_type.edited or "Field"
        end
        local default_table = marker_prop.default
        if default_table and type(default_table) == "table" then
            local type_default = default_table[obj_type]
            if type_default and type(type_default) == "table" then
                -- type_default might contain marker configuration
                if type_default.marker_fill and type(type_default.marker_fill) == "string" then
                    return type_default.marker_fill
                end
            elseif type(type_default) == "string" then
                return type_default
            end
        end
        return OBJECTS_DEFINITIONS_DEFAULTS.required_marker_str
    end
    
    -- Check if user has explicitly set a marker
    if marker_prop.initial and type(marker_prop.initial) == "string" then
        return marker_prop.initial
    end
    if marker_prop.edited and type(marker_prop.edited) == "string" then
        return marker_prop.edited
    end
    if marker_prop.marker and type(marker_prop.marker) == "string" then
        return marker_prop.marker
    end
    
    -- Resolve the marker value as fallback
    local marker = resolve_property_value(marker_prop)
    if type(marker) == "string" then
        return marker
    end
    
    return OBJECTS_DEFINITIONS_DEFAULTS.required_marker_str
end

-- Get error marker for a field
local function get_error_marker(obj)
    local marker_prop = obj.field_error_marker
    
    -- If property doesn't exist, use default
    if not marker_prop then
        return OBJECTS_DEFINITIONS_DEFAULTS.error_marker_str
    end
    
    -- Check if this is a property definition table (from OBJECTS_DEFINITIONS.new)
    -- If gui_field_name exists, it's a property definition, not a value
    if marker_prop.gui_field_name then
        -- Extract default value for this object's type
        local obj_type = obj.field_type or "Field"
        if type(obj_type) == "table" then
            obj_type = obj_type.initial or obj_type.edited or "Field"
        end
        local default_table = marker_prop.default
        if default_table and type(default_table) == "table" then
            local type_default = default_table[obj_type]
            if type_default and type(type_default) == "table" then
                -- type_default might contain marker configuration
                if type_default.marker_fill and type(type_default.marker_fill) == "string" then
                    return type_default.marker_fill
                end
            elseif type(type_default) == "string" then
                return type_default
            end
        end
        return OBJECTS_DEFINITIONS_DEFAULTS.error_marker_str
    end
    
    -- Check if user has explicitly set a marker
    if marker_prop.initial and type(marker_prop.initial) == "string" then
        return marker_prop.initial
    end
    if marker_prop.edited and type(marker_prop.edited) == "string" then
        return marker_prop.edited
    end
    if marker_prop.marker and type(marker_prop.marker) == "string" then
        return marker_prop.marker
    end
    
    -- Resolve the marker value as fallback
    local marker = resolve_property_value(marker_prop)
    if type(marker) == "string" then
        return marker
    end
    
    return OBJECTS_DEFINITIONS_DEFAULTS.error_marker_str
end

-- Align text within a given width
-- text: the text to align (can be a table with .initial or .edited)
-- width: the target width
-- align: "left", "center", or "right"
-- fill_char: optional character to use for padding (defaults to space)
-- Returns: the aligned text padded to the specified width
local function align_text(text, width, align, fill_char)
    -- Resolve text if it's a table (from properties like field_footer_title = {initial = value})
    if type(text) == "table" then
        text = resolve_property_value(text) or ""
    end
    -- Fallback to empty string if text is still a table or nil
    if type(text) ~= "string" and type(text) ~= "number" then
        text = ""
    end
    text = tostring(text)

    fill_char = fill_char or " "

    local text_dw = display_width(text)
    if text_dw >= width then
        return text
    end

    -- Validate align against text_align enum, fallback to center if invalid
    if not align or not is_valid_enum_value(align, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum) then
        align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center
    end

    local padding = width - text_dw
    if align == OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left then
        return text .. string.rep(fill_char, padding)
    elseif align == OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.right then
        return string.rep(fill_char, padding) .. text
    else -- center
        local left_pad = math.floor(padding / 2)
        local right_pad = padding - left_pad
        return string.rep(fill_char, left_pad) .. text .. string.rep(fill_char, right_pad)
    end
end

-- Get dimensions from object
-- Note: field_height and field_width have structure {min, max, initial, edited}
-- so we need to extract the .initial value
local function get_dimensions(obj)
    local height = get_gui_simple_value(obj, "field_height", 3)
    local width = get_gui_simple_value(obj, "field_width", 10)

    return height, width
end

-- Get border characters for a given style (uses get_gui_simple_value)
local function get_border_chars(obj)
    local border_style = get_gui_simple_value(obj, "field_border_style", OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none)
    -- Validate border_style against enum, fallback to none if invalid
    if not is_valid_enum_value(border_style, OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum) then
        border_style = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none
    end
    local obj_type = get_gui_simple_value(obj, "field_type", "Field")
    local chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default[obj_type]

    if chars and chars[border_style] then
        return chars[border_style]
    end
    -- Fallback to single style
    if chars and chars.single then
        return chars.single
    end
    -- Fallback to double style
    if chars and chars.double then
        return chars.double
    end
    -- Ultimate fallback to ASCII
    return {
        top_left = "+",
        top = "-",
        top_right = "+",
        left = "|",
        right = "|",
        bottom_left = "+",
        bottom = "-",
        bottom_right = "+"
    }
end

-- ===== GUI RENDERING FUNCTIONS =====

-- Render a GUI text field with label
function render_gui_text_field(obj, label_text, is_selected, is_required, has_error, override_width)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_simple_value(obj, "field_border_style", OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none)
    -- Validate border_style against enum
    if not is_valid_enum_value(border_style, OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum) then
        border_style = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none
    end
    local fill_char = get_fill_char(obj, " ")
    local value = get_gui_property(obj, "field_initial") or ""
    if value and type(value) == "table" then
        value = value.initial_value or ""
    end

    -- Get alignment properties
    local text_align = get_text_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center)
    local vertical_align = get_vertical_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.top)
    local title_align = get_title_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center)
    local footer_align = get_footer_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center)

    -- Get color properties
    local text_color = get_text_color(obj)
    local title_color = get_title_color(obj)
    local border_color = get_border_color(obj)
    local footer_color = get_footer_color(obj)

    -- Get prefix and suffix for title
    local title_prefix = get_title_prefix(obj)
    local title_suffix = get_title_suffix(obj)

    -- Get footer properties
    local footer_title = get_gui_property(obj, "field_footer_title") or ""
    local footer_fill_char = get_gui_simple_value(obj, "field_footer_fill_char", " ")
    local footer_required_marker = get_gui_property(obj, "field_footer_required_marker") or ""
    local footer_error_marker = get_gui_property(obj, "field_footer_error_marker") or ""

    -- Calculate minimum width based on content
    local label = label_text or ""
    local required_marker = ""
    local error_marker = ""
    if has_error then
        error_marker = get_error_marker(obj)
    elseif is_required then
        required_marker = get_required_marker(obj)
    end
    local full_label = title_prefix .. label .. required_marker .. error_marker .. title_suffix
    local full_label_dw = display_width(full_label)
    local value_dw = display_width(value)
    local min_width = math.max(full_label_dw + 2, value_dw + 2, width) -- +2 for border characters
    local actual_width = override_width or
                             (border_style == OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none and math.max(full_label_dw, value_dw, width) or min_width)

    -- If override_width is provided, strictly respect it (don't let label overflow)
    if override_width then
        actual_width = override_width
    end

    local lines = {}
    local border_chars = get_border_chars(obj)

    -- If there's a label, render it
    if label_text and label_text ~= "" then
        local label_color = has_error and "red" or (is_required and "yellow" or "default")

        -- Top line: label + field border
        if border_style ~= OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
            local content_width = actual_width - 2
            local label_content = align_text(full_label, content_width, title_align)
            -- Apply title color
            local colored_label = title_color .. label_content ..
                                      OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes.default
            local top_line = border_chars.top_left .. colored_label .. border_chars.top_right
            table.insert(lines, top_line)
        else
            table.insert(lines, full_label)
        end
    end

    -- Field content area
    if border_style == OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
        -- Apply text color
        table.insert(lines, text_color .. (value or "") .. OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes.default)
    else
        local content_height = height - (label_text and 1 or 0) - 1

        -- Handle vertical alignment
        if vertical_align == OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.middle then
            -- Add empty lines before content
            local empty_lines = math.floor((content_height - 1) / 2)
            for i = 1, empty_lines do
                table.insert(lines, border_chars.left .. string.rep(fill_char, actual_width - 2) .. border_chars.right)
            end
        elseif vertical_align == OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.bottom then
            -- Add empty lines before content (all but one)
            for i = 1, content_height - 1 do
                table.insert(lines, border_chars.left .. string.rep(fill_char, actual_width - 2) .. border_chars.right)
            end
        end

        -- Content line with text color and fill character
        local content = value or ""
        local content_dw = display_width(content)
        local inner_width = actual_width - 2
        local aligned_content = align_text(content, inner_width, text_align, fill_char)
        -- Apply text color to content
        table.insert(lines,
            border_chars.left .. text_color .. aligned_content ..
                OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes.default .. border_chars.right)

        -- Handle vertical alignment - add remaining empty lines after content
        if vertical_align == OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.middle then
            local empty_lines = content_height - 1 - math.floor((content_height - 1) / 2)
            for i = 1, empty_lines do
                table.insert(lines, border_chars.left .. string.rep(fill_char, actual_width - 2) .. border_chars.right)
            end
        elseif vertical_align == OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.top then
            -- Add remaining empty lines
            for i = 1, content_height - 1 do
                table.insert(lines, border_chars.left .. string.rep(fill_char, actual_width - 2) .. border_chars.right)
            end
        end
    end

    -- Footer line (if footer_title is set, replace bottom border with footer)
    if border_style ~= OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
        local footer_text = ""
        if footer_title and footer_title ~= "" then
            footer_text = resolve_property_value(footer_title) or ""
        elseif footer_required_marker and footer_required_marker ~= "" then
            footer_text = resolve_property_value(footer_required_marker) or ""
        elseif footer_error_marker and footer_error_marker ~= "" then
            footer_text = resolve_property_value(footer_error_marker) or ""
        elseif is_required then
            footer_text = OBJECTS_DEFINITIONS_DEFAULTS.required_marker_str
        elseif has_error then
            footer_text = OBJECTS_DEFINITIONS_DEFAULTS.error_marker_str
        end

        if footer_text ~= "" then
            -- Ensure footer_text is a string
            footer_text = tostring(footer_text)
            -- Create footer line with aligned text using fill character
            local content_width = actual_width - 2
            local footer_text_dw = display_width(footer_text)
            local footer_content = ""
            if content_width > footer_text_dw then
                local padding_needed = content_width - footer_text_dw
                if footer_align == OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left then
                    footer_content = footer_text .. string.rep(footer_fill_char, padding_needed)
                elseif footer_align == OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.right then
                    footer_content = string.rep(footer_fill_char, padding_needed) .. footer_text
                else -- center
                    local left_pad = math.floor(padding_needed / 2)
                    local right_pad = padding_needed - left_pad
                    footer_content = string.rep(footer_fill_char, left_pad) .. footer_text ..
                                         string.rep(footer_fill_char, right_pad)
                end
            else
                footer_content = footer_text
            end
            -- Apply footer color
            local colored_footer = footer_color .. footer_content ..
                                       OBJECTS_DEFINITIONS_DEFAULTS.color_enum.color_codes.default
            table.insert(lines, border_chars.bottom_left .. colored_footer .. border_chars.bottom_right)
        else
            -- Standard bottom border
            table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, actual_width - 2) ..
                border_chars.bottom_right)
        end
    end

    return table.concat(lines, "\n")
end

-- Render a GUI select field with label
function render_gui_select_field(obj, label_text, options, selected_index, is_required, has_error, override_width)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_simple_value(obj, "field_border_style", OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single)
    -- Validate border_style against enum
    if not is_valid_enum_value(border_style, OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum) then
        border_style = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    end
    local value = get_gui_property(obj, "field_initial") or false

    -- Get alignment properties
    local text_align = get_text_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left)
    local title_align = get_title_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center)

    local lines = {}
    local border_chars = get_border_chars(obj)

    -- Calculate minimum width based on label and options
    local required_marker = is_required and OBJECTS_DEFINITIONS_DEFAULTS.required_marker_str or ""
    local label = (label_text or "") .. required_marker
    local max_option_len = 0
    for _, option in ipairs(options or {}) do
        local option_text = " " .. OBJECTS_DEFINITIONS_DEFAULTS.selected_marker .. " " .. tostring(option)
        max_option_len = math.max(max_option_len, display_width(option_text))
    end
    local min_width = math.max(display_width(label) + 4, max_option_len + 2, width)
    local actual_width = override_width or
                             (border_style == OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none and math.max(display_width(label) + 2, max_option_len, width) or
                                 min_width)

    -- If override_width is provided, strictly respect it
    if override_width then
        actual_width = override_width
    end

    -- Top line with label
    if label_text and label_text ~= "" then
        local label_dw = display_width(label)
        local content_width = actual_width - 4 -- -4 for " " + " " + 2 border chars
        local label_content = align_text(label, content_width, title_align)
        local label_line = border_chars.top_left .. " " .. label_content .. " " .. border_chars.top_right
        table.insert(lines, label_line)
    else
        table.insert(lines,
            border_chars.top_left .. string.rep(border_chars.top, actual_width - 2) .. border_chars.top_right)
    end

    -- Options area
    for i, option in ipairs(options or {}) do
        local marker = (i == selected_index) and OBJECTS_DEFINITIONS_DEFAULTS.selected_marker or
                           OBJECTS_DEFINITIONS_DEFAULTS.unselected_marker
        local option_text = " " .. marker .. " " .. tostring(option)
        local option_dw = display_width(option_text)
        -- Truncate if too long
        if option_dw > actual_width - 2 then
            option_text = " " .. marker .. " " .. string.sub(tostring(option), 1, actual_width - 6)
            option_dw = display_width(option_text)
        end
        local inner_width = actual_width - 2
        local content = align_text(option_text, inner_width, text_align)
        table.insert(lines, border_chars.left .. content .. border_chars.right)
    end

    -- Bottom border
    local line_count = #lines
    if height > line_count then
        for i = line_count + 1, height do
            table.insert(lines, border_chars.left .. string.rep(" ", actual_width - 2) .. border_chars.right)
        end
    end
    table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, actual_width - 2) ..
        border_chars.bottom_right)

    return table.concat(lines, "\n")
end

-- Render a GUI list field with text or numeric values
function render_gui_list_field(obj, label_text, items, is_required, has_error, override_width)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_simple_value(obj, "field_border_style", OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single)
    -- Validate border_style against enum
    if not is_valid_enum_value(border_style, OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum) then
        border_style = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    end
    local border_chars = get_border_chars(obj)

    -- Get alignment properties
    local text_align = get_text_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left)
    local title_align = get_title_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center)

    local lines = {}
    local required_marker = is_required and OBJECTS_DEFINITIONS_DEFAULTS.required_marker_str or ""
    local label = (label_text or "") .. required_marker

    -- Calculate minimum width based on label and items
    local max_item_len = 0
    for _, item in ipairs(items or {}) do
        max_item_len = math.max(max_item_len, display_width("  " .. tostring(item))) -- +2 for "  " prefix
    end
    local min_width = math.max(display_width(label) + 4, max_item_len + 2, width)
    local actual_width = override_width or
                             (border_style == OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none and math.max(display_width(label) + 2, max_item_len, width) or
                                 min_width)

    -- If override_width is provided, strictly respect it
    if override_width then
        actual_width = override_width
    end

    -- Top border with label
    if border_style ~= OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
        local label_dw = display_width(label)
        local content_width = actual_width - 4
        local label_content = align_text(label, content_width, title_align)
        local top_line = border_chars.top_left .. " " .. label_content .. " " .. border_chars.top_right
        table.insert(lines, top_line)
    else
        table.insert(lines, label)
    end

    -- List items
    for i, item in ipairs(items or {}) do
        local item_text = "  " .. tostring(item)
        local item_dw = display_width(item_text)
        if item_dw > actual_width - 2 then
            item_text = "  " .. string.sub(tostring(item), 1, actual_width - 4)
            item_dw = display_width(item_text)
        end
        local inner_width = actual_width - 2
        local content = align_text(item_text, inner_width, text_align)
        if border_style ~= OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
            table.insert(lines, border_chars.left .. content .. border_chars.right)
        else
            table.insert(lines, content)
        end
    end

    -- Bottom border
    if border_style ~= OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
        table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, actual_width - 2) ..
            border_chars.bottom_right)
    end

    return table.concat(lines, "\n")
end

-- Render a GUI text or numeric field with label
function render_gui_textornum_with_label_field(obj, label_text, is_required, has_error, override_width)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local value = get_gui_property(obj, "field_initial") or ""
    if value and type(value) == "table" then
        value = value.initial_value or ""
    end

    -- Get alignment properties
    local text_align = get_text_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left)
    local vertical_align = get_vertical_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.top)
    local title_align = get_title_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center)

    local border_style = get_gui_simple_value(obj, "field_border_style", OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single)
    -- Validate border_style against enum
    if not is_valid_enum_value(border_style, OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum) then
        border_style = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    end
    local border_chars = get_border_chars(obj)
    local required_marker = is_required and OBJECTS_DEFINITIONS_DEFAULTS.required_marker_str or ""
    local label = (label_text or "") .. required_marker

    -- Calculate minimum width based on content
    local label_dw = display_width(label)
    local value_dw = display_width(value)
    local min_width = math.max(label_dw + 4, value_dw + 2, width) -- +4 for " " + " " padding, +2 for borders
    local actual_width = override_width or
                             (border_style == OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none and math.max(label_dw + 2, value_dw, width) or min_width)

    -- If override_width is provided, strictly respect it
    if override_width then
        actual_width = override_width
    end

    -- Ensure actual_width is at least wide enough for the label
    if border_style ~= OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
        actual_width = math.max(actual_width, label_dw + 4)
    end

    local lines = {}

    -- Top border with label
    if border_style ~= OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
        local content_width = actual_width - 4
        local label_content = align_text(label, content_width, title_align)
        local top_line = border_chars.top_left .. " " .. label_content .. " " .. border_chars.top_right
        table.insert(lines, top_line)
    else
        table.insert(lines, label .. ": " .. value)
        return table.concat(lines, "\n")
    end

    -- Value line with horizontal alignment
    local content_height = height - 1 -- -1 for the label line
    local inner_width = actual_width - 2
    local value_content = align_text(value, inner_width, text_align)

    -- Handle vertical alignment
    if vertical_align == OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.middle then
        local empty_lines_before = math.floor((content_height - 1) / 2)
        for i = 1, empty_lines_before do
            table.insert(lines, border_chars.left .. string.rep(" ", actual_width - 2) .. border_chars.right)
        end
        table.insert(lines, border_chars.left .. value_content .. border_chars.right)
        local empty_lines_after = content_height - 1 - empty_lines_before
        for i = 1, empty_lines_after do
            table.insert(lines, border_chars.left .. string.rep(" ", actual_width - 2) .. border_chars.right)
        end
    elseif vertical_align == OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.bottom then
        for i = 1, content_height - 1 do
            table.insert(lines, border_chars.left .. string.rep(" ", actual_width - 2) .. border_chars.right)
        end
        table.insert(lines, border_chars.left .. value_content .. border_chars.right)
    else -- top
        table.insert(lines, border_chars.left .. value_content .. border_chars.right)
        for i = 1, content_height - 1 do
            table.insert(lines, border_chars.left .. string.rep(" ", actual_width - 2) .. border_chars.right)
        end
    end

    -- Bottom border
    table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, actual_width - 2) ..
        border_chars.bottom_right)

    return table.concat(lines, "\n")
end

-- Render a GUI fieldset container
function render_gui_fieldset(obj, children, title, is_required, has_error, override_width)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_simple_value(obj, "field_border_style", OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double)
    -- Validate border_style against enum
    if not is_valid_enum_value(border_style, OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum) then
        border_style = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double
    end
    local border_chars = get_border_chars(obj)

    -- Get alignment properties
    local title_align = get_title_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center)
    local text_align = get_text_align(obj, OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left)

    local lines = {}
    local required_marker = is_required and OBJECTS_DEFINITIONS_DEFAULTS.required_marker_str or ""
    local title_text = (title or obj.field_name.initial or "Fieldset") .. required_marker

    -- Calculate minimum width based on title
    local title_dw = display_width(title_text)
    local min_width = math.max(title_dw + 4, width) -- +4 for spaces and border corners
    local actual_width = override_width or (border_style == OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none and math.max(title_dw, width) or min_width)

    -- Top border with title (using title_align)
    if border_style ~= OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
        local content_width = actual_width - 4 -- -4 for spaces and border corners
        local title_content = align_text(title_text, content_width, title_align)
        local top_line = border_chars.top_left .. " " .. title_content .. " " .. border_chars.top_right
        table.insert(lines, top_line)
    else
        table.insert(lines, title_text)
    end

    -- Render children
    if children and #children > 0 then
        -- Calculate max child width for consistent alignment
        local max_child_width = 0
        for _, child in ipairs(children) do
            local child_label = child.label or get_gui_property(child, "field_name") or ""
            local child_required = child.is_required or false
            local child_req_marker = child_required and OBJECTS_DEFINITIONS_DEFAULTS.required_marker_str or ""
            local child_full_label = child_label .. child_req_marker
            local child_value = get_gui_property(child, "field_initial") or ""
            if child_value and type(child_value) == "table" then
                child_value = child_value.initial_value or ""
            end
            local child_width = math.max(display_width(child_full_label) + 4, display_width(child_value) + 2)
            max_child_width = math.max(max_child_width, child_width)
        end
        -- Ensure minimum width for readability
        max_child_width = math.max(max_child_width, 20)

        -- If children need more space than available, expand the fieldset (but only if no override_width)
        -- If override_width is set (e.g., from a form), respect it
        if override_width then
            -- When in a form, respect the override_width
            max_child_width = math.min(max_child_width, actual_width - 6)
        else
            -- When standalone, expand fieldset to fit children
            if max_child_width > actual_width - 6 then
                actual_width = max_child_width + 6
            end
            max_child_width = math.min(max_child_width, actual_width - 6)
        end

        for _, child in ipairs(children) do
            local child_render = render_gui_object(child, {
                width = max_child_width
            })
            for line in child_render:gmatch("[^\n]+") do
                local line_dw = display_width(line)
                local inner_width = actual_width - 4
                local aligned_line = align_text(line, inner_width, text_align)
                table.insert(lines, border_chars.left .. " " .. aligned_line .. " " .. border_chars.right)
            end
        end
    else
        -- Empty space for children
        for i = 1, height - 2 do
            table.insert(lines, border_chars.left .. string.rep(" ", actual_width - 2) .. border_chars.right)
        end
    end

    -- Bottom border
    if border_style ~= OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
        table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, actual_width - 2) ..
            border_chars.bottom_right)
    end

    return table.concat(lines, "\n")
end

-- ===== MAIN GUI RENDERING FUNCTION =====

-- Main function to render any GUI object based on its type and properties
function render_gui_object(obj, custom_options)
    if not obj or not obj.field_type then
        return "[Invalid GUI Object]"
    end

    local obj_type = get_gui_property(obj, "field_type") or "Field"
    -- Try obj.gui_field_type first (set by create_gui_object), then custom_options
    local gui_type = obj.gui_field_type or (custom_options and custom_options.gui_field_type)
    local label = custom_options and custom_options.label
    local is_required = obj.is_required or (custom_options and custom_options.is_required) or false
    local has_error = obj.has_error or (custom_options and custom_options.has_error) or false
    local override_width = custom_options and custom_options.width -- Optional width override

    -- Determine GUI field type from object or custom options
    if not gui_type then
        -- Map OBJECTS_DEFINITIONS type to GUI type
        local type_map = {
            ["Field"] = "gui_text_field",
            ["Literal"] = "gui_literal_field",
            ["ProtectedLiteral"] = "gui_protected_literal_field",
            ["BooleanField"] = "gui_boolean_field",
            ["Image"] = "gui_image_field",
            ["Line"] = "gui_line_field",
            ["Fieldset"] = "gui_fieldset_field"
        }
        gui_type = type_map[obj_type] or "gui_text_field"
    end

    -- Get default label if not provided
    if not label then
        -- Try obj.label first (set by create_gui_object), then field_name, then obj_type
        label = obj.label or get_gui_property(obj, "field_name") or obj_type
    end

    -- Render based on GUI type, passing override_width if provided
    if gui_type == "gui_list_textornum_with_label_field" then
        return render_gui_textornum_with_label_field(obj, label, is_required, has_error, override_width)
    elseif gui_type == "gui_select_with_label_string" then
        local options = obj.options or (custom_options and custom_options.options) or {}
        local selected = obj.selected_index or (custom_options and custom_options.selected_index) or 1
        return render_gui_select_field(obj, label, options, selected, is_required, has_error, override_width)
    elseif gui_type == "gui_select_with_label_numeric" then
        local options = obj.options or (custom_options and custom_options.options) or {}
        local selected = obj.selected_index or (custom_options and custom_options.selected_index) or 1
        return render_gui_select_field(obj, label, options, selected, is_required, has_error, override_width)
    elseif gui_type == "gui_select_field" then
        local options = obj.options or (custom_options and custom_options.options) or {}
        local selected = obj.selected_index or (custom_options and custom_options.selected_index) or 1
        return render_gui_select_field(obj, label, options, selected, is_required, has_error, override_width)
    elseif gui_type == "gui_list_field" then
        local items = obj.items or (custom_options and custom_options.items) or {}
        return render_gui_list_field(obj, label, items, is_required, has_error, override_width)
    elseif gui_type == "gui_fieldset_field" then
        local children = obj.children or (custom_options and custom_options.children) or {}
        local title = custom_options and custom_options.title or label
        return render_gui_fieldset(obj, children, title, is_required, has_error, override_width)
    else
        -- Default text field rendering
        return render_gui_text_field(obj, label, false, is_required, has_error, override_width)
    end
end

-- ===== UTILITY FUNCTIONS =====

-- Create a GUI object with position and rendering properties
function create_gui_object(obj_type, options)
    local obj = OBJECTS_DEFINITIONS.new(obj_type, options)

    -- Add GUI-specific properties
    obj.gui_field_type = options and options.gui_field_type or "gui_text_field"
    obj.label = options and options.label or ""
    obj.is_required = options and options.is_required or false
    obj.has_error = options and options.has_error or false
    obj.options = options and options.options or {}
    obj.selected_index = options and options.selected_index or 1
    obj.children = options and options.children or {}

    -- Add alignment properties
    obj.text_align = options and options.text_align or nil
    obj.vertical_align = options and options.vertical_align or nil
    obj.title_align = options and options.title_align or nil
    obj.footer_align = options and options.footer_align or nil

    -- Set position if provided
    if options and options.position then
        obj.field_pos = {
            initial = options.position,
            edited = nil
        }
    end

    -- Set alignment properties if provided (maps to field_text_align, field_vertical_align, field_title_align, field_footer_align)
    if options then
        if options.text_align then
            obj.field_text_align = {
                initial = options.text_align,
                edited = nil
            }
        end
        if options.vertical_align then
            obj.field_vertical_align = {
                initial = options.vertical_align,
                edited = nil
            }
        end
        if options.title_align then
            obj.field_title_align = {
                initial = options.title_align,
                edited = nil
            }
        end
        if options.footer_align then
            obj.field_footer_align = {
                initial = options.footer_align,
                edited = nil
            }
        end
        -- Footer properties
        if options.footer_title then
            obj.field_footer_title = {
                initial = options.footer_title,
                edited = nil
            }
        end
        if options.footer_fill_char then
            obj.field_footer_fill_char = {
                initial = options.footer_fill_char,
                edited = nil
            }
        end
        -- Color properties
        if options.text_color then
            obj.field_text_color = {
                initial = options.text_color,
                edited = nil
            }
        end
        if options.title_color then
            obj.field_title_color = {
                initial = options.title_color,
                edited = nil
            }
        end
        if options.border_color then
            obj.field_border_color = {
                initial = options.border_color,
                edited = nil
            }
        end
        if options.footer_color then
            obj.field_footer_color = {
                initial = options.footer_color,
                edited = nil
            }
        end
        -- Prefix and suffix for title
        if options.title_prefix then
            obj.field_title_prefix = {
                initial = options.title_prefix,
                edited = nil
            }
        end
        if options.title_suffix then
            obj.field_title_suffix = {
                initial = options.title_suffix,
                edited = nil
            }
        end
        -- Fill character
        if options.fill_char then
            obj.field_fill_char = {
                initial = options.fill_char,
                edited = nil
            }
        end
    end

    -- Add render method
    obj.render_gui = function()
        return render_gui_object(obj, options)
    end

    return obj
end

-- Create a form with multiple GUI fields
function create_gui_form(fields, options)
    options = options or {}
    local form = {
        fields = fields or {},
        title = options.title or "Form",
        width = options.width or 80,
        height = options.height or 24,
        border_style = options.border_style or "double"
    }

    form.render = function()
        local lines = {}
        local border_chars = get_border_chars({
            field_border_style = {
                initial = {
                    style = form.border_style
                }
            }
        })

        -- Top border with title
        local title_dw = display_width(form.title)
        local padding = form.width - title_dw - 4
        local left_pad = math.floor(padding / 2)
        local right_pad = padding - left_pad
        table.insert(lines,
            border_chars.top_left .. string.rep(" ", left_pad) .. " " .. form.title .. " " .. string.rep(" ", right_pad) ..
                border_chars.top_right)

        -- Calculate maximum field width for consistent alignment
        local max_field_width = 20 -- Start with minimum readable width
        for _, field in ipairs(form.fields) do
            local label = field.label or get_gui_property(field, "field_name") or ""
            local required_marker = field.is_required and OBJECTS_DEFINITIONS_DEFAULTS.required_marker_str or ""
            local full_label = label .. required_marker
            local value = get_gui_property(field, "field_initial") or ""
            if value and type(value) == "table" then
                value = value.initial_value or ""
            end

            -- For fieldset children, calculate their widths too
            local field_width = math.max(display_width(full_label) + 4, display_width(value) + 2)
            if field.gui_field_type == "gui_fieldset_field" and field.children then
                for _, child in ipairs(field.children) do
                    local child_label = child.label or get_gui_property(child, "field_name") or ""
                    local child_required = child.is_required and OBJECTS_DEFINITIONS_DEFAULTS.required_marker_str or ""
                    local child_full_label = child_label .. child_required
                    local child_value = get_gui_property(child, "field_initial") or ""
                    if child_value and type(child_value) == "table" then
                        child_value = child_value.initial_value or ""
                    end
                    local child_width = math.max(display_width(child_full_label) + 4, display_width(child_value) + 2)
                    field_width = math.max(field_width, child_width + 6) -- Add fieldset border padding
                end
            end

            -- For select fields, consider option lengths
            if field.options and #field.options > 0 then
                for _, opt in ipairs(field.options) do
                    local opt_text = " " .. OBJECTS_DEFINITIONS_DEFAULTS.selected_marker .. " " .. tostring(opt)
                    field_width = math.max(field_width, display_width(opt_text) + 2)
                end
            end

            -- For list fields, consider item lengths
            if field.items and #field.items > 0 then
                for _, item in ipairs(field.items) do
                    local item_text = "  " .. tostring(item)
                    field_width = math.max(field_width, display_width(item_text) + 2)
                end
            end

            max_field_width = math.max(max_field_width, field_width)
        end
        -- Ensure max_field_width doesn't exceed form width minus margins
        max_field_width = math.min(max_field_width, form.width - 8)
        -- Ensure minimum width of 20 for readability
        max_field_width = math.max(max_field_width, 20)

        -- Render each field
        local current_line = 2 -- Start after the title line (line 1 is title, line 2 is first field)
        for _, field in ipairs(form.fields) do
            local pos = get_position(field)
            local height = get_gui_simple_value(field, "field_height", 3)
            local field_render_lines = {}

            -- Render the field with consistent width
            local field_render = render_gui_object(field, {
                width = max_field_width
            })
            for line in field_render:gmatch("[^\n]+") do
                table.insert(field_render_lines, line)
            end

            -- Target row from position or current line
            local target_row = pos.row or current_line

            -- Add empty lines to reach the target row
            while #lines + 1 < target_row do
                table.insert(lines, border_chars.left .. string.rep(" ", form.width - 2) .. border_chars.right)
            end

            -- Insert field render lines
            for _, line in ipairs(field_render_lines) do
                local line_dw = display_width(line)
                local padded_line =
                    border_chars.left .. " " .. line .. string.rep(" ", form.width - line_dw - 4) .. " " ..
                        border_chars.right
                table.insert(lines, padded_line)
            end

            -- Update current line position
            current_line = #lines + 1
        end

        -- Fill remaining space
        while #lines < form.height - 1 do
            table.insert(lines, border_chars.left .. string.rep(" ", form.width - 2) .. border_chars.right)
        end

        -- Bottom border
        table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, form.width - 2) ..
            border_chars.bottom_right)

        return table.concat(lines, "\n")
    end

    return form
end

-- ===== DEMO AND TEST =====

-- Example usage:
--[[
-- ============================================
-- EXAMPLE 1: HORIZONTAL TEXT ALIGNMENT
-- ============================================
-- Shows left, center, and right alignment of text content

print("=== EXAMPLE 1: HORIZONTAL TEXT ALIGNMENT ===")

local left_field = create_gui_object('Field', {
    label = "Left Aligned",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    text_align = "left",
    title_align = "center",
    field_border_style = {initial = "single"},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Left aligned ---")
print(left_field:render_gui())

local center_field = create_gui_object('Field', {
    label = "Center Aligned",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    text_align = "center",
    title_align = "center",
    field_border_style = {initial = "single"},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Center aligned ---")
print(center_field:render_gui())

local right_field = create_gui_object('Field', {
    label = "Right Aligned",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    text_align = "right",
    title_align = "center",
    field_border_style = {initial = "single"},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Right aligned ---")
print(right_field:render_gui())

-- ============================================
-- EXAMPLE 2: VERTICAL ALIGNMENT
-- ============================================
-- Shows top, middle, and bottom vertical alignment

print("\n\n=== EXAMPLE 2: VERTICAL ALIGNMENT ===")

local top_field = create_gui_object('Field', {
    label = "Top Aligned",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    vertical_align = "top",
    title_align = "center",
    field_border_style = {initial = "single"},
    field_height = {initial = 5},
    field_width = {initial = 25}
})
print("\n--- Top aligned (content at top) ---")
print(top_field:render_gui())

local middle_field = create_gui_object('Field', {
    label = "Middle Aligned",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    vertical_align = "middle",
    title_align = "center",
    field_border_style = {initial = "single"},
    field_height = {initial = 5},
    field_width = {initial = 25}
})
print("\n--- Middle aligned (content centered vertically) ---")
print(middle_field:render_gui())

local bottom_field = create_gui_object('Field', {
    label = "Bottom Aligned",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    vertical_align = "bottom",
    title_align = "center",
    field_border_style = {initial = "single"},
    field_height = {initial = 5},
    field_width = {initial = 25}
})
print("\n--- Bottom aligned (content at bottom) ---")
print(bottom_field:render_gui())

-- ============================================
-- EXAMPLE 3: TITLE ALIGNMENT
-- ============================================
-- Shows left, center, and right alignment of field labels/titles

print("\n\n=== EXAMPLE 3: TITLE ALIGNMENT ===")

local title_left_field = create_gui_object('Field', {
    label = "Left Title",
    field_initial = "value",
    gui_field_type = "gui_text_field",
    title_align = "left",
    field_border_style = {initial = "single"},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Title left aligned ---")
print(title_left_field:render_gui())

local title_center_field = create_gui_object('Field', {
    label = "Center Title",
    field_initial = "value",
    gui_field_type = "gui_text_field",
    title_align = "center",
    field_border_style = {initial = "single"},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Title center aligned ---")
print(title_center_field:render_gui())

local title_right_field = create_gui_object('Field', {
    label = "Right Title",
    field_initial = "value",
    gui_field_type = "gui_text_field",
    title_align = "right",
    field_border_style = {initial = "single"},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Title right aligned ---")
print(title_right_field:render_gui())

-- ============================================
-- EXAMPLE 4: FIELDSET TITLE ALIGNMENT
-- ============================================
-- Shows title alignment for fieldset containers

print("\n\n=== EXAMPLE 4: FIELDSET TITLE ALIGNMENT ===")

local fieldset_left = create_gui_object('Fieldset', {
    label = "Left Title",
    gui_field_type = "gui_fieldset_field",
    title_align = "left",
    field_border_style = {initial = "double"},
    field_height = {initial = 6},
    field_width = {initial = 35},
    children = {
        create_gui_object('Field', {
            label = "Name",
            field_initial = "John",
            text_align = "center"
        })
    }
})
print("\n--- Fieldset with left-aligned title ---")
print(fieldset_left:render_gui())

local fieldset_center = create_gui_object('Fieldset', {
    label = "Center Title",
    gui_field_type = "gui_fieldset_field",
    title_align = "center",
    field_border_style = {initial = "double"},
    field_height = {initial = 6},
    field_width = {initial = 35},
    children = {
        create_gui_object('Field', {
            label = "Name",
            field_initial = "John",
            text_align = "center"
        })
    }
})
print("\n--- Fieldset with center-aligned title ---")
print(fieldset_center:render_gui())

local fieldset_right = create_gui_object('Fieldset', {
    label = "Right Title",
    gui_field_type = "gui_fieldset_field",
    title_align = "right",
    field_border_style = {initial = "double"},
    field_height = {initial = 6},
    field_width = {initial = 35},
    children = {
        create_gui_object('Field', {
            label = "Name",
            field_initial = "John",
            text_align = "center"
        })
    }
})
print("\n--- Fieldset with right-aligned title ---")
print(fieldset_right:render_gui())

-- ============================================
-- EXAMPLE 5: COMBINED ALIGNMENTS
-- ============================================
-- Shows combinations of horizontal, vertical, and title alignments

print("\n\n=== EXAMPLE 5: COMBINED ALIGNMENTS ===")

-- Text: center, Vertical: middle, Title: right
local combined1 = create_gui_object('Field', {
    label = "Combined 1",
    field_initial = "data",
    gui_field_type = "gui_text_field",
    text_align = "center",
    vertical_align = "middle",
    title_align = "right",
    field_border_style = {initial = "single"},
    field_height = {initial = 5},
    field_width = {initial = 25}
})
print("\n--- Text:center + Vertical:middle + Title:right ---")
print(combined1:render_gui())

-- Text: right, Vertical: bottom, Title: left
local combined2 = create_gui_object('Field', {
    label = "Combined 2",
    field_initial = "data",
    gui_field_type = "gui_text_field",
    text_align = "right",
    vertical_align = "bottom",
    title_align = "left",
    field_border_style = {initial = "single"},
    field_height = {initial = 5},
    field_width = {initial = 25}
})
print("\n--- Text:right + Vertical:bottom + Title:left ---")
print(combined2:render_gui())

-- Text: left, Vertical: top, Title: center
local combined3 = create_gui_object('Field', {
    label = "Combined 3",
    field_initial = "data",
    gui_field_type = "gui_text_field",
    text_align = "left",
    vertical_align = "top",
    title_align = "center",
    field_border_style = {initial = "single"},
    field_height = {initial = 5},
    field_width = {initial = 25}
})
print("\n--- Text:left + Vertical:top + Title:center ---")
print(combined3:render_gui())

-- ============================================
-- EXAMPLE 6: ALIGNMENT IN SELECT FIELDS
-- ============================================
-- Shows alignment in select/checkbox fields

print("\n\n=== EXAMPLE 6: ALIGNMENT IN SELECT FIELDS ===")

local select_field = create_gui_object('BooleanField', {
    label = "Options",
    gui_field_type = "gui_select_field",
    options = {"Option 1", "Option 2", "Option 3"},
    selected_index = 2,
    text_align = "center",
    title_align = "center",
    field_border_style = {initial = "single"},
    field_height = {initial = 6},
    field_width = {initial = 25}
})
print("\n--- Select field with center-aligned options ---")
print(select_field:render_gui())

-- ============================================
-- EXAMPLE 7: COMPLETE FORM WITH ALL ALIGNMENTS
-- ============================================
-- Shows a complete form using various alignment combinations

print("\n\n=== EXAMPLE 7: COMPLETE FORM WITH ALL ALIGNMENTS ===")

local complete_form = create_gui_form({
    create_gui_object('Field', {
        label = "Name",
        field_initial = "John Doe",
        gui_field_type = "gui_textornum_with_label_field",
        is_required = true,
        text_align = "left",
        title_align = "left",
        field_border_style = {initial = "single"},
        position = {row = 1}
    }),
    create_gui_object('Field', {
        label = "Email",
        field_initial = "john@example.com",
        gui_field_type = "gui_textornum_with_label_field",
        is_required = true,
        text_align = "center",
        title_align = "center",
        field_border_style = {initial = "single"},
        position = {row = 4}
    }),
    create_gui_object('BooleanField', {
        label = "Subscribe",
        gui_field_type = "gui_select_field",
        options = {"Yes", "No"},
        selected_index = 1,
        text_align = "center",
        title_align = "right",
        field_border_style = {initial = "single"},
        position = {row = 7}
    }),
    create_gui_object('Field', {
        label = "Status",
        field_initial = "Active",
        gui_field_type = "gui_text_field",
        vertical_align = "middle",
        text_align = "center",
        title_align = "center",
        field_border_style = {initial = "single"},
        field_height = {initial = 5},
        position = {row = 10}
    }),
    create_gui_object('Fieldset', {
        label = "Address Information",
        gui_field_type = "gui_fieldset_field",
        title_align = "center",
        field_border_style = {initial = "double"},
        children = {
            create_gui_object('Field', {
                label = "Street",
                field_initial = "123 Main St",
                text_align = "left"
            }),
            create_gui_object('Field', {
                label = "City",
                field_initial = "New York",
                text_align = "center"
            }),
            create_gui_object('Field', {
                label = "ZIP",
                field_initial = "10001",
                text_align = "right"
            })
        },
        position = {row = 16}
    })
}, {title = "Complete Form with Alignments", width = 60, height = 25})

print(complete_form:render())

-- ============================================
-- EXAMPLE 8: DEFAULT ALIGNMENTS
-- ============================================
-- Shows behavior with default alignment (no explicit alignment set)

print("\n\n=== EXAMPLE 8: DEFAULT ALIGNMENTS ===")

local default_field = create_gui_object('Field', {
    label = "Default Alignment",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    field_border_style = {initial = "single"},
    field_height = {initial = 4},
    field_width = {initial = 25}
})
print("\n--- Field with default alignments (text:left, vertical:top, title:center) ---")
print(default_field:render_gui())

-- ============================================
-- EXAMPLE 9: NESTED FIELDSET WITH ALIGNMENTS
-- ============================================
-- Shows nested fieldsets with different title alignments

print("\n\n=== EXAMPLE 9: NESTED FIELDSET WITH ALIGNMENTS ===")

local nested_form = create_gui_form({
    create_gui_object('Fieldset', {
        label = "Personal Info",
        gui_field_type = "gui_fieldset_field",
        title_align = "center",
        field_border_style = {initial = "double"},
        children = {
            create_gui_object('Field', {
                label = "Name",
                field_initial = "John",
                text_align = "left"
            }),
            create_gui_object('Field', {
                label = "Age",
                field_initial = "30",
                text_align = "center"
            })
        },
        position = {row = 1}
    }),
    create_gui_object('Fieldset', {
        label = "Preferences",
        gui_field_type = "gui_fieldset_field",
        title_align = "left",
        field_border_style = {initial = "double"},
        children = {
            create_gui_object('Field', {
                label = "Theme",
                field_initial = "Dark",
                text_align = "center"
            }),
            create_gui_object('Field', {
                label = "Language",
                field_initial = "English",
                text_align = "right"
            })
        },
        position = {row = 9}
    })
}, {title = "Nested Fieldsets", width = 50, height = 20})

print(nested_form:render())

-- ============================================
-- EXAMPLE 10: FOOTER ALIGNMENT
-- ============================================
-- Shows footer alignment (left, center, right) with footer titles

print("\n\n=== EXAMPLE 10: FOOTER ALIGNMENT ===")

local footer_left_field = create_gui_object('Field', {
    label = "Footer Left",
    field_initial = "value",
    gui_field_type = "gui_text_field",
    footer_title = "-- left --",
    footer_align = "left",
    footer_fill_char = "-",
    field_border_style = {initial = "single"},
    field_height = {initial = 4},
    field_width = {initial = 25}
})
print("\n--- Footer with left alignment ---")
print(footer_left_field:render_gui())

local footer_center_field = create_gui_object('Field', {
    label = "Footer Center",
    field_initial = "value",
    gui_field_type = "gui_text_field",
    footer_title = "-- center --",
    footer_align = "center",
    footer_fill_char = "-",
    field_border_style = {initial = "single"},
    field_height = {initial = 4},
    field_width = {initial = 25}
})
print("\n--- Footer with center alignment ---")
print(footer_center_field:render_gui())

local footer_right_field = create_gui_object('Field', {
    label = "Footer Right",
    field_initial = "value",
    gui_field_type = "gui_text_field",
    footer_title = "-- right --",
    footer_align = "right",
    footer_fill_char = "-",
    field_border_style = {initial = "single"},
    field_height = {initial = 4},
    field_width = {initial = 25}
})
print("\n--- Footer with right alignment ---")
print(footer_right_field:render_gui())

-- ============================================
-- EXAMPLE 11: ALL ALIGNMENT PROPERTIES COMBINED
-- ============================================
-- Shows a field using all alignment properties together

print("\n\n=== EXAMPLE 11: ALL ALIGNMENT PROPERTIES COMBINED ===")

local all_align_field = create_gui_object('Field', {
    label = "All Alignments",
    field_initial = "data",
    gui_field_type = "gui_text_field",
    text_align = "center",
    vertical_align = "middle",
    title_align = "right",
    footer_title = "footer",
    footer_align = "left",
    footer_fill_char = "=",
    field_border_style = {initial = "double"},
    field_height = {initial = 6},
    field_width = {initial = 30}
})
print("\n--- Text:center + Vertical:middle + Title:right + Footer:left ---")
print(all_align_field:render_gui())

-- ============================================
-- EXAMPLE 12: USING ALL ALIGNMENT TYPES IN A FORM
-- ============================================
-- Complete form demonstrating all alignment types

print("\n\n=== EXAMPLE 12: USING ALL ALIGNMENT TYPES IN A FORM ===")

local all_alignments_form = create_gui_form({
    create_gui_object('Field', {
        label = "text_align:left",
        field_initial = "value",
        text_align = "left",
        field_border_style = {initial = "single"},
        position = {row = 1}
    }),
    create_gui_object('Field', {
        label = "text_align:center",
        field_initial = "value",
        text_align = "center",
        field_border_style = {initial = "single"},
        position = {row = 4}
    }),
    create_gui_object('Field', {
        label = "text_align:right",
        field_initial = "value",
        text_align = "right",
        field_border_style = {initial = "single"},
        position = {row = 7}
    }),
    create_gui_object('Field', {
        label = "vertical_align:middle",
        field_initial = "value",
        vertical_align = "middle",
        field_border_style = {initial = "single"},
        field_height = {initial = 5},
        position = {row = 10}
    }),
    create_gui_object('Field', {
        label = "title_align:left",
        field_initial = "value",
        title_align = "left",
        field_border_style = {initial = "single"},
        position = {row = 16}
    }),
    create_gui_object('Field', {
        label = "footer_align:right",
        field_initial = "value",
        footer_title = "footer",
        footer_align = "right",
        field_border_style = {initial = "single"},
        position = {row = 19}
    }),
    create_gui_object('Fieldset', {
        label = "title_align:center",
        gui_field_type = "gui_fieldset_field",
        title_align = "center",
        field_border_style = {initial = "double"},
        children = {
            create_gui_object('Field', {
                label = "Child",
                field_initial = "value"
            })
        },
        position = {row = 22}
    })
}, {title = "All Alignment Types", width = 50, height = 32})

print(all_alignments_form:render())
--]]

print("OBJECT-GUI-RENDERING.lua module loaded successfully")
return OBJECT_GUI_RENDERING
