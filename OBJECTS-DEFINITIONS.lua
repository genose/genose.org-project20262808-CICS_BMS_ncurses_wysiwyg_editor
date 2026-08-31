-- ***********************************************************
-- Project : CICS BMS ncurses WYSIWYG Editor
-- File    : OBJECTS-DEFINITIONS.lua
-- Designed-by : Sebastien Genose.org
-- Date    : 2024-06-20
-- Description : This file contains the definitions of the base object representation for BMS fields, which can be used to create various types of fields in a BMS application.
-- Description : The OBJECTS_DEFINITIONS table defines the properties and attributes of each field type, including their initial values, visual representations, and available options. The visual representation of each field type is defined using template ASCII characters, which can be customized as needed.
-- Description : The OBJECTS_DEFINITIONS table also includes properties for field height, length, border style, color, font, style, text alignment, position, and attributes. Each property has an initial value and an edited value, which can be modified as needed.
-- Description : The OBJECTS_DEFINITIONS table is designed to be flexible and extensible, allowing for the creation of custom field types and visual representations. The visual representation of each field type can be drawn line by line, with each line being drawn with appropriate properties such as color, font, style, etc.
-- Description : The OBJECTS_DEFINITIONS table is intended to be used in conjunction with the BMS application, which can render the fields based on their visual representations and properties. The BMS application can also handle user input and modify the properties of the fields as needed.
-- Description : The OBJECTS_DEFINITIONS can be created and managed using the OBJECTS_DEFINITIONS table, which provides a flexible and extensible way to define and manage the properties and attributes of each field type. The visual representation of each field type can be customized to fit the needs of the application, providing a powerful way to create user interfaces for BMS applications.
-- Description : All OBJECTS_DEFINITIONS Field have at least 3 row (field_height, field_height_min) and 3 col (field_width). The visual representation of each field type can be customized to fit the needs of the application, providing a powerful way to create user interfaces for BMS applications. The visual representation of each field type can be drawn line by line, with each line being drawn with appropriate properties such as color, font, style, etc.
-- Description : Each created object is internally saved in JSON format, which can be used to store and retrieve the properties and attributes of each field type. The JSON format provides a standardized way to represent the properties and attributes of each field type, allowing for easy integration with other applications and systems. The JSON format can also be used to export and import the properties and attributes of each field type, providing a flexible way to manage the fields in a BMS application.
-- Description : an instance of OBJECTS_DEFINITIONS can be created using the OBJECTS_DEFINITIONS.new(TYPE), which initialize an object with the specified TYPE, and set the initial values for each property based on the definitions in the OBJECTS_DEFINITIONS table. The new object can then be modified and customized as needed, allowing for the creation of custom field types and visual representations. The new object can also be saved in JSON format, providing a standardized way to store and retrieve the properties and attributes of each field type.
-- Description : an instance of OBJECTS_DEFINITIONS handle initial value and editing value state, which allow less code and memory usage, and allow to easily manage the properties and attributes of each field type. The initial value represents the default state of the field, while the edited value represents the modified state of the field after user input or other changes. The initial and edited values can be used to determine the current state of the field, allowing for easy management of the properties and attributes of each field type.
--     -- ***********************************************************
-- All OBJECTS_DEFINITIONS Field have at least 3 row (field_height, field_height_min) and 3 col (field_width).
-- 
-- ***********************************************************
OBJECTS_DEFINITIONS_GUI_TYPE =
    { -- definition of the GUI type for each field type, which can be used to render the fields in a WYSIWYG editor. The GUI type can be used to determine the visual representation of each field type, allowing for a flexible and extensible way to create user interfaces for BMS applications.
        gui_field_type = {
            gui_select_with_label_string = "gui_select_with_label_string", -- rendu graphique d'un select box (liste de choix) avec label et des kprops de type string
            gui_select_with_label_numeric = "gui_select_with_label_numeric", -- rendu graphique d'un select box (liste de choix) avec label et des kprops de type numeric
            gui_list_textornum_with_label_field = "gui_list_textornum_with_label_field", -- rendu graphique d'un field (liste de choix) avec label et des kprops de type string ou numeric
            gui_checkbox_with_label_field = "gui_checkbox_with_label_field", -- rendu graphique d'un field (checkbox avec label) avec des kprops de type string ou numeric
            gui_text_with_label_field = "gui_text_with_label_field" -- rendu graphique d'un field (text avec label) avec des kprops de type string ou numeric
        }

    }

OBJECTS_DEFINITIONS = {
    field_name = { -- Name of the object
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_text_with_label_field,
        enum = {
            Field = "Field",
            Literal = "Literal",
            ProtectedLiteral = "ProtectedLiteral",
            BooleanField = "BooleanField",
            Image = "Image",
            Line = "Line",
            Fieldset = "Fieldset"
        },
        default = {
            Field = "Field",
            Literal = "Literal",
            ProtectedLiteral = "ProtectedLiteral",
            BooleanField = "BooleanField",
            Image = "Image",
            Line = "Line",
            Fieldset = "Fieldset"
        }, -- Available field names
        initial = nil, -- Default field name
        edited = nil -- Field name after editing
    },

    field_type = { -- Type of the field, can be Field, Literal, ProtectedLiteral, BooleanField, Image, Line, Fieldset
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        enum = {
            Field = "Field",
            Literal = "Literal",
            ProtectedLiteral = "ProtectedLiteral",
            BooleanField = "BooleanField",
            Image = "Image",
            Line = "Line",
            Fieldset = "Fieldset"
        },
        default = nil, -- Available field types    
        initial = nil, -- Default field type
        edited = nil
    },

    field_min_height = { -- Height of the field, can be any positive integer
        enum = {
            Field = 3,
            Literal = 3,
            ProtectedLiteral = 3,
            BooleanField = 3,
            Image = 5,
            Line = 1,
            Fieldset = 3
        },
        default = nil, --  Default height for each field type
        initial = nil, -- Default height for the initial field type
        edited = nil
    },

    field_max_height = { -- Maximum height of the field, can be any positive integer
        enum = {
            Field = 80,
            Literal = 80,
            ProtectedLiteral = 80,
            BooleanField = 3,
            Image = 40,
            Line = 1,
            Fieldset = 80
        },
        default = nil, --  Default max height for each field type
        initial = nil, -- Default max height for the initial field type
        edited = nil
    },

    field_width_max = { -- Maximum length of the field, can be any positive integer
        enum = {
            Field = 255,
            Literal = 255,
            ProtectedLiteral = 255,
            BooleanField = 255,
            Image = 255,
            Line = 255,
            Fieldset = 255
        },
        default = nil, -- Default max length for each field type
        initial = nil, -- Default max length for the initial field type
        edited = nil -- Max length after editing
    },

    field_width_min = { -- Minimum length of the field, can be any positive integer
        enum = {
            Field = 3,
            Literal = 3,
            ProtectedLiteral = 3,
            BooleanField = 10,
            Image = 1,
            Line = 1,
            Fieldset = 3
        },
        default = nil, -- Default min length for each field type
        initial = nil, -- Default min length for the initial field type
        edited = nil -- Min length after editing
    },

    ----- ===== DIMENSIONS DU CHAMP =====
    field_height = { -- Height of the field, can be any positive integer
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_text_with_label_field,
        enum = {
            Field = 3,
            Literal = 3,
            ProtectedLiteral = 3,
            BooleanField = 3,
            Image = 5,
            Line = 1,
            Fieldset = 3
        },
        default = nil, -- Default height for each field type (min, max, initial, edited)
        initial = nil, -- Default height for the initial field type
        edited = nil
    },
    ----- ===== DIMENSIONS DU CHAMP =====
    field_width = { -- Length of the field, can be any positive integer
        enum = {
            Field = 10,
            Literal = 20,
            ProtectedLiteral = 20,
            BooleanField = 10,
            Image = 40,
            Line = 40,
            Fieldset = 40
        },
        default = nil, -- Default length for each field type (min, max, initial, edited)
        initial = nil, -- Default length for the initial field type
        edited = nil -- Length after editing
    },

    -- ===== COULEURS =====

    field_avail_color = {
        enum = {
            default = "default",
            black = "black",
            red = "red",
            green = "green",
            yellow = "yellow",
            blue = "blue",
            magenta = "magenta",
            cyan = "cyan",
            white = "white"
        },

        -- Table de mapping des couleurs (à initialiser avec start_color() en ncurses)
        avail_color_exported_value = {
            default = 0,
            black = 1,
            red = 2,
            green = 3,
            yellow = 4,
            blue = 5,
            magenta = 6,
            cyan = 7,
            white = 8
        },
        avail_color_help = {
            default = "Default color (no color)",
            black = "Black color",
            red = "Red color",
            green = "Green color (recommended for validated/protected fields)",
            yellow = "Yellow color (highlight/warning)",
            blue = "Blue color (recommended for containers/borders)",
            magenta = "Magenta color",
            cyan = "Cyan color (recommended for placeholders/lines)",
            white = "White color (default for read-only)"
        },
        default = nil, -- Combinaisons UX par type (1ere = valeur par defaut pour .initial)
        initial = nil,
        edited = nil
    },

    field_border_color = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        enum = {
            default = "default",
            black = "black",
            red = "red",
            green = "green",
            yellow = "yellow",
            blue = "blue",
            magenta = "magenta",
            cyan = "cyan",
            white = "white"
        },
        default = nil, -- Default border color for each field type
        initial = nil,
        edited = nil
    },

    field_title_color = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        enum = {
            default = "default",
            black = "black",
            red = "red",
            green = "green",
            yellow = "yellow",
            blue = "blue",
            magenta = "magenta",
            cyan = "cyan",
            white = "white"
        },
        default = nil,
        initial = nil,
        edited = nil
    },

    field_text_color = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        enum = {
            default = "default",
            black = "black",
            red = "red",
            green = "green",
            yellow = "yellow",
            blue = "blue",
            magenta = "magenta",
            cyan = "cyan",
            white = "white"
        },
        default = nil,
        initial = nil,
        edited = nil
    },
    field_avail_footer_color = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        enum = {
            default = "default",
            black = "black",
            red = "red",
            green = "green",
            yellow = "yellow",
            blue = "blue",
            magenta = "magenta",
            cyan = "cyan",
            white = "white"
        },
        default = nil,
        initial = nil,
        edited = nil
    },
    field_footer_color = {

        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        enum = {
            default = "default",
            black = "black",
            red = "red",
            green = "green",
            yellow = "yellow",
            blue = "blue",
            magenta = "magenta",
            cyan = "cyan",
            white = "white"
        },
        default = nil,
        initial = nil,
        edited = nil
    },
    ----- ===== POLICE DU TEXTE =====
    -- enum for font family: 3270/BMS terminals have ONLY ONE fixed-width font
    field_avail_font_family = {
        enum = {
            default = "default"
        },
        default = {
            ncurses = {
                default = "default"
            }, -- ncurses: uses terminal's current single font
            tn3270 = {
                default = "default"
            }, -- 3270: IBM 3270 character set (single fixed-width font)
            bms = {
                default = "default"
            } -- BMS: single fixed-width font (no selection available)
        },
        initial = "default" -- Default font family for the initial field type
    },

    -- Font family for each field type, referencing field_avail_font_family enum
    -- Note: 3270 terminals use a single fixed-width font; no font family selection in BMS
    field_font_family = {
        enum = {
            default = "default"
        },
        default = {
            -- All BMS field types reference the same single font (3270 has only one physical font)
            Field = "default",
            Literal = "default",
            ProtectedLiteral = "default",
            BooleanField = "default",
            Image = "default",
            Line = "default",
            Fieldset = "default"
        }, -- Default font family for each field type
        initial = "default", -- Default font family for the initial field type
        edited = nil -- Font family after editing
    },
    ----- ===== STYLE DU TEXTE =====
    -- enum for text style: default, bold, italic, underline, strikethrough, blink, reverse
    field_avail_style = {
        enum = {
            default = "default",
            bold = "bold",
            italic = "italic",
            underline = "underline",
            strikethrough = "strikethrough",
            blink = "blink",
            reverse = "reverse"
        },
        avail_style_help = {
            default = "Default style (no attributes)",
            bold = "Bold style (intensity on 3270)",
            italic = "Italic style (not supported by 3270 hardware)",
            underline = "Underline style",
            strikethrough = "Strikethrough style",
            blink = "Blinking text",
            reverse = "Reverse video"
        },
        avail_style_exported_value = {
            default = 0, -- No attributes
            bold = 1, -- A_BOLD (intensity on 3270)
            italic = 2, -- A_ITALIC (not supported by 3270 hardware)
            underline = 4, -- A_UNDERLINE
            strikethrough = 8, -- Custom extension
            blink = 16, -- A_BLINK
            reverse = 32 -- A_REVERSE
        },
        default = nil -- Styles disponibles par type (1er = valeur par defaut pour .initial)
    },
    -- Represents the style for each field type, referencing field_avail_style enum for consistency
    -- Adapted per field type considering user visual experience (UX)
    field_style = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        enum = {
            default = "default",
            bold = "bold",
            italic = "italic",
            underline = "underline",
            strikethrough = "strikethrough",
            blink = "blink",
            reverse = "reverse"
        },
        default = {
            -- Field: Input field - style par defaut
            Field = "default",
            -- Literal: Static text - style par defaut
            Literal = "default",
            -- ProtectedLiteral: Protected static text - UX optimized
            ProtectedLiteral = "default",
            -- BooleanField: Checkbox - style par defaut
            BooleanField = "default",
            -- Image: Placeholder - style par defaut
            Image = "default",
            -- Line: Horizontal rule - underline par defaut pour effet de ligne
            Line = "underline",
            -- Fieldset: Container - style par defaut
            Fieldset = "default"
        }, -- Style par defaut pour chaque type
        initial = nil, -- Default style for the initial field type
        edited = nil -- Style after editing
    },
    ----- ===== ALIGNEMENT DU TEXTE =====
    -- enum for text alignment: left, center, right
    field_avail_text_align = {
        enum = {
            left = "left",
            center = "center",
            right = "right"
        },
        default = nil, -- Available text alignment for each field type
        initial = nil, -- Default text alignment for the initial field type
        edited = nil -- Text alignment after editing
    },
    -- field_text_align represents the text alignment for each field type, which can be left, center, or right
    field_text_align = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        enum = {
            left = "left",
            center = "center",
            right = "right"
        },
        default = nil, -- Default text alignment for each field type
        initial = nil, -- Default text alignment for the initial field type
        edited = nil -- Text alignment after editing
    },
    field_avail_pos = { -- Represents the available positions for each field type in the BMS screen (row, col)
        enum = {
            zero = 0,
            position = {
                row = 0,
                col = 0,
                rowend = 0,
                colend = 0
            }
        },
        default = nil, -- Default position for each field type
        initial = nil, -- Default position for the initial field type
        edited = nil -- Position after editing
    },
    field_pos = { -- Represents the position of the field in the BMS screen (row, col)
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_text_with_label_field,
        default = nil, -- Default position for each field type
        initial = nil, -- Default position for the initial field type
        edited = nil -- Position after editing
    },

    -- ===== PERSONNALISATION DES CARACTERES =====
    -- Caractères de bordure personnalisables (pour remplacer ┌─┐│├└┘)
    field_avail_border_chars = {
        default = nil, -- Default border characters for each field type
        initial = nil,
        edited = nil
    },

    field_avail_border_style = { -- Available border styles for each field type: single, double, dashed, none
        enum = {
            single = "single",
            double = "double",
            dashed = "dashed",
            none = "none"
        },
        default = nil -- Combinaisons UX par type (1ere = valeur par defaut pour .initial)
    },

    field_border_style = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        default = nil, -- Default border style for each field type
        initial = nil, -- Default border style for the initial field type
        edited = nil -- Border style after editing
    },
    ----- ===== BORDURE =====
    -- respresents the border style for each field type, which can be "single", "double", "dashed", or "none". The border style can be customized for each field type, allowing for a flexible and extensible way to create user interfaces for BMS applications. The border style can be used to indicate the state of the field, such as whether it is required or in an error state. The border style can also be used to enhance the visual appearance of the field, providing a more engaging user experience.
    -- this is used to render the border of the field, which can be customized for each field type. The border style can be used to indicate the state of the field, such as whether it is required or in an error state. The border style can also be used to enhance the visual appearance of the field, providing a more engaging user experience.
    field_border = nil, -- Will be set after table construction to reference field_avail_border_style and field_avail_border_chars

    -- Caractère de remplissage pour le titre dans la bordure supérieure
    field_title_fill_char = {
        enum = {
            space = " ",
            dash = "─"
        },
        default = nil,
        initial = nil,
        edited = nil
    },

    -- Caractère de remplissage pour les champs vides (ex: "_" pour Field)
    field_fill_char = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        enum = {
            underscore = "_",
            space = " ",
            dash = "─"
        },
        default = nil,
        initial = nil,
        edited = nil
    },
    field_avail_vertical_align = {
        enum = {
            top = "top",
            middle = "middle",
            bottom = "bottom"
        },
        default = nil,
        initial = nil,
        edited = nil
    },
    -- ===== ALIGNEMENT VERTICAL =====
    field_vertical_align = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        enum = {
            top = "top",
            middle = "middle",
            bottom = "bottom"
        },
        default = nil,
        initial = nil,
        edited = nil
    },

    field_vertical_margin = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        enum = {
            none = 0
        },
        default = nil,
        initial = nil,
        edited = nil
    },

    -- ===== AUTRES PROPRIETES =====
    -- Prefixe du titre (ex: "✱ " pour Fieldset requis)

    -- Marqueur pour les champs requis (ex: " *")
    field_avail_required_marker = {
        enum = {
            required = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            }
        },
        default = nil,
        initial = nil,
        edited = nil
    },
    -- represents the required marker for each field type, which can be used to indicate that a field is required. The marker can be a string or a boolean value, and can be enabled or disabled for each field type.
    field_required_marker = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        default = nil,
        initial = nil,
        edited = nil
    },
    field_footer_required_marker = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        default = nil,
        initial = nil,
        edited = nil
    },
    field_footer_error_marker = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        default = nil,
        initial = nil,
        edited = nil
    },
    -- Marqueur pour les champs en erreur (ex: " /!\")
    field_avail_error_marker = {
        enum = {
            error = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            }
        },
        default = nil,
        initial = nil,
        edited = nil
    },
    -- represents the error marker for each field type, which can be used to indicate that a field is in an error state. The marker can be a string or a boolean value, and can be enabled or disabled for each field type.
    field_error_marker = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        default = nil,
        initial = nil,
        edited = nil
    },
    -- title of a field is composed of a prefix, a main title, and a suffix, by applying alignment and color properties. The prefix and suffix can be used to indicate required fields or other information, while the main title represents the name of the field. The title can be customized for each field type, allowing for a flexible and extensible way to create user interfaces for BMS applications.
    -- field_title_suffix represents the suffix for the title of each field type, which can be used to indicate required fields or other information
    field_title_suffix = {
        default = nil,
        initial = nil,
        edited = nil
    },
    -- field_title_prefix represents the prefix for the title of each field type, which can be used to indicate required fields or other information
    field_title_prefix = {
        default = nil,
        initial = nil,
        edited = nil
    },
    field_footer_align = {
        enum = {
            left = "left",
            center = "center",
            right = "right"
        },
        default = nil,
        initial = nil,
        edited = nil
    },

    field_footer_fill_char = {
        enum = {
            space = " ",
            dash = "-"
        },
        default = nil,
        initial = nil,
        edited = nil
    },
    field_footer_title = {
        default = nil,
        initial = nil,
        edited = nil
    },
    -- field_footer represents the footer for the title of each field type, which can be used to indicate required fields or other information
    field_footer = {
        default = nil,
        initial = nil,
        edited = nil
    },

    -- Alignement du titre (left/center/right)
    field_title_align = {
        enum = {
            left = "left",
            center = "center",
            right = "right"
        },
        default = nil,
        initial = nil,
        edited = nil
    },

    -- Champs enfants (pour Fieldset)
    field_children = {
        default = nil,
        initial = nil,
        edited = nil
    },
    ----- ===== ATTRIBUTS DU CHAMP =====
    -- represents the attributes of the field, such as whether it is editable, visible, required, readonly, enabled, focused, selected, highlighted, hidden, protected, or numeric
    field_attrb = {
        default = nil, -- BMS available field attributes
        initial = nil, -- Default field attribute
        edited = nil -- Field attribute after editing
    },
    ----- ===== VALEURS INITIALES =====
    ----- Represents the initial values for each field type, which can be used to set the default state of the field when it is created
    field_initial = { -- initial_value: for fieldset/group, represents the title of the fieldset/group;; for image, option_value: represents the ASCII code + file path; for other field types, represents the initial value
        default = nil,

        initial_value = nil, -- Default initial value for the initial field type
        edited_value = nil -- Initial value after editing
    },

    visual_representation = { -- Represents the visual representation of each field type
        -- line 0: reserved for border top + title (for fieldset/group)
        -- line 1 to N-1: reserved for border left/right + content/value
        -- line N: reserved for border bottom + footer
        default = {
            Field = function(obj)
                return render_bordered_field(obj)
            end,
            Literal = function(obj)
                return render_bordered_field(obj)
            end,
            ProtectedLiteral = function(obj)
                return render_bordered_field(obj)
            end,
            BooleanField = function(obj)
                return render_bordered_field(obj)
            end,
            Image = function(obj)
                -- Display ASCII art from option_value.ascii_code
                local ascii = obj.field_initial.initial.option_value.ascii_code
                if ascii and type(ascii) == "table" then
                    return table.concat(ascii, "\n")
                end
                return render_bordered_field(obj, "[Image]")
            end,
            Line = function(obj)
                return render_line(obj)
            end,
            Fieldset = function(obj)
                return render_fieldset(obj)
            end
        }, -- Default visual representation for each field type
        initial = nil, -- Default visual representation for the initial field type
        edited = nil -- Visual representation after editing
    }
}

-- ===== ALIASES POUR COMPATIBILITE =====
-- Alias field_min_width -> field_width_min et field_max_width -> field_width_max
OBJECTS_DEFINITIONS.field_min_width = OBJECTS_DEFINITIONS.field_width_min
OBJECTS_DEFINITIONS.field_max_width = OBJECTS_DEFINITIONS.field_width_max

-- ===== POST-CONSTRUCTION: Dynamic references for field_border =====
-- ===== NIVEAU 1: Proprietes avec valeurs statiques simples (aucune dependance) =====
-- field_name: Noms des types de champs
OBJECTS_DEFINITIONS.field_name.default = {
    Field = OBJECTS_DEFINITIONS.field_type.enum.Field,
    Literal = OBJECTS_DEFINITIONS.field_type.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_type.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_type.enum.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_type.enum.Image,
    Line = OBJECTS_DEFINITIONS.field_type.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_type.enum.Fieldset
} -- Available field types 

-- field_type: Types de champs
OBJECTS_DEFINITIONS.field_type.default = {
    Field = OBJECTS_DEFINITIONS.field_type.enum.Field,
    Literal = OBJECTS_DEFINITIONS.field_type.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_type.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_type.enum.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_type.enum.Image,
    Line = OBJECTS_DEFINITIONS.field_type.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_type.enum.Fieldset
} --  Default height for each field type

-- field_min_height: Hauteurs minimales
OBJECTS_DEFINITIONS.field_min_height.default = {
    Field = OBJECTS_DEFINITIONS.field_min_height.enum.Field,
    Literal = OBJECTS_DEFINITIONS.field_min_height.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_min_height.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_min_height.enum.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_min_height.enum.Image,
    Line = OBJECTS_DEFINITIONS.field_min_height.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_min_height.enum.Fieldset
} --  Default max height for each field type

-- field_max_height: Hauteurs maximales
OBJECTS_DEFINITIONS.field_max_height.default = {
    Field = OBJECTS_DEFINITIONS.field_max_height.enum.Field,
    Literal = OBJECTS_DEFINITIONS.field_max_height.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_max_height.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_max_height.enum.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_max_height.enum.Image,
    Line = OBJECTS_DEFINITIONS.field_max_height.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_max_height.enum.Fieldset
} -- Default max length for each field type

-- field_max_width: Largeurs maximales
OBJECTS_DEFINITIONS.field_max_width.default = {
    Field = OBJECTS_DEFINITIONS.field_max_width.enum.Field,
    Literal = OBJECTS_DEFINITIONS.field_max_width.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_max_width.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_max_width.enum.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_max_width.enum.Image,
    Line = OBJECTS_DEFINITIONS.field_max_width.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_max_width.enum.Fieldset
} -- Default max length for each field type

-- field_min_width: Largeurs minimales
OBJECTS_DEFINITIONS.field_min_width.default = {
    Field = OBJECTS_DEFINITIONS.field_min_width.enum.Field,
    Literal = OBJECTS_DEFINITIONS.field_min_width.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_min_width.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_min_width.enum.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_min_width.enum.Image,
    Line = OBJECTS_DEFINITIONS.field_min_width.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_min_width.enum.Fieldset
}

-- field_height: Hauteurs par defaut (utilise min/max du Niveau 1)
OBJECTS_DEFINITIONS.field_height.default = {
    Field = {
        min = OBJECTS_DEFINITIONS.field_min_height.default.Field,
        max = OBJECTS_DEFINITIONS.field_max_height.default.Field,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Field,
        edited = nil
    },
    Literal = {
        min = OBJECTS_DEFINITIONS.field_min_height.default.Literal,
        max = OBJECTS_DEFINITIONS.field_max_height.default.Literal,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Literal,
        edited = nil
    },
    ProtectedLiteral = {
        min = OBJECTS_DEFINITIONS.field_min_height.default.ProtectedLiteral,
        max = OBJECTS_DEFINITIONS.field_max_height.default.ProtectedLiteral,
        initial = OBJECTS_DEFINITIONS.field_height.enum.ProtectedLiteral,
        edited = nil
    },
    BooleanField = {
        min = OBJECTS_DEFINITIONS.field_min_height.default.BooleanField,
        max = OBJECTS_DEFINITIONS.field_max_height.default.BooleanField,
        initial = OBJECTS_DEFINITIONS.field_height.enum.BooleanField,
        edited = nil
    },
    Image = {
        min = OBJECTS_DEFINITIONS.field_min_height.default.Image,
        max = OBJECTS_DEFINITIONS.field_max_height.default.Image,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Image,
        edited = nil
    },
    Line = {
        min = OBJECTS_DEFINITIONS.field_min_height.default.Line,
        max = OBJECTS_DEFINITIONS.field_max_height.default.Line,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Line,
        edited = nil
    },
    Fieldset = {
        min = OBJECTS_DEFINITIONS.field_min_height.default.Fieldset,
        max = OBJECTS_DEFINITIONS.field_max_height.default.Fieldset,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Fieldset,
        edited = nil
    }
} -- Default length for each field type (min, max, initial, edited)

-- field_width: Largeurs par defaut (utilise min/max du Niveau 1)
OBJECTS_DEFINITIONS.field_width.default = {
    Field = {
        min = OBJECTS_DEFINITIONS.field_width_min.default.Field,
        max = OBJECTS_DEFINITIONS.field_width_max.default.Field,
        initial = OBJECTS_DEFINITIONS.field_width.enum.Field,
        edited = nil
    },
    Literal = {
        min = OBJECTS_DEFINITIONS.field_width_min.default.Literal,
        max = OBJECTS_DEFINITIONS.field_width_max.default.Literal,
        initial = OBJECTS_DEFINITIONS.field_width.enum.Literal,
        edited = nil
    },
    ProtectedLiteral = {
        min = OBJECTS_DEFINITIONS.field_width_min.default.ProtectedLiteral,
        max = OBJECTS_DEFINITIONS.field_width_max.default.ProtectedLiteral,
        initial = OBJECTS_DEFINITIONS.field_width.enum.ProtectedLiteral,
        edited = nil
    },
    BooleanField = {
        min = OBJECTS_DEFINITIONS.field_width_min.default.BooleanField,
        max = OBJECTS_DEFINITIONS.field_width_max.default.BooleanField,
        initial = OBJECTS_DEFINITIONS.field_width.enum.BooleanField,
        edited = nil
    },
    Image = {
        min = OBJECTS_DEFINITIONS.field_width_min.default.Image,
        max = OBJECTS_DEFINITIONS.field_width_max.default.Image,
        initial = OBJECTS_DEFINITIONS.field_width.enum.Image,
        edited = nil
    },
    Line = {
        min = OBJECTS_DEFINITIONS.field_width_min.default.Line,
        max = OBJECTS_DEFINITIONS.field_width_max.default.Line,
        initial = OBJECTS_DEFINITIONS.field_width.enum.Line,
        edited = nil
    },
    Fieldset = {
        min = OBJECTS_DEFINITIONS.field_width_min.default.Fieldset,
        max = OBJECTS_DEFINITIONS.field_width_max.default.Fieldset,
        initial = OBJECTS_DEFINITIONS.field_width.enum.Fieldset,
        edited = nil
    }
} -- Default length for each field type (min, max, initial, edited)

-- ===== NIVEAU 1 (suite): Proprietes de couleurs avec enumerations simples =====

-- field_avail_color: Couleurs disponibles
OBJECTS_DEFINITIONS.field_avail_color.default = {
    -- Field : Couleurs pour champs de saisie (default = neutre, white = visible sur fond sombre)
    Field = {
        default = OBJECTS_DEFINITIONS.field_avail_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_avail_color.enum.white,
        green = OBJECTS_DEFINITIONS.field_avail_color.enum.green,
        yellow = OBJECTS_DEFINITIONS.field_avail_color.enum.yellow,
        blue = OBJECTS_DEFINITIONS.field_avail_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_avail_color.enum.cyan
    },

    -- Literal : Texte statique (default = neutre, white/yellow = lisible, green = accent)
    Literal = {
        default = OBJECTS_DEFINITIONS.field_avail_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_avail_color.enum.white,
        yellow = OBJECTS_DEFINITIONS.field_avail_color.enum.yellow,
        green = OBJECTS_DEFINITIONS.field_avail_color.enum.green,
        cyan = OBJECTS_DEFINITIONS.field_avail_color.enum.cyan,
        blue = OBJECTS_DEFINITIONS.field_avail_color.enum.blue
    },

    -- ProtectedLiteral : Texte protege (white = par defaut pour read-only, green = protege, cyan = informatif)
    ProtectedLiteral = {
        white = OBJECTS_DEFINITIONS.field_avail_color.enum.white,
        green = OBJECTS_DEFINITIONS.field_avail_color.enum.green,
        cyan = OBJECTS_DEFINITIONS.field_avail_color.enum.cyan,
        yellow = OBJECTS_DEFINITIONS.field_avail_color.enum.yellow,
        default = OBJECTS_DEFINITIONS.field_avail_color.enum.default
    },

    -- BooleanField : Cases a cocher (default = neutre, green = coche/valide, white = non coche)
    BooleanField = {
        default = OBJECTS_DEFINITIONS.field_avail_color.enum.default,
        green = OBJECTS_DEFINITIONS.field_avail_color.enum.green,
        white = OBJECTS_DEFINITIONS.field_avail_color.enum.white,
        yellow = OBJECTS_DEFINITIONS.field_avail_color.enum.yellow,
        blue = OBJECTS_DEFINITIONS.field_avail_color.enum.blue
    },

    -- Image : Placeholder (default = transparent, white/blue = contour visible, cyan = water mark)
    Image = {
        default = OBJECTS_DEFINITIONS.field_avail_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_avail_color.enum.white,
        blue = OBJECTS_DEFINITIONS.field_avail_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_avail_color.enum.cyan
    },

    -- Line : Separateurs (default = invisible, white/blue/cyan = visibles mais discrets)
    Line = {
        default = OBJECTS_DEFINITIONS.field_avail_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_avail_color.enum.white,
        blue = OBJECTS_DEFINITIONS.field_avail_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_avail_color.enum.cyan
    },

    -- Fieldset : Conteneurs (blue = standard pour bordures, default = neutre, white/cyan = alternatifs)
    Fieldset = {
        blue = OBJECTS_DEFINITIONS.field_avail_color.enum.blue,
        default = OBJECTS_DEFINITIONS.field_avail_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_avail_color.enum.white,
        cyan = OBJECTS_DEFINITIONS.field_avail_color.enum.cyan,
        green = OBJECTS_DEFINITIONS.field_avail_color.enum.green
    }
} -- Combinaisons UX par type (1ere = valeur par defaut pour .initial)

-- field_border_color: Couleurs de bordure
OBJECTS_DEFINITIONS.field_border_color.default = {
    Field = {
        default = OBJECTS_DEFINITIONS.field_border_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_border_color.enum.white,
        green = OBJECTS_DEFINITIONS.field_border_color.enum.green,
        yellow = OBJECTS_DEFINITIONS.field_border_color.enum.yellow,
        blue = OBJECTS_DEFINITIONS.field_border_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_border_color.enum.cyan
    },
    Literal = {
        default = OBJECTS_DEFINITIONS.field_border_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_border_color.enum.white,
        yellow = OBJECTS_DEFINITIONS.field_border_color.enum.yellow,
        green = OBJECTS_DEFINITIONS.field_border_color.enum.green,
        cyan = OBJECTS_DEFINITIONS.field_border_color.enum.cyan,
        blue = OBJECTS_DEFINITIONS.field_border_color.enum.blue
    },
    ProtectedLiteral = {
        white = OBJECTS_DEFINITIONS.field_border_color.enum.white,
        green = OBJECTS_DEFINITIONS.field_border_color.enum.green,
        cyan = OBJECTS_DEFINITIONS.field_border_color.enum.cyan,
        yellow = OBJECTS_DEFINITIONS.field_border_color.enum.yellow,
        default = OBJECTS_DEFINITIONS.field_border_color.enum.default
    },
    BooleanField = {
        default = OBJECTS_DEFINITIONS.field_border_color.enum.default,
        green = OBJECTS_DEFINITIONS.field_border_color.enum.green,
        white = OBJECTS_DEFINITIONS.field_border_color.enum.white,
        yellow = OBJECTS_DEFINITIONS.field_border_color.enum.yellow,
        blue = OBJECTS_DEFINITIONS.field_border_color.enum.blue
    },
    Image = {
        default = OBJECTS_DEFINITIONS.field_border_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_border_color.enum.white,
        blue = OBJECTS_DEFINITIONS.field_border_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_border_color.enum.cyan
    },
    Line = {
        default = OBJECTS_DEFINITIONS.field_border_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_border_color.enum.white,
        blue = OBJECTS_DEFINITIONS.field_border_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_border_color.enum.cyan
    },
    Fieldset = {
        blue = OBJECTS_DEFINITIONS.field_border_color.enum.blue,
        default = OBJECTS_DEFINITIONS.field_border_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_border_color.enum.white,
        cyan = OBJECTS_DEFINITIONS.field_border_color.enum.cyan,
        green = OBJECTS_DEFINITIONS.field_border_color.enum.green
    }
}

-- field_title_color: Couleurs de titre
OBJECTS_DEFINITIONS.field_title_color.default = {
    Field = {
        default = OBJECTS_DEFINITIONS.field_title_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_title_color.enum.white,
        green = OBJECTS_DEFINITIONS.field_title_color.enum.green,
        yellow = OBJECTS_DEFINITIONS.field_title_color.enum.yellow,
        blue = OBJECTS_DEFINITIONS.field_title_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_title_color.enum.cyan
    },
    Literal = {
        default = OBJECTS_DEFINITIONS.field_title_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_title_color.enum.white,
        yellow = OBJECTS_DEFINITIONS.field_title_color.enum.yellow,
        green = OBJECTS_DEFINITIONS.field_title_color.enum.green,
        cyan = OBJECTS_DEFINITIONS.field_title_color.enum.cyan,
        blue = OBJECTS_DEFINITIONS.field_title_color.enum.blue
    },
    ProtectedLiteral = {
        white = OBJECTS_DEFINITIONS.field_title_color.enum.white,
        green = OBJECTS_DEFINITIONS.field_title_color.enum.green,
        cyan = OBJECTS_DEFINITIONS.field_title_color.enum.cyan,
        yellow = OBJECTS_DEFINITIONS.field_title_color.enum.yellow,
        default = OBJECTS_DEFINITIONS.field_title_color.enum.default
    },
    BooleanField = {
        default = OBJECTS_DEFINITIONS.field_title_color.enum.default,
        green = OBJECTS_DEFINITIONS.field_title_color.enum.green,
        white = OBJECTS_DEFINITIONS.field_title_color.enum.white,
        yellow = OBJECTS_DEFINITIONS.field_title_color.enum.yellow,
        blue = OBJECTS_DEFINITIONS.field_title_color.enum.blue
    },
    Image = {
        default = OBJECTS_DEFINITIONS.field_title_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_title_color.enum.white,
        blue = OBJECTS_DEFINITIONS.field_title_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_title_color.enum.cyan
    },
    Line = {
        default = OBJECTS_DEFINITIONS.field_title_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_title_color.enum.white,
        blue = OBJECTS_DEFINITIONS.field_title_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_title_color.enum.cyan
    },
    Fieldset = {
        blue = OBJECTS_DEFINITIONS.field_title_color.enum.blue,
        default = OBJECTS_DEFINITIONS.field_title_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_title_color.enum.white,
        cyan = OBJECTS_DEFINITIONS.field_title_color.enum.cyan,
        green = OBJECTS_DEFINITIONS.field_title_color.enum.green
    }
}

-- field_text_color: Couleurs de texte
OBJECTS_DEFINITIONS.field_text_color.default = {
    Field = {
        default = OBJECTS_DEFINITIONS.field_text_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_text_color.enum.white,
        green = OBJECTS_DEFINITIONS.field_text_color.enum.green,
        yellow = OBJECTS_DEFINITIONS.field_text_color.enum.yellow,
        blue = OBJECTS_DEFINITIONS.field_text_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_text_color.enum.cyan
    },
    Literal = {
        default = OBJECTS_DEFINITIONS.field_text_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_text_color.enum.white,
        yellow = OBJECTS_DEFINITIONS.field_text_color.enum.yellow,
        green = OBJECTS_DEFINITIONS.field_text_color.enum.green,
        cyan = OBJECTS_DEFINITIONS.field_text_color.enum.cyan,
        blue = OBJECTS_DEFINITIONS.field_text_color.enum.blue
    },
    ProtectedLiteral = {
        white = OBJECTS_DEFINITIONS.field_text_color.enum.white,
        green = OBJECTS_DEFINITIONS.field_text_color.enum.green,
        cyan = OBJECTS_DEFINITIONS.field_text_color.enum.cyan,
        yellow = OBJECTS_DEFINITIONS.field_text_color.enum.yellow,
        default = OBJECTS_DEFINITIONS.field_text_color.enum.default
    },
    BooleanField = {
        default = OBJECTS_DEFINITIONS.field_text_color.enum.default,
        green = OBJECTS_DEFINITIONS.field_text_color.enum.green,
        white = OBJECTS_DEFINITIONS.field_text_color.enum.white,
        yellow = OBJECTS_DEFINITIONS.field_text_color.enum.yellow,
        blue = OBJECTS_DEFINITIONS.field_text_color.enum.blue
    },
    Image = {
        default = OBJECTS_DEFINITIONS.field_text_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_text_color.enum.white,
        blue = OBJECTS_DEFINITIONS.field_text_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_text_color.enum.cyan
    },
    Line = {
        default = OBJECTS_DEFINITIONS.field_text_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_text_color.enum.white,
        blue = OBJECTS_DEFINITIONS.field_text_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_text_color.enum.cyan
    },
    Fieldset = {
        blue = OBJECTS_DEFINITIONS.field_text_color.enum.blue,
        default = OBJECTS_DEFINITIONS.field_text_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_text_color.enum.white,
        cyan = OBJECTS_DEFINITIONS.field_text_color.enum.cyan,
        green = OBJECTS_DEFINITIONS.field_text_color.enum.green
    }
}
-- field_footer_color: Couleurs de pied de page
OBJECTS_DEFINITIONS.field_avail_footer_color.default = {
    Field = {
        default = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.white,
        green = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.green,
        yellow = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.yellow,
        blue = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.cyan
    },
    Literal = {
        default = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.white,
        yellow = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.yellow,
        green = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.green,
        cyan = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.cyan,
        blue = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.blue
    },
    ProtectedLiteral = {
        white = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.white,
        green = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.green,
        cyan = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.cyan,
        yellow = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.yellow,
        default = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.default
    },
    BooleanField = {
        default = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.default,
        green = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.green,
        white = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.white,
        yellow = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.yellow,
        blue = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.blue
    },
    Image = {
        default = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.white,
        blue = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.cyan
    },
    Line = {
        default = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.white,
        blue = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.blue,
        cyan = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.cyan
    },
    Fieldset = {
        blue = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.blue,
        default = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.default,
        white = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.white,
        cyan = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.cyan,
        green = OBJECTS_DEFINITIONS.field_avail_footer_color.enum.green
    }
}

-- field_footer_color: Couleurs de pied de page
OBJECTS_DEFINITIONS.field_footer_color.default = {
    Field = OBJECTS_DEFINITIONS.field_avail_footer_color.default.Field,
    Literal = OBJECTS_DEFINITIONS.field_avail_footer_color.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_footer_color.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_footer_color.default.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_avail_footer_color.default.Image,
    Line = OBJECTS_DEFINITIONS.field_avail_footer_color.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_footer_color.default.Fieldset
}

-- ===== NIVEAU 1 (suite): Autres enumerations simples =====

-- field_avail_font_family: Famille de police (3270 a une seule police)
OBJECTS_DEFINITIONS.field_avail_font_family.default = {
    ncurses = {
        default = "default"
    }, -- ncurses: uses terminal's current single font
    tn3270 = {
        default = "default"
    }, -- 3270: IBM 3270 character set (single fixed-width font)
    bms = {
        default = "default"
    } -- BMS: single fixed-width font (no selection available)
}

-- field_font_family: Famille de police par type de champ
OBJECTS_DEFINITIONS.field_font_family.default = {
    -- All BMS field types reference the same single font (3270 has only one physical font)
    Field = {
        default = OBJECTS_DEFINITIONS.field_avail_font_family.enum.default
    },
    Literal = {
        default = OBJECTS_DEFINITIONS.field_avail_font_family.enum.default
    },
    ProtectedLiteral = {
        default = OBJECTS_DEFINITIONS.field_avail_font_family.enum.default
    },
    BooleanField = {
        default = OBJECTS_DEFINITIONS.field_avail_font_family.enum.default
    },
    Image = {
        default = OBJECTS_DEFINITIONS.field_avail_font_family.enum.default
    },
    Line = {
        default = OBJECTS_DEFINITIONS.field_avail_font_family.enum.default
    },
    Fieldset = {
        default = OBJECTS_DEFINITIONS.field_avail_font_family.enum.default
    }
} -- Default font family for each field type

-- field_avail_style: Styles de texte disponibles
OBJECTS_DEFINITIONS.field_avail_style.default = {
    -- Field: Tous les styles disponibles
    Field = {
        default = OBJECTS_DEFINITIONS.field_avail_style.enum.default,
        bold = OBJECTS_DEFINITIONS.field_avail_style.enum.bold,
        italic = OBJECTS_DEFINITIONS.field_avail_style.enum.italic,
        underline = OBJECTS_DEFINITIONS.field_avail_style.enum.underline,
        strikethrough = OBJECTS_DEFINITIONS.field_avail_style.enum.strikethrough,
        blink = OBJECTS_DEFINITIONS.field_avail_style.enum.blink,
        reverse = OBJECTS_DEFINITIONS.field_avail_style.enum.reverse
    },

    -- Literal: Tous les styles (texte statique peut etre mis en valeur)
    Literal = {
        default = OBJECTS_DEFINITIONS.field_avail_style.enum.default,
        bold = OBJECTS_DEFINITIONS.field_avail_style.enum.bold,
        italic = OBJECTS_DEFINITIONS.field_avail_style.enum.italic,
        underline = OBJECTS_DEFINITIONS.field_avail_style.enum.underline,
        blink = OBJECTS_DEFINITIONS.field_avail_style.enum.blink,
        reverse = OBJECTS_DEFINITIONS.field_avail_style.enum.reverse
    },

    -- ProtectedLiteral: Pas de italic/strikethrough/blink (distrayant pour read-only)
    ProtectedLiteral = {
        default = OBJECTS_DEFINITIONS.field_avail_style.enum.default,
        bold = OBJECTS_DEFINITIONS.field_avail_style.enum.bold,
        underline = OBJECTS_DEFINITIONS.field_avail_style.enum.underline,
        reverse = OBJECTS_DEFINITIONS.field_avail_style.enum.reverse
    },

    -- BooleanField: Pas de italic/strikethrough; blink pour attention
    BooleanField = {
        default = OBJECTS_DEFINITIONS.field_avail_style.enum.default,
        bold = OBJECTS_DEFINITIONS.field_avail_style.enum.bold,
        underline = OBJECTS_DEFINITIONS.field_avail_style.enum.underline,
        blink = OBJECTS_DEFINITIONS.field_avail_style.enum.blink,
        reverse = OBJECTS_DEFINITIONS.field_avail_style.enum.reverse
    },

    -- Image: Pas de italic/strikethrough/blink (distrayant pour placeholder)
    Image = {
        default = OBJECTS_DEFINITIONS.field_avail_style.enum.default,
        bold = OBJECTS_DEFINITIONS.field_avail_style.enum.bold,
        underline = OBJECTS_DEFINITIONS.field_avail_style.enum.underline,
        reverse = OBJECTS_DEFINITIONS.field_avail_style.enum.reverse
    },

    -- Line: underline pour effet tirete, strikethrough pour ligne brisee
    Line = {
        underline = OBJECTS_DEFINITIONS.field_avail_style.enum.underline,
        strikethrough = OBJECTS_DEFINITIONS.field_avail_style.enum.strikethrough,
        default = OBJECTS_DEFINITIONS.field_avail_style.enum.default,
        bold = OBJECTS_DEFINITIONS.field_avail_style.enum.bold,
        reverse = OBJECTS_DEFINITIONS.field_avail_style.enum.reverse
    },

    -- Fieldset: Tous les styles sauf strikethrough (peu pertinent pour bordures)
    Fieldset = {
        default = OBJECTS_DEFINITIONS.field_avail_style.enum.default,
        bold = OBJECTS_DEFINITIONS.field_avail_style.enum.bold,
        underline = OBJECTS_DEFINITIONS.field_avail_style.enum.underline,
        blink = OBJECTS_DEFINITIONS.field_avail_style.enum.blink,
        reverse = OBJECTS_DEFINITIONS.field_avail_style.enum.reverse
    }
} -- Styles disponibles par type (1er = valeur par defaut pour .initial)

-- field_style: Style par defaut pour chaque type de champ
OBJECTS_DEFINITIONS.field_style.default = {
    -- Field: Input field - style par defaut
    Field = OBJECTS_DEFINITIONS.field_avail_style.default.Field,
    -- Literal: Static text - style par defaut
    Literal = OBJECTS_DEFINITIONS.field_avail_style.default.Literal,
    -- ProtectedLiteral: Protected static text - UX optimized
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_style.default.ProtectedLiteral,
    -- BooleanField: Checkbox - style par defaut
    BooleanField = OBJECTS_DEFINITIONS.field_avail_style.default.BooleanField,
    -- Image: Placeholder - style par defaut
    Image = OBJECTS_DEFINITIONS.field_avail_style.default.Image,
    -- Line: Horizontal rule - underline par defaut pour effet de ligne
    Line = OBJECTS_DEFINITIONS.field_avail_style.default.Line,
    -- Fieldset: Container - style par defaut
    Fieldset = OBJECTS_DEFINITIONS.field_avail_style.default.Fieldset
} -- Style par defaut pour chaque type

-- field_avail_text_align: Alignements de texte disponibles
OBJECTS_DEFINITIONS.field_avail_text_align.default = {
    Field = {
        left = OBJECTS_DEFINITIONS.field_avail_text_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_avail_text_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_avail_text_align.enum.right
    },
    Literal = {
        left = OBJECTS_DEFINITIONS.field_avail_text_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_avail_text_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_avail_text_align.enum.right
    },
    ProtectedLiteral = {
        left = OBJECTS_DEFINITIONS.field_avail_text_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_avail_text_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_avail_text_align.enum.right
    },
    BooleanField = {
        left = OBJECTS_DEFINITIONS.field_avail_text_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_avail_text_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_avail_text_align.enum.right
    },
    Image = {
        left = OBJECTS_DEFINITIONS.field_avail_text_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_avail_text_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_avail_text_align.enum.right
    },
    Line = {
        left = OBJECTS_DEFINITIONS.field_avail_text_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_avail_text_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_avail_text_align.enum.right
    },
    Fieldset = {
        left = OBJECTS_DEFINITIONS.field_avail_text_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_avail_text_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_avail_text_align.enum.right
    }
} -- Available text alignment for each field type

-- field_text_align: Alignement de texte par defaut
OBJECTS_DEFINITIONS.field_text_align.default = {
    Field = {
        align = OBJECTS_DEFINITIONS.field_avail_text_align.default.Field
    },
    Literal = {
        align = OBJECTS_DEFINITIONS.field_avail_text_align.default.Literal
    },
    ProtectedLiteral = {
        align = OBJECTS_DEFINITIONS.field_avail_text_align.default.ProtectedLiteral
    },
    BooleanField = {
        align = OBJECTS_DEFINITIONS.field_avail_text_align.default.BooleanField
    },
    Image = {
        align = OBJECTS_DEFINITIONS.field_avail_text_align.default.Image
    },
    Line = {
        align = OBJECTS_DEFINITIONS.field_avail_text_align.default.Line
    },
    Fieldset = {
        align = OBJECTS_DEFINITIONS.field_avail_text_align.default.Fieldset
    }
} -- Default text alignment for each field type

-- field_avail_border_chars: Caracteres de bordure disponibles
OBJECTS_DEFINITIONS.field_avail_border_chars.default = {
    Field = {
        single = {
            top_left = "┌",
            top = "─",
            top_right = "┐",
            left = "│",
            right = "│",
            bottom_left = "└",
            bottom = "─",
            bottom_right = "┘"
        },
        double = {
            top_left = "╔",
            top = "═",
            top_right = "╗",
            left = "║",
            right = "║",
            bottom_left = "╚",
            bottom = "═",
            bottom_right = "╝"
        },
        dashed = {
            top_left = "+",
            top = "-",
            top_right = "+",
            left = "|",
            right = "|",
            bottom_left = "+",
            bottom = "-",
            bottom_right = "+"
        },
        none = {
            top_left = "",
            top = "",
            top_right = "",
            left = "",
            right = "",
            bottom_left = "",
            bottom = "",
            bottom_right = ""
        }
    },
    Literal = {
        single = {
            top_left = "┌",
            top = "─",
            top_right = "┐",
            left = "│",
            right = "│",
            bottom_left = "└",
            bottom = "─",
            bottom_right = "┘"
        },
        double = {
            top_left = "╔",
            top = "═",
            top_right = "╗",
            left = "║",
            right = "║",
            bottom_left = "╚",
            bottom = "═",
            bottom_right = "╝"
        },
        dashed = {
            top_left = "+",
            top = "-",
            top_right = "+",
            left = "|",
            right = "|",
            bottom_left = "+",
            bottom = "-",
            bottom_right = "+"
        },
        none = {
            top_left = "",
            top = "",
            top_right = "",
            left = "",
            right = "",
            bottom_left = "",
            bottom = "",
            bottom_right = ""
        }
    },
    ProtectedLiteral = {
        single = {
            top_left = "┌",
            top = "─",
            top_right = "┐",
            left = "│",
            right = "│",
            bottom_left = "└",
            bottom = "─",
            bottom_right = "┘"
        },
        double = {
            top_left = "╔",
            top = "═",
            top_right = "╗",
            left = "║",
            right = "║",
            bottom_left = "╚",
            bottom = "═",
            bottom_right = "╝"
        },
        dashed = {
            top_left = "+",
            top = "-",
            top_right = "+",
            left = "|",
            right = "|",
            bottom_left = "+",
            bottom = "-",
            bottom_right = "+"
        },
        none = {
            top_left = "",
            top = "",
            top_right = "",
            left = "",
            right = "",
            bottom_left = "",
            bottom = "",
            bottom_right = ""
        }
    },
    BooleanField = {
        single = {
            top_left = "┌",
            top = "─",
            top_right = "┐",
            left = "│",
            right = "│",
            bottom_left = "└",
            bottom = "─",
            bottom_right = "┘"
        },
        double = {
            top_left = "╔",
            top = "═",
            top_right = "╗",
            left = "║",
            right = "║",
            bottom_left = "╚",
            bottom = "═",
            bottom_right = "╝"
        },
        dashed = {
            top_left = "+",
            top = "-",
            top_right = "+",
            left = "|",
            right = "|",
            bottom_left = "+",
            bottom = "-",
            bottom_right = "+"
        },
        none = {
            top_left = "",
            top = "",
            top_right = "",
            left = "",
            right = "",
            bottom_left = "",
            bottom = "",
            bottom_right = ""
        }
    },
    Image = {
        single = {
            top_left = "┌",
            top = "─",
            top_right = "┐",
            left = "│",
            right = "│",
            bottom_left = "└",
            bottom = "─",
            bottom_right = "┘"
        },
        double = {
            top_left = "╔",
            top = "═",
            top_right = "╗",
            left = "║",
            right = "║",
            bottom_left = "╚",
            bottom = "═",
            bottom_right = "╝"
        },
        dashed = {
            top_left = "+",
            top = "-",
            top_right = "+",
            left = "|",
            right = "|",
            bottom_left = "+",
            bottom = "-",
            bottom_right = "+"
        },
        none = {
            top_left = "",
            top = "",
            top_right = "",
            left = "",
            right = "",
            bottom_left = "",
            bottom = "",
            bottom_right = ""
        }
    },
    Line = {
        single = {
            top_left = "┌",
            top = "─",
            top_right = "┐",
            left = "│",
            right = "│",
            bottom_left = "└",
            bottom = "─",
            bottom_right = "┘"
        },
        double = {
            top_left = "╔",
            top = "═",
            top_right = "╗",
            left = "║",
            right = "║",
            bottom_left = "╚",
            bottom = "═",
            bottom_right = "╝"
        },
        dashed = {
            top_left = "+",
            top = "-",
            top_right = "+",
            left = "|",
            right = "|",
            bottom_left = "+",
            bottom = "-",
            bottom_right = "+"
        },
        none = {
            top_left = "",
            top = "",
            top_right = "",
            left = "",
            right = "",
            bottom_left = "",
            bottom = "",
            bottom_right = ""
        }
    },
    Fieldset = {
        single = {
            top_left = "┌",
            top = "─",
            top_right = "┐",
            left = "│",
            right = "│",
            bottom_left = "└",
            bottom = "─",
            bottom_right = "┘"
        },
        double = {
            top_left = "╔",
            top = "═",
            top_right = "╗",
            left = "║",
            right = "║",
            bottom_left = "╚",
            bottom = "═",
            bottom_right = "╝"
        },
        dashed = {
            top_left = "+",
            top = "-",
            top_right = "+",
            left = "|",
            right = "|",
            bottom_left = "+",
            bottom = "-",
            bottom_right = "+"
        },
        none = {
            top_left = "",
            top = "",
            top_right = "",
            left = "",
            right = "",
            bottom_left = "",
            bottom = "",
            bottom_right = ""
        }
    }
} -- Default border characters for each field type

-- field_avail_border_style: Styles de bordure disponibles
OBJECTS_DEFINITIONS.field_avail_border_style.default = {
    -- Field : Bordure simple par defaut (standard pour champs de saisie)
    Field = {
        single = OBJECTS_DEFINITIONS.field_avail_border_style.enum.single,
        double = OBJECTS_DEFINITIONS.field_avail_border_style.enum.double,
        dashed = OBJECTS_DEFINITIONS.field_avail_border_style.enum.dashed,
        none = OBJECTS_DEFINITIONS.field_avail_border_style.enum.none
    },

    -- Literal : Pas de bordure par defaut (texte statique n'en a pas besoin)
    Literal = {
        none = OBJECTS_DEFINITIONS.field_avail_border_style.enum.none,
        single = OBJECTS_DEFINITIONS.field_avail_border_style.enum.single,
        dashed = OBJECTS_DEFINITIONS.field_avail_border_style.enum.dashed
    },

    -- ProtectedLiteral : Bordure pointillee pour indiquer protection
    ProtectedLiteral = {
        dashed = OBJECTS_DEFINITIONS.field_avail_border_style.enum.dashed,
        single = OBJECTS_DEFINITIONS.field_avail_border_style.enum.single,
        double = OBJECTS_DEFINITIONS.field_avail_border_style.enum.double,
        none = OBJECTS_DEFINITIONS.field_avail_border_style.enum.none
    },

    -- BooleanField : Bordure simple pour cases a cocher
    BooleanField = {
        single = OBJECTS_DEFINITIONS.field_avail_border_style.enum.single,
        double = OBJECTS_DEFINITIONS.field_avail_border_style.enum.double,
        dashed = OBJECTS_DEFINITIONS.field_avail_border_style.enum.dashed
    },

    -- Image : Bordure double pour encadrer les placeholders
    Image = {
        double = OBJECTS_DEFINITIONS.field_avail_border_style.enum.double,
        single = OBJECTS_DEFINITIONS.field_avail_border_style.enum.single,
        dashed = OBJECTS_DEFINITIONS.field_avail_border_style.enum.dashed,
        none = OBJECTS_DEFINITIONS.field_avail_border_style.enum.none
    },

    -- Line : Pas de bordure (c'est deja une ligne)
    Line = {
        none = OBJECTS_DEFINITIONS.field_avail_border_style.enum.none
    },

    -- Fieldset : Bordure double pour conteneurs (standard UI)
    Fieldset = {
        double = OBJECTS_DEFINITIONS.field_avail_border_style.enum.double,
        single = OBJECTS_DEFINITIONS.field_avail_border_style.enum.single,
        dashed = OBJECTS_DEFINITIONS.field_avail_border_style.enum.dashed
    }
} -- Combinaisons UX par type (1ere = valeur par defaut pour .initial)

-- field_border_style: Style de bordure par defaut pour chaque type
OBJECTS_DEFINITIONS.field_border_style.default = {
    Field = OBJECTS_DEFINITIONS.field_avail_border_style.default.Field,
    Literal = OBJECTS_DEFINITIONS.field_avail_border_style.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_border_style.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_border_style.default.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_avail_border_style.default.Image,
    Line = OBJECTS_DEFINITIONS.field_avail_border_style.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_border_style.default.Fieldset
} -- Default border style for each field type

-- field_title_fill_char: Caractere de remplissage pour le titre
OBJECTS_DEFINITIONS.field_title_fill_char.default = {
    Field = {
        space = OBJECTS_DEFINITIONS.field_title_fill_char.enum.space
    },
    Literal = {
        space = OBJECTS_DEFINITIONS.field_title_fill_char.enum.space
    },
    ProtectedLiteral = {
        space = OBJECTS_DEFINITIONS.field_title_fill_char.enum.space
    },
    BooleanField = {
        space = OBJECTS_DEFINITIONS.field_title_fill_char.enum.space
    },
    Image = {
        dash = OBJECTS_DEFINITIONS.field_title_fill_char.enum.dash
    },
    Line = {
        dash = OBJECTS_DEFINITIONS.field_title_fill_char.enum.dash
    },
    Fieldset = {
        dash = OBJECTS_DEFINITIONS.field_title_fill_char.enum.dash
    }
}

-- field_fill_char: Caractere de remplissage pour les champs vides
OBJECTS_DEFINITIONS.field_fill_char.default = {
    Field = {
        underscore = OBJECTS_DEFINITIONS.field_fill_char.enum.underscore
    },
    Literal = {
        space = OBJECTS_DEFINITIONS.field_fill_char.enum.space
    },
    ProtectedLiteral = {
        space = OBJECTS_DEFINITIONS.field_fill_char.enum.space
    },
    BooleanField = {
        space = OBJECTS_DEFINITIONS.field_fill_char.enum.space
    },
    Image = {
        space = OBJECTS_DEFINITIONS.field_fill_char.enum.space
    },
    Line = {
        dash = OBJECTS_DEFINITIONS.field_fill_char.enum.dash
    },
    Fieldset = {
        space = OBJECTS_DEFINITIONS.field_fill_char.enum.space
    }
}

-- field_avail_vertical_align: Alignements verticaux disponibles
OBJECTS_DEFINITIONS.field_avail_vertical_align.default = {
    Field = {
        top = OBJECTS_DEFINITIONS.field_avail_vertical_align.enum.top
    },
    Literal = {
        top = OBJECTS_DEFINITIONS.field_avail_vertical_align.enum.top
    },
    ProtectedLiteral = {
        top = OBJECTS_DEFINITIONS.field_avail_vertical_align.enum.top
    },
    BooleanField = {
        top = OBJECTS_DEFINITIONS.field_avail_vertical_align.enum.top
    },
    Image = {
        top = OBJECTS_DEFINITIONS.field_avail_vertical_align.enum.top
    },
    Line = {
        top = OBJECTS_DEFINITIONS.field_avail_vertical_align.enum.top
    },
    Fieldset = {
        top = OBJECTS_DEFINITIONS.field_avail_vertical_align.enum.top
    }
}

-- field_vertical_align: Alignement vertical par defaut
OBJECTS_DEFINITIONS.field_vertical_align.default = {
    Field = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top
    },
    Literal = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top
    },
    ProtectedLiteral = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top
    },
    BooleanField = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top
    },
    Image = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top
    },
    Line = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top
    },
    Fieldset = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top
    }
}

-- field_vertical_margin: Marge verticale
OBJECTS_DEFINITIONS.field_vertical_margin.default = {
    Field = {
        none = OBJECTS_DEFINITIONS.field_vertical_margin.enum.none
    },
    Literal = {
        none = OBJECTS_DEFINITIONS.field_vertical_margin.enum.none
    },
    ProtectedLiteral = {
        none = OBJECTS_DEFINITIONS.field_vertical_margin.enum.none
    },
    BooleanField = {
        none = OBJECTS_DEFINITIONS.field_vertical_margin.enum.none
    },
    Image = {
        none = OBJECTS_DEFINITIONS.field_vertical_margin.enum.none
    },
    Line = {
        none = OBJECTS_DEFINITIONS.field_vertical_margin.enum.none
    },
    Fieldset = {
        none = OBJECTS_DEFINITIONS.field_vertical_margin.enum.none
    }
}

-- ===== NIVEAU 2: Proprietes qui referencent le Niveau 1 =====

-- field_avail_required_marker: Marqueur pour champs requis
OBJECTS_DEFINITIONS.field_avail_required_marker.default = {
    Field = {
        required = OBJECTS_DEFINITIONS.field_avail_required_marker.enum.required
    },
    Literal = {
        required = OBJECTS_DEFINITIONS.field_avail_required_marker.enum.required
    },
    ProtectedLiteral = {
        required = OBJECTS_DEFINITIONS.field_avail_required_marker.enum.required
    },
    BooleanField = {
        required = OBJECTS_DEFINITIONS.field_avail_required_marker.enum.required
    },
    Image = {
        required = OBJECTS_DEFINITIONS.field_avail_required_marker.enum.required
    },
    Line = {
        required = OBJECTS_DEFINITIONS.field_avail_required_marker.enum.required
    },
    Fieldset = {
        required = OBJECTS_DEFINITIONS.field_avail_required_marker.enum.required
    }
}

-- field_required_marker: Marqueur de champs requis (reference Niveau 2)
OBJECTS_DEFINITIONS.field_required_marker.default = {
    Field = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Field,
    Literal = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_required_marker.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_required_marker.default.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Image,
    Line = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Fieldset
}

-- field_avail_error_marker: Marqueur pour champs en erreur
OBJECTS_DEFINITIONS.field_avail_error_marker.default = {
    Field = {
        error = OBJECTS_DEFINITIONS.field_avail_error_marker.enum.error
    },
    Literal = {
        error = OBJECTS_DEFINITIONS.field_avail_error_marker.enum.error
    },
    ProtectedLiteral = {
        error = OBJECTS_DEFINITIONS.field_avail_error_marker.enum.error
    },
    BooleanField = {
        error = OBJECTS_DEFINITIONS.field_avail_error_marker.enum.error
    },
    Image = {
        error = OBJECTS_DEFINITIONS.field_avail_error_marker.enum.error
    },
    Line = {
        error = OBJECTS_DEFINITIONS.field_avail_error_marker.enum.error
    },
    Fieldset = {
        error = OBJECTS_DEFINITIONS.field_avail_error_marker.enum.error
    }
}

-- field_error_marker: Marqueur de champs en erreur (reference Niveau 2)
OBJECTS_DEFINITIONS.field_error_marker.default = {
    Field = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Field,
    Literal = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_error_marker.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_error_marker.default.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Image,
    Line = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Fieldset
}
OBJECTS_DEFINITIONS.field_footer_required_marker.default = {
    Field = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Field,
    Literal = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_required_marker.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_required_marker.default.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Image,
    Line = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Fieldset
}
OBJECTS_DEFINITIONS.field_footer_error_marker.default = {
    Field = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Field,
    Literal = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_error_marker.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_error_marker.default.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Image,
    Line = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Fieldset
}
-- ===== NIVEAU 3: Proprietes qui referencent le Niveau 2 =====

-- field_title_suffix: Suffixe du titre (reference required_marker et error_marker du Niveau 2)
OBJECTS_DEFINITIONS.field_title_suffix.default = {
    Field = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Field,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Field,
        none = ""
    },
    Literal = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Literal,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Literal,
        none = ""
    },
    ProtectedLiteral = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.ProtectedLiteral,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.ProtectedLiteral,
        none = ""
    },
    BooleanField = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.BooleanField,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.BooleanField,
        none = ""
    },
    Image = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Image,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Image,
        none = ""
    },
    Line = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Line,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Line,
        none = ""
    },
    Fieldset = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Fieldset,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Fieldset,
        none = ""
    }
}

-- field_title_prefix: Prefixe du titre (reference required_marker et error_marker du Niveau 2)
OBJECTS_DEFINITIONS.field_title_prefix.default = {
    Field = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Field,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Field,
        none = ""
    },
    Literal = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Literal,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Literal,
        none = ""
    },
    ProtectedLiteral = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.ProtectedLiteral,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.ProtectedLiteral,
        none = ""
    },
    BooleanField = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.BooleanField,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.BooleanField,
        none = ""
    },
    Image = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Image,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Image,
        none = ""
    },
    Line = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Line,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Line,
        none = ""
    },
    Fieldset = {
        enabled = false,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Fieldset,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Fieldset,
        none = ""
    }
}

-- field_footer_title: Titre du pied de page
OBJECTS_DEFINITIONS.field_footer_title.default = {
    Field = {
        title = ""
    },
    Literal = {
        title = ""
    },
    ProtectedLiteral = {
        title = ""
    },
    BooleanField = {
        title = ""
    },
    Image = {
        title = ""
    },
    Line = {
        title = ""
    },
    Fieldset = {
        title = ""
    }
}

-- field_footer_fill_char: Caractere de remplissage pour le pied de page
OBJECTS_DEFINITIONS.field_footer_fill_char.default = {
    Field = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash
    },
    Literal = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash
    },
    ProtectedLiteral = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash
    },
    BooleanField = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash
    },
    Image = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash
    },
    Line = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash
    },
    Fieldset = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash
    }
}

-- field_footer_align: Alignement du pied de page
OBJECTS_DEFINITIONS.field_footer_align.default = {
    Field = {
        left = OBJECTS_DEFINITIONS.field_footer_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_footer_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_footer_align.enum.right
    },
    Literal = {
        left = OBJECTS_DEFINITIONS.field_footer_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_footer_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_footer_align.enum.right
    },
    ProtectedLiteral = {
        left = OBJECTS_DEFINITIONS.field_footer_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_footer_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_footer_align.enum.right
    },
    BooleanField = {
        left = OBJECTS_DEFINITIONS.field_footer_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_footer_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_footer_align.enum.right
    },
    Image = {
        left = OBJECTS_DEFINITIONS.field_footer_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_footer_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_footer_align.enum.right
    },
    Line = {
        left = OBJECTS_DEFINITIONS.field_footer_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_footer_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_footer_align.enum.right
    },
    Fieldset = {
        left = OBJECTS_DEFINITIONS.field_footer_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_footer_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_footer_align.enum.right
    }
}

-- field_footer: Pied de page (reference required_marker et error_marker du Niveau 2)
OBJECTS_DEFINITIONS.field_footer.default = {
    Field = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Field,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Field,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Field,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Field,
        required_marker = OBJECTS_DEFINITIONS.field_footer_required_marker.default.Field,
        error_marker = OBJECTS_DEFINITIONS.field_footer_error_marker.default.Field
    },
    Literal = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Literal,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Literal,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Literal,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Literal,
        required_marker = OBJECTS_DEFINITIONS.field_footer_required_marker.default.Literal,
        error_marker = OBJECTS_DEFINITIONS.field_footer_error_marker.default.Literal
    },
    ProtectedLiteral = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.ProtectedLiteral,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.ProtectedLiteral,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.ProtectedLiteral,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.ProtectedLiteral,
        required_marker = OBJECTS_DEFINITIONS.field_footer_required_marker.default.ProtectedLiteral,
        error_marker = OBJECTS_DEFINITIONS.field_footer_error_marker.default.ProtectedLiteral
    },
    BooleanField = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.BooleanField,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.BooleanField,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.BooleanField,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.BooleanField,
        required_marker = OBJECTS_DEFINITIONS.field_footer_required_marker.default.BooleanField,
        error_marker = OBJECTS_DEFINITIONS.field_footer_error_marker.default.BooleanField
    },
    Image = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Image,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Image,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Image,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Image,
        required_marker = OBJECTS_DEFINITIONS.field_footer_required_marker.default.Image,
        error_marker = OBJECTS_DEFINITIONS.field_footer_error_marker.default.Image
    },
    Line = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Line,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Line,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Line,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Line,
        required_marker = OBJECTS_DEFINITIONS.field_footer_required_marker.default.Line,
        error_marker = OBJECTS_DEFINITIONS.field_footer_error_marker.default.Line
    },
    Fieldset = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Fieldset,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Fieldset,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Fieldset,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Fieldset,
        required_marker = OBJECTS_DEFINITIONS.field_footer_required_marker.default.Fieldset,
        error_marker = OBJECTS_DEFINITIONS.field_footer_error_marker.default.Fieldset
    }
}

-- ===== NIVEAU 3 (suite): Proprietes de position et alignement =====
OBJECTS_DEFINITIONS.field_avail_pos.default = {
    Field = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.enum.position
    },
    Literal = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.enum.position
    },
    ProtectedLiteral = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.enum.position
    },
    BooleanField = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.enum.position
    },
    Image = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.enum.position
    },
    Line = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.enum.position
    },
    Fieldset = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.enum.position
    }
} -- Available positions for each field type
-- field_pos: Position par defaut pour chaque type de champ (reference field_avail_pos du Niveau 3)
OBJECTS_DEFINITIONS.field_pos.default = {
    Field = OBJECTS_DEFINITIONS.field_avail_pos.default.Field,
    Literal = OBJECTS_DEFINITIONS.field_avail_pos.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_pos.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_pos.default.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_avail_pos.default.Image,
    Line = OBJECTS_DEFINITIONS.field_avail_pos.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_pos.default.Fieldset
} -- Default position for each field type

-- field_title_align: Alignement du titre
OBJECTS_DEFINITIONS.field_title_align.default = {
    -- Field/Literal/ProtectedLiteral/BooleanField/Image: titre a gauche par defaut
    Field = {
        left = OBJECTS_DEFINITIONS.field_title_align.enum.left,
        right = OBJECTS_DEFINITIONS.field_title_align.enum.right,
        center = OBJECTS_DEFINITIONS.field_title_align.enum.center
    },
    Literal = {
        left = OBJECTS_DEFINITIONS.field_title_align.enum.left
    },
    ProtectedLiteral = {
        left = OBJECTS_DEFINITIONS.field_title_align.enum.left
    },
    BooleanField = {
        left = OBJECTS_DEFINITIONS.field_title_align.enum.left,
        right = OBJECTS_DEFINITIONS.field_title_align.enum.right
    },
    Image = {
        left = OBJECTS_DEFINITIONS.field_title_align.enum.left,
        middle = OBJECTS_DEFINITIONS.field_title_align.enum.center,
        right = OBJECTS_DEFINITIONS.field_title_align.enum.right
    },
    -- Line: titre centre par defaut (ligne horizontale)
    Line = {
        left = OBJECTS_DEFINITIONS.field_title_align.enum.center
    },
    -- Fieldset: titre centre par defaut (conteneur)
    Fieldset = {
        left = OBJECTS_DEFINITIONS.field_title_align.enum.center
    }
}

-- field_children: Autorisation des enfants (pour Fieldset)
OBJECTS_DEFINITIONS.field_children.default = {
    Field = {
        authorised = false,
        none = true
    },
    Literal = {
        authorised = false,
        none = true
    },
    ProtectedLiteral = {
        authorised = false,
        none = true
    },
    BooleanField = {
        authorised = false,
        none = true
    },
    Image = {
        authorised = false,
        none = true
    },
    Line = {
        authorised = false,
        none = true
    },
    Fieldset = {
        authorised = true,
        none = false
    }
}

-- ===== NIVEAU 4: Proprietes qui referencent le Niveau 3 =====

-- field_height: Hauteur complete avec min/max (reference field_min_height et field_max_height du Niveau 1)
OBJECTS_DEFINITIONS.field_height.default = {
    Field = {
        min = OBJECTS_DEFINITIONS.field_min_height.enum.Field,
        max = OBJECTS_DEFINITIONS.field_max_height.enum.Field,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Field,
        edited = nil
    },
    Literal = {
        min = OBJECTS_DEFINITIONS.field_min_height.enum.Literal,
        max = OBJECTS_DEFINITIONS.field_max_height.enum.Literal,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Literal,
        edited = nil
    },
    ProtectedLiteral = {
        min = OBJECTS_DEFINITIONS.field_min_height.enum.ProtectedLiteral,
        max = OBJECTS_DEFINITIONS.field_max_height.enum.ProtectedLiteral,
        initial = OBJECTS_DEFINITIONS.field_height.enum.ProtectedLiteral,
        edited = nil
    },
    BooleanField = {
        min = OBJECTS_DEFINITIONS.field_min_height.enum.BooleanField,
        max = OBJECTS_DEFINITIONS.field_max_height.enum.BooleanField,
        initial = OBJECTS_DEFINITIONS.field_height.enum.BooleanField,
        edited = nil
    },
    Image = {
        min = OBJECTS_DEFINITIONS.field_min_height.enum.Image,
        max = OBJECTS_DEFINITIONS.field_max_height.enum.Image,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Image,
        edited = nil
    },
    Line = {
        min = OBJECTS_DEFINITIONS.field_min_height.enum.Line,
        max = OBJECTS_DEFINITIONS.field_max_height.enum.Line,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Line,
        edited = nil
    },
    Fieldset = {
        min = OBJECTS_DEFINITIONS.field_min_height.enum.Fieldset,
        max = OBJECTS_DEFINITIONS.field_max_height.enum.Fieldset,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Fieldset,
        edited = nil
    }
} -- Default height for each field type (min, max, initial, edited)

-- field_width: Largeur complete avec min/max (reference field_min_width et field_max_width du Niveau 1)
OBJECTS_DEFINITIONS.field_width.default = {
    Field = {
        min = OBJECTS_DEFINITIONS.field_min_width.default.Field,
        max = OBJECTS_DEFINITIONS.field_max_width.default.Field,
        initial = OBJECTS_DEFINITIONS.field_width.enum.Field,
        edited = nil
    },
    Literal = {
        min = OBJECTS_DEFINITIONS.field_min_width.default.Literal,
        max = OBJECTS_DEFINITIONS.field_max_width.default.Literal,
        initial = OBJECTS_DEFINITIONS.field_width.enum.Literal,
        edited = nil
    },
    ProtectedLiteral = {
        min = OBJECTS_DEFINITIONS.field_min_width.default.ProtectedLiteral,
        max = OBJECTS_DEFINITIONS.field_max_width.default.ProtectedLiteral,
        initial = OBJECTS_DEFINITIONS.field_width.enum.ProtectedLiteral,
        edited = nil
    },
    BooleanField = {
        min = OBJECTS_DEFINITIONS.field_min_width.default.BooleanField,
        max = OBJECTS_DEFINITIONS.field_max_width.default.BooleanField,
        initial = OBJECTS_DEFINITIONS.field_width.enum.BooleanField,
        edited = nil
    },
    Image = {
        min = OBJECTS_DEFINITIONS.field_min_width.default.Image,
        max = OBJECTS_DEFINITIONS.field_max_width.default.Image,
        initial = OBJECTS_DEFINITIONS.field_width.enum.Image,
        edited = nil
    },
    Line = {
        min = OBJECTS_DEFINITIONS.field_min_width.default.Line,
        max = OBJECTS_DEFINITIONS.field_max_width.default.Line,
        initial = OBJECTS_DEFINITIONS.field_width.enum.Line,
        edited = nil
    },
    Fieldset = {
        min = OBJECTS_DEFINITIONS.field_min_width.default.Fieldset,
        max = OBJECTS_DEFINITIONS.field_max_width.default.Fieldset,
        initial = OBJECTS_DEFINITIONS.field_width.enum.Fieldset,
        edited = nil
    }
} -- Default length for each field type (min, max, initial, edited)

-- ===== NIVEAU 5: Proprietes qui referencent le Niveau 4 ou autres =====

-- field_attrb: Attributs des champs
OBJECTS_DEFINITIONS.field_attrb.default = {
    Field = {
        field_in_edit_mode = false,
        field_visible = true,
        field_required = false,
        field_has_error = false,
        field_readonly = false,
        field_enabled = true,
        field_focused = false,
        field_selected = false,
        field_highlighted = false,
        field_hidden = false,
        field_protected = false,
        field_numeric = false
    },
    Literal = {
        field_in_edit_mode = false,
        field_visible = true,
        field_required = false,
        field_has_error = false,
        field_readonly = true,
        field_enabled = true,
        field_focused = false,
        field_selected = false,
        field_highlighted = false,
        field_hidden = false,
        field_protected = false,
        field_numeric = false
    },
    ProtectedLiteral = {
        field_in_edit_mode = false,
        field_visible = true,
        field_required = false,
        field_has_error = false,
        field_readonly = true,
        field_enabled = true,
        field_focused = false,
        field_selected = false,
        field_highlighted = false,
        field_hidden = false,
        field_protected = true,
        field_numeric = false
    },
    BooleanField = {
        field_in_edit_mode = false,
        field_visible = true,
        field_required = false,
        field_has_error = false,
        field_readonly = false,
        field_enabled = true,
        field_focused = false,
        field_selected = false,
        field_highlighted = false,
        field_hidden = false,
        field_protected = false,
        field_numeric = false
    },
    Image = {
        field_in_edit_mode = false,
        field_visible = true,
        field_required = false,
        field_has_error = false,
        field_readonly = true,
        field_enabled = true,
        field_focused = false,
        field_selected = false,
        field_highlighted = false,
        field_hidden = false,
        field_protected = false,
        field_numeric = false
    },
    Line = {
        field_in_edit_mode = false,
        field_visible = true,
        field_required = false,
        field_has_error = false,
        field_readonly = true,
        field_enabled = true,
        field_focused = false,
        field_selected = false,
        field_highlighted = false,
        field_hidden = false,
        field_protected = false,
        field_numeric = false
    },
    Fieldset = {
        field_in_edit_mode = false,
        field_visible = true,
        field_required = false,
        field_has_error = false,
        field_readonly = true,
        field_enabled = true,
        field_focused = false,
        field_selected = false,
        field_highlighted = false,
        field_hidden = false,
        field_protected = false,
        field_numeric = false
    }

} -- BMS available field attributes

-- field_initial: Valeurs initiales
OBJECTS_DEFINITIONS.field_initial.default = {
    Field = {
        initial_value = "text",
        option_value = nil
    },
    Literal = {
        initial_value = "text",
        option_value = nil
    },
    ProtectedLiteral = {
        initial_value = "text",
        option_value = nil
    },
    BooleanField = {
        initial_value = false,
        option_value = nil
    }, -- Case non cochée par défaut
    Image = {
        initial_value = nil,
        option_value = {
            ascii_code = nil,
            file_path = nil
        }
    }, -- Valeur initiale pour Image (ASCII code + chemin du fichier)
    Line = {
        initial_value = nil,
        option_value = nil
    },
    Fieldset = {
        initial_value = "title",
        option_value = nil
    }
}

-- visual_representation: Representations visuelles (fonctions, pas de .default a extraire)
OBJECTS_DEFINITIONS.field_border = {
    default = {
        -- Field : Toutes les combinaisons style+chars disponibles pour Field
        Field = {
            style = OBJECTS_DEFINITIONS.field_avail_border_style.default.Field,
            chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default.Field
        },
        -- Literal : Toutes les combinaisons style+chars disponibles pour Literal
        Literal = {
            style = OBJECTS_DEFINITIONS.field_avail_border_style.default.Literal,
            chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default.Literal
        },
        -- ProtectedLiteral : Toutes les combinaisons style+chars disponibles pour ProtectedLiteral
        ProtectedLiteral = {
            style = OBJECTS_DEFINITIONS.field_avail_border_style.default.ProtectedLiteral,
            chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default.ProtectedLiteral
        },
        -- BooleanField : Toutes les combinaisons style+chars disponibles pour BooleanField
        BooleanField = {
            style = OBJECTS_DEFINITIONS.field_avail_border_style.default.BooleanField,
            chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default.BooleanField
        },
        -- Image : Toutes les combinaisons style+chars disponibles pour Image
        Image = {
            style = OBJECTS_DEFINITIONS.field_avail_border_style.default.Image,
            chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default.Image
        },
        -- Line : Toutes les combinaisons style+chars disponibles pour Line
        Line = {
            style = OBJECTS_DEFINITIONS.field_avail_border_style.default.Line,
            chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default.Line
        },
        -- Fieldset : Toutes les combinaisons style+chars disponibles pour Fieldset
        Fieldset = {
            style = OBJECTS_DEFINITIONS.field_avail_border_style.default.Fieldset,
            chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default.Fieldset
        }
    }, -- Toutes les combinaisons style+chars disponibles par type (GUI: [style|v] + [chars|v])
    initial = nil, -- Default border for the initial field type
    edited = nil -- Border after editing
}

-- ===== PROPRIETES MANQUANTES POUR field_footer =====

-- ===== NOUVELLE VERSION DE field_footer AVEC LES 5 PROPRIETES =====

-- ===== FONCTIONS HELPERS POUR LE RENDU =====

-- Helper: Get the current value for a property (edited if set, otherwise initial)
local function get_property(obj, prop_name)
    local prop = obj[prop_name]
    if not prop then
        return nil
    end
    if type(prop) == "table" and prop.edited ~= nil then
        return prop.edited
    elseif type(prop) == "table" and prop.initial ~= nil then
        return prop.initial
    else
        return prop
    end
end

-- Helper: Get border characters for a given style
local function get_border_chars(obj)
    local border_style = get_property(obj, "field_border_style") or "none"
    local obj_type = get_property(obj, "field_type") or "Field"
    local chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default[obj_type]
    if chars and chars[border_style] then
        return chars[border_style]
    end
    -- Fallback to single style
    if chars and chars.single then
        return chars.single
    end
    -- Ultimate fallback
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

-- Helper: Render a simple bordered field (Field, Literal, ProtectedLiteral, BooleanField, Image)
function render_bordered_field(obj, custom_content)
    local height = get_property(obj, "field_height") or 3
    local width = get_property(obj, "field_width") or 10
    local border_style = get_property(obj, "field_border_style") or "none"
    local border_chars = get_border_chars(obj)
    local fill_char = get_property(obj, "field_fill_char") or " "
    local obj_type = get_property(obj, "field_type") or "Field"

    -- Determine content
    local content
    if custom_content then
        content = custom_content
    elseif obj_type == "BooleanField" then
        local initial_value = get_property(obj, "field_initial")
        if initial_value and type(initial_value) == "table" then
            content = initial_value.initial_value and "[X]" or "[ ]"
        else
            content = "[ ]"
        end
    else
        local initial_value = get_property(obj, "field_initial")
        if initial_value and type(initial_value) == "table" and initial_value.initial_value then
            content = tostring(initial_value.initial_value)
        else
            content = ""
        end
    end

    local lines = {}

    -- If no border, just return content centered
    if border_style == "none" then
        if height >= 1 then
            local line = string.rep(fill_char, width)
            -- Try to center content in the line
            if content and #content > 0 and #content <= width then
                local padding = math.floor((width - #content) / 2)
                line = string.rep(" ", padding) .. content .. string.rep(" ", width - padding - #content)
            else
                line = content or line
            end
            table.insert(lines, line)
        end
        return table.concat(lines, "\n")
    end

    -- Top border
    if height >= 1 then
        local top_line = border_chars.top_left .. string.rep(border_chars.top, width) .. border_chars.top_right
        table.insert(lines, top_line)
    end

    -- Content area
    local content_lines = {}
    if content and #content > 0 then
        -- Split content by newlines
        for subline in content:gmatch("[^\n]+") do
            table.insert(content_lines, subline)
        end
    else
        table.insert(content_lines, "")
    end

    -- Center content vertically
    local content_height = #content_lines
    local content_start = 1
    if height > 2 then
        content_start = math.floor((height - 1 - content_height) / 2) + 1
    end

    for i = 1, height - 2 do
        if i >= content_start and i < content_start + content_height then
            local content_line = content_lines[i - content_start + 1]
            local padding = width - #content_line
            if padding > 0 then
                local left_pad = math.floor(padding / 2)
                local right_pad = padding - left_pad
                content_line = string.rep(" ", left_pad) .. content_line .. string.rep(" ", right_pad)
            end
            -- Truncate if too long
            content_line = content_line:sub(1, width)
            -- Pad with fill_char if still too short
            content_line = content_line .. string.rep(fill_char, width - #content_line)
            table.insert(lines, border_chars.left .. content_line .. border_chars.right)
        else
            table.insert(lines, border_chars.left .. string.rep(fill_char, width) .. border_chars.right)
        end
    end

    -- Bottom border
    if height >= 2 then
        local bottom_line = border_chars.bottom_left .. string.rep(border_chars.bottom, width) ..
                                border_chars.bottom_right
        table.insert(lines, bottom_line)
    end

    return table.concat(lines, "\n")
end

-- Helper: Render a Line (horizontal line)
function render_line(obj)
    local width = get_property(obj, "field_width") or 40
    local border_style = get_property(obj, "field_border_style") or "none"
    local border_chars = get_border_chars(obj)
    local line_char = border_chars.top -- Use top border char for horizontal line

    if border_style == "none" then
        return string.rep("-", width)
    end

    return string.rep(line_char, width)
end

-- Helper: Render a Fieldset (container with title)
function render_fieldset(obj)
    local height = get_property(obj, "field_height") or 3
    local width = get_property(obj, "field_width") or 40
    local border_style = get_property(obj, "field_border_style") or "double"
    local border_chars = get_border_chars(obj)
    local fill_char = get_property(obj, "field_fill_char") or " "
    local title = get_property(obj, "field_name") or get_property(obj, "field_initial")

    -- Get actual title
    if title and type(title) == "table" then
        title = title.initial or title.edited or "Fieldset"
    else
        title = "Fieldset"
    end

    local lines = {}

    -- Top border with title
    local title_fill = get_property(obj, "field_title_fill_char") or " "
    if height >= 1 then
        local title_str = " " .. title .. " "
        local title_len = #title_str
        local content_width = width - 2

        if title_len > content_width then
            title_str = title_str:sub(1, content_width)
            title_len = content_width
        end

        local padding = content_width - title_len
        local left_fill = math.floor(padding / 2)
        local right_fill = padding - left_fill
        title_str = string.rep(title_fill, left_fill) .. title_str .. string.rep(title_fill, right_fill)

        local top_line = border_chars.top_left .. title_str .. border_chars.top_right
        table.insert(lines, top_line)
    end

    -- Content area
    for i = 1, height - 2 do
        table.insert(lines, border_chars.left .. string.rep(fill_char, width) .. border_chars.right)
    end

    -- Bottom border
    if height >= 2 then
        local bottom_line = border_chars.bottom_left .. string.rep(border_chars.bottom, width) ..
                                border_chars.bottom_right
        table.insert(lines, bottom_line)
    end

    return table.concat(lines, "\n")
end

-- ===== CONSTRUCTEUR D'OBJETS =====
function OBJECTS_DEFINITIONS.new(obj_type, overrides)
    local self = {}

    -- Copie de toutes les propriétés depuis la définition globale
    for prop_name, prop_def in pairs(OBJECTS_DEFINITIONS) do
        if type(prop_def) == "table" and prop_def.default and prop_def.default[obj_type] and
            type(prop_def.default[obj_type]) ~= "function" and prop_name ~= "visual_representation" then
            -- Initialisation : .default[Type] → .initial
            self[prop_name] = {
                initial = prop_def.default[obj_type],
                edited = nil
            }
        elseif type(prop_def) == "table" then
            -- Propriétés sans default (ex: visual_representation, field_children)
            -- Copie directe de la structure
            self[prop_name] = {}
            for k, v in pairs(prop_def) do
                self[prop_name][k] = v
            end
        else
            self[prop_name] = prop_def
        end
    end

    -- Type de l'objet (forcé)
    self.field_type = {
        initial = obj_type,
        edited = nil
    }

    -- Appliquer les surcharges si fournies (pour compatibilité ascendante)
    if overrides then
        for prop_name, prop_value in pairs(overrides) do
            if type(prop_value) == "table" and prop_value.initial ~= nil then
                -- Si on fournit {initial = valeur}, on le met dans .initial
                self[prop_name].initial = prop_value.initial
            elseif type(prop_value) == "table" and prop_value.edited ~= nil then
                -- Si on fournit {edited = valeur}, on le met dans .edited
                self[prop_name].edited = prop_value.edited
            elseif self[prop_name] and type(self[prop_name]) == "table" then
                -- Sinon, on suppose que c'est une valeur directe pour .initial
                self[prop_name].initial = prop_value
            else
                self[prop_name] = prop_value
            end
        end
    end

    -- Fonction de rendu attachée à l'objet
    self.render = function()
        return render_object(self)
    end

    return self
end

-- ===== FONCTION DE RENDU GENÉRIQUE =====
-- Rend n'importe quel objet (Field, Fieldset, etc.) selon ses propriétés
function render_object(obj)
    if not obj or not obj.field_type or not obj.field_type.initial then
        return "[Invalid Object]"
    end

    local obj_type = obj.field_type.initial
    local template = obj.visual_representation.default[obj_type]

    if not template then
        return "[" .. obj_type .. "]"
    end

    -- If template is a function, call it directly
    if type(template) == "function" then
        return template(obj)
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
