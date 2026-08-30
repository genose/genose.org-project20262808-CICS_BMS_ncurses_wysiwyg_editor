-- Test script for big-fieldset with visual rendering of ALL field types
-- Tests the hierarchical data access pattern and visual representation after refactoring

dofile('core/src/bms/OBJECTS-DEFINITIONS.lua')

print("=== VISUAL TEST: Big Fieldset with All Field Types ===\n")

-- Create main container
local bigFieldset = OBJECTS_DEFINITIONS.new('Fieldset', {
    field_name = {initial = "main_container"},
    field_initial = {initial_value = "ALL FIELD TYPES DEMO"}
})

-- Get fieldset border style (double by default)
local fs_border = bigFieldset.field_border.initial
local fs_chars = fs_border.chars.double

-- RENDER MAIN FIELDSET CONTAINER
print("MAIN FIELDSET CONTAINER:")
print(fs_chars.top_left .. string.rep(fs_chars.top, 50) .. fs_chars.top_right)
print(fs_chars.left .. string.rep(" ", 50) .. fs_chars.right)
local title = bigFieldset.field_initial.initial.initial_value or bigFieldset.field_name.initial or "" 
print(fs_chars.left .. "  " .. title .. string.rep(" ", 50 - #title - 4) .. fs_chars.right)
print(fs_chars.left .. string.rep(" ", 50) .. fs_chars.right)

-- CREATE AND RENDER EACH FIELD TYPE
local fieldTypes = {'Field', 'Literal', 'ProtectedLiteral', 'BooleanField', 'Image', 'Line', 'Fieldset'}

for i, ftype in ipairs(fieldTypes) do
    print("\n" .. string.rep("-", 54))
    print("[" .. ftype .. "]")
    
    local field
    if ftype == "Image" then
        field = OBJECTS_DEFINITIONS.new(ftype, {
            field_name = {initial = "ascii_cat"},
            field_initial = {
                initial = {
                    initial_value = "Cat Image",
                    option_value = {
                        ascii_code = {
                            "    /\\___/\\    ",
                            "   (  o   o  )   ",
                            "   /  >X<  \\   "
                        },
                        file_path = nil
                    }
                }
            }
        })
    elseif ftype == "BooleanField" then
        field = OBJECTS_DEFINITIONS.new(ftype, {
            field_name = {initial = "checkbox_example"},
            field_initial = {initial_value = true}
        })
    else
        field = OBJECTS_DEFINITIONS.new(ftype, {
            field_name = {initial = ftype .. "_example"},
            field_initial = {initial_value = "Sample " .. ftype}
        })
    end
    
    local border = field.field_border.initial
    local chars = border.chars
    local style = border.style[1]
    local b = chars[style] or chars.double or chars.single
    
    local width = 40
    if ftype == "Line" then width = 50 end
    if ftype == "BooleanField" then width = 10 end
    
    print("   " .. b.top_left .. string.rep(b.top, width) .. b.top_right)
    
    if ftype == "Image" then
        local ascii = field.field_initial.initial.option_value.ascii_code
        for _, line in ipairs(ascii) do
            local padding = width - #line
            print("   " .. b.left .. " " .. line .. string.rep(" ", padding) .. " " .. b.right)
        end
    elseif ftype == "BooleanField" then
        local value = field.field_initial.initial.initial_value and "[X]" or "[ ]"
        local label = field.field_name.initial
        print("   " .. b.left .. " " .. value .. " " .. label .. string.rep(" ", width - #value - #label - 4) .. b.right)
    elseif ftype == "Line" then
        local line_char = (b.top ~= "" and b.top or "-")
        print("   " .. string.rep(line_char, width + 2))
    else
        local content = field.field_initial.initial.initial_value or field.field_name.initial
        print("   " .. b.left .. " " .. content .. string.rep(" ", width - #content - 1) .. b.right)
    end
    
    if ftype ~= "Line" then
        print("   " .. b.bottom_left .. string.rep(b.bottom, width) .. b.bottom_right)
    end
end

print("\n" .. string.rep("=", 54))
print("SUMMARY:")
for _, ftype in ipairs(fieldTypes) do
    local field = OBJECTS_DEFINITIONS.new(ftype)
    local border = field.field_border.initial
    print("  " .. ftype .. ": border_style=" .. table.concat(border.style, ",") .. 
          ", height=" .. OBJECTS_DEFINITIONS.field_height.default[ftype] .. 
          ", width=" .. OBJECTS_DEFINITIONS.field_width.default[ftype])
end

print("\n=== Test Complete ===")
