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
OBJECTS_DEFINITIONS = {
    field_name = { -- Name of the object
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
        }, -- Available field types    
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
        default = {
            Field = 3,
            Literal = 3,
            ProtectedLiteral = 3,
            BooleanField = 3,
            Image = 5,
            Line = 1,
            Fieldset = 3
        }, --  Default height for each field type
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
        default = {
            Field = 80,
            Literal = 80,
            ProtectedLiteral = 80,
            BooleanField = 3,
            Image = 40,
            Line = 1,
            Fieldset = 80
        }, --  Default max height for each field type
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
        default = {
            Field = 255,
            Literal = 255,
            ProtectedLiteral = 255,
            BooleanField = 255,
            Image = 255,
            Line = 255,
            Fieldset = 255
        }, -- Default max length for each field type
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
        default = {
            Field = 3,
            Literal = 3,
            ProtectedLiteral = 3,
            BooleanField = 10,
            Image = 1,
            Line = 1,
            Fieldset = 3
        }, -- Default min length for each field type
        initial = nil, -- Default min length for the initial field type
        edited = nil -- Min length after editing
    },

    ----- ===== DIMENSIONS DU CHAMP =====
    field_height = { -- Height of the field, can be any positive integer
        enum = {
            Field = 3,
            Literal = 3,
            ProtectedLiteral = 3,
            BooleanField = 3,
            Image = 5,
            Line = 1,
            Fieldset = 3
        },
        default = {
            Field = 3,
            Literal = 3,
            ProtectedLiteral = 3,
            BooleanField = 3,
            Image = 5,
            Line = 1,
            Fieldset = 3
        }, --  Default height for each field type
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
        default = {
            Field = 10,
            Literal = 20,
            ProtectedLiteral = 20,
            BooleanField = 10,
            Image = 40,
            Line = 40,
            Fieldset = 40
        }, -- Default length for each field type
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
        default = {
            -- Field : Couleurs pour champs de saisie (default = neutre, white = visible sur fond sombre)
            Field = {"default", "white", "green", "yellow", "blue", "cyan"},

            -- Literal : Texte statique (default = neutre, white/yellow = lisible, green = accent)
            Literal = {"default", "white", "yellow", "green", "cyan", "blue"},

            -- ProtectedLiteral : Texte protege (white = par defaut pour read-only, green = protege, cyan = informatif)
            ProtectedLiteral = {"white", "green", "cyan", "yellow", "default"},

            -- BooleanField : Cases a cocher (default = neutre, green = coche/valide, white = non coche)
            BooleanField = {"default", "green", "white", "yellow", "blue"},

            -- Image : Placeholder (default = transparent, white/blue = contour visible, cyan = water mark)
            Image = {"default", "white", "blue", "cyan"},

            -- Line : Separateurs (default = invisible, white/blue/cyan = visibles mais discrets)
            Line = {"default", "white", "blue", "cyan"},

            -- Fieldset : Conteneurs (blue = standard pour bordures, default = neutre, white/cyan = alternatifs)
            Fieldset = {"blue", "default", "white", "cyan", "green"}
        }, -- Combinaisons UX par type (1ere = valeur par defaut pour .initial)
        initial = nil,
        edited = nil
    },

    field_border_color = {
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
        default = {
            Field = {"default", "white", "green", "yellow", "blue", "cyan"},
            Literal = {"default", "white", "yellow", "green", "cyan", "blue"},
            ProtectedLiteral = {"white", "green", "cyan", "yellow", "default"},
            BooleanField = {"default", "green", "white", "yellow", "blue"},
            Image = {"default", "white", "blue", "cyan"},
            Line = {"default", "white", "blue", "cyan"},
            Fieldset = {"blue", "default", "white", "cyan", "green"}
        }, -- Default border color for each field type
        initial = nil,
        edited = nil
    },

    field_title_color = {
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
        default = {
            Field = {"default", "white", "green", "yellow", "blue", "cyan"},
            Literal = {"default", "white", "yellow", "green", "cyan", "blue"},
            ProtectedLiteral = {"white", "green", "cyan", "yellow", "default"},
            BooleanField = {"default", "green", "white", "yellow", "blue"},
            Image = {"default", "white", "blue", "cyan"},
            Line = {"default", "white", "blue", "cyan"},
            Fieldset = {"blue", "default", "white", "cyan", "green"}
        },
        initial = nil,
        edited = nil
    },

    field_text_color = {
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
        default = {
            Field = {"default", "white", "green", "yellow", "blue", "cyan"},
            Literal = {"default", "white", "yellow", "green", "cyan", "blue"},
            ProtectedLiteral = {"white", "green", "cyan", "yellow", "default"},
            BooleanField = {"default", "green", "white", "yellow", "blue"},
            Image = {"default", "white", "blue", "cyan"},
            Line = {"default", "white", "blue", "cyan"},
            Fieldset = {"blue", "default", "white", "cyan", "green"}
        },
        initial = nil,
        edited = nil
    },

    field_footer_color = {
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
        default = {
            Field = {"default", "white", "green", "yellow", "blue", "cyan"},
            Literal = {"default", "white", "yellow", "green", "cyan", "blue"},
            ProtectedLiteral = {"white", "green", "cyan", "yellow", "default"},
            BooleanField = {"default", "green", "white", "yellow", "blue"},
            Image = {"default", "white", "blue", "cyan"},
            Line = {"default", "white", "blue", "cyan"},
            Fieldset = {"blue", "default", "white", "cyan", "green"}
        },
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
        default = {
            -- Field: Tous les styles disponibles
            Field = {"default", "bold", "italic", "underline", "strikethrough", "blink", "reverse"},

            -- Literal: Tous les styles (texte statique peut etre mis en valeur)
            Literal = {"default", "bold", "italic", "underline", "blink", "reverse"},

            -- ProtectedLiteral: Pas de italic/strikethrough/blink (distrayant pour read-only)
            ProtectedLiteral = {"default", "bold", "underline", "reverse"},

            -- BooleanField: Pas de italic/strikethrough; blink pour attention
            BooleanField = {"default", "bold", "underline", "blink", "reverse"},

            -- Image: Pas de italic/strikethrough/blink (distrayant pour placeholder)
            Image = {"default", "bold", "underline", "reverse"},

            -- Line: underline pour effet tirete, strikethrough pour ligne brisee
            Line = {"underline", "strikethrough", "default", "bold", "reverse"},

            -- Fieldset: Tous les styles sauf strikethrough (peu pertinent pour bordures)
            Fieldset = {"default", "bold", "underline", "blink", "reverse"}
        } -- Styles disponibles par type (1er = valeur par defaut pour .initial)
    },
    -- Represents the style for each field type, referencing field_avail_style enum for consistency
    -- Adapted per field type considering user visual experience (UX)
    field_style = {
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
        default = {
            Field = {"left", "center", "right"},
            Literal = {"left", "center", "right"},
            ProtectedLiteral = {"left", "center", "right"},
            BooleanField = {"left", "center", "right"},
            Image = {"left", "center", "right"},
            Line = {"left", "center", "right"},
            Fieldset = {"left", "center", "right"}
        }, -- Available text alignment for each field type
        initial = nil, -- Default text alignment for the initial field type
        edited = nil -- Text alignment after editing
    },
    -- field_text_align represents the text alignment for each field type, which can be left, center, or right
    field_text_align = {
        enum = {
            left = "left",
            center = "center",
            right = "right"
        },
        default = {
            Field = {"left", "center", "right"},
            Literal = {"left", "center", "right"},
            ProtectedLiteral = {"left", "center", "right"},
            BooleanField = {"left", "center", "right"},
            Image = {"left", "center", "right"},
            Line = {"left", "center", "right"},
            Fieldset = {"left", "center", "right"}
        }, -- Default text alignment for each field type
        initial = nil, -- Default text alignment for the initial field type
        edited = nil -- Text alignment after editing
    },

    field_pos = { -- Represents the position of the field in the BMS screen (row, col)
        enum = {
            zero = 0
        },
        default = {
            Field = {
                col = "zero",
                row = "zero",
                rowend = "zero",
                colend = "zero"
            },
            Literal = {
                col = "zero",
                row = "zero",
                rowend = "zero",
                colend = "zero"
            },
            ProtectedLiteral = {
                col = "zero",
                row = "zero",
                rowend = "zero",
                colend = "zero"
            },
            BooleanField = {
                col = "zero",
                row = "zero",
                rowend = "zero",
                colend = "zero"
            },
            Image = {
                col = "zero",
                row = "zero",
                rowend = "zero",
                colend = "zero"
            },
            Line = {
                col = "zero",
                row = "zero",
                rowend = "zero",
                colend = "zero"
            },
            Fieldset = {
                col = "zero",
                row = "zero",
                rowend = "zero",
                colend = "zero"
            }
        }, -- Default position for each field type
        initial = nil, -- Default position for the initial field type
        edited = nil -- Position after editing
    },

    -- ===== PERSONNALISATION DES CARACTERES =====
    -- Caractères de bordure personnalisables (pour remplacer ┌─┐│├└┘)
    field_avail_border_chars = {
        default = {
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
        }, -- Default border characters for each field type
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
        default = {
            -- Field : Bordure simple par defaut (standard pour champs de saisie)
            Field = {"single", "double", "dashed", "none"},

            -- Literal : Pas de bordure par defaut (texte statique n'en a pas besoin)
            Literal = {"none", "single", "dashed"},

            -- ProtectedLiteral : Bordure pointillee pour indiquer protection
            ProtectedLiteral = {"dashed", "single", "double", "none"},

            -- BooleanField : Bordure simple pour cases a cocher
            BooleanField = {"single", "double", "dashed"},

            -- Image : Bordure double pour encadrer les placeholders
            Image = {"double", "single", "dashed", "none"},

            -- Line : Pas de bordure (c'est deja une ligne)
            Line = {"none"},

            -- Fieldset : Bordure double pour conteneurs (standard UI)
            Fieldset = {"double", "single", "dashed"}
        } -- Combinaisons UX par type (1ere = valeur par defaut pour .initial)
    },

    field_border_style = {
        enum = {
            single = "single",
            double = "double",
            dashed = "dashed",
            none = "none"
        },
        default = {
            Field = {"single", "double", "dashed", "none"},
            Literal = {"none", "single", "dashed"},
            ProtectedLiteral = {"dashed", "single", "double", "none"},
            BooleanField = {"single", "double", "dashed"},
            Image = {"double", "single", "dashed", "none"},
            Line = {"none"},
            Fieldset = {"double", "single", "dashed"}
        }, -- Default border style for each field type
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
        default = {
            Field = "space",
            Literal = "space",
            ProtectedLiteral = "space",
            BooleanField = "space",
            Image = "space",
            Line = "dash",
            Fieldset = "dash"
        },
        initial = "dash",
        edited = "dash"
    },

    -- Caractère de remplissage pour les champs vides (ex: "_" pour Field)
    field_fill_char = {
        enum = {
            underscore = "_",
            space = " ",
            dash = "─"
        },
        default = {
            Field = "underscore",
            Literal = "space",
            ProtectedLiteral = "space",
            BooleanField = "space",
            Image = "space",
            Line = "dash",
            Fieldset = "space"
        },
        initial = nil,
        edited = nil
    },
    field_avail_vertical_align = {
        enum = {
            top = "top",
            middle = "middle",
            bottom = "bottom"
        },
        default = {
            Field = "top",
            Literal = "top",
            ProtectedLiteral = "top",
            BooleanField = "top",
            Image = "top",
            Line = "top",
            Fieldset = "top"
        },
        initial = nil,
        edited = nil
    },
    -- ===== ALIGNEMENT VERTICAL =====
    field_vertical_align = {
        enum = {
            top = "top",
            middle = "middle",
            bottom = "bottom"
        },
        default = {
            Field = "top",
            Literal = "top",
            ProtectedLiteral = "top",
            BooleanField = "top",
            Image = "top",
            Line = "top",
            Fieldset = "top"
        },
        initial = nil,
        edited = nil
    },

    field_vertical_margin = {
        enum = {
            none = 0
        },
        default = {
            Field = "none",
            Literal = "none",
            ProtectedLiteral = "none",
            BooleanField = "none",
            Image = "none",
            Line = "none",
            Fieldset = "none"
        },
        initial = "none",
        edited = "none"
    },

    -- ===== AUTRES PROPRIETES =====
    -- Prefixe du titre (ex: "✱ " pour Fieldset requis)

    -- Marqueur pour les champs requis (ex: " *")
    field_avail_required_marker = {
        default = {
            Field = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            },
            Literal = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            },
            ProtectedLiteral = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            },
            BooleanField = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            },
            Image = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            },
            Line = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            },
            Fieldset = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            }
        },
        initial = nil,
        edited = nil
    },
    -- represents the required marker for each field type, which can be used to indicate that a field is required. The marker can be a string or a boolean value, and can be enabled or disabled for each field type.
    field_required_marker = {
        default = {
            Field = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            },
            Literal = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            },
            ProtectedLiteral = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            },
            BooleanField = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            },
            Image = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            },
            Line = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            },
            Fieldset = {
                enabled = false,
                enabled_marker = false,
                marker = " *",
                enabled_sentence = false,
                sentence = "= required fields",
                none = ""
            }
        }
    },
    -- Marqueur pour les champs en erreur (ex: " /!\")
    field_avail_error_marker = {
        default = {
            Field = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            Literal = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            ProtectedLiteral = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            BooleanField = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            Image = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            Line = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            Fieldset = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            }
        },
        initial = nil,
        edited = nil
    },
    -- represents the error marker for each field type, which can be used to indicate that a field is in an error state. The marker can be a string or a boolean value, and can be enabled or disabled for each field type.
    field_error_marker = {
        default = {
            Field = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            Literal = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            ProtectedLiteral = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            BooleanField = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            Image = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            Line = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            },
            Fieldset = {
                enabled = false,
                enabled_marker = false,
                marker = " /!\\",
                enabled_sentence = false,
                sentence = "= error fields",
                none = ""
            }
        },
        initial = nil,
        edited = nil
    },
    -- title of a field is composed of a prefix, a main title, and a suffix, by applying alignment and color properties. The prefix and suffix can be used to indicate required fields or other information, while the main title represents the name of the field. The title can be customized for each field type, allowing for a flexible and extensible way to create user interfaces for BMS applications.
    -- field_title_suffix represents the suffix for the title of each field type, which can be used to indicate required fields or other information
    field_title_suffix = {
        default = {
            Field = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            Literal = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            ProtectedLiteral = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            BooleanField = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            Image = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            Line = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            Fieldset = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            }
        },
        initial = nil,
        edited = nil
    },
    -- field_title_prefix represents the prefix for the title of each field type, which can be used to indicate required fields or other information
    field_title_prefix = {
        default = {
            Field = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            Literal = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            ProtectedLiteral = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            BooleanField = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            Image = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            Line = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            Fieldset = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            }
        },
        initial = nil,
        edited = nil
    },

    -- field_footer represents the footer for the title of each field type, which can be used to indicate required fields or other information
    field_footer = {
        default = {
            Field = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            Literal = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            ProtectedLiteral = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            BooleanField = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            Image = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            Line = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            },
            Fieldset = {
                enabled = false,
                required = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " *",
                    enabled_sentence = false,
                    sentence = "= required fields",
                    none = ""
                },
                errors = {
                    enabled = false,
                    enabled_marker = false,
                    marker = " /!\\",
                    enabled_sentence = false,
                    sentence = "= error fields",
                    none = ""
                },
                none = ""
            }
        },
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
        default = {
            -- Field/Literal/ProtectedLiteral/BooleanField/Image: titre a gauche par defaut
            Field = "left",
            Literal = "left",
            ProtectedLiteral = "left",
            BooleanField = "left",
            Image = "left",
            -- Line: titre centre par defaut (ligne horizontale)
            Line = "center",
            -- Fieldset: titre centre par defaut (conteneur)
            Fieldset = "center"
        },
        initial = nil,
        edited = nil
    },

    -- Champs enfants (pour Fieldset)
    field_children = {
        default = {
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
        },
        initial = nil,
        edited = nil
    },
    ----- ===== ATTRIBUTS DU CHAMP =====
    -- represents the attributes of the field, such as whether it is editable, visible, required, readonly, enabled, focused, selected, highlighted, hidden, protected, or numeric
    field_attrb = {
        default = {
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

        }, -- BMS available field attributes
        initial = nil, -- Default field attribute
        edited = nil -- Field attribute after editing
    },
    ----- ===== VALEURS INITIALES =====
    ----- Represents the initial values for each field type, which can be used to set the default state of the field when it is created
    field_initial = { -- initial_value: for fieldset/group, represents the title of the fieldset/group;; for image, option_value: represents the ASCII code + file path; for other field types, represents the initial value
        default = {
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
        },

        initial_value = nil, -- Default initial value for the initial field type
        edited_value = nil -- Initial value after editing
    },

    visual_representation = { -- Represents the visual representation of each field type
        -- line 0: reserved for border top + title (for fieldset/group)
        -- line 1 to N-1: reserved for border left/right + content/value
        -- line N: reserved for border bottom + footer
        default = {
            Image = function(obj)
                -- Display ASCII art from option_value.ascii_code
                local ascii = obj.field_initial.initial.option_value.ascii_code
                if ascii and type(ascii) == "table" then
                    return table.concat(ascii, "\n")
                end
                return "[Image]"
            end
        }, -- Default visual representation for each field type
        initial = nil, -- Default visual representation for the initial field type
        edited = nil -- Visual representation after editing
    }
}

-- ===== POST-CONSTRUCTION: Dynamic references for field_border =====
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

-- ===== CONSTRUCTEUR D'OBJETS =====
function OBJECTS_DEFINITIONS.new(obj_type, overrides)
    local self = {}

    -- Copie de toutes les propriétés depuis la définition globale
    for prop_name, prop_def in pairs(OBJECTS_DEFINITIONS) do
        if type(prop_def) == "table" and prop_def.default and prop_def.default[obj_type] and type(prop_def.default[obj_type]) ~= "function" and prop_name ~= "visual_representation" then
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
