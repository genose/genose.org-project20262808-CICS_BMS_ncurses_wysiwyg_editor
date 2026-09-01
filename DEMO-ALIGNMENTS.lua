-- Test file for alignment examples
-- This file demonstrates all alignment properties and their combinations

dofile('OBJECT-GUI-RENDERING.lua')

print(string.rep("=", 60))
print("ALIGNMENT EXAMPLES FOR CICS BMS ncurses WYSIWYG EDITOR")
print(string.rep("=", 60))

-- ============================================
-- EXAMPLE 1: Basic Horizontal Text Alignment
-- ============================================
print("\n\n=== EXAMPLE 1: HORIZONTAL TEXT ALIGNMENT ===")

local left_field = create_gui_object('Field', {
    label = "Left Aligned",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Left aligned text ---")
print(left_field:render_gui())

local center_field = create_gui_object('Field', {
    label = "Center Aligned",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Center aligned text ---")
print(center_field:render_gui())

local right_field = create_gui_object('Field', {
    label = "Right Aligned",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.right,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Right aligned text ---")
print(right_field:render_gui())

-- ============================================
-- EXAMPLE 2: Vertical Alignment
-- ============================================
print("\n\n=== EXAMPLE 2: VERTICAL ALIGNMENT ===")

local top_field = create_gui_object('Field', {
    label = "Top Aligned",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    vertical_align = OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.top,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 5},
    field_width = {initial = 25}
})
print("\n--- Top aligned (content at top) ---")
print(top_field:render_gui())

local middle_field = create_gui_object('Field', {
    label = "Middle Aligned",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    vertical_align = OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.middle,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 5},
    field_width = {initial = 25}
})
print("\n--- Middle aligned (content centered vertically) ---")
print(middle_field:render_gui())

local bottom_field = create_gui_object('Field', {
    label = "Bottom Aligned",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    vertical_align = OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.bottom,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 5},
    field_width = {initial = 25}
})
print("\n--- Bottom aligned (content at bottom) ---")
print(bottom_field:render_gui())

-- ============================================
-- EXAMPLE 3: Title Alignment
-- ============================================
print("\n\n=== EXAMPLE 3: TITLE ALIGNMENT ===")

local title_left_field = create_gui_object('Field', {
    label = "Left Title",
    field_initial = "value",
    gui_field_type = "gui_text_field",
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Title left aligned ---")
print(title_left_field:render_gui())

local title_center_field = create_gui_object('Field', {
    label = "Center Title",
    field_initial = "value",
    gui_field_type = "gui_text_field",
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Title center aligned ---")
print(title_center_field:render_gui())

local title_right_field = create_gui_object('Field', {
    label = "Right Title",
    field_initial = "value",
    gui_field_type = "gui_text_field",
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.right,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Title right aligned ---")
print(title_right_field:render_gui())

-- ============================================
-- EXAMPLE 4: Footer Alignment
-- ============================================
print("\n\n=== EXAMPLE 4: FOOTER ALIGNMENT ===")

local footer_left_field = create_gui_object('Field', {
    label = "Footer Left",
    field_initial = "value",
    gui_field_type = "gui_text_field",
    footer_title = "-- left --",
    footer_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left,
    footer_fill_char = "-",
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
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
    footer_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    footer_fill_char = "-",
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
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
    footer_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.right,
    footer_fill_char = "-",
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 4},
    field_width = {initial = 25}
})
print("\n--- Footer with right alignment ---")
print(footer_right_field:render_gui())

-- ============================================
-- EXAMPLE 5: Combined Alignments
-- ============================================
print("\n\n=== EXAMPLE 5: COMBINED ALIGNMENTS ===")

-- Text: center, Vertical: middle, Title: right
local combined1 = create_gui_object('Field', {
    label = "Combined 1",
    field_initial = "data",
    gui_field_type = "gui_text_field",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    vertical_align = OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.middle,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.right,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
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
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.right,
    vertical_align = OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.bottom,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
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
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left,
    vertical_align = OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.top,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 5},
    field_width = {initial = 25}
})
print("\n--- Text:left + Vertical:top + Title:center ---")
print(combined3:render_gui())

-- ============================================
-- EXAMPLE 6: Fieldset Title Alignment
-- ============================================
print("\n\n=== EXAMPLE 6: FIELDSET TITLE ALIGNMENT ===")

local fieldset_left = create_gui_object('Fieldset', {
    label = "Left Title",
    gui_field_type = "gui_fieldset_field",
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double},
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
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double},
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
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.right,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double},
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
-- EXAMPLE 7: Select Field Alignment
-- ============================================
print("\n\n=== EXAMPLE 7: ALIGNMENT IN SELECT FIELDS ===")

local select_field = create_gui_object('BooleanField', {
    label = "Options",
    gui_field_type = "gui_select_field",
    options = {"Option 1", "Option 2", "Option 3"},
    selected_index = 2,
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 6},
    field_width = {initial = 25}
})
print("\n--- Select field with center-aligned options ---")
print(select_field:render_gui())

-- ============================================
-- EXAMPLE 8: Complete Form with All Alignments
-- ============================================
print("\n\n=== EXAMPLE 8: COMPLETE FORM WITH ALL ALIGNMENTS ===")

local complete_form = create_gui_form({
    create_gui_object('Field', {
        label = "Name",
        field_initial = "John Doe",
        gui_field_type = "gui_textornum_with_label_field",
        is_required = true,
        text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left,
        title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left,
        field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
        position = {row = 1}
    }),
    create_gui_object('Field', {
        label = "Email",
        field_initial = "john@example.com",
        gui_field_type = "gui_textornum_with_label_field",
        is_required = true,
        text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
        title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
        field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
        position = {row = 4}
    }),
    create_gui_object('BooleanField', {
        label = "Subscribe",
        gui_field_type = "gui_select_field",
        options = {"Yes", "No"},
        selected_index = 1,
        text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
        title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.right,
        field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
        position = {row = 7}
    }),
    create_gui_object('Field', {
        label = "Status",
        field_initial = "Active",
        gui_field_type = "gui_text_field",
        vertical_align = OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.middle,
        text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
        title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
        field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
        field_height = {initial = 5},
        position = {row = 10}
    }),
    create_gui_object('Fieldset', {
        label = "Address Information",
        gui_field_type = "gui_fieldset_field",
        title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
        field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double},
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
-- EXAMPLE 9: Default Alignments
-- ============================================
print("\n\n=== EXAMPLE 9: DEFAULT ALIGNMENTS ===")

local default_field = create_gui_object('Field', {
    label = "Default Alignment",
    field_initial = "text",
    gui_field_type = "gui_text_field",
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 4},
    field_width = {initial = 25}
})
print("\n--- Field with default alignments (text:left, vertical:top, title:center) ---")
print(default_field:render_gui())

-- ============================================
-- EXAMPLE 10: All Alignment Properties Combined
-- ============================================
print("\n\n=== EXAMPLE 10: ALL ALIGNMENT PROPERTIES COMBINED ===")

local all_align_field = create_gui_object('Field', {
    label = "All Alignments",
    field_initial = "data",
    gui_field_type = "gui_text_field",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    vertical_align = OBJECTS_DEFINITIONS_DEFAULTS.vertical_align.enum.middle,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.right,
    footer_title = "footer",
    footer_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left,
    footer_fill_char = "=",
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double},
    field_height = {initial = 6},
    field_width = {initial = 30}
})
print("\n--- Text:center + Vertical:middle + Title:right + Footer:left ---")
print(all_align_field:render_gui())

-- ============================================
-- EXAMPLE 11: PREFIX AND SUFFIX
-- ============================================
print("\n\n=== EXAMPLE 11: PREFIX AND SUFFIX ===")

local prefix_field = create_gui_object('Field', {
    label = "Name",
    field_initial = "John Doe",
    gui_field_type = "gui_text_field",
    title_prefix = "[ ",
    title_suffix = " ]",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Field with prefix '[ ' and suffix ' ]' ---")
print(prefix_field:render_gui())

local suffix_required_field = create_gui_object('Field', {
    label = "Email",
    field_initial = "user@example.com",
    gui_field_type = "gui_text_field",
    is_required = true,
    title_suffix = " *",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.left,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 30}
})
print("\n--- Field with required suffix ---")
print(suffix_required_field:render_gui())

-- ============================================
-- EXAMPLE 12: FILL CHARACTER
-- ============================================
print("\n\n=== EXAMPLE 12: FILL CHARACTER ===")

local fill_dash_field = create_gui_object('Field', {
    label = "Fill with dashes",
    field_initial = "value",
    gui_field_type = "gui_text_field",
    fill_char = "-",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 5},
    field_width = {initial = 25}
})
print("\n--- Field with dash fill character ---")
print(fill_dash_field:render_gui())

local fill_dot_field = create_gui_object('Field', {
    label = "Fill with dots",
    field_initial = "value",
    gui_field_type = "gui_text_field",
    fill_char = ".",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double},
    field_height = {initial = 4},
    field_width = {initial = 25}
})
print("\n--- Field with dot fill character ---")
print(fill_dot_field:render_gui())

-- ============================================
-- EXAMPLE 13: TEXT COLOR
-- ============================================
print("\n\n=== EXAMPLE 13: TEXT COLOR ===")

local red_text_field = create_gui_object('Field', {
    label = "Error Field",
    field_initial = "Invalid input",
    gui_field_type = "gui_text_field",
    text_color = "red",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Field with red text color ---")
print(red_text_field:render_gui())

local green_text_field = create_gui_object('Field', {
    label = "Success Field",
    field_initial = "Valid input",
    gui_field_type = "gui_text_field",
    text_color = "green",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Field with green text color ---")
print(green_text_field:render_gui())

local blue_text_field = create_gui_object('Field', {
    label = "Info Field",
    field_initial = "Information",
    gui_field_type = "gui_text_field",
    text_color = "blue",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Field with blue text color ---")
print(blue_text_field:render_gui())

-- ============================================
-- EXAMPLE 14: TITLE COLOR
-- ============================================
print("\n\n=== EXAMPLE 14: TITLE COLOR ===")

local yellow_title_field = create_gui_object('Field', {
    label = "Warning",
    field_initial = "Caution!",
    gui_field_type = "gui_text_field",
    title_color = "yellow",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Field with yellow title color ---")
print(yellow_title_field:render_gui())

local magenta_title_field = create_gui_object('Field', {
    label = "Important",
    field_initial = "Read this",
    gui_field_type = "gui_text_field",
    title_color = "magenta",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 3},
    field_width = {initial = 25}
})
print("\n--- Field with magenta title color ---")
print(magenta_title_field:render_gui())

-- ============================================
-- EXAMPLE 15: FOOTER COLOR
-- ============================================
print("\n\n=== EXAMPLE 15: FOOTER COLOR ===")

local red_footer_field = create_gui_object('Field', {
    label = "Alert",
    field_initial = "warning",
    gui_field_type = "gui_text_field",
    footer_title = "URGENT",
    footer_color = "red",
    footer_fill_char = "-",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 4},
    field_width = {initial = 25}
})
print("\n--- Field with red footer color ---")
print(red_footer_field:render_gui())

local cyan_footer_field = create_gui_object('Field', {
    label = "Info",
    field_initial = "details",
    gui_field_type = "gui_text_field",
    footer_title = "INFO",
    footer_color = "cyan",
    footer_fill_char = "=",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single},
    field_height = {initial = 4},
    field_width = {initial = 25}
})
print("\n--- Field with cyan footer color ---")
print(cyan_footer_field:render_gui())

-- ============================================
-- EXAMPLE 16: COMBINED PREFIX/SUFFIX/FILL/COLOR
-- ============================================
print("\n\n=== EXAMPLE 16: COMBINED PREFIX/SUFFIX/FILL/COLOR ===")

local combined_field = create_gui_object('Field', {
    label = "Status",
    field_initial = "ACTIVE",
    gui_field_type = "gui_text_field",
    title_prefix = "[ ",
    title_suffix = " ]",
    title_color = "green",
    text_color = "white",
    fill_char = ".",
    footer_title = "ready",
    footer_color = "cyan",
    footer_fill_char = "-",
    text_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    title_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    footer_align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double},
    field_height = {initial = 5},
    field_width = {initial = 30}
})
print("\n--- Field with prefix, suffix, fill_char, text_color, title_color, footer_color ---")
print(combined_field:render_gui())

print("\n\n" .. string.rep("=", 60))
print("ALL EXAMPLES COMPLETED")
print(string.rep("=", 60))
