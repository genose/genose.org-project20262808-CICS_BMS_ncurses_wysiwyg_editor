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
OBJECTS_DEFINITIONS_GUI_TYPE = {
    gui_field_type = {
        enum = {
            gui_text_field = "gui_text_field",
            gui_literal_field = "gui_literal_field",
            gui_protected_literal_field = "gui_protected_literal_field",
            gui_boolean_field = "gui_boolean_field",
            gui_image_field = "gui_image_field",
            gui_line_field = "gui_line_field",
            gui_fieldset_field = "gui_fieldset_field",
            gui_list_textornum_with_label_field = "gui_list_textornum_with_label_field",
            gui_select_with_label_string = "gui_select_with_label_string",
            gui_select_with_label_numeric = "gui_select_with_label_numeric",
            gui_select_field = "gui_select_field",
            gui_list_field = "gui_list_field"
        }
    }
}

-- ===== GUI RENDERING CONSTANTS =====
local GUI_CONSTANTS = {
    default_border_style = "single",
    default_color = "default",
    default_fill_char = " ",
    label_separator = ": ",
    selected_marker = "[X]",
    unselected_marker = "[ ]",
    required_marker = " *",
    error_marker = " /!\\"
}

-- ===== HELPER FUNCTIONS =====

-- Display width calculation that treats multi-byte UTF-8 box-drawing characters as width 1
-- This is necessary because Lua's # operator counts bytes, not display width
local function display_width(s)
    if s == nil then return 0 end
    local width = 0
    local i = 1
    while i <= #s do
        local byte = string.byte(s, i)
        -- UTF-8 multi-byte sequence detection
        if byte >= 128 then
            -- Multi-byte character - count as 1 display width
            -- Determine how many bytes this character uses
            if byte >= 240 then
                i = i + 4  -- 4-byte sequence
            elseif byte >= 224 then
                i = i + 3  -- 3-byte sequence
            elseif byte >= 192 then
                i = i + 2  -- 2-byte sequence
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
    if not prop then return nil end
    if type(prop) == "table" then
        if prop.edited ~= nil then return prop.edited end
        if prop.initial ~= nil then return prop.initial end
    end
    return prop
end

-- Get position table from object
local function get_position(obj)
    local pos = get_gui_property(obj, "field_pos")
    if pos and type(pos) == "table" then
        return pos.initial or pos.edited or pos
    end
    return {row = 0, col = 0, rowend = 0, colend = 0}
end

-- Helper: Extract a simple value from a property that might be a table
-- For tables like {key = "value"}, returns the first value
-- For tables like {min=1, max=10, initial=5}, returns .initial
local function get_gui_simple_value(obj, prop_name, default_value)
    local prop = get_gui_property(obj, prop_name)
    if prop == nil then return default_value end
    
    if type(prop) == "table" then
        -- Check if it's a {min, max, initial, edited} structure (field_height, field_width)
        if prop.initial ~= nil then
            return prop.initial
        end
        -- For style tables (field_border_style), check for known style values
        -- Define priority order for border styles
        local style_priority = {"single", "double", "dashed", "none"}
        for _, style in ipairs(style_priority) do
            if prop[style] ~= nil then
                return style
            end
        end
        -- For other tables, get the first value
        for _, v in pairs(prop) do
            if type(v) == "string" then
                return v
            end
        end
        return default_value
    end
    
    return prop or default_value
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
    local border_style = get_gui_simple_value(obj, "field_border_style", "none")
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
        top_left = "+", top = "-", top_right = "+",
        left = "|", right = "|",
        bottom_left = "+", bottom = "-", bottom_right = "+"
    }
end

-- ===== GUI RENDERING FUNCTIONS =====

-- Render a GUI text field with label
function render_gui_text_field(obj, label_text, is_selected, is_required, has_error, override_width)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_simple_value(obj, "field_border_style", "none")
    local fill_char = get_gui_simple_value(obj, "field_fill_char", " ")
    local value = get_gui_property(obj, "field_initial") or ""
    if value and type(value) == "table" then value = value.initial_value or "" end
    
    -- Calculate minimum width based on content
    local label = label_text or ""
    local required_marker = is_required and GUI_CONSTANTS.required_marker or ""
    local full_label = label .. required_marker
    local full_label_dw = display_width(full_label)
    local value_dw = display_width(value)
    local min_width = math.max(full_label_dw + 2, value_dw + 2, width)  -- +2 for border characters
    local actual_width = override_width or (border_style == "none" and math.max(full_label_dw, value_dw, width) or min_width)
    
    -- If override_width is provided, strictly respect it (don't let label overflow)
    if override_width then
        actual_width = override_width
    end
    
    local lines = {}
    
    -- If there's a label, render it
    if label_text and label_text ~= "" then
        local label_color = has_error and "red" or (is_required and "yellow" or "default")
        
        -- Top line: label + field border
        if border_style ~= "none" then
            local border_chars = get_border_chars(obj)
            local content_width = actual_width - 2
            local label_padding = math.max(0, math.floor((content_width - full_label_dw) / 2))
            local top_line = border_chars.top_left .. string.rep(" ", label_padding) .. full_label .. 
                            string.rep(" ", content_width - label_padding - full_label_dw) .. border_chars.top_right
            table.insert(lines, top_line)
        else
            table.insert(lines, full_label)
        end
    end
    
    -- Field content area
    if border_style == "none" then
        table.insert(lines, value or "")
    else
        local border_chars = get_border_chars(obj)
        for i = 1, height - (label_text and 1 or 0) - 1 do
            local content = (i == 1) and (value or "") or ""
            local content_dw = display_width(content)
            local padding = actual_width - 2 - content_dw
            if padding > 0 then
                local left_pad = math.floor(padding / 2)
                content = string.rep(" ", left_pad) .. content .. string.rep(" ", padding - left_pad)
            end
            table.insert(lines, border_chars.left .. content .. border_chars.right)
        end
    end
    
    return table.concat(lines, "\n")
end

-- Render a GUI select field with label
function render_gui_select_field(obj, label_text, options, selected_index, is_required, has_error, override_width)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_simple_value(obj, "field_border_style", "single")
    local value = get_gui_property(obj, "field_initial") or false
    
    local lines = {}
    local border_chars = get_border_chars(obj)
    
    -- Calculate minimum width based on label and options
    local required_marker = is_required and GUI_CONSTANTS.required_marker or ""
    local label = (label_text or "") .. required_marker
    local max_option_len = 0
    for _, option in ipairs(options or {}) do
        local option_text = " " .. GUI_CONSTANTS.selected_marker .. " " .. tostring(option)
        max_option_len = math.max(max_option_len, display_width(option_text))
    end
    local min_width = math.max(display_width(label) + 4, max_option_len + 2, width)
    local actual_width = override_width or (border_style == "none" and math.max(display_width(label) + 2, max_option_len, width) or min_width)
    
    -- If override_width is provided, strictly respect it
    if override_width then
        actual_width = override_width
    end
    
    -- Top line with label
    if label_text and label_text ~= "" then
        local label_dw = display_width(label)
        local label_line = border_chars.top_left .. " " .. label .. string.rep(" ", actual_width - label_dw - 4) .. " " .. border_chars.top_right
        table.insert(lines, label_line)
    else
        table.insert(lines, border_chars.top_left .. string.rep(border_chars.top, actual_width - 2) .. border_chars.top_right)
    end
    
    -- Options area
    for i, option in ipairs(options or {}) do
        local marker = (i == selected_index) and GUI_CONSTANTS.selected_marker or GUI_CONSTANTS.unselected_marker
        local option_text = " " .. marker .. " " .. tostring(option)
        local option_dw = display_width(option_text)
        -- Truncate if too long
        if option_dw > actual_width - 2 then
            option_text = " " .. marker .. " " .. string.sub(tostring(option), 1, actual_width - 6)
            option_dw = display_width(option_text)
        end
        local padding = actual_width - option_dw - 2
        local content = option_text .. string.rep(" ", padding)
        table.insert(lines, border_chars.left .. content .. border_chars.right)
    end
    
    -- Bottom border
    local line_count = #lines
    if height > line_count then
        for i = line_count + 1, height do
            table.insert(lines, border_chars.left .. string.rep(" ", actual_width - 2) .. border_chars.right)
        end
    end
    table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, actual_width - 2) .. border_chars.bottom_right)
    
    return table.concat(lines, "\n")
end

-- Render a GUI list field with text or numeric values
function render_gui_list_field(obj, label_text, items, is_required, has_error, override_width)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_simple_value(obj, "field_border_style", "single")
    local border_chars = get_border_chars(obj)
    
    local lines = {}
    local required_marker = is_required and GUI_CONSTANTS.required_marker or ""
    local label = (label_text or "") .. required_marker
    
    -- Calculate minimum width based on label and items
    local max_item_len = 0
    for _, item in ipairs(items or {}) do
        max_item_len = math.max(max_item_len, display_width("  " .. tostring(item)))  -- +2 for "  " prefix
    end
    local min_width = math.max(display_width(label) + 4, max_item_len + 2, width)
    local actual_width = override_width or (border_style == "none" and math.max(display_width(label) + 2, max_item_len, width) or min_width)
    
    -- If override_width is provided, strictly respect it
    if override_width then
        actual_width = override_width
    end
    
    -- Top border with label
    if border_style ~= "none" then
        local label_dw = display_width(label)
        local top_line = border_chars.top_left .. " " .. label .. string.rep(" ", actual_width - label_dw - 4) .. " " .. border_chars.top_right
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
        local padding = actual_width - item_dw - 2
        local content = item_text .. string.rep(" ", padding)
        if border_style ~= "none" then
            table.insert(lines, border_chars.left .. content .. border_chars.right)
        else
            table.insert(lines, content)
        end
    end
    
    -- Bottom border
    if border_style ~= "none" then
        table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, actual_width - 2) .. border_chars.bottom_right)
    end
    
    return table.concat(lines, "\n")
end

-- Render a GUI text or numeric field with label
function render_gui_textornum_with_label_field(obj, label_text, is_required, has_error, override_width)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local value = get_gui_property(obj, "field_initial") or ""
    if value and type(value) == "table" then value = value.initial_value or "" end
    
    local border_style = get_gui_simple_value(obj, "field_border_style", "single")
    local border_chars = get_border_chars(obj)
    local required_marker = is_required and GUI_CONSTANTS.required_marker or ""
    local label = (label_text or "") .. required_marker
    
    -- Calculate minimum width based on content
    local label_dw = display_width(label)
    local value_dw = display_width(value)
    local min_width = math.max(label_dw + 4, value_dw + 2, width)  -- +4 for " " + " " padding, +2 for borders
    local actual_width = override_width or (border_style == "none" and math.max(label_dw + 2, value_dw, width) or min_width)
    
    -- If override_width is provided, strictly respect it
    if override_width then
        actual_width = override_width
    end
    
    -- Ensure actual_width is at least wide enough for the label
    if border_style ~= "none" then
        actual_width = math.max(actual_width, label_dw + 4)
    end
    
    local lines = {}
    
    -- Top border with label
    if border_style ~= "none" then
        local padding = actual_width - label_dw - 4
        local top_line = border_chars.top_left .. " " .. label .. string.rep(" ", padding > 0 and padding or 0) .. " " .. border_chars.top_right
        table.insert(lines, top_line)
    else
        table.insert(lines, label .. ": " .. value)
        return table.concat(lines, "\n")
    end
    
    -- Value line
    local value_line = " " .. value .. string.rep(" ", actual_width - value_dw - 2)
    table.insert(lines, border_chars.left .. value_line .. border_chars.right)
    
    -- Fill remaining height
    local line_count = #lines
    for i = line_count + 1, height do
        table.insert(lines, border_chars.left .. string.rep(" ", actual_width - 2) .. border_chars.right)
    end
    
    -- Bottom border
    table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, actual_width - 2) .. border_chars.bottom_right)
    
    return table.concat(lines, "\n")
end

-- Render a GUI fieldset container
function render_gui_fieldset(obj, children, title, is_required, has_error, override_width)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_simple_value(obj, "field_border_style", "double")
    local border_chars = get_border_chars(obj)
    
    local lines = {}
    local required_marker = is_required and GUI_CONSTANTS.required_marker or ""
    local title_text = (title or obj.field_name.initial or "Fieldset") .. required_marker
    
    -- Calculate minimum width based on title
    local title_dw = display_width(title_text)
    local min_width = math.max(title_dw + 4, width)  -- +4 for spaces and border corners
    local actual_width = override_width or (border_style == "none" and math.max(title_dw, width) or min_width)
    
    -- Top border with title
    if border_style ~= "none" then
        local title_len = title_dw + 2  -- +2 for spaces
        local padding = actual_width - title_len - 2  -- -2 for border corners
        local left_pad = math.floor(padding / 2)
        local right_pad = padding - left_pad
        local top_line = border_chars.top_left .. 
                       string.rep(" ", left_pad) .. " " .. title_text .. " " .. 
                       string.rep(" ", right_pad) .. border_chars.top_right
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
            local child_req_marker = child_required and GUI_CONSTANTS.required_marker or ""
            local child_full_label = child_label .. child_req_marker
            local child_value = get_gui_property(child, "field_initial") or ""
            if child_value and type(child_value) == "table" then child_value = child_value.initial_value or "" end
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
            local child_render = render_gui_object(child, {width = max_child_width})
            for line in child_render:gmatch("[^\n]+") do
                local line_dw = display_width(line)
                table.insert(lines, border_chars.left .. " " .. line .. string.rep(" ", actual_width - line_dw - 4) .. " " .. border_chars.right)
            end
        end
    else
        -- Empty space for children
        for i = 1, height - 2 do
            table.insert(lines, border_chars.left .. string.rep(" ", actual_width - 2) .. border_chars.right)
        end
    end
    
    -- Bottom border
    if border_style ~= "none" then
        table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, actual_width - 2) .. border_chars.bottom_right)
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
    local override_width = custom_options and custom_options.width  -- Optional width override
    
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
    
    -- Set position if provided
    if options and options.position then
        obj.field_pos = {
            initial = options.position,
            edited = nil
        }
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
        local border_chars = get_border_chars({field_border_style = {initial = {style = form.border_style}}})
        
        -- Top border with title
        local title_dw = display_width(form.title)
        local padding = form.width - title_dw - 4
        local left_pad = math.floor(padding / 2)
        local right_pad = padding - left_pad
        table.insert(lines, border_chars.top_left .. 
                   string.rep(" ", left_pad) .. " " .. form.title .. " " .. 
                   string.rep(" ", right_pad) .. border_chars.top_right)
        
        -- Calculate maximum field width for consistent alignment
        local max_field_width = 20  -- Start with minimum readable width
        for _, field in ipairs(form.fields) do
            local label = field.label or get_gui_property(field, "field_name") or ""
            local required_marker = field.is_required and GUI_CONSTANTS.required_marker or ""
            local full_label = label .. required_marker
            local value = get_gui_property(field, "field_initial") or ""
            if value and type(value) == "table" then value = value.initial_value or "" end
            
            -- For fieldset children, calculate their widths too
            local field_width = math.max(display_width(full_label) + 4, display_width(value) + 2)
            if field.gui_field_type == "gui_fieldset_field" and field.children then
                for _, child in ipairs(field.children) do
                    local child_label = child.label or get_gui_property(child, "field_name") or ""
                    local child_required = child.is_required and GUI_CONSTANTS.required_marker or ""
                    local child_full_label = child_label .. child_required
                    local child_value = get_gui_property(child, "field_initial") or ""
                    if child_value and type(child_value) == "table" then child_value = child_value.initial_value or "" end
                    local child_width = math.max(display_width(child_full_label) + 4, display_width(child_value) + 2)
                    field_width = math.max(field_width, child_width + 6)  -- Add fieldset border padding
                end
            end
            
            -- For select fields, consider option lengths
            if field.options and #field.options > 0 then
                for _, opt in ipairs(field.options) do
                    local opt_text = " " .. GUI_CONSTANTS.selected_marker .. " " .. tostring(opt)
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
        local current_line = 2  -- Start after the title line (line 1 is title, line 2 is first field)
        for _, field in ipairs(form.fields) do
            local pos = get_position(field)
            local height = get_gui_simple_value(field, "field_height", 3)
            local field_render_lines = {}
            
            -- Render the field with consistent width
            local field_render = render_gui_object(field, {width = max_field_width})
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
                local padded_line = border_chars.left .. " " .. line .. string.rep(" ", form.width - line_dw - 4) .. " " .. border_chars.right
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
        table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, form.width - 2) .. border_chars.bottom_right)
        
        return table.concat(lines, "\n")
    end
    
    return form
end

-- ===== DEMO AND TEST =====

-- Example usage:
--[[
-- Create a simple form with GUI fields
local form = create_gui_form({
    create_gui_object('Field', {
        label = "Username",
        gui_field_type = "gui_textornum_with_label_field",
        is_required = true,
        position = {row = 1, col = 1, rowend = 3, colend = 30}
    }),
    create_gui_object('Field', {
        label = "Password",
        gui_field_type = "gui_textornum_with_label_field",
        is_required = true,
        position = {row = 5, col = 1, rowend = 7, colend = 30}
    }),
    create_gui_object('BooleanField', {
        label = "Remember me",
        gui_field_type = "gui_select_field",
        is_required = false,
        options = {"Yes", "No"},
        selected_index = 2,
        position = {row = 9, col = 1, rowend = 11, colend = 30}
    }),
    create_gui_object('Fieldset', {
        label = "Address",
        gui_field_type = "gui_fieldset_field",
        is_required = false,
        children = {
            create_gui_object('Field', {label = "Street", position = {row = 1, col = 1}}),
            create_gui_object('Field', {label = "City", position = {row = 2, col = 1}})
        },
        position = {row = 13, col = 1, rowend = 19, colend = 60}
    })
}, {title = "Login Form", width = 60, height = 22})

print(form:render())
--]]

print("OBJECT-GUI-RENDERING.lua module loaded successfully")
return OBJECT_GUI_RENDERING
