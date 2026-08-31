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

-- Get dimensions from object
local function get_dimensions(obj)
    local height = get_gui_property(obj, "field_height") or 3
    local width = get_gui_property(obj, "field_width") or 10
    return height, width
end

-- ===== GUI RENDERING FUNCTIONS =====

-- Render a GUI text field with label
function render_gui_text_field(obj, label_text, is_selected, is_required, has_error)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_property(obj, "field_border_style") or "none"
    local fill_char = get_gui_property(obj, "field_fill_char") or " "
    local value = get_gui_property(obj, "field_initial") or ""
    if value and type(value) == "table" then value = value.initial_value or "" end
    
    local lines = {}
    
    -- If there's a label, render it
    if label_text and label_text ~= "" then
        local label_color = has_error and "red" or (is_required and "yellow" or "default")
        local required_marker = is_required and GUI_CONSTANTS.required_marker or ""
        local label = label_text .. required_marker
        
        -- Top line: label + field border
        if border_style ~= "none" then
            local border_chars = get_border_chars(obj)
            local content_width = width - 2
            local label_padding = math.max(0, math.floor((content_width - #label) / 2))
            local top_line = border_chars.top_left .. string.rep(" ", label_padding) .. label .. 
                            string.rep(" ", content_width - label_padding - #label) .. border_chars.top_right
            table.insert(lines, top_line)
        else
            table.insert(lines, label)
        end
    end
    
    -- Field content area
    if border_style == "none" then
        table.insert(lines, value or "")
    else
        local border_chars = get_border_chars(obj)
        for i = 1, height - (label_text and 1 or 0) - 1 do
            local content = (i == 1) and (value or "") or ""
            local padding = width - #content
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
function render_gui_select_field(obj, label_text, options, selected_index, is_required, has_error)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_property(obj, "field_border_style") or "single"
    local value = get_gui_property(obj, "field_initial") or false
    
    local lines = {}
    local border_chars = get_border_chars(obj)
    
    -- Top line with label
    if label_text and label_text ~= "" then
        local required_marker = is_required and GUI_CONSTANTS.required_marker or ""
        local label = label_text .. required_marker
        local label_line = border_chars.top_left .. " " .. label .. string.rep(" ", width - #label - 4) .. " " .. border_chars.top_right
        table.insert(lines, label_line)
    else
        table.insert(lines, border_chars.top_left .. string.rep(border_chars.top, width) .. border_chars.top_right)
    end
    
    -- Options area
    for i, option in ipairs(options or {}) do
        local marker = (i == selected_index) and GUI_CONSTANTS.selected_marker or GUI_CONSTANTS.unselected_marker
        local option_text = " " .. marker .. " " .. tostring(option)
        -- Truncate if too long
        if #option_text > width - 2 then
            option_text = " " .. marker .. " " .. string.sub(tostring(option), 1, width - 6)
        end
        local padding = width - #option_text - 2
        local content = option_text .. string.rep(" ", padding)
        table.insert(lines, border_chars.left .. content .. border_chars.right)
    end
    
    -- Bottom border
    if height > #lines then
        for i = #lines + 1, height do
            table.insert(lines, border_chars.left .. string.rep(" ", width) .. border_chars.right)
        end
    end
    table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, width) .. border_chars.bottom_right)
    
    return table.concat(lines, "\n")
end

-- Render a GUI list field with text or numeric values
function render_gui_list_field(obj, label_text, items, is_required, has_error)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_property(obj, "field_border_style") or "single"
    local border_chars = get_border_chars(obj)
    
    local lines = {}
    local required_marker = is_required and GUI_CONSTANTS.required_marker or ""
    local label = (label_text or "") .. required_marker
    
    -- Top border with label
    if border_style ~= "none" then
        local top_line = border_chars.top_left .. " " .. label .. string.rep(" ", width - #label - 4) .. " " .. border_chars.top_right
        table.insert(lines, top_line)
    else
        table.insert(lines, label)
    end
    
    -- List items
    for i, item in ipairs(items or {}) do
        local item_text = "  " .. tostring(item)
        if #item_text > width - 2 then
            item_text = "  " .. string.sub(tostring(item), 1, width - 4)
        end
        local padding = width - #item_text - 2
        local content = item_text .. string.rep(" ", padding)
        if border_style ~= "none" then
            table.insert(lines, border_chars.left .. content .. border_chars.right)
        else
            table.insert(lines, content)
        end
    end
    
    -- Bottom border
    if border_style ~= "none" then
        table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, width) .. border_chars.bottom_right)
    end
    
    return table.concat(lines, "\n")
end

-- Render a GUI text or numeric field with label
function render_gui_textornum_with_label_field(obj, label_text, is_required, has_error)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local value = get_gui_property(obj, "field_initial") or ""
    if value and type(value) == "table" then value = value.initial_value or "" end
    
    local border_style = get_gui_property(obj, "field_border_style") or "single"
    local border_chars = get_border_chars(obj)
    local required_marker = is_required and GUI_CONSTANTS.required_marker or ""
    local label = (label_text or "") .. required_marker
    
    local lines = {}
    
    -- Top border with label
    if border_style ~= "none" then
        local label_len = #label
        local padding = width - label_len - 4
        local top_line = border_chars.top_left .. " " .. label .. string.rep(" ", padding > 0 and padding or 0) .. " " .. border_chars.top_right
        table.insert(lines, top_line)
    else
        table.insert(lines, label .. ": " .. value)
        return table.concat(lines, "\n")
    end
    
    -- Value line
    local value_line = " " .. value .. string.rep(" ", width - #value - 2)
    table.insert(lines, border_chars.left .. value_line .. border_chars.right)
    
    -- Fill remaining height
    for i = #lines + 1, height do
        table.insert(lines, border_chars.left .. string.rep(" ", width) .. border_chars.right)
    end
    
    -- Bottom border
    table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, width) .. border_chars.bottom_right)
    
    return table.concat(lines, "\n")
end

-- Render a GUI fieldset container
function render_gui_fieldset(obj, children, title, is_required, has_error)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_property(obj, "field_border_style") or "double"
    local border_chars = get_border_chars(obj)
    
    local lines = {}
    local required_marker = is_required and GUI_CONSTANTS.required_marker or ""
    local title_text = (title or obj.field_name.initial or "Fieldset") .. required_marker
    
    -- Top border with title
    if border_style ~= "none" then
        local title_len = #title_text + 2  -- +2 for spaces
        local padding = width - title_len - 2  -- -2 for border corners
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
        for _, child in ipairs(children) do
            local child_render = render_gui_object(child)
            for line in child_render:gmatch("[^\n]+") do
                table.insert(lines, border_chars.left .. " " .. line .. string.rep(" ", width - #line - 3) .. " " .. border_chars.right)
            end
        end
    else
        -- Empty space for children
        for i = 1, height - 2 do
            table.insert(lines, border_chars.left .. string.rep(" ", width) .. border_chars.right)
        end
    end
    
    -- Bottom border
    if border_style ~= "none" then
        table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, width) .. border_chars.bottom_right)
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
    local gui_type = custom_options and custom_options.gui_field_type
    local label = custom_options and custom_options.label
    local is_required = custom_options and custom_options.is_required or false
    local has_error = custom_options and custom_options.has_error or false
    
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
        label = get_gui_property(obj, "field_name") or obj_type
    end
    
    -- Render based on GUI type
    if gui_type == "gui_list_textornum_with_label_field" then
        return render_gui_textornum_with_label_field(obj, label, is_required, has_error)
    elseif gui_type == "gui_select_with_label_string" then
        local options = custom_options and custom_options.options or {}
        local selected = custom_options and custom_options.selected_index or 1
        return render_gui_select_field(obj, label, options, selected, is_required, has_error)
    elseif gui_type == "gui_select_with_label_numeric" then
        local options = custom_options and custom_options.options or {}
        local selected = custom_options and custom_options.selected_index or 1
        return render_gui_select_field(obj, label, options, selected, is_required, has_error)
    elseif gui_type == "gui_select_field" then
        local options = custom_options and custom_options.options or {}
        local selected = custom_options and custom_options.selected_index or 1
        return render_gui_select_field(obj, label, options, selected, is_required, has_error)
    elseif gui_type == "gui_list_field" then
        local items = custom_options and custom_options.items or {}
        return render_gui_list_field(obj, label, items, is_required, has_error)
    elseif gui_type == "gui_fieldset_field" then
        local children = custom_options and custom_options.children or {}
        local title = custom_options and custom_options.title or label
        return render_gui_fieldset(obj, children, title, is_required, has_error)
    else
        -- Default text field rendering
        return render_gui_text_field(obj, label, false, is_required, has_error)
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
        local title = form.title
        local padding = form.width - #title - 4
        local left_pad = math.floor(padding / 2)
        local right_pad = padding - left_pad
        table.insert(lines, border_chars.top_left .. 
                   string.rep(" ", left_pad) .. " " .. title .. " " .. 
                   string.rep(" ", right_pad) .. border_chars.top_right)
        
        -- Render each field
        local current_row = 1
        for _, field in ipairs(form.fields) do
            local pos = get_position(field)
            local row = pos.row or current_row
            local col = pos.col or 1
            local height = get_gui_property(field, "field_height") or 3
            
            -- Add empty lines to reach the field's row
            while #lines < row do
                table.insert(lines, border_chars.left .. string.rep(" ", form.width - 2) .. border_chars.right)
            end
            
            -- Render the field
            local field_render = render_gui_object(field)
            for line in field_render:gmatch("[^\n]+") do
                -- Pad line to form width
                local padded_line = border_chars.left .. " " .. line .. string.rep(" ", form.width - #line - 4) .. " " .. border_chars.right
                table.insert(lines, padded_line)
            end
            
            current_row = current_row + height + 1
        end
        
        -- Fill remaining space
        while #lines < form.height - 1 do
            table.insert(lines, border_chars.left .. string.rep(" ", form.width - 2) .. border_chars.right)
        end
        
        -- Bottom border
        table.insert(lines, border_chars.bottom_left .. string.rep(border_chars.bottom, form.width) .. border_chars.bottom_right)
        
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
