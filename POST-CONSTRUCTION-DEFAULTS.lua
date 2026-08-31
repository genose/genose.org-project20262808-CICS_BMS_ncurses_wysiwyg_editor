-- ===========================================================
-- POST-CONSTRUCTION-DEFAULTS.lua
-- Extractions des definitions .default pour resoudre les dependances circulaires
-- A inclure APRES la definition de OBJECTS_DEFINITIONS
-- ===========================================================

-- Check if OBJECTS_DEFINITIONS is loaded
if not OBJECTS_DEFINITIONS then
    dofile("OBJECTS-DEFINITIONS.lua")
end

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
    Field = OBJECTS_DEFINITIONS.field_min_height.enum.Field,
    Literal = OBJECTS_DEFINITIONS.field_min_height.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_min_height.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_min_height.enum.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_min_height.enum.Image,
    Line = OBJECTS_DEFINITIONS.field_min_height.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_min_height.enum.Fieldset
} --  Default height for each field type

-- field_min_height: Hauteurs minimales
OBJECTS_DEFINITIONS.field_min_height.default = {
    Field = OBJECTS_DEFINITIONS.field_max_height.enum.Field,
    Literal = OBJECTS_DEFINITIONS.field_max_height.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_max_height.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_max_height.enum.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_max_height.enum.Image,
    Line = OBJECTS_DEFINITIONS.field_max_height.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_max_height.enum.Fieldset
} --  Default max height for each field type

-- field_max_height: Hauteurs maximales
OBJECTS_DEFINITIONS.field_max_height.default = {
    Field = OBJECTS_DEFINITIONS.field_width_max.enum.Field,
    Literal = OBJECTS_DEFINITIONS.field_width_max.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_width_max.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_width_max.enum.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_width_max.enum.Image,
    Line = OBJECTS_DEFINITIONS.field_width_max.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_width_max.enum.Fieldset
} -- Default max length for each field type

-- field_max_width: Largeurs maximales
OBJECTS_DEFINITIONS.field_max_width.default = {
    Field = OBJECTS_DEFINITIONS.field_width_max.enum.Field,
    Literal = OBJECTS_DEFINITIONS.field_width_max.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_width_max.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_width_max.enum.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_width_max.enum.Image,
    Line = OBJECTS_DEFINITIONS.field_width_max.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_width_max.enum.Fieldset
} -- Default max length for each field type

-- field_min_width: Largeurs minimales
OBJECTS_DEFINITIONS.field_min_width.default = {
    Field = OBJECTS_DEFINITIONS.field_width_min.enum.Field,
    Literal = OBJECTS_DEFINITIONS.field_width_min.enum.Literal,
    ProtectedLiteral = OBJECTS_DEFINITIONS.field_width_min.enum.ProtectedLiteral,
    BooleanField = OBJECTS_DEFINITIONS.field_width_min.enum.BooleanField,
    Image = OBJECTS_DEFINITIONS.field_width_min.enum.Image,
    Line = OBJECTS_DEFINITIONS.field_width_min.enum.Line,
    Fieldset = OBJECTS_DEFINITIONS.field_width_min.enum.Fieldset
}

-- field_height: Hauteurs par defaut (utilise min/max du Niveau 1)
OBJECTS_DEFINITIONS.field_height.default = {
    Field = {
        min = OBJECTS_DEFINITIONS.field_height_min.default.Field,
        max = OBJECTS_DEFINITIONS.field_height_max.default.Field,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Field,
        edited = nil
    },
    Literal = {
        min = OBJECTS_DEFINITIONS.field_height_min.default.Literal,
        max = OBJECTS_DEFINITIONS.field_height_max.default.Literal,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Literal,
        edited = nil
    },
    ProtectedLiteral = {
        min = OBJECTS_DEFINITIONS.field_height_min.default.ProtectedLiteral,
        max = OBJECTS_DEFINITIONS.field_height_max.default.ProtectedLiteral,
        initial = OBJECTS_DEFINITIONS.field_height.enum.ProtectedLiteral,
        edited = nil
    },
    BooleanField = {
        min = OBJECTS_DEFINITIONS.field_height_min.default.BooleanField,
        max = OBJECTS_DEFINITIONS.field_height_max.default.BooleanField,
        initial = OBJECTS_DEFINITIONS.field_height.enum.BooleanField,
        edited = nil
    },
    Image = {
        min = OBJECTS_DEFINITIONS.field_height_min.default.Image,
        max = OBJECTS_DEFINITIONS.field_height_max.default.Image,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Image,
        edited = nil
    },
    Line = {
        min = OBJECTS_DEFINITIONS.field_height_min.default.Line,
        max = OBJECTS_DEFINITIONS.field_height_max.default.Line,
        initial = OBJECTS_DEFINITIONS.field_height.enum.Line,
        edited = nil
    },
    Fieldset = {
        min = OBJECTS_DEFINITIONS.field_height_min.default.Fieldset,
        max = OBJECTS_DEFINITIONS.field_height_max.default.Fieldset,
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
OBJECTS_DEFINITIONS.field_footer_color.default = {
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

-- field_pos: Position (row, col, rowend, colend)
OBJECTS_DEFINITIONS.field_pos.default = {
    Field = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.default.Field
    },
    Literal = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.default.Literal
    },
    ProtectedLiteral = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.default.ProtectedLiteral
    },
    BooleanField = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.default.BooleanField
    },
    Image = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.default.Image
    },
    Line = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.default.Line
    },
    Fieldset = {
        position = OBJECTS_DEFINITIONS.field_avail_pos.default.Fieldset
    }
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

-- ===== NOUVELLE VERSION DE field_footer AVEC LES 5 PROPRIETES =====
-- Remplace la version existante dans la table principale

-- D'abord, supprimons les references circulaires dans field_footer en utilisant les nouvelles proprietes
OBJECTS_DEFINITIONS.field_footer.default = {
    Field = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Field,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Field,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Field,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Field,
        required_marker = OBJECTS_DEFINITIONS.field_required_marker.default.Field,
        error_marker = OBJECTS_DEFINITIONS.field_error_marker.default.Field
    },
    Literal = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Literal,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Literal,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Literal,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Literal,
        required_marker = OBJECTS_DEFINITIONS.field_required_marker.default.Literal,
        error_marker = OBJECTS_DEFINITIONS.field_error_marker.default.Literal
    },
    ProtectedLiteral = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_footer_color.default.ProtectedLiteral,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.ProtectedLiteral,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.ProtectedLiteral,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.ProtectedLiteral,
        required_marker = OBJECTS_DEFINITIONS.field_required_marker.default.ProtectedLiteral,
        error_marker = OBJECTS_DEFINITIONS.field_error_marker.default.ProtectedLiteral
    },
    BooleanField = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_footer_color.default.BooleanField,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.BooleanField,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.BooleanField,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.BooleanField,
        required_marker = OBJECTS_DEFINITIONS.field_required_marker.default.BooleanField,
        error_marker = OBJECTS_DEFINITIONS.field_error_marker.default.BooleanField
    },
    Image = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Image,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Image,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Image,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Image,
        required_marker = OBJECTS_DEFINITIONS.field_required_marker.default.Image,
        error_marker = OBJECTS_DEFINITIONS.field_error_marker.default.Image
    },
    Line = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Line,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Line,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Line,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Line,
        required_marker = OBJECTS_DEFINITIONS.field_required_marker.default.Line,
        error_marker = OBJECTS_DEFINITIONS.field_error_marker.default.Line
    },
    Fieldset = {
        enabled = false,
        color = OBJECTS_DEFINITIONS.field_footer_color.default.Fieldset,
        align = OBJECTS_DEFINITIONS.field_footer_align.default.Fieldset,
        fill_char = OBJECTS_DEFINITIONS.field_footer_fill_char.default.Fieldset,
        title = OBJECTS_DEFINITIONS.field_footer_title.default.Fieldset,
        required_marker = OBJECTS_DEFINITIONS.field_required_marker.default.Fieldset,
        error_marker = OBJECTS_DEFINITIONS.field_error_marker.default.Fieldset
    }
}
