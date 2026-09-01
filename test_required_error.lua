-- Test script for required/error field markers
-- Load the definitions file
dofile("OBJECTS-DEFINITIONS.lua")

print("Testing OBJECTS-DEFINITIONS required/error markers...")
print("==================================================\n")

-- Test 1: Field with required marker in suffix
print("1. Field (required, marker in suffix):")
local field_required = OBJECTS_DEFINITIONS.new("Field", {
    field_width = { initial = 20 },
    field_height = { initial = 3 },
    field_border_style = { initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single },
    field_name = { initial = "Username" },
    field_initial = { initial = { initial_value = "" } },
    field_attrb = { 
        initial = {
            field_required = true,
            field_has_error = false
        }
    },
    field_title_suffix = {
        initial = {
            enabled = true,
            required = true,
            suffix_char = " "
        }
    }
})
print(field_required:render())
print()

-- Test 2: Field with error marker in suffix
print("2. Field (error, marker in suffix):")
local field_error = OBJECTS_DEFINITIONS.new("Field", {
    field_width = { initial = 20 },
    field_height = { initial = 3 },
    field_border_style = { initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single },
    field_name = { initial = "Email" },
    field_initial = { initial = { initial_value = "invalid@email" } },
    field_attrb = { 
        initial = {
            field_required = false,
            field_has_error = true
        }
    },
    field_title_suffix = {
        initial = {
            enabled = true,
            errors = true,
            suffix_char = " "
        }
    }
})
print(field_error:render())
print()

-- Test 3: Field with both required and error
print("3. Field (both required and error):")
local field_both = OBJECTS_DEFINITIONS.new("Field", {
    field_width = { initial = 25 },
    field_height = { initial = 3 },
    field_border_style = { initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double },
    field_name = { initial = "Password" },
    field_initial = { initial = { initial_value = "" } },
    field_attrb = { 
        initial = {
            field_required = true,
            field_has_error = true
        }
    },
    field_title_suffix = {
        initial = {
            enabled = true,
            required = true,
            errors = true,
            suffix_char = " "
        }
    }
})
print(field_both:render())
print()

-- Test 4: Field with required marker in prefix
print("4. Field (required, marker in prefix):")
local field_prefix = OBJECTS_DEFINITIONS.new("Field", {
    field_width = { initial = 20 },
    field_height = { initial = 3 },
    field_border_style = { initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single },
    field_name = { initial = "Age" },
    field_initial = { initial = { initial_value = "" } },
    field_attrb = { 
        initial = {
            field_required = true,
            field_has_error = false
        }
    },
    field_title_prefix = {
        initial = {
            enabled = true,
            required = true,
            prefix_char = " "
        }
    }
})
print(field_prefix:render())
print()

-- Test 5: Fieldset with required marker
print("5. Fieldset (required):")
local fieldset_required = OBJECTS_DEFINITIONS.new("Fieldset", {
    field_width = { initial = 30 },
    field_height = { initial = 5 },
    field_name = { initial = "Personal Info" },
    field_border_style = { initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double },
    field_attrb = { 
        initial = {
            field_required = true,
            field_has_error = false
        }
    },
    field_title_suffix = {
        initial = {
            enabled = true,
            required = true,
            suffix_char = " "
        }
    }
})
print(fieldset_required:render())
print()

-- Test 6: Field with footer showing required marker
print("6. Field (with footer showing required marker):")
local field_footer = OBJECTS_DEFINITIONS.new("Field", {
    field_width = { initial = 25 },
    field_height = { initial = 4 },
    field_border_style = { initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single },
    field_name = { initial = "Confirm Password" },
    field_initial = { initial = { initial_value = "" } },
    field_attrb = { 
        initial = {
            field_required = true,
            field_has_error = false
        }
    },
    field_footer = {
        initial = {
            title = "Required field",
            align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
            fill_char = "-",
            required_marker = {
                initial = {
                    marker = " [*] "
                }
            }
        }
    }
})
print(field_footer:render())
print()

-- Test 7: Field with footer showing error marker
print("7. Field (with footer showing error marker):")
local field_footer_error = OBJECTS_DEFINITIONS.new("Field", {
    field_width = { initial = 25 },
    field_height = { initial = 4 },
    field_border_style = { initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single },
    field_name = { initial = "Confirm Password" },
    field_initial = { initial = { initial_value = "mismatch" } },
    field_attrb = { 
        initial = {
            field_required = false,
            field_has_error = true
        }
    },
    field_footer = {
        initial = {
            title = "Error: Passwords do not match",
            align = OBJECTS_DEFINITIONS_DEFAULTS.text_align.enum.center,
            fill_char = "-",
            error_marker = {
                initial = {
                    marker = " [X] "
                }
            }
        }
    }
})
print(field_footer_error:render())
print()

-- Test 8: Literal with custom required marker
print("8. Literal (required with custom marker '***')")
local literal_required = OBJECTS_DEFINITIONS.new("Literal", {
    field_width = { initial = 30 },
    field_height = { initial = 3 },
    field_border_style = { initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single },
    field_name = { initial = "Important Note" },
    field_initial = { initial = { initial_value = "Please read this" } },
    field_attrb = { 
        initial = {
            field_required = true,
            field_has_error = false
        }
    },
    field_required_marker = {
        initial = {
            marker = "***"
        }
    },
    field_title_suffix = {
        initial = {
            enabled = true,
            required = true,
            suffix_char = " "
        }
    }
})
print(literal_required:render())
print()

-- Test 9: Field with no markers (normal state)
print("9. Field (normal, no markers):")
local field_normal = OBJECTS_DEFINITIONS.new("Field", {
    field_width = { initial = 20 },
    field_height = { initial = 3 },
    field_border_style = { initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single },
    field_name = { initial = "Optional Field" },
    field_initial = { initial = { initial_value = "Some text" } },
    field_attrb = { 
        initial = {
            field_required = false,
            field_has_error = false
        }
    }
})
print(field_normal:render())
print()

print("All required/error marker tests completed!")
