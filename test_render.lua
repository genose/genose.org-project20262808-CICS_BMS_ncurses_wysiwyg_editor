-- Test script for OBJECTS-DEFINITIONS rendering
-- Load the definitions file
dofile("OBJECTS-DEFINITIONS.lua")

print("Testing OBJECTS-DEFINITIONS rendering...")
print("=====================================\n")

-- Test 1: Field with default properties
print("1. Field (default, with border):")
local field1 = OBJECTS_DEFINITIONS.new("Field")
print(field1:render())
print()

-- Test 2: Field with custom properties
print("2. Field (width=20, height=5, single border):")
local field2 = OBJECTS_DEFINITIONS.new("Field", {
    field_width = { initial = 20 },
    field_height = { initial = 5 },
    field_border_style = { initial = "single" },
    field_initial = { initial = { initial_value = "Hello World" } }
})
print(field2:render())
print()

-- Test 3: Literal with no border
print("3. Literal (no border, width=30):")
local literal1 = OBJECTS_DEFINITIONS.new("Literal", {
    field_width = { initial = 30 },
    field_border_style = { initial = "none" },
    field_initial = { initial = { initial_value = "This is a literal text" } }
})
print(literal1:render())
print()

-- Test 4: BooleanField
print("4. BooleanField (checked):")
local bool1 = OBJECTS_DEFINITIONS.new("BooleanField", {
    field_initial = { initial = { initial_value = true } },
    field_border_style = { initial = "single" }
})
print(bool1:render())
print()

-- Test 5: BooleanField unchecked
print("5. BooleanField (unchecked):")
local bool2 = OBJECTS_DEFINITIONS.new("BooleanField", {
    field_initial = { initial = { initial_value = false } },
    field_border_style = { initial = "single" }
})
print(bool2:render())
print()

-- Test 6: Line
print("6. Line (width=50, double style):")
local line1 = OBJECTS_DEFINITIONS.new("Line", {
    field_width = { initial = 50 },
    field_border_style = { initial = "double" }
})
print(line1:render())
print()

-- Test 7: Fieldset
print("7. Fieldset (width=30, height=5, double border):")
local fieldset1 = OBJECTS_DEFINITIONS.new("Fieldset", {
    field_width = { initial = 30 },
    field_height = { initial = 5 },
    field_name = { initial = "My Fieldset" },
    field_border_style = { initial = "double" }
})
print(fieldset1:render())
print()

-- Test 8: ProtectedLiteral with dashed border
print("8. ProtectedLiteral (dashed border):")
local prot1 = OBJECTS_DEFINITIONS.new("ProtectedLiteral", {
    field_width = { initial = 25 },
    field_border_style = { initial = "dashed" },
    field_initial = { initial = { initial_value = "Protected" } }
})
print(prot1:render())
print()

-- Test 9: Field with custom dimensions and double border
print("9. Field (double border, width=20, height=4):")
local field3 = OBJECTS_DEFINITIONS.new("Field", {
    field_width = { initial = 20 },
    field_height = { initial = 4 },
    field_border_style = { initial = "double" },
    field_fill_char = { initial = "_" },
    field_initial = { initial = { initial_value = "Test" } }
})
print(field3:render())
print()

-- Test 10: Image (without ASCII art)
print("10. Image (no ASCII art, with border):")
local image1 = OBJECTS_DEFINITIONS.new("Image", {
    field_width = { initial = 20 },
    field_height = { initial = 5 },
    field_border_style = { initial = "double" }
})
print(image1:render())
print()

print("All tests completed!")
