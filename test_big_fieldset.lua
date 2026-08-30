-- Test script for big-fieldset with visual rendering of ALL field types
-- Tests the hierarchical data access pattern and visual representation after refactoring

dofile('OBJECTS-DEFINITIONS.lua')

print("=== VISUAL TEST: Big Fieldset with All Field Types ===\n")

-- Create main container
local bigFieldset = OBJECTS_DEFINITIONS.new('Fieldset', {
    field_name = {initial = "main_container"},
    field_initial = {initial_value = "ALL FIELD TYPES DEMO"}
})

-- Get fieldset border style (double by default)
local fs_border = bigFieldset.field_border.initial
local fs_chars = fs_border.chars.double
local width = OBJECTS_DEFINITIONS.field_width.default.Fieldset

-- RENDER MAIN FIELDSET CONTAINER
-- line 0: border top + title
-- lines 1 to N-1: border left/right + content
-- line N: border bottom
print("MAIN FIELDSET CONTAINER:")
local title = bigFieldset.field_initial.initial.initial_value or bigFieldset.field_name.initial or ""
-- Line 0: top border with title centered
local title_padding = math.floor((width - #title - 2) / 2)
print(fs_chars.top_left .. string.rep(fs_chars.top, title_padding) .. " " .. title .. " " .. string.rep(fs_chars.top, width - title_padding - #title - 4) .. fs_chars.top_right)
-- Line 1 to N-1: content
print(fs_chars.left .. string.rep(" ", width) .. fs_chars.right)
print(fs_chars.left .. string.rep(" ", width) .. fs_chars.right)
-- Line N: bottom border
print(fs_chars.bottom_left .. string.rep(fs_chars.bottom, width) .. fs_chars.bottom_right)

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
    
    local fwidth = OBJECTS_DEFINITIONS.field_width.default[ftype]
    if ftype == "Line" then fwidth = 50 end
    if ftype == "BooleanField" then fwidth = 15 end
    
    -- Line 0: top border
    print("   " .. b.top_left .. string.rep(b.top, fwidth) .. b.top_right)
    
    -- Lines 1 to N-1: content
    if ftype == "Image" then
        local ascii = field.field_initial.initial.option_value.ascii_code
        for _, line in ipairs(ascii) do
            local padding = fwidth - #line
            print("   " .. b.left .. " " .. line .. string.rep(" ", padding) .. " " .. b.right)
        end
    elseif ftype == "BooleanField" then
        local value = field.field_initial.initial.initial_value and "[X]" or "[ ]"
        local label = field.field_name.initial
        print("   " .. b.left .. " " .. value .. " " .. label .. string.rep(" ", fwidth - #value - #label - 4) .. b.right)
    elseif ftype == "Line" then
        local line_char = (b.top ~= "" and b.top or "-")
        print("   " .. string.rep(line_char, fwidth + 2))
    else
        local content = field.field_initial.initial.initial_value or field.field_name.initial
        print("   " .. b.left .. " " .. content .. string.rep(" ", fwidth - #content - 1) .. b.right)
    end
    
    -- Line N: bottom border (skip for Line)
    if ftype ~= "Line" then
        print("   " .. b.bottom_left .. string.rep(b.bottom, fwidth) .. b.bottom_right)
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
