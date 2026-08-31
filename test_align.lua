local field = create_gui_object('Field', {
    label = 'Top',
    field_initial = 'text',
    gui_field_type = 'gui_text_field',
    field_border_style = {initial = 'single'},
    field_height = {initial = 5},
    field_width = {initial = 15}
})

-- Inject debug into render_gui_text_field
local original_render = render_gui_text_field
render_gui_text_field = function(obj, label_text, is_selected, is_required, has_error, override_width)
    local pos = get_position(obj)
    local height, width = get_dimensions(obj)
    local border_style = get_gui_simple_value(obj, 'field_border_style', 'none')
    print('DEBUG: label_text =', label_text)
    print('DEBUG: border_style =', border_style)
    print('DEBUG: height =', height)
    print('DEBUG: width =', width)
    return original_render(obj, label_text, is_selected, is_required, has_error, override_width)
end

print(field:render_gui())
