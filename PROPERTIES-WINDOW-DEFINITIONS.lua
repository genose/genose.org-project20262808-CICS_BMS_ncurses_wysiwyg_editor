---------------------
-- Properties window
---------------------
-- A Object type embedded in the editor window, to edit the properties of the selected object, with a list of available fields to add, and a list of existing fields to edit or delete
-- each field has a type, a name, a value, and a set of properties (default, initial, edited) to manage the state of the field in the GUI
-- each field has a set of properties (default, initial, edited) to manage the state of the field in the GUI
-- each TYPE OBJECT has a set of available fields, each field has a type, a name, a value, and a set of properties (default, initial, edited) to manage the state of the field in the GUI
-- to render the properties window, we use a template visual, combining the properties of the fields according to the conditions
-- here exemples of available fields for each TYPE OBJECT, with their default properties, to be used in the properties window
 field_border={
 Field={style=field_avail_border_style.default.Field, chars=field_avail_border_chars.default.Field},
---------------------
Rendu GUI : 
---------------------
 border style : [ ... |v]
 border chars : [ ... |v]
---------------------