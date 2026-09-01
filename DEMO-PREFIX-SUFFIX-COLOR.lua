-- Demo file for prefix, suffix, fill, and color properties
dofile('OBJECT-GUI-RENDERING.lua')

print(string.rep("=", 60))
print("DEMO: PREFIX, SUFFIX, FILL CHARACTER, AND COLOR PROPERTIES")
print(string.rep("=", 60))

-- ============================================
-- PREFIX AND SUFFIX EXAMPLES
-- ============================================
print("\n=== PREFIX AND SUFFIX ===")

-- Example 1: Simple prefix and suffix
local field1 = create_gui_object('Field', {
    label = "Username",
    field_initial = "john_doe",
    title_prefix = "[ ",
    title_suffix = " ]",
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    },
    field_height = {
        initial = 3
    },
    field_width = {
        initial = 25
    }
})
print("\n--- Prefix '[ ' and Suffix ' ]' ---")
print(field1:render_gui())

-- Example 2: Required marker as suffix
local field2 = create_gui_object('Field', {
    label = "Password",
    field_initial = "********",
    is_required = true,
    title_suffix = " *",
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    },
    field_height = {
        initial = 3
    },
    field_width = {
        initial = 25
    }
})
print("\n--- Required field with suffix ---")
print(field2:render_gui())

-- ============================================
-- FILL CHARACTER EXAMPLES
-- ============================================
print("\n\n=== FILL CHARACTER ===")

-- Example 3: Dash fill
local field3 = create_gui_object('Field', {
    label = "Status",
    field_initial = "ON",
    fill_char = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum.dash,
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    },
    field_height = {
        initial = 5
    },
    field_width = {
        initial = 20
    }
})
print("\n--- Fill with dashes ---")
print(field3:render_gui())

-- Example 4: Dot fill
local field4 = create_gui_object('Field', {
    label = "Progress",
    field_initial = "50%",
    fill_char = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum.dot,
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double
    },
    field_height = {
        initial = 4
    },
    field_width = {
        initial = 20
    }
})
print("\n--- Fill with dots ---")
print(field4:render_gui())

-- Example 5: Underscore fill
local field5 = create_gui_object('Field', {
    label = "Input",
    field_initial = "text",
    fill_char = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum.underscore,
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left,
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    },
    field_height = {
        initial = 4
    },
    field_width = {
        initial = 20
    }
})
print("\n--- Fill with underscores ---")
print(field5:render_gui())

-- ============================================
-- TEXT COLOR EXAMPLES
-- ============================================
print("\n\n=== TEXT COLOR ===")

-- Example 6: Red text (error)
local field6 = create_gui_object('Field', {
    label = "Error",
    field_initial = "Invalid value",
    text_color = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum.red,
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    },
    field_height = {
        initial = 3
    },
    field_width = {
        initial = 25
    }
})
print("\n--- Red text (error) ---")
print(field6:render_gui())

-- Example 7: Green text (success)
local field7 = create_gui_object('Field', {
    label = "Success",
    field_initial = "Operation completed",
    text_color = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum.green,
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    },
    field_height = {
        initial = 3
    },
    field_width = {
        initial = 28
    }
})
print("\n--- Green text (success) ---")
print(field7:render_gui())

-- Example 8: Blue text (info)
local field8 = create_gui_object('Field', {
    label = "Info",
    field_initial = "System message",
    text_color = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum.blue,
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    },
    field_height = {
        initial = 3
    },
    field_width = {
        initial = 25
    }
})
print("\n--- Blue text (info) ---")
print(field8:render_gui())

-- ============================================
-- TITLE COLOR EXAMPLES
-- ============================================
print("\n\n=== TITLE COLOR ===")

-- Example 9: Yellow title (warning)
local field9 = create_gui_object('Field', {
    label = "Warning",
    field_initial = "Caution: Low disk space",
    title_color = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum.yellow,
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left,
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    },
    field_height = {
        initial = 4
    },
    field_width = {
        initial = 35
    }
})
print("\n--- Yellow title (warning) ---")
print(field9:render_gui())

-- Example 10: Magenta title
local field10 = create_gui_object('Field', {
    label = "Important Notice",
    field_initial = "Please read carefully",
    title_color = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum.magenta,
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double
    },
    field_height = {
        initial = 3
    },
    field_width = {
        initial = 35
    }
})
print("\n--- Magenta title ---")
print(field10:render_gui())

-- ============================================
-- FOOTER COLOR EXAMPLES
-- ============================================
print("\n\n=== FOOTER COLOR ===")

-- Example 11: Red footer
local field11 = create_gui_object('Field', {
    label = "Alert",
    field_initial = "Warning message",
    footer_title = "URGENT",
    footer_color = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum.red,
    footer_fill_char = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum.exclamation,
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    },
    field_height = {
        initial = 4
    },
    field_width = {
        initial = 25
    }
})
print("\n--- Red footer ---")
print(field11:render_gui())

-- Example 12: Cyan footer
local field12 = create_gui_object('Field', {
    label = "Note",
    field_initial = "Additional info",
    footer_title = "INFO",
    footer_color = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum.cyan,
    footer_fill_char = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum.equal,
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    },
    field_height = {
        initial = 4
    },
    field_width = {
        initial = 25
    }
})
print("\n--- Cyan footer ---")
print(field12:render_gui())

-- ============================================
-- COMBINED EXAMPLES
-- ============================================
print("\n\n=== COMBINED PROPERTIES ===")

-- Example 13: All properties combined
local field13 = create_gui_object('Field', {
    label = "System Status",
    field_initial = "RUNNING",
    title_prefix = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum.less_than,
    title_suffix = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum.greater_than,
    title_color = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum.green,
    text_color = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum.white,
    fill_char = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum.dot,
    footer_title = "OK",
    footer_color = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum.green,
    footer_fill_char = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum.equal,
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    footer_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double
    },
    field_height = {
        initial = 5
    },
    field_width = {
        initial = 30
    }
})
print("\n--- All properties combined ---")
print(field13:render_gui())

-- Example 14: Form-like field with prefix, color, and fill
local field14 = create_gui_object('Field', {
    label = "Email Address",
    field_initial = "user@example.com",
    title_prefix = "[*]",
    title_color = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum.yellow,
    text_color = OBJECTS_DEFINITIONS_DEFAULTS.color_enum.enum.cyan,
    fill_char = OBJECTS_DEFINITIONS_DEFAULTS.fill_char.enum.space,
    is_required = true,
    field_border_style = {
        initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
    },
    field_height = {
        initial = 3
    },
    field_width = {
        initial = 35
    }
})
print("\n--- Form field with prefix and colors ---")
print(field14:render_gui())

print("\n" .. string.rep("=", 60))
print("ALL EXAMPLES COMPLETED")
print(string.rep("=", 60))
