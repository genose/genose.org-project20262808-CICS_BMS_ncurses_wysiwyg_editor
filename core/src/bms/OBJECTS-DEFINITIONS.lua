-- ***********************************************************
Project : CICS BMS ncurses WYSIWYG Editor
File    : OBJECTS-DEFINITIONS.lua
Designed-by : Sebastien Genose.org
Date    : 2024-06-20
Description : This file contains the definitions of the base object representation for BMS fields, which can be used to create various types of fields in a BMS application.
Description : The OBJECTS_DEFINITIONS table defines the properties and attributes of each field type, including their initial values, visual representations, and available options. The visual representation of each field type is defined using template ASCII characters, which can be customized as needed.
Description : The OBJECTS_DEFINITIONS table also includes properties for field height, length, border style, color, font, style, text alignment, position, and attributes. Each property has an initial value and an edited value, which can be modified as needed.
Description : The OBJECTS_DEFINITIONS table is designed to be flexible and extensible, allowing for the creation of custom field types and visual representations. The visual representation of each field type can be drawn line by line, with each line being drawn with appropriate properties such as color, font, style, etc.
Description : The OBJECTS_DEFINITIONS table is intended to be used in conjunction with the BMS application, which can render the fields based on their visual representations and properties. The BMS application can also handle user input and modify the properties of the fields as needed.
Description : The OBJECTS_DEFINITIONS can be created and managed using the OBJECTS_DEFINITIONS table, which provides a flexible and extensible way to define and manage the properties and attributes of each field type. The visual representation of each field type can be customized to fit the needs of the application, providing a powerful way to create user interfaces for BMS applications.
Description : All OBJECTS_DEFINITIONS Field have at least 3 row (field_height, field_height_min) and 3 col (field_width). The visual representation of each field type can be customized to fit the needs of the application, providing a powerful way to create user interfaces for BMS applications. The visual representation of each field type can be drawn line by line, with each line being drawn with appropriate properties such as color, font, style, etc.
Description : Each created object is internally saved in JSON format, which can be used to store and retrieve the properties and attributes of each field type. The JSON format provides a standardized way to represent the properties and attributes of each field type, allowing for easy integration with other applications and systems. The JSON format can also be used to export and import the properties and attributes of each field type, providing a flexible way to manage the fields in a BMS application.
Description : an instance of OBJECTS_DEFINITIONS can be created using the OBJECTS_DEFINITIONS.new(TYPE), which initialize an object with the specified TYPE, and set the initial values for each property based on the definitions in the OBJECTS_DEFINITIONS table. The new object can then be modified and customized as needed, allowing for the creation of custom field types and visual representations. The new object can also be saved in JSON format, providing a standardized way to store and retrieve the properties and attributes of each field type.
Description : an instance of OBJECTS_DEFINITIONS handle initial value and editing value state, which allow less code and memory usage, and allow to easily manage the properties and attributes of each field type. The initial value represents the default state of the field, while the edited value represents the modified state of the field after user input or other changes. The initial and edited values can be used to determine the current state of the field, allowing for easy management of the properties and attributes of each field type.
    -- ***********************************************************
All OBJECTS_DEFINITIONS Field have at least 3 row (field_height, field_height_min) and 3 col (field_width).


-- ***********************************************************
typedef OBJECTS_DEFINITIONS={
    field_name={-- Name of the object
        initial="....OBJECTS_NAME....",
        edited=nil,
    },
    field_type={-- Type of the field, can be "Field", "Literal", "ProtectedLiteral", "BooleanField", "Image", "Line", "Fieldset"
        avail_initial={"Field", "Literal", "ProtectedLiteral", "BooleanField", "Image", "Line", "Fieldset"}, -- BMS available field types
        initial="Field",-- Default field type
        edited="Field",-- Field type after editing
    },

    field_height={-- Height of the field, can be any positive integer
        avail_initial={"Field":3, "Literal":3, "ProtectedLiteral":3, "BooleanField":3, "Image":5, "Line":1, "Fieldset":3},--  Default height for each field type
        initial={"Field":3}, -- Default height for the initial field type
        edited={"Field":3},
    },
    
    field_min_height={-- Height of the field, can be any positive integer
        avail_initial={"Field":3, "Literal":3, "ProtectedLiteral":3, "BooleanField":3, "Image":5, "Line":1, "Fieldset":3},--  Default height for each field type
        initial={"Field":3}, -- Default height for the initial field type
        edited={"Field":3},
    },
    
    field_max_height={-- Maximum height of the field, can be any positive integer
        avail_initial={"Field":80, "Literal":80, "ProtectedLiteral":80, "BooleanField":3, "Image":40, "Line":1, "Fieldset":80},--  Default max height for each field type
        initial={"Field":255}, -- Default max height for the initial field type
        edited={"Field":255},
    },


    field_width={-- Length of the field, can be any positive integer
        avail_initial={"Field":10, "Literal":20, "ProtectedLiteral":20, "BooleanField":10, "Image":40, "Line":40, "Fieldset":40},-- Default length for each field type
        initial={"Field":10}, -- Default length for the initial field type
        edited={"Field":10},
    },
    
    field_width_max={-- Maximum length of the field, can be any positive integer
        avail_initial={"Field":255, "Literal":255, "ProtectedLiteral":255, "BooleanField":255, "Image":255, "Line":255, "Fieldset":255},-- Default max length for each field type
        initial={"Field":255}, -- Default max length for the initial field type
        edited={"Field":255},
    },
    
    field_width_min={-- Minimum length of the field, can be any positive integer
        avail_initial={"Field":3, "Literal":3, "ProtectedLiteral":3, "BooleanField":10, "Image":1, "Line":1, "Fieldset":3},-- Default min length for each field type
        initial={"Field":3}, -- Default min length for the initial field type
        edited={"Field":3},
    },

    field_border={
        avail_initial={"Field":"none", "Literal":"none", "ProtectedLiteral":"none", "BooleanField":"none", "Image":"none", "Line":"none", "Fieldset":"none", "none":"none"},-- Default border style
        initial={"none":"none"},-- Default border style
        edited={"none":"none"},-- Border style after editing
    },
    
    field_footer={
        initial=nil, -- nil = pas de footer
        edited=nil
    },
    
    field_border_style={
        avail_initial={
            "Field":["single", "double", "none"],
            "Literal":["single", "double", "none"], 
            "ProtectedLiteral":["single", "double", "none"],
            "BooleanField":["single", "double", "none"],
            "Image":["single", "double", "none"],
            "Line":["single", "double", "none"], 
            "Fieldset":["single", "double", "none"]
        },-- Default border style for each field type
        initial={"Field":"none"}, -- Default border style for the initial field type
        edited={"Field":"none"}, -- Border style after editing
    },
    
    -- ===== COULEURS =====
    field_border_color={
        avail_initial={"Field":"default", "Literal":"default", "ProtectedLiteral":"default", "BooleanField":"default", "Image":"default", "Line":"default", "Fieldset":"blue"},
        initial="default",
        edited="default"
    },
    
    field_title_color={
        avail_initial={"Field":"default", "Literal":"default", "ProtectedLiteral":"default", "BooleanField":"default", "Image":"default", "Line":"default", "Fieldset":"yellow"},
        initial="default",
        edited="default"
    },
    
    field_text_color={
        avail_initial={"Field":"white", "Literal":"green", "ProtectedLiteral":"gray", "BooleanField":"cyan", "Image":"default", "Line":"default", "Fieldset":"default"},
        initial="default",
        edited="default"
    },
    
    field_footer_color={
        avail_initial={"Field":"default", "Literal":"default", "ProtectedLiteral":"default", "BooleanField":"default", "Image":"default", "Line":"default", "Fieldset":"red"},
        initial="default",
        edited="default"
    },
    
    field_color={
        avail_initial={"Field":"default", "Literal":"default", "ProtectedLiteral":"default", "BooleanField":"default", "Image":"default", "Line":"default", "Fieldset":"default"},-- Default color for each field type
        initial={"Field":"default"}, -- Default color for the initial field type
        edited={"Field":"default"}, -- Color after editing
    },
    
    field_font={
        avail_initial={"Field":"default", "Literal":"default", "ProtectedLiteral":"default", "BooleanField":"default", "Image":"default", "Line":"default", "Fieldset":"default"},-- Default font for each field type
        initial={"Field":"default"}, -- Default font for the initial field type
        edited={"Field":"default"}, -- Font after editing
    },
    
    field_style={
        avail_initial={"Field":["default", "bold", "italic", "underline", "strikethrough"], 
                       "Literal":["default", "bold", "italic", "underline", "strikethrough"], 
                       "ProtectedLiteral":["default", "bold", "italic", "underline", "strikethrough"], 
                       "BooleanField":["default", "bold", "italic", "underline", "strikethrough"], 
                       "Image":["default", "bold", "italic", "underline", "strikethrough"], 
                       "Line":["default", "bold", "italic", "underline", "strikethrough"], 
                       "Fieldset":["default", "bold", "italic", "underline", "strikethrough"]
                    },-- Default style for each field type
        initial={"Field":"default"}, -- Default style for the initial field type
        edited={"Field":"default"}, -- Style after editing
    },
    
    field_text_align={
        avail_initial= {
            "Field":["left", "center", "right"], 
            "Literal":["left", "center", "right"],
            "ProtectedLiteral":["left", "center", "right"],
            "BooleanField":["left", "center", "right"],
            "Image":["left", "center", "right"],
            "Line":["left", "center", "right"],
            "Fieldset":["left", "center", "right"]
        },-- Default text alignment for each field type
        initial={"Field":"left"}, -- Default text alignment for the initial field type
        edited={"Field":"left"}, -- Text alignment after editing
    },
    
    field_pos={
        initial={"col":0, "row":0, "rowend":0, "colend":0},-- Default position
        edited={"col":0, "row":0, "rowend":0, "colend":0},-- Position after editing
    },
    
    -- ===== PERSONNALISATION DES CARACTERES =====
    -- Caractères de bordure personnalisables (pour remplacer ┌─┐│├└┘)
    field_border_chars={
        avail_initial={
            ["Field"] = {top_left = "", top = "", top_right = "", left = "", right = "", bottom_left = "", bottom = "", bottom_right = ""},
            ["Literal"] = {top_left = "", top = "", top_right = "", left = "", right = "", bottom_left = "", bottom = "", bottom_right = ""},
            ["ProtectedLiteral"] = {top_left = "", top = "", top_right = "", left = "", right = "", bottom_left = "", bottom = "", bottom_right = ""},
            ["BooleanField"] = {top_left = "", top = "", top_right = "", left = "", right = "", bottom_left = "", bottom = "", bottom_right = ""},
            ["Image"] = {top_left = "", top = "", top_right = "", left = "", right = "", bottom_left = "", bottom = "", bottom_right = ""},
            ["Line"] = {top_left = "", top = "─", top_right = "", left = "", right = "", bottom_left = "", bottom = "─", bottom_right = ""},
            ["Fieldset"] = {top_left = "┌", top = "─", top_right = "┐", left = "│", right = "│", bottom_left = "└", bottom = "─", bottom_right = "┘"}
        },
        initial = {top_left = "┌", top = "─", top_right = "┐", left = "│", right = "│", bottom_left = "└", bottom = "─", bottom_right = "┘"},
        edited = {}
    },
    
    -- Caractère de remplissage pour le titre dans la bordure supérieure
    field_title_fill_char={
        avail_initial={
            ["Field"] = " ",
            ["Literal"] = " ",
            ["ProtectedLiteral"] = " ",
            ["BooleanField"] = " ",
            ["Image"] = " ",
            ["Line"] = "─",
            ["Fieldset"] = "─"
        },
        initial = "─",
        edited = "─"
    },
    
    -- Caractère de remplissage pour les champs vides (ex: "_" pour Field)
    field_fill_char={
        avail_initial={
            ["Field"] = "_",
            ["Literal"] = " ",
            ["ProtectedLiteral"] = " ",
            ["BooleanField"] = " ",
            ["Image"] = " ",
            ["Line"] = "─",
            ["Fieldset"] = " "
        },
        initial = "_",
        edited = "_"
    },
    
    -- ===== ALIGNEMENT VERTICAL =====
    field_vertical_align={
        avail_initial={
            ["Field"] = "middle",
            ["Literal"] = "middle",
            ["ProtectedLiteral"] = "middle",
            ["BooleanField"] = "middle",
            ["Image"] = "middle",
            ["Line"] = "middle",
            ["Fieldset"] = "top"
        },
        initial = "middle",
        edited = "middle"
    },
    
    field_vertical_margin={
        initial = 0,
        edited = 0
    },
    
    -- ===== AUTRES PROPRIETES =====
    -- Prefixe du titre (ex: "✱ " pour Fieldset requis)
    field_title_prefix={
        avail_initial={"Field":"", "Literal":"", "ProtectedLiteral":"", "BooleanField":"", "Image":"", "Line":"", "Fieldset":"✱ "},
        initial="",
        edited=""
    },
    
    -- Marqueur pour les champs requis (ex: " *")
    field_required_marker={
        initial=" *",
        edited=" *"
    },
    
    -- Footer pour la ligne N (ex: "* = required fields")
    field_footer={
        initial=nil,
        edited=nil
    },
    
    -- Alignement du titre (left/center/right)
    field_title_align={
        avail_initial={"Field":"left", "Literal":"left", "ProtectedLiteral":"left", "BooleanField":"left", "Image":"left", "Line":"left", "Fieldset":"left"},
        initial="left",
        edited="left"
    },
    
    -- Champs enfants (pour Fieldset)
    field_children={
        initial = {},
        edited = {}
    },
    
    field_attrb={
        avail_initial={"Field": ["Prot", "Num", "Alpha", "Hidden", "Highlite", "Focus", "Select"],
                       "Literal": ["Prot", "Num", "Alpha", "Hidden", "Highlite", "Focus", "Select"],
                       "ProtectedLiteral": ["Prot", "Num", "Alpha", "Hidden", "Highlite", "Focus", "Select"],
                       "BooleanField": ["Prot", "Hidden", "Highlite", "Focus", "Select"],
                       "Image": [],
                       "Line": [],
                       "Fieldset": ["Prot", "Hidden", "Highlite", "Focus", "Select"]
                    }, -- BMS available field attributes
        initial=nil, -- Default field attribute
        edited=nil, -- Field attribute after editing
    },
    
    field_initial={-- for fieldset/group, represents the title of the fieldset/group; for image, represents the ASCII code + file path; for other field types, represents the initial value
        avail_initial={
            "Field" = {initial_value = "text", option_value = nil},
            "Literal" = {initial_value = "text", option_value = nil},
            "ProtectedLiteral" = {initial_value = "text", option_value = nil},
            "BooleanField" = {initial_value = false, option_value = nil},  -- Case non cochée par défaut
            "Image" = {initial_value = nil, option_value = nil},
            "Line" = {initial_value = nil, option_value = nil},
            "Fieldset" = {initial_value = "title", option_value = nil}
        },
        
        initial_value = {"Field" = {initial_value = "text", option_value = nil}},
        edited = {"Field" = {initial_value = "text", option_value = nil}}
    },
    
    field_editable={
        initial=true,-- Default editable state
        edited=true,-- Editable state after editing
    },
    
    field_visible={
        initial=true,-- Default visible state
        edited=true,-- Visible state after editing
    },
    
    field_required={
        initial=false,-- Default required state
        edited=false,-- Required state after editing
    },
    
    field_readonly={
        initial=false,-- Default readonly state
        edited=false,-- Readonly state after editing
    },
    
    field_enabled={
        initial=true,-- Default enabled state
        edited=true,-- Enabled state after editing
    },
    
    field_focused={
        initial=false,-- Default focused state
        edited=false,-- Focused state after editing
    },
    
    field_selected={
        initial=false,-- Default selected state
        edited=false,-- Selected state after editing
    },
    
    field_highlighted={
        initial=false,-- Default highlighted state
        edited=false,-- Highlighted state after editing
    },
    
    field_hidden={
        initial=false,-- Default hidden state
        edited=false,-- Hidden state after editing
    },
    
    field_protected={
        initial=false,-- Default protected state
        edited=false,-- Protected state after editing
    },
    
    field_numeric={
        initial=false,-- Default numeric state
        edited=false,-- Numeric state after editing
    },

    visual_representation={-- Represents the visual representation of each field type
        -- line 0: reserved for border top + title (for fieldset/group)
        -- line 1 to N-1: reserved for border left/right + content/value
        -- line N: reserved for border bottom + footer
        
        -- Helper function to get property value respecting the hierarchy: edited -> initial -> avail_initial[field_type]
        property_value = function(prop, field_type)
            if prop == nil then return nil end
            local edited_val = (type(prop.edited) == "table" and prop.edited[field_type] or prop.edited)
            if edited_val ~= nil then return edited_val end
            local initial_val = (type(prop.initial) == "table" and prop.initial[field_type] or prop.initial)
            if initial_val ~= nil then return initial_val end
            if prop.avail_initial and prop.avail_initial[field_type] then
                return prop.avail_initial[field_type]
            end
            return nil
        end,
        
        avail_default={
            ["Field"] = {
                0 = nil,
                1 = function(obj)
                    local label = "[" .. (obj.field_name.edited or obj.field_name.initial or obj.field_type.initial) .. "]"
                    local width = obj.visual_representation.property_value(obj.field_width, obj.field_type.initial) or obj.field_width_min.avail_initial[obj.field_type.initial]
                    local fill_char = obj.visual_representation.property_value(obj.field_fill_char, obj.field_type.initial)
                    local initial_val = obj.visual_representation.property_value(obj.field_initial, obj.field_type.initial) or {initial_value = ""}
                    local value = initial_val.initial_value or string.rep(fill_char, width - #label)
                    local required_marker = obj.field_required.initial and obj.visual_representation.property_value(obj.field_required_marker, obj.field_type.initial) or ""
                    return label .. value .. required_marker
                end,
                2 = nil
            },
            ["Literal"] = {
                0 = nil,
                1 = function(obj)
                    local initial_val = obj.visual_representation.property_value(obj.field_initial, obj.field_type.initial) or {initial_value = "Static Text"}
                    return "[" .. (obj.field_name.edited or obj.field_name.initial or obj.field_type.initial) .. "]" .. initial_val.initial_value
                end,
                2 = nil
            },
            ["ProtectedLiteral"] = {
                0 = nil,
                1 = function(obj)
                    local prefix = obj.field_protected.edited and "(Protected) " or obj.field_protected.initial and "(Protected) " or ""
                    local initial_val = obj.visual_representation.property_value(obj.field_initial, obj.field_type.initial) or {initial_value = "Text"}
                    return "[" .. (obj.field_name.edited or obj.field_name.initial or obj.field_type.initial) .. "]" .. prefix .. initial_val.initial_value
                end,
                2 = nil
            },
            ["BooleanField"] = {
                0 = nil,
                1 = function(obj)
                    local initial_val = obj.visual_representation.property_value(obj.field_initial, obj.field_type.initial) or {initial_value = false}
                    local checked = initial_val.initial_value and "✓" or " "
                    return "[ " .. checked .. " ] " .. (obj.field_name.edited or obj.field_name.initial or obj.field_type.initial)
                end,
                2 = nil
            },
            ["Image"] = {
                0 = nil,
                1 = function(obj)
                    local initial_val = obj.visual_representation.property_value(obj.field_initial, obj.field_type.initial) or {initial_value = {"[Image Placeholder]"}}
                    local content = initial_val.initial_value
                    if type(content) == "string" then content = {content} end
                    return table.concat(content, "\n")
                end,
                2 = nil
            },
            ["Line"] = {
                0 = nil,
                1 = function(obj)
                    local fill_char = obj.visual_representation.property_value(obj.field_fill_char, obj.field_type.initial)
                    local width = obj.visual_representation.property_value(obj.field_width, obj.field_type.initial) or obj.field_width_min.avail_initial[obj.field_type.initial]
                    return string.rep(fill_char, width)
                end,
                2 = nil
            },
            ["Fieldset"] = {
                0 = function(obj)
                    local initial_val = obj.visual_representation.property_value(obj.field_initial, obj.field_type.initial) or {initial_value = "Untitled"}
                    local title = initial_val.initial_value or obj.field_name.edited or obj.field_name.initial or obj.field_type.initial
                    local prefix = obj.field_required.initial and obj.visual_representation.property_value(obj.field_title_prefix, obj.field_type.initial) or ""
                    local align = obj.visual_representation.property_value(obj.field_title_align, obj.field_type.initial)
                    local width = obj.visual_representation.property_value(obj.field_width, obj.field_type.initial) or obj.field_width_min.avail_initial[obj.field_type.initial]
                    
                    local border_chars = obj.visual_representation.property_value(obj.field_border_chars, obj.field_type.initial)
                    local top_left = border_chars.top_left or "┌"
                    local top = border_chars.top or "─"
                    local top_right = border_chars.top_right or "┐"
                    
                    local fill_char = obj.visual_representation.property_value(obj.field_title_fill_char, obj.field_type.initial)
                    
                    local title_block = "[" .. prefix .. title .. "]"
                    local padding_left, padding_right = 0, 0
                    if align == "center" then
                        padding_left = math.floor((width - #title_block) / 2)
                        padding_right = width - #title_block - padding_left
                    elseif align == "right" then
                        padding_left = width - #title_block
                    end
                    
                    return top_left .. string.rep(fill_char, padding_left) .. title_block ..
                           string.rep(fill_char, padding_right) .. top_right
                end,
                
                1 = function(obj)
                    local width = obj.visual_representation.property_value(obj.field_width, obj.field_type.initial) or obj.field_width_min.avail_initial[obj.field_type.initial]
                    local border_chars = obj.visual_representation.property_value(obj.field_border_chars, obj.field_type.initial)
                    local vert_char = border_chars.left or "│"
                    local vertical_align = obj.visual_representation.property_value(obj.field_vertical_align, obj.field_type.initial)
                    local margin = obj.visual_representation.property_value(obj.field_vertical_margin, obj.field_type.initial)
                    local height = obj.visual_representation.property_value(obj.field_height, obj.field_type.initial) or obj.field_min_height.avail_initial[obj.field_type.initial]
                    local available_lines = (height - 2 - margin)
                    
                    if obj.field_children and obj.field_children.initial and #obj.field_children.initial > 0 then
                        local child_lines = {}
                        for _, child in ipairs(obj.field_children.initial) do
                            local child_str = render_object(child)
                            if child_str then
                                for subline in child_str:gmatch("[^\n]+") do
                                    table.insert(child_lines, subline)
                                end
                            end
                        end
                        
                        local total_child_height = #child_lines
                        
                        if vertical_align == "top" then
                            for i, line in ipairs(child_lines) do
                                table.insert(child_lines, i, vert_char .. " " .. line .. string.rep(" ", width - #line - 2) .. " " .. vert_char)
                            end
                            for i = 1, available_lines - total_child_height do
                                table.insert(child_lines, vert_char .. string.rep(" ", width) .. vert_char)
                            end
                        elseif vertical_align == "bottom" then
                            for i = 1, available_lines - total_child_height do
                                table.insert(child_lines, 1, vert_char .. string.rep(" ", width) .. vert_char)
                            end
                            for i, line in ipairs(child_lines) do
                                if i <= #child_lines - (available_lines - total_child_height) then
                                    child_lines[i] = vert_char .. " " .. line .. string.rep(" ", width - #line - 2) .. " " .. vert_char
                                end
                            end
                        else -- middle
                            local top_margin = math.floor((available_lines - total_child_height) / 2)
                            local bottom_margin = available_lines - total_child_height - top_margin
                            
                            for i = 1, top_margin do
                                table.insert(child_lines, 1, vert_char .. string.rep(" ", width) .. vert_char)
                            end
                            for i = top_margin + 1, top_margin + total_child_height do
                                local line = child_lines[i - top_margin] or ""
                                child_lines[i] = vert_char .. " " .. line .. string.rep(" ", width - #line - 2) .. " " .. vert_char
                            end
                            for i = 1, bottom_margin do
                                table.insert(child_lines, vert_char .. string.rep(" ", width) .. vert_char)
                            end
                        end
                        return table.concat(child_lines, "\n")
                    else
                        local lines = {}
                        for i = 1, available_lines do
                            table.insert(lines, vert_char .. string.rep(" ", width) .. vert_char)
                        end
                        return table.concat(lines, "\n")
                    end
                end,
                
                2 = function(obj)
                    local width = obj.visual_representation.property_value(obj.field_width, obj.field_type.initial) or obj.field_width_min.avail_initial[obj.field_type.initial]
                    local border_chars = obj.visual_representation.property_value(obj.field_border_chars, obj.field_type.initial)
                    local bottom_left = border_chars.bottom_left or "└"
                    local bottom = border_chars.bottom or "─"
                    local bottom_right = border_chars.bottom_right or "┘"
                    local footer = obj.visual_representation.property_value(obj.field_footer, obj.field_type.initial) or ""
                    
                    if footer ~= "" then
                        local fill_char = obj.visual_representation.property_value(obj.field_title_fill_char, obj.field_type.initial)
                        local footer_padding = width - #footer - 2
                        return bottom_left .. string.rep(fill_char, footer_padding) .. " " .. footer .. " " .. bottom_right
                    else
                        return bottom_left .. string.rep(bottom, width) .. bottom_right
                    end
                end
            }
        },
        
        avail_initial={
            ["Field"] = {0 = nil, 1 = "[Field]________", 2 = nil},
            ["Literal"] = {0 = nil, 1 = "[Literal]Static Text", 2 = nil},
            ["ProtectedLiteral"] = {0 = nil, 1 = "[ProtectedLiteral](Protected) Text", 2 = nil},
            ["BooleanField"] = {0 = nil, 1 = "[ ] Boolean", 2 = nil},
            ["Image"] = {0 = nil, 1 = "[Image Placeholder]", 2 = nil},
            ["Line"] = {0 = nil, 1 = "────────────────────────", 2 = nil},
            ["Fieldset"] = {
                0 = "┌─[✱ Title]─────────────────────┐",
                1 = "│ [Field1]__________ [Field2]___ │",
                2 = "└────────── * = required fields ──┘"
            }
        },
        
        initial={
            ["Field"] = "[Field]________",
            ["BooleanField"] = "[ ] Boolean",
            ["Fieldset"] = {
                0 = "┌─[✱ Title]─────────────────────┐",
                1 = "│                                   │",
                2 = "└────────── * = required fields ──┘"
            }
        }
    },

    visual_representation_edited={
       output=nil, -- Visual representation after editing
    },
}

-- ===== FONCTION DE RENDU GENÉRIQUE =====
-- Rend n'importe quel objet (Field, Fieldset, etc.) selon ses propriétés
function render_object(obj)
    if not obj or not obj.field_type or not obj.field_type.initial then
        return "[Invalid Object]"
    end
    
    local type = obj.field_type.initial
    local template = obj.visual_representation.avail_default[type]
    
    if not template then
        return "[" .. type .. "]"
    end
    
    local lines = {}
    for i = 0, 2 do
        if template[i] then
            if type(template[i]) == "function" then
                local line = template[i](obj)
                if line then
                    for subline in line:gmatch("[^\n]+") do
                        table.insert(lines, subline)
                    end
                end
            else
                table.insert(lines, tostring(template[i]))
            end
        end
    end
    
    if #lines == 0 then
        return "[" .. (obj.field_name.edited or obj.field_name.initial or obj.field_type.initial) .. "]"
    end
    
    return table.concat(lines, "\n")
end


-- ===== HELPERS POUR NCURSES (À UTILISER AVEC LA BIBLIOTHÈQUE NCURSES) =====
-- Ces fonctions sont des placeholders pour l'intégration avec ncurses
-- En pratique, vous devrez initialiser ncurses et définir les paires de couleurs

-- Table de mapping des couleurs (à initialiser avec start_color() en ncurses)
local COLOR_MAP = {
    default = 0,
    black = 1,
    red = 2,
    green = 3,
    yellow = 4,
    blue = 5,
    magenta = 6,
    cyan = 7,
    white = 8
}

-- Table de mapping des styles ncurses
local STYLE_MAP = {
    default = 0,
    bold = 1,      -- A_BOLD
    italic = 2,    -- A_ITALIC (si supporté)
    underline = 4, -- A_UNDERLINE
    strikethrough = 8,
    blink = 16,    -- A_BLINK
    reverse = 32   -- A_REVERSE
}

-- Fonction pour appliquer couleur + style (placeholder pour ncurses)
function apply_attributes(color, style)
    -- En pratique:
    -- local attr = 0
    -- if color and color ~= "default" and COLOR_MAP[color] then
    --     attr = attr + COLOR_PAIR(COLOR_MAP[color])
    -- end
    -- if style and style ~= "default" and STYLE_MAP[style] then
    --     attr = attr + STYLE_MAP[style]
    -- end
    -- attron(attr)
    -- Pour ce fichier standalone, on ne fait rien
end

-- Fonction pour désactiver les attributs (placeholder)
function reset_attributes()
    -- En pratique: attroff(A_ALL)
end


-- ===== EXEMPLES D'UTILISATION =====

-- Exemple 1: Fieldset standard avec bordures Unicode
local standardFieldset = OBJECTS_DEFINITIONS.new({
    field_name = {initial = "LoginForm"},
    field_type = {initial = "Fieldset"},
    field_initial = {initial_value = "User Login"},
    field_required = {initial = true},
    field_width = {initial = 50},
    field_border_style = {initial = "single"},
    field_vertical_align = {initial = "middle"},
    field_children = {initial = {
        OBJECTS_DEFINITIONS.new({
            field_type = {initial = "Field"},
            field_name = {initial = "Username"},
            field_width = {initial = 20},
            field_required = {initial = true}
        }),
        OBJECTS_DEFINITIONS.new({
            field_type = {initial = "Field"},
            field_name = {initial = "Password"},
            field_width = {initial = 20},
            field_protected = {initial = true},
            field_required = {initial = true}
        })
    }}
});

print("=== Exemple 1: Fieldset Standard ===")
print(render_object(standardFieldset))

-- Exemple 2: Fieldset avec bordures en asterisques
local starFieldset = OBJECTS_DEFINITIONS.new({
    field_type = {initial = "Fieldset"},
    field_initial = {initial_value = "Custom Box"},
    field_width = {initial = 40},
    field_border_chars = {initial = {
        top_left = "*", top = "*", top_right = "*",
        left = "*", right = "*",
        bottom_left = "*", bottom = "*", bottom_right = "*"
    }},
    field_title_fill_char = {initial = "*"},
    field_children = {initial = {
        OBJECTS_DEFINITIONS.new({
            field_type = {initial = "Field"},
            field_name = {initial = "Name"},
            field_width = {initial = 15}
        })
    }}
});

print("\n=== Exemple 2: Bordures en Asterisques ===")
print(render_object(starFieldset))

-- Exemple 3: Fieldset avec bordures mixtes et remplissage egal
local mixedFieldset = OBJECTS_DEFINITIONS.new({
    field_type = {initial = "Fieldset"},
    field_initial = {initial_value = "Options"},
    field_width = {initial = 40},
    field_border_chars = {initial = {
        top_left = "+", top = "=", top_right = "+",
        left = "|", right = "|",
        bottom_left = "+", bottom = "-", bottom_right = "+"
    }},
    field_title_fill_char = {initial = "="},
    field_footer = {initial = "* required"},
    field_children = {initial = {
        OBJECTS_DEFINITIONS.new({
            field_type = {initial = "BooleanField"},
            field_name = {initial = "Enable"},
            field_initial = {initial_value = true}
        })
    }}
});

print("\n=== Exemple 3: Bordures Mixtes ===")
print(render_object(mixedFieldset))

-- Exemple 4: Champ avec remplissage en points
local dottedField = OBJECTS_DEFINITIONS.new({
    field_type = {initial = "Field"},
    field_name = {initial = "Input"},
    field_width = {initial = 20},
    field_fill_char = {initial = "·"},
    field_required = {initial = true}
});

print("\n=== Exemple 4: Remplissage en Points ===")
print(render_object(dottedField))

-- Exemple 5: Ligne avec tirets
local dashLine = OBJECTS_DEFINITIONS.new({
    field_type = {initial = "Line"},
    field_width = {initial = 30},
    field_fill_char = {initial = "-"}
});

print("\n=== Exemple 5: Ligne en Tirets ===")
print(render_object(dashLine))

-- Exemple 6: Fieldset avec alignement vertical centré
local centeredFieldset = OBJECTS_DEFINITIONS.new({
    field_type = {initial = "Fieldset"},
    field_initial = {initial_value = "Centered"},
    field_width = {initial = 40},
    field_height = {initial = 7}, -- 7 lignes pour voir l'alignement
    field_border_style = {initial = "single"},
    field_vertical_align = {initial = "middle"},
    field_children = {initial = {
        OBJECTS_DEFINITIONS.new({
            field_type = {initial = "Field"},
            field_name = {initial = "Data"},
            field_width = {initial = 10}
        })
    }}
});

print("\n=== Exemple 6: Alignement Vertical Centre ===")
print(render_object(centeredFieldset))

-- Exemple 7: Fieldset avec toutes les personnalisations
local fullyCustomFieldset = OBJECTS_DEFINITIONS.new({
    field_type = {initial = "Fieldset"},
    field_initial = {initial_value = "Fully Custom"},
    field_required = {initial = true},
    field_width = {initial = 50},
    field_border_style = {initial = "double"},
    field_border_chars = {initial = {
        top_left = "╔", top = "═", top_right = "╗",
        left = "║", right = "║",
        bottom_left = "╚", bottom = "═", bottom_right = "╝"
    }},
    field_title_fill_char = {initial = "═"},
    field_title_align = {initial = "center"},
    field_title_prefix = {initial = "✱ "},
    field_vertical_align = {initial = "top"},
    field_footer = {initial = "* = required fields"},
    field_children = {initial = {
        OBJECTS_DEFINITIONS.new({
            field_type = {initial = "Field"},
            field_name = {initial = "User"},
            field_width = {initial = 15},
            field_fill_char = {initial = "_"},
            field_required = {initial = true}
        }),
        OBJECTS_DEFINITIONS.new({
            field_type = {initial = "Field"},
            field_name = {initial = "Email"},
            field_width = {initial = 20},
            field_fill_char = {initial = "_"},
            field_required = {initial = true}
        }),
        OBJECTS_DEFINITIONS.new({
            field_type = {initial = "BooleanField"},
            field_name = {initial = "Active"},
            field_initial = {initial_value = true}
        })
    }}
});

print("\n=== Exemple 7: Fieldset Complètement Personnalise ===")
print(render_object(fullyCustomFieldset))
