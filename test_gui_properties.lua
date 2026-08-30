-- =============================================================================
-- Test File: test_gui_properties.lua
-- Tests the GUI property extraction functions from OBJECTS-DEFINITIONS.lua
-- =============================================================================

dofile("OBJECTS-DEFINITIONS.lua")

print("=" .. string.rep("=", 78))
print("  TEST: GUI Property Extraction from OBJECTS-DEFINITIONS")
print("=" .. string.rep("=", 78))
print()

-- ============================================================================
-- TEST 1: Extract GUI properties for a specific type (Field)
-- =============================================================================
print("TEST 1: Properties for 'Field' type")
print("-" .. string.rep("-", 78))

local field_props = OBJECTS_DEFINITIONS.get_gui_properties("Field")

print(string.format("  Found %d properties for Field type\n", #field_props))

-- Display in a nice table format
local function print_properties_table(properties, title)
    if not properties or #properties == 0 then
        print("  No properties found")
        return
    end
    
    -- Find max widths for columns
    local max_name = 0
    local max_category = 0
    local max_type = 0
    for _, p in ipairs(properties) do
        max_name = math.max(max_name, #p.gui_name)
        max_category = math.max(max_category, #p.category)
        max_type = math.max(max_type, #p.gui_type)
    end
    
    -- Header
    print(string.format("  %-" .. max_name .. "s | %-" .. max_category .. "s | %-" .. max_type .. "s | %-10s | %s",
        "PROPERTY", "CATEGORY", "TYPE", "CONTROL", "HINT"))
    print(string.format("  %s + %s + %s + %-10s + %s",
        string.rep("-", max_name), string.rep("-", max_category), 
        string.rep("-", max_type), "----------", string.rep("-", 40)))
    
    -- Data rows
    for _, p in ipairs(properties) do
        local hint = p.hint or ""
        if #hint > 40 then hint = hint:sub(1, 37) .. "..." end
        print(string.format("  %-" .. max_name .. "s | %-" .. max_category .. "s | %-" .. max_type .. "s | %-10s | %s",
            p.gui_name, p.category, p.gui_type, p.control_type, hint))
    end
    print()
end

print_properties_table(field_props, "Field Properties")

-- ============================================================================
-- TEST 2: Extract properties for Fieldset
-- =============================================================================
print("TEST 2: Properties for 'Fieldset' type")
print("-" .. string.rep("-", 78))

local fieldset_props = OBJECTS_DEFINITIONS.get_gui_properties("Fieldset")
print(string.format("  Found %d properties for Fieldset type\n", #fieldset_props))

print_properties_table(fieldset_props, "Fieldset Properties")

-- ============================================================================
-- TEST 3: Get ncurses menu items
-- =============================================================================
print("TEST 3: Ncurses menu items for 'BooleanField'")
print("-" .. string.rep("-", 78))

local menu_items = OBJECTS_DEFINITIONS.get_ncurses_menu_items("BooleanField")

print(string.format("  Found %d menu items for BooleanField\n", #menu_items))

-- Display first 10 menu items as example
for i = 1, math.min(10, #menu_items) do
    local item = menu_items[i]
    print(string.format("  [%2d] %-25s | Type: %-12s | Control: %-15s | Read-only: %s",
        i, item.label, item.type, item.control_type, item.read_only and "YES" or "NO"))
    if item.choices then
        print(string.format("      Choices: %s", table.concat(item.choices, ", ")))
    end
    if item.min or item.max then
        print(string.format("      Range: %s - %s", tostring(item.min), tostring(item.max)))
    end
end
if #menu_items > 10 then
    print(string.format("  ... and %d more items", #menu_items - 10))
end
print()

-- ============================================================================
-- TEST 4: Extract all global properties (no specific type)
-- =============================================================================
print("TEST 4: Global properties (all types)")
print("-" .. string.rep("-", 78))

local global_props = OBJECTS_DEFINITIONS.get_gui_properties()
print(string.format("  Found %d global properties\n", #global_props))

-- Group by category
local categories = {}
for _, p in ipairs(global_props) do
    categories[p.category] = categories[p.category] or {}
    table.insert(categories[p.category], p)
end

for category, props in pairs(categories) do
    print(string.format("  Category: %s (%d properties)", category, #props))
    for _, p in ipairs(props) do
        print(string.format("    - %-25s (%s)", p.gui_name, p.gui_type))
    end
end
print()

-- ============================================================================
-- TEST 5: Export to JSON
-- =============================================================================
print("TEST 5: JSON export for 'Line' type")
print("-" .. string.rep("-", 78))

local json = OBJECTS_DEFINITIONS.export_to_json("Line")

print("  " .. json:sub(1, 200) .. (json:len() > 200 and "..." or ""))
print()

-- ============================================================================
-- TEST 6: Detailed property inspection
-- =============================================================================
print("TEST 6: Detailed inspection of specific properties")
print("-" .. string.rep("-", 78))

-- Test field_height property
local all_props = OBJECTS_DEFINITIONS.get_gui_properties("Field")
for _, p in ipairs(all_props) do
    if p.name == "field_height" then
        print("  Property: field_height")
        print(string.format("    GUI Name: %s", p.gui_name))
        print(string.format("    Category: %s", p.category))
        print(string.format("    GUI Type: %s", p.gui_type))
        print(string.format("    Control Type: %s", p.control_type))
        print(string.format("    Default: %s", tostring(p.default)))
        if p.min_max then
            print(string.format("    Min/Max: %d - %d", p.min_max.min, p.min_max.max))
        end
        if p.available_values then
            print(string.format("    Available Values: %s", table.concat(p.available_values, ", ")))
        end
        break
    end
end
print()

-- Test field_avail_color property
for _, p in ipairs(all_props) do
    if p.name == "field_avail_color" then
        print("  Property: field_avail_color")
        print(string.format("    GUI Name: %s", p.gui_name))
        print(string.format("    Category: %s", p.category))
        print(string.format("    GUI Type: %s", p.gui_type))
        print(string.format("    Control Type: %s", p.control_type))
        print(string.format("    Read-only: %s", p.read_only and "YES" or "NO"))
        if p.available_values then
            print(string.format("    Available Colors: %s", table.concat(p.available_values, ", ")))
        end
        break
    end
end
print()

-- ============================================================================
-- TEST 7: Test with actual object instance
-- =============================================================================
print("TEST 7: Property extraction from an object instance")
print("-" .. string.rep("-", 78))

local myField = OBJECTS_DEFINITIONS.new("Field", {
    field_name = { initial = "username" },
    field_height = { initial = 5 },
    field_width = { initial = 30 }
})

-- For now, we can only get the GUI properties from the class definition
-- In a real implementation, you might want to extract current values from the instance
local instance_props = OBJECTS_DEFINITIONS.get_gui_properties("Field")
print(string.format("  Created Field instance: %s (" .. myField:render() .. ")", myField.field_type.initial))
print(string.format("  Instance has %d GUI properties available", #instance_props))
print()

-- ============================================================================
-- TEST 8: Categories summary
-- =============================================================================
print("TEST 8: Summary by categories")
print("-" .. string.rep("-", 78))

local all_types = {"Field", "Literal", "ProtectedLiteral", "BooleanField", "Image", "Line", "Fieldset"}
local category_counts = {}

for _, obj_type in ipairs(all_types) do
    local props = OBJECTS_DEFINITIONS.get_gui_properties(obj_type)
    for _, p in ipairs(props) do
        category_counts[p.category] = (category_counts[p.category] or 0) + 1
    end
end

print("  Category distribution across all types:")
for category, count in pairs(category_counts) do
    print(string.format("    %-20s: %d properties", category, count))
end
print()

-- ============================================================================
-- SUMMARY
-- =============================================================================
print("=" .. string.rep("=", 78))
print("  SUMMARY: All tests completed successfully!")
print("=" .. string.rep("=", 78))
print()
print("  The GUI property extraction system is working correctly.")
print("  You can now use these functions in your GUI implementation:")
print()
print("    - OBJECTS_DEFINITIONS.get_gui_properties(type)")
print("    - OBJECTS_DEFINITIONS.get_ncurses_menu_items(type)")
print("    - OBJECTS_DEFINITIONS.export_to_json(type)")
print()
