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
-- Description : All OBJECTS_DEFINITIONS have at least 3 row (field_height, field_height_min) and 3 col (field_width). The visual representation of each field type can be customized to fit the needs of the application, providing a powerful way to create user interfaces for BMS applications. The visual representation of each field type can be drawn line by line, with each line being drawn with appropriate properties such as color, font, style, etc.
-- Description : Each created object is internally saved in JSON format, which can be used to store and retrieve the properties and attributes of each field type. The JSON format provides a standardized way to represent the properties and attributes of each field type, allowing for easy integration with other applications and systems. The JSON format can also be used to export and import the properties and attributes of each field type, providing a flexible way to manage the fields in a BMS application.
-- Description : an instance of OBJECTS_DEFINITIONS can be created using the OBJECTS_DEFINITIONS.new(TYPE), which initialize an object with the specified TYPE, and set the initial values for each property based on the definitions in the OBJECTS_DEFINITIONS table. The new object can then be modified and customized as needed, allowing for the creation of custom field types and visual representations. The new object can also be saved in JSON format, providing a standardized way to store and retrieve the properties and attributes of each field type.
-- Description : an instance of OBJECTS_DEFINITIONS handle initial value and editing value state, which allow less code and memory usage, and allow to easily manage the properties and attributes of each field type. The initial value represents the default state of the field, while the edited value represents the modified state of the field after user input or other changes. The initial and edited values can be used to determine the current state of the field, allowing for easy management of the properties and attributes of each field type.
--     -- ***********************************************************
-- All OBJECTS_DEFINITIONS have at least 3 row (field_height, field_height_min) and 3 col (field_width).
-- 
-- ***********************************************************
OBJECTS_DEFINITIONS_GUI_TYPE =
    { -- definition of the GUI type for each field type, which can be used to render the fields in a WYSIWYG editor. The GUI type can be used to determine the visual representation of each field type, allowing for a flexible and extensible way to create user interfaces for BMS applications.
        gui_field_type = {
            gui_select_with_label_string = "gui_select_with_label_string", -- rendu graphique d'un select box (liste de choix) avec label et des kprops de type string
            gui_select_with_label_numeric = "gui_select_with_label_numeric", -- rendu graphique d'un select box (liste de choix) avec label et des kprops de type numeric
            gui_list_textornum_with_label_field = "gui_list_textornum_with_label_field", -- rendu graphique d'un field (liste de choix) avec label et des kprops de type string ou numeric
            gui_checkbox_with_label_field = "gui_checkbox_with_label_field", -- rendu graphique d'un field (checkbox avec label) avec des kprops de type string ou numeric
            gui_text_with_label_field = "gui_text_with_label_field", -- rendu graphique d'un field (text avec label) avec des kprops de type string ou numeric
            gui_text_field = "gui_text_field",
            gui_literal_field = "gui_literal_field",
            gui_protected_literal_field = "gui_protected_literal_field",
            gui_boolean_field = "gui_boolean_field",
            gui_image_field = "gui_image_field",
            gui_line_field = "gui_line_field",
            gui_fieldset_field = "gui_fieldset_field"
        }
    }
OBJECTS_DEFINITIONS_DEFAULTS = {
    objects_types = {
        field_type = {
            enum = {FieldTextORNumeric, Literal, ProtectedLiteral, BooleanField, ImageAsciiArt, Line, Fieldset},
            default = nil -- Default field type
        },
        field_name = {
            enum = {
                FieldTextORNumeric = "Field Text or Numeric",
                Literal = "Literal Text",
                ProtectedLiteral = "Protected Literal",
                BooleanField = "Boolean Field",
                ImageAsciiArt = "ImageAsciiArt / Ascii Art",
                Line = "Line Separator",
                Fieldset = "Fieldset Group"
            },
            default = nil -- Default field name
        }

    },
    field_size = {
        fixed_size = {
            enum = {
                small = "small",
                medium = "medium",
                large = "large"
            },
            default = "medium"
        },
        width = {
            FieldTextORNumeric = 10,
            Literal = 20,
            ProtectedLiteral = 20,
            BooleanField = 10,
            ImageAsciiArt = 40,
            Line = 40,
            Fieldset = 40
        },
        height = {
            FieldTextORNumeric = 3,
            Literal = 3,
            ProtectedLiteral = 3,
            BooleanField = 3,
            ImageAsciiArt = 5,
            Line = 1,
            Fieldset = 3
        },
        min_width = {
            FieldTextORNumeric = 5,
            Literal = 10,
            ProtectedLiteral = 10,
            BooleanField = 5,
            ImageAsciiArt = 20,
            Line = 20,
            Fieldset = 20
        },
        max_width = {
            FieldTextORNumeric = 255,
            Literal = 255,
            ProtectedLiteral = 255,
            BooleanField = 255,
            ImageAsciiArt = 255,
            Line = 255,
            Fieldset = 255
        },
        min_height = {
            FieldTextORNumeric = 1,
            Literal = 1,
            ProtectedLiteral = 1,
            BooleanField = 1,
            ImageAsciiArt = 3,
            Line = 1,
            Fieldset = 1
        },
        max_height = {
            FieldTextORNumeric = 80,
            Literal = 80,
            ProtectedLiteral = 80,
            BooleanField = 3,
            ImageAsciiArt = 40,
            Line = 1,
            Fieldset = 80
        }
    },
    field_pos = {
        enum = {
            position = {
                row = 0,
                col = 0,
                rowend = 0,
                colend = 0
            }
        },
        default = {
            position = {
                row = 0,
                col = 0,
                rowend = 0,
                colend = 0
            }
        }
    },
    color_enum = {
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
        color_codes = {
            default = "\27[0m",
            black = "\27[30m",
            red = "\27[31m",
            green = "\27[32m",
            yellow = "\27[33m",
            blue = "\27[34m",
            magenta = "\27[35m",
            cyan = "\27[36m",
            white = "\27[37m",
            bright_black = "\27[90m",
            bright_red = "\27[91m",
            bright_green = "\27[92m",
            bright_yellow = "\27[93m",
            bright_blue = "\27[94m",
            bright_magenta = "\27[95m",
            bright_cyan = "\27[96m",
            bright_white = "\27[97m"
        }
    },
    font_family = {
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
        }
    },
    text_style = {
        styles = {
            enum = {
                default = "default",
                bold = "bold",
                italic = "italic",
                underline = "underline",
                blink = "blink",
                reverse = "reverse"
            },
            default = "default"
        },
        enum_style_codes = {
            enum = {
                default = "\27[0m",
                bold = "\27[1m",
                italic = "\27[3m",
                underline = "\27[4m",
                blink = "\27[5m",
                reverse = "\27[7m"
            },
            default = "\27[0m" -- Default style code
            -- Correction: It should be "\27[0m" to match the default style code
        },
        style_help = {
            enum = {
                default = "Default style (no attributes)",
                bold = "Bold style (intensity on 3270)",
                italic = "Italic style (not supported by 3270 hardware)",
                underline = "Underline style",
                blink = "Blinking text",
                reverse = "Reverse video"
            },
            default = "default" -- Default style help
        },
        style_exported_value = {
            enum = {
                default = 0, -- No attributes
                bold = 1, -- A_BOLD (intensity on 3270)
                italic = 2, -- A_ITALIC (not supported by 3270 hardware)
                underline = 4, -- A_UNDERLINE
                blink = 16, -- A_BLINK
                reverse = 32 -- A_REVERSE
            },
            default = 0 -- No attributes
        }
    },
    text_align = {
        enum = {
            left = "left",
            center = "center",
            right = "right"
        },
        default = "left"
    },
    vertical_align = {
        enum = {
            top = "top",
            middle = "middle",
            bottom = "bottom"
        },
        default = "top"
    },
    vertical_margin = {
        enum = {
            none = 0,
            small = 1,
            medium = 2,
            large = 3
        },
        default = 0
    },
    border_style = {
        enum = {
            none = "none",
            solid = "solid",
            dashed = "dashed",
            dotted = "dotted"
        },
        default = "none" -- Default border style
    },
    fill_char = {
        enum = {
            space = " ",
            dash = "─",
            equal = "=",
            underscore = "_",
            dot = ".",
            asterisk = "*",
            pipe = "|",
            exclamation = "!",
            plus = "+",
            question = "?"
        },
        default = "space" -- Default fill character
    },
    required_marker = {
        enum = {
            none = {
                enabled = false,
                enabled_marker = false,
                marker_fill = "",
                enabled_sentence = false,
                sentence = "",
                none = ""
            },
            required = {
                enabled = false,
                enabled_marker = false,
                marker_fill = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            }
        },
        default = "none" -- Default required marker configuration
    },
    error_marker = {
        enum = {
            error = {
                enabled = false,
                enabled_marker = false,
                marker_fill = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            none = {
                enabled = false,
                enabled_marker = false,
                marker_fill = "",
                enabled_sentence = false,
                sentence = "",
                none = ""
            }
        },
        default = "none" -- Default error marker configuration
    },
    border_char = {
        enum = {
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
        default = "none" -- Default border character set
    }
}

-- ***********************************************************
-- ***********************************************************
OBJECTS_DEFINITIONS = {
    field_name = { -- Name of the object
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_text_with_label_field,
        gui_field_name = "Name",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.objects_types.field_name.enum,
        default = OBJECTS_DEFINITIONS_DEFAULTS.objects_types.field_name.default, -- Available field names
        initial = nil, -- Default field name
        edited = nil -- name after editing
    },

    field_type = { -- Type of the field, can be Field, Literal, ProtectedLiteral, BooleanField, ImageAsciiArt, Line, Fieldset
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Type",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.objects_types.field_type.enum, -- Available field types
        default = OBJECTS_DEFINITIONS_DEFAULTS.objects_types.field_type.default, -- default type for each field type
        initial = nil, -- Default field type
        edited = nil
    },

    field_min_height = { -- Height of the field, can be any positive integer
        enum = OBJECTS_DEFINITIONS_DEFAULTS.field_size.min_height,
        default = nil, --  Default height for each field type
        initial = nil, -- Default height for the initial field type
        edited = nil
    },

    field_max_height = { -- Maximum height of the field, can be any positive integer
        enum = OBJECTS_DEFINITIONS_DEFAULTS.field_size.max_height,
        default = nil, --  Default max height for each field type
        initial = nil, -- Default max height for the initial field type
        edited = nil
    },

    field_width_max = { -- Maximum length of the field, can be any positive integer
        enum = OBJECTS_DEFINITIONS_DEFAULTS.field_size.max_width,
        default = nil, -- Default max length for each field type
        initial = nil, -- Default max length for the initial field type
        edited = nil -- Max length after editing
    },

    field_width_min = { -- Minimum length of the field, can be any positive integer
        enum = OBJECTS_DEFINITIONS_DEFAULTS.field_size.min_width,
        default = nil, -- Default min length for each field type
        initial = nil, -- Default min length for the initial field type
        edited = nil -- Min length after editing
    },

    ----- ===== DIMENSIONS DU CHAMP =====
    field_height = { -- Height of the field, can be any positive integer
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_text_with_label_field,
        gui_field_name = "Height",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.field_size.height,
        default = nil, -- Default height for each field type (min, max, initial, edited)
        initial = nil, -- Default height for the initial field type
        edited = nil
    },
    ----- ===== DIMENSIONS DU CHAMP =====
    field_width = { -- Length of the field, can be any positive integer
        enum = OBJECTS_DEFINITIONS_DEFAULTS.field_size.width,
        default = nil, -- Default length for each field type (min, max, initial, edited)
        initial = nil, -- Default length for the initial field type
        edited = nil -- Length after editing
    },

    -- ===== COULEURS =====

    field_avail_color = {
        enum = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum,
        enum_color_codes = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum_color_codes,
        -- Table de mapping des couleurs (à initialiser avec start_color() en ncurses)
        avail_color_exported_value = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.exported_value,
        avail_color_help = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.help,
        default = nil, -- Combinaisons UX par type (1ere = valeur par defaut pour .initial)
        initial = nil,
        edited = nil
    },

    field_border_color = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Border Color",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum,
        enum_color_codes = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum_color_codes,
        default = nil, -- Default border color for each field type
        initial = nil,
        edited = nil
    },

    field_title_color = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Title Color",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum,
        enum_color_codes = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum_color_codes,
        default = nil,
        initial = nil,
        edited = nil
    },

    field_text_color = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Text Color",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum,
        enum_color_codes = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum_color_codes,
        default = nil,
        initial = nil,
        edited = nil
    },
    field_avail_footer_color = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Avail Footer Color",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum,
        enum_color_codes = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum_color_codes,
        default = nil,
        initial = nil,
        edited = nil
    },
    field_footer_color = {

        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Footer Color",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum, -- Available colors for each field type

        enum_color_codes = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum_color_codes, -- Color codes for each field type
        default = nil,
        initial = nil,
        edited = nil
    },
    ----- ===== POLICE DU TEXTE =====
    -- enum for font family: 3270/BMS terminals have ONLY ONE fixed-width font
    field_avail_font_family = {
        enum = OBJECTS_DEFINITIONS_DEFAULTS.font_family.enum,
        enum_color_codes = OBJECTS_DEFINITIONS_DEFAULTS.font_family.enum_color_codes,
        default = {
            -- All BMS field types reference the same single font (3270 has only one physical font)
            FieldTextORNumeric = {
                ncurses = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.ncurses.default,
                tn3270 = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.tn3270.default,
                bms = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.bms.default
            },
            Literal = {
                ncurses = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.ncurses.default,
                tn3270 = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.tn3270.default,
                bms = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.bms.default
            },
            ProtectedLiteral = {
                ncurses = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.ncurses.default,
                tn3270 = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.tn3270.default,
                bms = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.bms.default
            },
            BooleanField = {
                ncurses = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.ncurses.default,
                tn3270 = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.tn3270.default,
                bms = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.bms.default
            },
            ImageAsciiArt = {
                ncurses = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.ncurses.default,
                tn3270 = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.tn3270.default,
                bms = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.bms.default
            },
            Line = {
                ncurses = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.ncurses.default,
                tn3270 = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.tn3270.default,
                bms = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.bms.default
            },
            Fieldset = {
                ncurses = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.ncurses.default,
                tn3270 = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.tn3270.default,
                bms = OBJECTS_DEFINITIONS_DEFAULTS.font_family.default.bms.default
            }
        },
        initial = nil, -- Default font family for the initial field type
        edited = nil -- Font family after editing
    },

    -- Font family for each field type, referencing field_avail_font_family enum
    -- Note: 3270 terminals use a single fixed-width font; no font family selection in BMS
    field_font_family = {
        enum = OBJECTS_DEFINITIONS_DEFAULTS.font_family.enum,
        default = nil, -- Default font family for each field type
        initial = nil, -- Default font family for the initial field type
        edited = nil -- Font family after editing
    },
    ----- ===== STYLE DU TEXTE =====
    -- enum for text style: default, bold, italic, underline, strikethrough, blink, reverse
    field_avail_style = {
        enum = OBJECTS_DEFINITIONS_DEFAULTS.text_style.styles.enum,
        avail_style_help = OBJECTS_DEFINITIONS_DEFAULTS.text_style.style_help,
        avail_style_exported_value = OBJECTS_DEFINITIONS_DEFAULTS.text_style.style_exported_value,
        default = nil -- Styles disponibles par type (1er = valeur par defaut pour .initial)
    },
    -- Represents the style for each field type, referencing field_avail_style enum for consistency
    -- Adapted per field type considering user visual experience (UX)
    field_style = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Style",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.text_style.styles.enum,
        avail_style_help = OBJECTS_DEFINITIONS_DEFAULTS.text_style.style_help,
        avail_style_exported_value = OBJECTS_DEFINITIONS_DEFAULTS.text_style.style_exported_value,
        default = nil, -- Style par defaut pour chaque type
        initial = nil, -- Default style for the initial field type
        edited = nil -- Style after editing
    },
    ----- ===== ALIGNEMENT DU TEXTE =====
    -- enum for text alignment: left, center, right
    field_avail_text_align = {
        enum = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum,
        default = nil, -- Available text alignment for each field type
        initial = nil, -- Default text alignment for the initial field type
        edited = nil -- Text alignment after editing
    },
    -- field_text_align represents the text alignment for each field type, which can be left, center, or right
    field_text_align = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Text Align",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum,
        default = nil, -- Default text alignment for each field type
        initial = nil, -- Default text alignment for the initial field type
        edited = nil -- Text alignment after editing
    },
    field_avail_pos = { -- Represents the available positions for each field type in the BMS screen (row, col)
        enum = OBJECTS_DEFINITIONS_DEFAULTS.field_pos.enum,
        default = nil, -- Default position for each field type
        initial = nil, -- Default position for the initial field type
        edited = nil -- Position after editing
    },
    field_pos = { -- Represents the position of the field in the BMS screen (row, col)
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_text_with_label_field,
        gui_field_name = "Position",
        collapsed = false,
        collapsable = true,
        default = nil, -- Default position for each field type
        initial = nil, -- Default position for the initial field type
        edited = nil -- Position after editing
    },
    -- ===== PERSONNALISATION DES BORDURES =====

    -- ===== PERSONNALISATION DES CARACTERES =====
    -- Caractères de bordure personnalisables (pour remplacer ┌─┐│├└┘)
    field_avail_border_chars = {
        default = OBJECTS_DEFINITIONS_DEFAULTS.border_char.enum, -- Default border characters for each field type
        initial = nil,
        edited = nil
    },

    field_avail_border_style = { -- Available border styles for each field type: single, double, dashed, none
        enum = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum,
        default = nil, -- Combinaisons UX par type (1ere = valeur par defaut pour .initial)
        initial = nil,
        edited = nil
    },

    field_border_style = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Border Style",
        collapsed = false,
        collapsable = true,
        default = nil, -- Default border style for each field type
        initial = nil, -- Default border style for the initial field type
        edited = nil -- Border style after editing
    },
    ----- ===== BORDURE =====
    -- respresents the border style for each field type, which can be "single", "double", "dashed", or "none". The border style can be customized for each field type, allowing for a flexible and extensible way to create user interfaces for BMS applications. The border style can be used to indicate the state of the field, such as whether it is required or in an error state. The border style can also be used to enhance the visual appearance of the field, providing a more engaging user experience.
    -- this is used to render the border of the field, which can be customized for each field type. The border style can be used to indicate the state of the field, such as whether it is required or in an error state. The border style can also be used to enhance the visual appearance of the field, providing a more engaging user experience.
    field_border = nil, -- Will be set after table construction to reference field_avail_border_style and field_avail_border_chars
    field_border_chars = nil, -- Will be set after table construction to reference field_avail_border_chars

    -- Caractère de remplissage pour le titre dans la bordure supérieure
    field_title_fill_char = {
        enum = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum,
        default = nil,
        initial = nil,
        edited = nil
    },

    -- Caractère de remplissage pour les champs vides (ex: "_" pour Field)
    field_fill_char = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Fill Char",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum,
        default = nil,
        initial = nil,
        edited = nil
    },
    field_avail_vertical_align = {
        enum = OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum,
        default = nil,
        initial = nil,
        edited = nil
    },
    -- ===== ALIGNEMENT VERTICAL =====
    field_vertical_align = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Vertical Align",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum,
        default = nil,
        initial = nil,
        edited = nil
    },

    field_vertical_margin = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Vertical Margin",
        collapsed = false,
        collapsable = true,
        enum = OBJECTS_DEFINITIONS_DEFAULTS.vertical_margin.enum,
        default = nil,
        initial = nil,
        edited = nil
    },

    -- ===== AUTRES PROPRIETES =====
    -- Prefixe du titre (ex: "✱ " pour Fieldset requis)

    -- Marqueur pour les champs requis (ex: " *")
    field_avail_required_marker = {
        enum = OBJECTS_DEFINITIONS_DEFAULTS.required_marker.enum,
        default = nil,
        initial = nil,
        edited = nil
    },
    -- represents the required marker for each field type, which can be used to indicate that a field is required. The marker can be a string or a boolean value, and can be enabled or disabled for each field type.
    field_required_marker = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Required Marker",
        collapsed = false,
        collapsable = true,
        default = nil,
        initial = nil,
        edited = nil
    },
    field_footer_required_marker = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Footer Required Marker",
        collapsed = false,
        collapsable = true,
        default = nil,
        initial = nil,
        edited = nil
    },
    field_footer_error_marker = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Footer Error Marker",
        collapsed = false,
        collapsable = true,
        default = nil,
        initial = nil,
        edited = nil
    },
    -- Marqueur pour les champs en erreur (ex: " /!\")
    field_avail_error_marker = {
        enum = OBJECTS_DEFINITIONS_DEFAULTS.error_marker.enum,
        default = nil,
        initial = nil,
        edited = nil
    },
    -- represents the error marker for each field type, which can be used to indicate that a field is in an error state. The marker can be a string or a boolean value, and can be enabled or disabled for each field type.
    field_error_marker = {
        gui_field_type = OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string,
        gui_field_name = "Error Marker",
        collapsed = false,
        collapsable = true,
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
        enum = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum,
        default = nil,
        initial = nil,
        edited = nil
    },

    field_footer_fill_char = {
        enum = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum,
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
        enum = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum,
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
        edited = nil -- attribute after editing
    },
    ----- ===== VALEURS INITIALES =====
    ----- Represents the initial values for each field type, which can be used to set the default state of the field when it is created
    field_initial = { -- initial_value: for fieldset/group, represents the title of the fieldset/group;; for ImageAsciiArt, option_value: represents the ASCII code + file path; for other field types, represents the initial value
        default = nil,

        initial_value = nil, -- Default initial value for the initial field type
        edited_value = nil -- Initial value after editing
    },

    visual_representation = { -- Represents the visual representation of each field type
        -- line 0: reserved for border top + title (for fieldset/group)
        -- line 1 to N-1: reserved for border left/right + content/value
        -- line N: reserved for border bottom + footer
        default = {
            FieldTextORNumeric = function(obj)
                if obj.field_border_style.edited ~= nil and obj.field_border_style.edited.style ~=
                    OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
                    return render_bordered_field(obj)
                else
                    return render_field(obj)
                end
            end,
            Literal = function(obj)
                if obj.field_border_style.edited ~= nil and obj.field_border_style.edited.style ~=
                    OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
                    return render_bordered_field(obj)
                else
                    return render_field(obj)
                end
            end,
            ProtectedLiteral = function(obj)
                if obj.field_border_style.edited ~= nil and obj.field_border_style.edited.style ~=
                    OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
                    return render_bordered_field(obj)
                else
                    return render_field(obj)
                end
            end,
            BooleanField = function(obj)
                if obj.field_border_style.edited ~= nil and obj.field_border_style.edited.style ~=
                    OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none then
                    return render_bordered_field(obj)
                else
                    return render_field(obj)
                end
            end,
            ImageAsciiArt = function(obj)
                -- Display ASCII art from option_value.ascii_code
                local ascii = obj.field_initial.initial.option_value.ascii_code
                if ascii and type(ascii) == "table" then
                    return table.concat(ascii, "\n")
                end
                if ascii and type(ascii) == "string" then
                    return ascii
                else
                    return render_bordered_field(obj, "[ImageAsciiArt(nil): No ASCII art available]")
                end
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
-- Alias for height
OBJECTS_DEFINITIONS.field_min_height = OBJECTS_DEFINITIONS.field_min_height
OBJECTS_DEFINITIONS.field_max_height = OBJECTS_DEFINITIONS.field_max_height
-- Alias for height_min and height_max (reverse mapping for compatibility)
OBJECTS_DEFINITIONS.field_height_min = OBJECTS_DEFINITIONS.field_min_height
OBJECTS_DEFINITIONS.field_height_max = OBJECTS_DEFINITIONS.field_max_height

-- ===== POST-CONSTRUCTION: Dynamic references for field_border =====
-- ===== NIVEAU 1: Proprietes avec valeurs statiques simples (aucune dependance) =====
-- field_name: Noms des types de champs
OBJECTS_DEFINITIONS.field_name.default = {
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_type.enum.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_type.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_type.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_type.enum.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_type.enum.ImageAsciiArt,
    Line = OBJECTS_DEFINITIONS.field_type.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_type.enum.Fieldset
} -- Available field types 

-- field_type: Types de champs
OBJECTS_DEFINITIONS.field_type.default = {
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_type.enum.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_type.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_type.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_type.enum.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_type.enum.ImageAsciiArt,
    Line = OBJECTS_DEFINITIONS.field_type.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_type.enum.Fieldset
} --  Default height for each field type

-- field_min_height: Hauteurs minimales
OBJECTS_DEFINITIONS.field_min_height.default = {
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_min_height.enum.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_min_height.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_min_height.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_min_height.enum.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_min_height.enum.ImageAsciiArt,
    Line = OBJECTS_DEFINITIONS.field_min_height.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_min_height.enum.Fieldset
} --  Default max height for each field type

-- field_max_height: Hauteurs maximales
OBJECTS_DEFINITIONS.field_max_height.default = {
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_max_height.enum.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_max_height.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_max_height.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_max_height.enum.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_max_height.enum.ImageAsciiArt,
    Line = OBJECTS_DEFINITIONS.field_max_height.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_max_height.enum.Fieldset
} -- Default max length for each field type

-- field_max_width: Largeurs maximales
OBJECTS_DEFINITIONS.field_max_width.default = {
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_max_width.enum.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_max_width.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_max_width.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_max_width.enum.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_max_width.enum.ImageAsciiArt,
    Line = OBJECTS_DEFINITIONS.field_max_width.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_max_width.enum.Fieldset
} -- Default max length for each field type

-- field_min_width: Largeurs minimales
OBJECTS_DEFINITIONS.field_min_width.default = {
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_min_width.enum.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_min_width.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_min_width.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_min_width.enum.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_min_width.enum.ImageAsciiArt,
    Line = OBJECTS_DEFINITIONS.field_min_width.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_min_width.enum.Fieldset
} --  Default min width for each field type
-- field_height: Hauteurs par defaut (utilise min/max du Niveau 1)
OBJECTS_DEFINITIONS.field_height.default = {
    FieldTextORNumeric = {
        min = OBJECTS_DEFINITIONS.field_min_height.default.FieldTextORNumeric,
        max = OBJECTS_DEFINITIONS.field_max_height.default.FieldTextORNumeric,
        initial = OBJECTS_DEFINITIONS.field_height.enum.FieldTextORNumeric,
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
    ImageAsciiArt = {
        min = OBJECTS_DEFINITIONS.field_min_height.default.ImageAsciiArt,
        max = OBJECTS_DEFINITIONS.field_max_height.default.ImageAsciiArt,
        initial = OBJECTS_DEFINITIONS.field_height.enum.ImageAsciiArt,
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
    FieldTextORNumeric = {
        min = OBJECTS_DEFINITIONS.field_width_min.default.FieldTextORNumeric,
        max = OBJECTS_DEFINITIONS.field_width_max.default.FieldTextORNumeric,
        initial = OBJECTS_DEFINITIONS.field_width.enum.FieldTextORNumeric,
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
    ImageAsciiArt = {
        min = OBJECTS_DEFINITIONS.field_width_min.default.ImageAsciiArt,
        max = OBJECTS_DEFINITIONS.field_width_max.default.ImageAsciiArt,
        initial = OBJECTS_DEFINITIONS.field_width.enum.ImageAsciiArt,
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
    -- : Couleurs pour champs de saisie (default = neutre, white = visible sur fond sombre)
    FieldTextORNumeric = {
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

    -- ImageAsciiArt : Placeholder (default = transparent, white/blue = contour visible, cyan = water mark)
    ImageAsciiArt = {
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
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_avail_footer_color.default.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_avail_footer_color.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_footer_color.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_footer_color.default.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_avail_footer_color.default.ImageAsciiArt,
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
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = {
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

    -- ImageAsciiArt: Pas de italic/strikethrough/blink (distrayant pour placeholder)
    ImageAsciiArt = {
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
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_avail_style.default.FieldTextORNumeric,
    -- Literal: Static text - style par defaut
    Literal = OBJECTS_DEFINITIONS.field_avail_style.default.Literal,
    -- ProtectedLiteral: Protected static text - UX optimized
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_style.default.ProtectedLiteral,
    -- BooleanField: Checkbox - style par defaut
    BooleanField = OBJECTS_DEFINITIONS.field_avail_style.default.BooleanField,
    -- ImageAsciiArt: Placeholder - style par defaut
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_avail_style.default.ImageAsciiArt,
    -- Line: Horizontal rule - underline par defaut pour effet de ligne
    Line = OBJECTS_DEFINITIONS.field_avail_style.default.Line,
    -- Fieldset: Container - style par defaut
    Fieldset = OBJECTS_DEFINITIONS.field_avail_style.default.Fieldset
} -- Style par defaut pour chaque type

-- field_avail_text_align: Alignements de texte disponibles
OBJECTS_DEFINITIONS.field_avail_text_align.default = {
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = {
        align = OBJECTS_DEFINITIONS.field_avail_text_align.default.FieldTextORNumeric
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
    ImageAsciiArt = {
        align = OBJECTS_DEFINITIONS.field_avail_text_align.default.ImageAsciiArt
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
    FieldTextORNumeric = OBJECTS_DEFINITIONS_DEFAULTS.border_char.enum,
    Literal = OBJECTS_DEFINITIONS_DEFAULTS.border_char.enum,
    ProtectedLiteral = OBJECTS_DEFINITIONS_DEFAULTS.border_char.enum,
    BooleanField = OBJECTS_DEFINITIONS_DEFAULTS.border_char.enum,
    ImageAsciiArt = OBJECTS_DEFINITIONS_DEFAULTS.border_char.enum,
    Line = OBJECTS_DEFINITIONS_DEFAULTS.border_char.enum,
    Fieldset = OBJECTS_DEFINITIONS_DEFAULTS.border_char.enum
} -- Available border characters for each field type

OBJECTS_DEFINITIONS.field_border_chars = {
    default = {
        FieldTextORNumeric = OBJECTS_DEFINITIONS.field_avail_border_chars.default.FieldTextORNumeric,
        Literal = OBJECTS_DEFINITIONS.field_avail_border_chars.default.Literal,
        ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_border_chars.default.ProtectedLiteral,
        BooleanField = OBJECTS_DEFINITIONS.field_avail_border_chars.default.BooleanField,
        ImageAsciiArt = OBJECTS_DEFINITIONS.field_avail_border_chars.default.ImageAsciiArt,
        Line = OBJECTS_DEFINITIONS.field_avail_border_chars.default.Line,
        Fieldset = OBJECTS_DEFINITIONS.field_avail_border_chars.default.Fieldset
    }
} -- Default border characters for each field type

-- field_avail_border_style: Styles de bordure disponibles
OBJECTS_DEFINITIONS.field_avail_border_style.default = {
    -- : Bordure simple par defaut (standard pour champs de saisie)
    FieldTextORNumeric = {
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

    -- ImageAsciiArt : Bordure double pour encadrer les placeholders
    ImageAsciiArt = {
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
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_avail_border_style.default.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_avail_border_style.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_border_style.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_border_style.default.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_avail_border_style.default.ImageAsciiArt,
    Line = OBJECTS_DEFINITIONS.field_avail_border_style.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_border_style.default.Fieldset
} -- Default border style for each field type

-- field_title_fill_char: Caractere de remplissage pour le titre
OBJECTS_DEFINITIONS.field_title_fill_char.default = {
    FieldTextORNumeric = {
        space = OBJECTS_DEFINITIONS.field_title_fill_char.enum.space,
        underscore = OBJECTS_DEFINITIONS.field_title_fill_char.enum.underscore,
        dash = OBJECTS_DEFINITIONS.field_title_fill_char.enum.dash,
        dot = OBJECTS_DEFINITIONS.field_title_fill_char.enum.dot
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
    ImageAsciiArt = {
        dash = OBJECTS_DEFINITIONS.field_title_fill_char.enum.dash
    },
    Line = {
        dash = OBJECTS_DEFINITIONS.field_title_fill_char.enum.dash
    },
    Fieldset = {
        dash = OBJECTS_DEFINITIONS.field_title_fill_char.enum.dash,
        space = OBJECTS_DEFINITIONS.field_title_fill_char.enum.space,
        underscore = OBJECTS_DEFINITIONS.field_title_fill_char.enum.underscore,
        asterisk = OBJECTS_DEFINITIONS.field_title_fill_char.enum.asterisk,
        dot = OBJECTS_DEFINITIONS.field_title_fill_char.enum.dot,
        pipe = OBJECTS_DEFINITIONS.field_title_fill_char.enum.pipe
    }
}

-- field_fill_char: Caractere de remplissage pour les champs vides
OBJECTS_DEFINITIONS.field_fill_char.default = {
    FieldTextORNumeric = {
        underscore = OBJECTS_DEFINITIONS.field_fill_char.enum.underscore,
        space = OBJECTS_DEFINITIONS.field_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_fill_char.enum.dash,
        dot = OBJECTS_DEFINITIONS.field_fill_char.enum.dot,
        asterisk = OBJECTS_DEFINITIONS.field_fill_char.enum.asterisk
    },
    Literal = {
        space = OBJECTS_DEFINITIONS.field_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_fill_char.enum.dash
    },
    ProtectedLiteral = {
        space = OBJECTS_DEFINITIONS.field_fill_char.enum.space
    },
    BooleanField = {
        space = OBJECTS_DEFINITIONS.field_fill_char.enum.space
    },
    ImageAsciiArt = {
        space = OBJECTS_DEFINITIONS.field_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_fill_char.enum.dash,
        asterisk = OBJECTS_DEFINITIONS.field_fill_char.enum.asterisk,
        dot = OBJECTS_DEFINITIONS.field_fill_char.enum.dot
    },
    Line = {
        dash = OBJECTS_DEFINITIONS.field_fill_char.enum.dash
    },
    Fieldset = {
        space = OBJECTS_DEFINITIONS.field_fill_char.enum.space,
        asterisk = OBJECTS_DEFINITIONS.field_fill_char.enum.asterisk
    }
}

-- field_avail_vertical_align: Alignements verticaux disponibles
OBJECTS_DEFINITIONS.field_avail_vertical_align.default = {
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top,
        middle = OBJECTS_DEFINITIONS.field_vertical_align.enum.middle,
        bottom = OBJECTS_DEFINITIONS.field_vertical_align.enum.bottom
    },
    Literal = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top
    },
    ProtectedLiteral = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top,
        middle = OBJECTS_DEFINITIONS.field_vertical_align.enum.middle,
        bottom = OBJECTS_DEFINITIONS.field_vertical_align.enum.bottom
    },
    BooleanField = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top,
        middle = OBJECTS_DEFINITIONS.field_vertical_align.enum.middle,
        bottom = OBJECTS_DEFINITIONS.field_vertical_align.enum.bottom
    },
    ImageAsciiArt = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top
    },
    Line = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top
    },
    Fieldset = {
        top = OBJECTS_DEFINITIONS.field_vertical_align.enum.top,
        bottom = OBJECTS_DEFINITIONS.field_vertical_align.enum.bottom
    }
}

-- field_vertical_margin: Marge verticale
OBJECTS_DEFINITIONS.field_vertical_margin.default = {
    FieldTextORNumeric = {
        none = OBJECTS_DEFINITIONS.field_vertical_margin.enum.none
        -- small = OBJECTS_DEFINITIONS.field_vertical_margin.enum.small,
        -- medium = OBJECTS_DEFINITIONS.field_vertical_margin.enum.medium,
        -- large = OBJECTS_DEFINITIONS.field_vertical_margin.enum.large
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_avail_required_marker.default.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_required_marker.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_required_marker.default.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_avail_required_marker.default.ImageAsciiArt,
    Line = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Fieldset
}

-- field_avail_error_marker: Marqueur pour champs en erreur
OBJECTS_DEFINITIONS.field_avail_error_marker.default = {
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_avail_error_marker.default.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_error_marker.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_error_marker.default.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_avail_error_marker.default.ImageAsciiArt,
    Line = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Fieldset
}
OBJECTS_DEFINITIONS.field_footer_required_marker.default = {
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_avail_required_marker.default.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_required_marker.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_required_marker.default.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_avail_required_marker.default.ImageAsciiArt,
    Line = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_required_marker.default.Fieldset
}
OBJECTS_DEFINITIONS.field_footer_error_marker.default = {
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_avail_error_marker.default.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_error_marker.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_error_marker.default.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_avail_error_marker.default.ImageAsciiArt,
    Line = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_error_marker.default.Fieldset
}
-- ===== NIVEAU 3: Proprietes qui referencent le Niveau 2 =====

-- field_title_suffix: Suffixe du titre (reference required_marker et error_marker du Niveau 2)
OBJECTS_DEFINITIONS.field_title_suffix.default = {
    FieldTextORNumeric = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_title_color.default.FieldTextORNumeric,
        suffix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.FieldTextORNumeric,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.FieldTextORNumeric,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.FieldTextORNumeric,
        none = ""
    },
    Literal = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_title_color.default.Literal,
        suffix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.Literal,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Literal,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Literal,
        none = ""
    },
    ProtectedLiteral = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_title_color.default.ProtectedLiteral,
        suffix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.ProtectedLiteral,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.ProtectedLiteral,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.ProtectedLiteral,
        none = ""
    },
    BooleanField = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_title_color.default.BooleanField,
        suffix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.BooleanField,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.BooleanField,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.BooleanField,
        none = ""
    },
    ImageAsciiArt = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_title_color.default.ImageAsciiArt,
        suffix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.ImageAsciiArt,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.ImageAsciiArt,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.ImageAsciiArt,
        none = ""
    },
    Line = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_title_color.default.Line,
        suffix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.Line,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Line,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Line,
        none = ""
    },
    Fieldset = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_title_color.default.Fieldset,
        suffix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.Fieldset,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Fieldset,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Fieldset,
        none = ""
    }
}

-- field_title_prefix: Prefixe du titre (reference required_marker et error_marker du Niveau 2)
OBJECTS_DEFINITIONS.field_title_prefix.default = {
    FieldTextORNumeric = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_title_color.default.FieldTextORNumeric,
        prefix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.FieldTextORNumeric,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.FieldTextORNumeric,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.FieldTextORNumeric,
        none = ""
    },
    Literal = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_title_color.default.Literal,
        prefix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.Literal,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Literal,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Literal,
        none = ""
    },
    ProtectedLiteral = {
        enabled = false,
        prefix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.ProtectedLiteral,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.ProtectedLiteral,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.ProtectedLiteral,
        none = ""
    },
    BooleanField = {
        enabled = false,
        prefix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.BooleanField,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.BooleanField,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.BooleanField,
        none = ""
    },
    ImageAsciiArt = {
        enabled = false,
        prefix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.ImageAsciiArt,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.ImageAsciiArt,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.ImageAsciiArt,
        none = ""
    },
    Line = {
        enabled = false,
        prefix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.Line,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Line,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Line,
        none = ""
    },
    Fieldset = {
        enabled = false,
        prefix_char = OBJECTS_DEFINITIONS.field_title_fill_char.default.Fieldset,
        required = OBJECTS_DEFINITIONS.field_required_marker.default.Fieldset,
        errors = OBJECTS_DEFINITIONS.field_error_marker.default.Fieldset,
        none = ""
    }
}

-- field_footer_title: Titre du pied de page
OBJECTS_DEFINITIONS.field_footer_title.default = {
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash,
        underscore = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.underscore,
        dot = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dot
    },
    Literal = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash,
        underscore = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.underscore,
        dot = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dot
    },
    ProtectedLiteral = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash,
        underscore = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.underscore,
        dot = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dot
    },
    BooleanField = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash,
        underscore = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.underscore,
        dot = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dot
    },
    ImageAsciiArt = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash,
        underscore = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.underscore,
        dot = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dot
    },
    Line = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash,
        underscore = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.underscore,
        dot = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dot
    },
    Fieldset = {
        space = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.space,
        dash = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dash,
        underscore = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.underscore,
        dot = OBJECTS_DEFINITIONS.field_footer_fill_char.enum.dot
    }
}

-- field_footer_align: Alignement du pied de page
OBJECTS_DEFINITIONS.field_footer_align.default = {
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.FieldTextORNumeric,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.FieldTextORNumeric,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.FieldTextORNumeric,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.FieldTextORNumeric,
        required_marker = nil,
        error_marker = nil
    },
    Literal = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Literal,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Literal,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Literal,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Literal,
        required_marker = nil,
        error_marker = nil
    },
    ProtectedLiteral = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.ProtectedLiteral,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.ProtectedLiteral,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.ProtectedLiteral,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.ProtectedLiteral,
        required_marker = nil,
        error_marker = nil
    },
    BooleanField = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.BooleanField,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.BooleanField,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.BooleanField,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.BooleanField,
        required_marker = nil,
        error_marker = nil
    },
    ImageAsciiArt = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.ImageAsciiArt,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.ImageAsciiArt,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.ImageAsciiArt,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.ImageAsciiArt,
        required_marker = nil,
        error_marker = nil
    },
    Line = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Line,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Line,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Line,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Line,
        required_marker = nil,
        error_marker = nil
    },
    Fieldset = {
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Fieldset,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Fieldset,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Fieldset,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Fieldset,
        required_marker = nil,
        error_marker = nil
    }
}

-- ===== NIVEAU 3 (suite): Proprietes de position et alignement =====
OBJECTS_DEFINITIONS.field_avail_pos.default = {
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = OBJECTS_DEFINITIONS.field_avail_pos.default.FieldTextORNumeric,
    Literal = OBJECTS_DEFINITIONS.field_avail_pos.default.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_avail_pos.default.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_avail_pos.default.BooleanField,
    ImageAsciiArt = OBJECTS_DEFINITIONS.field_avail_pos.default.ImageAsciiArt,
    Line = OBJECTS_DEFINITIONS.field_avail_pos.default.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_avail_pos.default.Fieldset
} -- Default position for each field type

-- field_title_align: Alignement du titre
OBJECTS_DEFINITIONS.field_title_align.default = {
    -- Field/Literal/ProtectedLiteral/BooleanField/ImageAsciiArt: titre a gauche par defaut
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
        left = OBJECTS_DEFINITIONS.field_title_align.enum.left,
        center = OBJECTS_DEFINITIONS.field_title_align.enum.center
    }
}

-- field_children: Autorisation des enfants (pour Fieldset)
OBJECTS_DEFINITIONS.field_children.default = {
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = {
        min = OBJECTS_DEFINITIONS.field_min_height.enum.FieldTextORNumeric,
        max = OBJECTS_DEFINITIONS.field_max_height.enum.FieldTextORNumeric,
        initial = OBJECTS_DEFINITIONS.field_height.enum.FieldTextORNumeric,
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
    ImageAsciiArt = {
        min = OBJECTS_DEFINITIONS.field_min_height.enum.ImageAsciiArt,
        max = OBJECTS_DEFINITIONS.field_max_height.enum.ImageAsciiArt,
        initial = OBJECTS_DEFINITIONS.field_height.enum.ImageAsciiArt,
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
    FieldTextORNumeric = {
        min = OBJECTS_DEFINITIONS.field_min_width.default.FieldTextORNumeric,
        max = OBJECTS_DEFINITIONS.field_max_width.default.FieldTextORNumeric,
        initial = OBJECTS_DEFINITIONS.field_width.enum.FieldTextORNumeric,
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
    ImageAsciiArt = {
        min = OBJECTS_DEFINITIONS.field_min_width.default.ImageAsciiArt,
        max = OBJECTS_DEFINITIONS.field_max_width.default.ImageAsciiArt,
        initial = OBJECTS_DEFINITIONS.field_width.enum.ImageAsciiArt,
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
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
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
    FieldTextORNumeric = {
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
    ImageAsciiArt = {
        initial_value = nil,
        option_value = {
            ascii_code = nil,
            file_path = nil
        }
    }, -- Valeur initiale pour ImageAsciiArt (ASCII code + chemin du fichier)
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
        -- : Toutes les combinaisons style+chars disponibles pour Field
        FieldTextORNumeric = {
            style = OBJECTS_DEFINITIONS.field_avail_border_style.default.FieldTextORNumeric,
            chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default.FieldTextORNumeric
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
        -- ImageAsciiArt : Toutes les combinaisons style+chars disponibles pour ImageAsciiArt
        ImageAsciiArt = {
            style = OBJECTS_DEFINITIONS.field_avail_border_style.default.ImageAsciiArt,
            chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default.ImageAsciiArt
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
    local border_style = get_property(obj, "field_border_style") or
                             OBJECTS_DEFINITIONS_DEFAULTS.field_border_style.default
    local obj_type = get_property(obj, "field_type") or OBJECTS_DEFINITIONS.field_type.default
    local chars = OBJECTS_DEFINITIONS.field_avail_border_chars.default[obj_type]
    if chars and chars[border_style] then
        return chars[border_style]
    end
    -- Fallback to single style
    if chars and chars.single then
        return chars.single
    end
    -- Ultimate fallback
    return OBJECTS_DEFINITIONS.field_avail_border_chars.default.FieldTextORNumeric
end

-- Helper: Get the marker string for required fields
local function get_required_marker(obj)
    local attrb = get_property(obj, "field_attrb")
    if not attrb or not attrb.field_required then
        return ""
    end
    local marker_prop = get_property(obj, "field_required_marker")
    if marker_prop and type(marker_prop) == "table" then
        -- marker_prop might be:
        -- 1. A table with 'required' key (default structure)
        -- 2. A table with 'marker' key directly (user simplified structure)
        -- 3. A table with initial/edited keys
        local marker_details = marker_prop.required or marker_prop
        if marker_details and type(marker_details) == "table" then
            -- marker_details might have 'marker' key or initial/edited keys
            local marker_str = marker_details.marker
            if marker_str and type(marker_str) == "string" then
                return marker_str
            end
            -- Try initial/edited
            marker_details = marker_details.initial or marker_details.edited
            if marker_details and type(marker_details) == "table" and marker_details.marker then
                return marker_details.marker
            end
        end
        -- Try direct marker
        if marker_prop.marker and type(marker_prop.marker) == "string" then
            return marker_prop.marker
        end
    end
    -- Fallback to default marker
    return " *"
end

-- Helper: Get the marker string for error fields
local function get_error_marker(obj)
    local attrb = get_property(obj, "field_attrb")
    if not attrb or not attrb.field_has_error then
        return ""
    end
    local marker_prop = get_property(obj, "field_error_marker")
    if marker_prop and type(marker_prop) == "table" then
        -- marker_prop might be:
        -- 1. A table with 'error' key (default structure)
        -- 2. A table with 'marker' key directly (user simplified structure)
        -- 3. A table with initial/edited keys
        local marker_details = marker_prop.error or marker_prop
        if marker_details and type(marker_details) == "table" then
            -- marker_details might have 'marker' key or initial/edited keys
            local marker_str = marker_details.marker
            if marker_str and type(marker_str) == "string" then
                return marker_str
            end
            -- Try initial/edited
            marker_details = marker_details.initial or marker_details.edited
            if marker_details and type(marker_details) == "table" and marker_details.marker then
                return marker_details.marker
            end
        end
        -- Try direct marker
        if marker_prop.marker and type(marker_prop.marker) == "string" then
            return marker_prop.marker
        end
    end
    -- Fallback to default marker
    return " /!\\"
end

-- Helper: Build title with prefix, main title, and suffix
local function build_title(obj)
    local name = get_property(obj, "field_name")
    local initial = get_property(obj, "field_initial")

    -- Extract the actual string value
    local name_str = ""
    if name and type(name) == "table" then
        name_str = name.edited or name.initial or ""
    elseif type(name) == "string" then
        name_str = name
    end

    local initial_str = ""
    if initial and type(initial) == "table" then
        initial_str = initial.edited or initial.initial or ""
        if type(initial_str) == "table" then
            initial_str = initial_str.initial_value or ""
        end
    elseif type(initial) == "string" then
        initial_str = initial
    end

    local title = name_str or initial_str or ""

    if title == "" then
        return ""
    end

    -- Get prefix and suffix configurations
    local title_prefix_raw = get_property(obj, "field_title_prefix")
    local title_suffix_raw = get_property(obj, "field_title_suffix")
    -- These might be the initial/edited values directly, or tables with initial/edited keys
    local title_prefix = title_prefix_raw
    if title_prefix_raw and type(title_prefix_raw) == "table" and title_prefix_raw.initial then
        title_prefix = title_prefix_raw.edited or title_prefix_raw.initial
    end
    local title_suffix = title_suffix_raw
    if title_suffix_raw and type(title_suffix_raw) == "table" and title_suffix_raw.initial then
        title_suffix = title_suffix_raw.edited or title_suffix_raw.initial
    end
    local attrb = get_property(obj, "field_attrb")

    -- Build prefix
    local prefix = ""
    if title_prefix and type(title_prefix) == "table" and title_prefix.enabled then
        local marker = ""
        -- Check if we should show required or error marker
        if attrb and attrb.field_required and title_prefix.required then
            -- Use the helper function to get the marker
            marker = get_required_marker(obj)
        end
        if marker == "" and attrb and attrb.field_has_error and title_prefix.errors then
            -- Use the helper function to get the marker
            marker = get_error_marker(obj)
        end
        if marker ~= "" then
            prefix = marker
        elseif title_prefix.prefix_char then
            local char = title_prefix.prefix_char
            if type(char) == "table" then
                char = char.space or char.dash or char.underscore or char.dot or " "
            end
            prefix = char
        end
    end

    -- Build suffix
    local suffix = ""
    if title_suffix and type(title_suffix) == "table" and title_suffix.enabled then
        local marker = ""
        -- Check if we should show required or error marker
        if attrb and attrb.field_required and title_suffix.required then
            -- Use the helper function to get the marker
            marker = get_required_marker(obj)
        end
        if marker == "" and attrb and attrb.field_has_error and title_suffix.errors then
            -- Use the helper function to get the marker
            marker = get_error_marker(obj)
        end
        if marker ~= "" then
            suffix = marker
        elseif title_suffix.suffix_char then
            local char = title_suffix.suffix_char
            if type(char) == "table" then
                char = char.space or char.dash or char.underscore or char.dot or " "
            end
            suffix = char
        end
    end

    return prefix .. title .. suffix
end

-- Helper: Build footer line
local function build_footer(obj, width)
    local footer_config = get_property(obj, "field_footer")
    if not footer_config or type(footer_config) ~= "table" then
        return ""
    end

    -- Handle fill_char: can be a string or a table with fill char options
    local fill_char_raw = footer_config.fill_char or " "
    local fill_char = " "
    if type(fill_char_raw) == "table" then
        -- Extract first available fill char
        fill_char = fill_char_raw.space or fill_char_raw.dash or fill_char_raw.underscore or fill_char_raw.dot or " "
    elseif type(fill_char_raw) == "string" then
        fill_char = fill_char_raw
    end

    -- Handle title: can be a string or a table { title = "..." }
    local title = footer_config.title or ""
    if type(title) == "table" and title.title then
        title = title.title
    elseif type(title) ~= "string" then
        title = ""
    end
    -- Handle align: can be a string or a table with alignment options
    local align_raw = footer_config.align or "center"
    local align = "center"
    if type(align_raw) == "table" then
        -- Extract first available alignment
        align = align_raw.center or align_raw.left or align_raw.right or "center"
    elseif type(align_raw) == "string" then
        align = align_raw
    end

    -- Handle color: can be a string or a table with color options
    local color_raw = footer_config.color or "default"
    local color = "default"
    if type(color_raw) == "table" then
        -- Extract first available color
        color = color_raw.default or color_raw.white or color_raw.green or color_raw.yellow or "default"
    elseif type(color_raw) == "string" then
        color = color_raw
    end

    -- Check if we should show required/error markers
    local attrb = get_property(obj, "field_attrb")
    local show_required = false
    local show_error = false

    if attrb then
        show_required = attrb.field_required or false
        show_error = attrb.field_has_error or false
    end

    -- Get markers
    local required_marker = ""
    local error_marker = ""

    if footer_config.required_marker then
        local rm = footer_config.required_marker
        -- Handle different structures:
        -- 1. { initial = { marker = "..." } } or { edited = { marker = "..." } }
        -- 2. { required = { marker = "..." } } (from default)
        -- 3. { marker = "..." } (direct)
        -- 4. "..." (string)
        if type(rm) == "string" then
            required_marker = rm
        elseif type(rm) == "table" then
            local marker_table = rm.edited or rm.initial or rm.required or rm
            if type(marker_table) == "table" and marker_table.marker then
                required_marker = marker_table.marker
            elseif type(marker_table) == "string" then
                required_marker = marker_table
            elseif type(rm.marker) == "string" then
                required_marker = rm.marker
            end
        end
    end

    if footer_config.error_marker then
        local em = footer_config.error_marker
        -- Handle different structures:
        -- 1. { initial = { marker = "..." } } or { edited = { marker = "..." } }
        -- 2. { error = { marker = "..." } } (from default)
        -- 3. { marker = "..." } (direct)
        -- 4. "..." (string)
        if type(em) == "string" then
            error_marker = em
        elseif type(em) == "table" then
            local marker_table = em.edited or em.initial or em.error or em
            if type(marker_table) == "table" and marker_table.marker then
                error_marker = marker_table.marker
            elseif type(marker_table) == "string" then
                error_marker = marker_table
            elseif type(em.marker) == "string" then
                error_marker = em.marker
            end
        end
    end

    -- Only show footer if title is non-empty or there are markers to display
    if title == "" and not show_required and not show_error then
        return ""
    end

    -- Build footer content parts
    local marker_content = ""
    if show_required and required_marker ~= "" then
        marker_content = marker_content .. required_marker
    end
    if show_error and error_marker ~= "" then
        marker_content = marker_content .. error_marker
    end

    -- Check if title + markers fit
    local total_length = #title + #marker_content
    local display_title = title

    -- If total is too long, truncate title to make room for markers
    if total_length > width then
        local available_for_title = width - #marker_content
        if available_for_title > 0 then
            display_title = title:sub(1, available_for_title)
        else
            display_title = ""
        end
    end

    local content = display_title .. marker_content

    -- If no content after adding markers, return empty
    if content == "" then
        return ""
    end

    -- Apply alignment
    local padding = width - #content
    if padding > 0 then
        if align == "left" then
            content = content .. string.rep(fill_char, padding)
        elseif align == "right" then
            content = string.rep(fill_char, padding) .. content
        else -- center
            local left_pad = math.floor(padding / 2)
            local right_pad = padding - left_pad
            content = string.rep(fill_char, left_pad) .. content .. string.rep(fill_char, right_pad)
        end
    else
        -- If still too long after truncating title, do a final truncation
        content = content:sub(1, width)
    end

    return content
end

-- Helper: Render a simple bordered field (Field, Literal, ProtectedLiteral, BooleanField, ImageAsciiArt)
function render_bordered_field(obj, custom_content)
    local height_raw = get_property(obj, "field_height")
    -- field_height might be a table with {min, max, initial, edited} structure
    local height = 3
    if height_raw and type(height_raw) == "table" then
        height = height_raw.initial or height_raw.edited or 3
    elseif type(height_raw) == "number" then
        height = height_raw
    end

    local width_raw = get_property(obj, "field_width")
    local width = 10
    if width_raw and type(width_raw) == "table" then
        width = width_raw.initial or width_raw.edited or 10
    elseif type(width_raw) == "number" then
        width = width_raw
    end
    local border_style = get_property(obj, "field_border_style") or
                             OBJECTS_DEFINITIONS_DEFAULTS.field_border_style.default
    local border_chars = get_border_chars(obj)
    local fill_char_raw = get_property(obj, "field_fill_char")
    -- Extract the fill character from the table (default to space)
    local fill_char = " "
    if fill_char_raw and type(fill_char_raw) == "table" then
        local fc_config = fill_char_raw.edited or fill_char_raw.initial
        if fc_config and type(fc_config) == "table" then
            -- Use first available fill char
            fill_char = fc_config.space or fc_config.dash or fc_config.underscore or fc_config.dot or fc_config.equal or
                            " "
        elseif type(fc_config) == "string" then
            fill_char = fc_config
        end
    elseif type(fill_char_raw) == "string" then
        fill_char = fill_char_raw
    end
    local obj_type = get_property(obj, "field_type") or "Field"

    -- Determine content
    local content
    if custom_content then
        content = custom_content
    elseif obj_type == "BooleanField" then -- For BooleanField, show [X] or [ ] based on initial_value, shall use prefix notation + suffix notation if available
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

    -- Top border with title if height >= 1
    if height >= 1 then
        local title = build_title(obj)
        local title_fill = get_property(obj, "field_title_fill_char")
        -- Extract the fill character from the table (default to space)
        local fill_char_str = " "
        if title_fill and type(title_fill) == "table" then
            local tf_config = title_fill.edited or title_fill.initial
            if tf_config and type(tf_config) == "table" then
                -- Use first available fill char
                fill_char_str = tf_config.space or tf_config.dash or tf_config.underscore or tf_config.dot or " "
            end
        end

        if title ~= "" then
            -- Create title line with border
            local title_str = title
            local title_len = #title_str
            -- Title occupies full width between borders
            local content_width = width

            if title_len > content_width then
                title_str = title_str:sub(1, content_width)
                title_len = content_width
            end

            -- Get title alignment
            local title_align = get_property(obj, "field_title_align") or "center"
            -- Handle title_align: might be a table with { left, center, right } keys
            if type(title_align) == "table" then
                title_align = title_align.left or title_align.center or title_align.right or "center"
            end

            local padding = content_width - title_len
            local left_fill = 0
            local right_fill = 0
            if title_align == "left" then
                right_fill = padding
            elseif title_align == "right" then
                left_fill = padding
            else -- center
                left_fill = math.floor(padding / 2)
                right_fill = padding - left_fill
            end
            title_str = string.rep(fill_char_str, left_fill) .. title_str .. string.rep(fill_char_str, right_fill)

            table.insert(lines, border_chars.top_left .. title_str .. border_chars.top_right)
        else
            -- No title, just border
            table.insert(lines, border_chars.top_left .. string.rep(border_chars.top, width) .. border_chars.top_right)
        end
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

    -- Get text alignment
    local text_align = get_property(obj, "field_text_align") or "left"
    -- Handle text_align: might be a table with { align = { left, center, right } } structure
    if type(text_align) == "table" then
        if text_align.align and type(text_align.align) == "table" then
            text_align = text_align.align.left or text_align.align.center or text_align.align.right or "left"
        else
            text_align = text_align.left or text_align.center or text_align.right or "left"
        end
    end

    for i = 1, height - 2 do
        if i >= content_start and i < content_start + content_height then
            local content_line = content_lines[i - content_start + 1]
            local padding = width - #content_line
            if padding > 0 then
                local left_pad = 0
                local right_pad = 0
                if text_align == "left" then
                    right_pad = padding
                elseif text_align == "right" then
                    left_pad = padding
                else -- center
                    left_pad = math.floor(padding / 2)
                    right_pad = padding - left_pad
                end
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

    -- Bottom border or footer
    if height >= 2 then
        local footer = build_footer(obj, width)
        if footer ~= "" then
            -- Footer line
            table.insert(lines, border_chars.bottom_left .. footer .. border_chars.bottom_right)
        else
            -- Regular bottom border
            local bottom_line = border_chars.bottom_left .. string.rep(border_chars.bottom, width) ..
                                    border_chars.bottom_right
            table.insert(lines, bottom_line)
        end
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
    local height_raw = get_property(obj, "field_height")
    local height = 3
    if height_raw and type(height_raw) == "table" then
        height = height_raw.initial or height_raw.edited or 3
    elseif type(height_raw) == "number" then
        height = height_raw
    end

    local width_raw = get_property(obj, "field_width")
    local width = 40
    if width_raw and type(width_raw) == "table" then
        width = width_raw.initial or width_raw.edited or 40
    elseif type(width_raw) == "number" then
        width = width_raw
    end
    local border_style = get_property(obj, "field_border_style") or "double"
    local border_chars = get_border_chars(obj)
    local fill_char_raw = get_property(obj, "field_fill_char")
    -- Extract the fill character from the table (default to space)
    local fill_char = " "
    if fill_char_raw and type(fill_char_raw) == "table" then
        local fc_config = fill_char_raw.edited or fill_char_raw.initial
        if fc_config and type(fc_config) == "table" then
            -- Use first available fill char
            fill_char = fc_config.space or fc_config.dash or fc_config.underscore or fc_config.dot or fc_config.equal or
                            " "
        elseif type(fc_config) == "string" then
            fill_char = fc_config
        end
    elseif type(fill_char_raw) == "string" then
        fill_char = fill_char_raw
    end

    local lines = {}

    -- Top border with title
    local title_fill = get_property(obj, "field_title_fill_char")
    -- Extract the fill character from the table (default to space)
    local fill_char_str = " "
    if title_fill and type(title_fill) == "table" then
        local tf_config = title_fill.edited or title_fill.initial
        if tf_config and type(tf_config) == "table" then
            -- Use first available fill char
            fill_char_str = tf_config.space or tf_config.dash or tf_config.underscore or tf_config.dot or " "
        end
    end

    if height >= 1 then
        local title = build_title(obj)

        if title ~= "" then
            local title_str = title
            local title_len = #title_str
            -- Title occupies full width between borders
            local content_width = width

            if title_len > content_width then
                title_str = title_str:sub(1, content_width)
                title_len = content_width
            end

            -- Get title alignment
            local title_align = get_property(obj, "field_title_align") or "center"
            -- Handle title_align: might be a table with { left, center, right } keys
            if type(title_align) == "table" then
                title_align = title_align.left or title_align.center or title_align.right or "center"
            end

            local padding = content_width - title_len
            local left_fill = 0
            local right_fill = 0
            if title_align == "left" then
                right_fill = padding
            elseif title_align == "right" then
                left_fill = padding
            else -- center
                left_fill = math.floor(padding / 2)
                right_fill = padding - left_fill
            end
            title_str = string.rep(fill_char_str, left_fill) .. title_str .. string.rep(fill_char_str, right_fill)

            local top_line = border_chars.top_left .. title_str .. border_chars.top_right
            table.insert(lines, top_line)
        else
            -- No title, just border
            local top_line = border_chars.top_left .. string.rep(border_chars.top, width) .. border_chars.top_right
            table.insert(lines, top_line)
        end
    end

    -- Content area
    for i = 1, height - 2 do
        table.insert(lines, border_chars.left .. string.rep(fill_char, width) .. border_chars.right)
    end

    -- Bottom border or footer
    if height >= 2 then
        local footer = build_footer(obj, width)
        if footer ~= "" then
            -- Footer line
            table.insert(lines, border_chars.bottom_left .. footer .. border_chars.bottom_right)
        else
            -- Regular bottom border
            local bottom_line = border_chars.bottom_left .. string.rep(border_chars.bottom, width) ..
                                    border_chars.bottom_right
            table.insert(lines, bottom_line)
        end
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

-- =============================================================================
-- GUI PROPERTY EXTRACTION FUNCTIONS
-- =============================================================================

-- Categorize properties by their purpose
local property_categories = {
    dimensions = {"field_height", "field_width", "field_min_height", "field_max_height", "field_width_min",
                  "field_width_max"},
    colors = {"field_avail_color", "field_border_color", "field_title_color", "field_text_color",
              "field_avail_footer_color", "field_footer_color"},
    font = {"field_avail_font_family", "field_font_family"},
    style = {"field_avail_style", "field_style"},
    alignment = {"field_avail_text_align", "field_text_align", "field_title_align", "field_vertical_align",
                 "field_footer_align"},
    position = {"field_avail_pos", "field_pos"},
    borders = {"field_avail_border_chars", "field_avail_border_style", "field_border", "field_border_style"},
    fill = {"field_title_fill_char", "field_fill_char", "field_footer_fill_char"},
    markers = {"field_avail_required_marker", "field_required_marker", "field_avail_error_marker", "field_error_marker",
               "field_footer_required_marker", "field_footer_error_marker"},
    prefix_suffix = {"field_title_prefix", "field_title_suffix", "field_footer_title", "field_footer"},
    attributes = {"field_attrb"},
    values = {"field_initial", "field_name", "field_type"},
    children = {"field_children"},
    visual = {"visual_representation"}
}

-- Helper to find category for a property
local function get_property_category(prop_name)
    for category, props in pairs(property_categories) do
        for _, p in ipairs(props) do
            if p == prop_name then
                return category
            end
        end
    end
    return "other"
end

-- Helper to get GUI type from property definition
local function get_gui_type(prop_def)
    if prop_def.gui_field_type then
        return prop_def.gui_field_type
    end
    if prop_def.enum then
        return OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string
    end
    if prop_def.default and type(prop_def.default) == "table" then
        return OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_text_with_label_field
    end
    return OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_text_with_label_field
end

-- Helper to get control type from GUI type
local function get_control_type(gui_type)
    if gui_type == OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_string then
        return "select"
    elseif gui_type == OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_select_with_label_numeric then
        return "select"
    elseif gui_type == OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_text_with_label_field then
        return "text"
    elseif gui_type == OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_checkbox_with_label_field then
        return "checkbox"
    elseif gui_type == OBJECTS_DEFINITIONS_GUI_TYPE.gui_field_type.gui_list_textornum_with_label_field then
        return "list"
    else
        return "text"
    end
end

-- Helper to convert property name to GUI-friendly name
local function property_name_to_gui_name(prop_name)
    local gui_name = prop_name:gsub("[_%%]", " ")
    gui_name = gui_name:gsub("(%a)(%a*)", function(first, rest)
        return first:upper() .. rest:lower()
    end)
    return gui_name
end

-- Helper to check if a property is read-only
local function is_property_readonly(prop_name, prop_def)
    local readonly_properties = {
        field_type = true,
        field_name = true,
        visual_representation = true,
        field_avail_color = true,
        field_avail_font_family = true,
        field_avail_style = true,
        field_avail_text_align = true,
        field_avail_border_chars = true,
        field_avail_border_style = true,
        field_avail_pos = true,
        field_avail_vertical_align = true,
        field_avail_required_marker = true,
        field_avail_error_marker = true
    }
    return readonly_properties[prop_name] or false
end

-- Main function: Extract GUI properties for a specific type or all types
function OBJECTS_DEFINITIONS.get_gui_properties(obj_type)
    local properties = {}

    for prop_name, prop_def in pairs(OBJECTS_DEFINITIONS) do
        if type(prop_def) == "table" then
            local gui_type = get_gui_type(prop_def)
            local category = get_property_category(prop_name)
            local control_type = get_control_type(gui_type)
            local readonly = is_property_readonly(prop_name, prop_def)

            local available_values = {}
            if prop_def.enum then
                for k, v in pairs(prop_def.enum) do
                    if type(v) == "string" then
                        table.insert(available_values, v)
                    end
                end
            end

            local default_val
            if prop_def.default and obj_type then
                if prop_def.default[obj_type] then
                    if type(prop_def.default[obj_type]) == "table" and prop_def.default[obj_type].initial then
                        default_val = prop_def.default[obj_type].initial
                    else
                        default_val = prop_def.default[obj_type]
                    end
                end
            end

            local min_max = nil
            if prop_name == "field_height" or prop_name == "field_width" then
                local min_val = 1
                local max_val = 255
                if prop_def.default and obj_type and prop_def.default[obj_type] then
                    local defaults = prop_def.default[obj_type]
                    if type(defaults) == "table" then
                        min_val = defaults.min or 1
                        max_val = defaults.max or 255
                    end
                end
                min_max = {
                    min = min_val,
                    max = max_val
                }
            end

            local hint = ""
            if prop_def.avail_color_help then
                hint = "Color selection for field styling"
            elseif prop_def.avail_style_help then
                hint = "Text style options"
            elseif prop_def.help then
                hint = prop_def.help
            else
                hint = "Configure " .. prop_name:gsub("[_%%]", " ")
            end

            table.insert(properties, {
                name = prop_name,
                gui_name = property_name_to_gui_name(prop_name),
                category = category,
                gui_type = gui_type,
                control_type = control_type,
                default = default_val,
                min_max = min_max,
                available_values = #available_values > 0 and available_values or nil,
                read_only = readonly,
                hint = hint
            })
        end
    end

    return properties
end

-- Function: Get ncurses menu items for a specific type
function OBJECTS_DEFINITIONS.get_ncurses_menu_items(obj_type)
    local properties = OBJECTS_DEFINITIONS.get_gui_properties(obj_type)
    local menu_items = {}

    for _, prop in ipairs(properties) do
        local menu_item = {
            label = prop.gui_name,
            name = prop.name,
            type = prop.gui_type,
            control_type = prop.control_type,
            read_only = prop.read_only,
            default = prop.default,
            min = prop.min_max and prop.min_max.min,
            max = prop.min_max and prop.min_max.max,
            choices = prop.available_values,
            category = prop.category,
            hint = prop.hint
        }
        table.insert(menu_items, menu_item)
    end

    return menu_items
end

-- Function: Export to JSON
function OBJECTS_DEFINITIONS.export_to_json(obj_type)
    local function serialize_value(v)
        if v == nil then
            return "null"
        elseif type(v) == "string" then
            return '"' .. v:gsub("\\", "\\\\"):gsub('"', '\\"') .. '"'
        elseif type(v) == "number" then
            return tostring(v)
        elseif type(v) == "boolean" then
            return v and "true" or "false"
        elseif type(v) == "table" then
            local items = {}
            for k, val in pairs(v) do
                if type(k) == "string" and k:match("^[a-zA-Z_]+") then
                    table.insert(items, '"' .. k .. '": ' .. serialize_value(val))
                end
            end
            return "{" .. table.concat(items, ", ") .. "}"
        else
            return "null"
        end
    end

    if obj_type then
        local type_def = {}
        for prop_name, prop_def in pairs(OBJECTS_DEFINITIONS) do
            if type(prop_def) == "table" and prop_def.default and prop_def.default[obj_type] then
                type_def[prop_name] = prop_def.default[obj_type]
            end
        end
        return serialize_value(type_def)
    else
        local all_types = {}
        for prop_name, prop_def in pairs(OBJECTS_DEFINITIONS) do
            if type(prop_def) == "table" and prop_def.default then
                all_types[prop_name] = prop_def.default
            end
        end
        return serialize_value(all_types)
    end
end
