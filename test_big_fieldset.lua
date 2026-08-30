-- Test script for big-fieldset with visual rendering
-- Tests the hierarchical data access pattern and visual representation after refactoring

-- Load the OBJECTS-DEFINITIONS module
dofile('core/src/bms/OBJECTS-DEFINITIONS.lua')

print("=== Testing Big Fieldset with Visual Rendering ===\n")

-- Create a Fieldset (the main container)
local bigFieldset = OBJECTS_DEFINITIONS.new('Fieldset', {
    field_name = {initial = "big_test_fieldset"},
    field_initial = {initial_value = "Main Container"}
})

print("1. Created Main Fieldset:")
print("   Type: " .. bigFieldset.field_type.initial)
print("   Name: " .. (bigFieldset.field_name.initial or "nil"))
print("   Initial value: " .. (bigFieldset.field_initial.initial_value or "nil"))

-- Test visual rendering of the fieldset border
-- Note: field_border.initial contains the full border definition for Fieldset
print("\n2. Fieldset Border Configuration:")
local border = bigFieldset.field_border.initial
print("   Style: " .. table.concat(border.style, ", "))
print("   Chars - top_left: '" .. border.chars.double.top_left .. "'")
print("   Chars - top: '" .. string.rep(border.chars.double.top, 3) .. "'")
print("   Chars - top_right: '" .. border.chars.double.top_right .. "'")

-- Render a simple ASCII representation of the fieldset
print("\n3. ASCII Visual Representation:")
local style = border.style[1]  -- First style is default for Fieldset
local chars = border.chars
if chars[style] then
    local b = chars[style]
    print("   " .. b.top_left .. string.rep(b.top, 20) .. b.top_right)
    print("   " .. b.left .. string.rep(" ", 20) .. b.right)
    print("   " .. b.bottom_left .. string.rep(b.bottom, 20) .. b.bottom_right)
end

-- Create nested structure and render
print("\n4. Nested Field Types Visualization:")

local fieldTypes = {'Field', 'Literal', 'ProtectedLiteral', 'BooleanField', 'Image', 'Line'}

for _, type in ipairs(fieldTypes) do
    local obj = OBJECTS_DEFINITIONS.new(type, {
        field_name = {initial = "test_" .. type}
    })
    
    -- Get border info for this type (from initial)
    local objBorder = obj.field_border.initial
    local objStyle = objBorder.style
    local objChars = objBorder.chars
    
    print("\n   [" .. type .. "] Border:")
    print("   Style options: " .. table.concat(objStyle, ", "))
    
    -- Use first available style for rendering
    local firstStyle = objStyle[1]
    if objChars[firstStyle] then
        local b = objChars[firstStyle]
        local width = 15
        print("   " .. b.top_left .. string.rep(b.top, width) .. b.top_right)
        local content = " " .. type .. " Field "
        if #content > width + 2 then content = string.sub(content, 1, width) end
        print("   " .. b.left .. content .. string.rep(" ", width + 2 - #content - 2) .. b.right)
        print("   " .. b.bottom_left .. string.rep(b.bottom, width) .. b.bottom_right)
    end
end

-- Test color availability for GUI
print("\n5. GUI Color Options by Type:")
for _, type in ipairs(fieldTypes) do
    local colors = OBJECTS_DEFINITIONS.field_avail_color.default[type]
    print("   " .. type .. ": " .. table.concat(colors, ", "))
end

-- Test border style options for GUI
print("\n6. GUI Border Style Options by Type:")
for _, type in ipairs(fieldTypes) do
    local styles = OBJECTS_DEFINITIONS.field_avail_border_style.default[type]
    print("   " .. type .. ": " .. table.concat(styles, ", "))
end

-- Test that dynamic references work correctly
print("\n7. Dynamic References Verification:")
print("   bigFieldset.field_border.initial.style == OBJECTS_DEFINITIONS.field_avail_border_style.default.Fieldset: " .. 
      tostring(bigFieldset.field_border.initial.style == OBJECTS_DEFINITIONS.field_avail_border_style.default.Fieldset))
print("   bigFieldset.field_border.initial.chars == OBJECTS_DEFINITIONS.field_avail_border_chars.default.Fieldset: " .. 
      tostring(bigFieldset.field_border.initial.chars == OBJECTS_DEFINITIONS.field_avail_border_chars.default.Fieldset))

-- Test render function if available
print("\n8. Render Function Test:")
if bigFieldset.render then
    local rendered = bigFieldset.render(bigFieldset)
    print("   Rendered: " .. rendered)
else
    print("   No render function available")
end

print("\n=== Test Complete ===")
