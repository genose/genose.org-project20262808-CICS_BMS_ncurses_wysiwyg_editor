-- ===========================================================
-- PROPERTIES-WINDOW-DEFINITIONS.lua
-- Properties window definitions
-- ===========================================================
-- A Object type embedded in the editor window, to edit the properties of the selected object, with a list of available fields to add, and a list of existing fields to edit or delete
-- each field has a type, a name, a value, and a set of properties (default, initial, edited) to manage the state of the field in the GUI
-- each field has a set of properties (default, initial, edited) to manage the state of the field in the GUI
-- each TYPE OBJECT has a set of available fields, each field has a type, a name, a value, and a set of properties (default, initial, edited) to manage the state of the field in the GUI
-- to render the properties window, we use a template visual, combining the properties of the fields according to the conditions
-- here exemples of available fields for each TYPE OBJECT, with their default properties, to be used in the properties window
-- Check if OBJECTS_DEFINITIONS is loaded
if not OBJECTS_DEFINITIONS then
    dofile("OBJECTS-DEFINITIONS.lua")
end

-- ***********************************************************
-- the window properties used to edit the properties of the selected object,
-- with a list of available fields associated to the selected object type, 
-- and a list of existing values of the selected object, to edit or delete them
-- ***********************************************************
-- using OBJECTS_DEFINITIONS, we define the properties window for each TYPE OBJECT,
-- with the available marked fields, and the existing values of the selected object, to edit or delete them
-- the values of the existing fields are adapted according to the selected object's current state and the available fields for its type, to allow editing or deleting them
-- showing the adapted values of the existing fields in the properties window
-- ===========================================================
-- exemple for an OBJECTS possessing a set of available fields, with their default properties, to be used in the properties window
-- here the following exemple show one of the rendering of the properties window for an OBJECTS of type "Field",
-- with the available fields and their default properties, to be used in the properties window
-- the order for rendering value fields in the properties window is .default, .initial, .edited, to allow the user to edit the properties of the selected object, and to show the adapted values of the existing fields in the properties window
-- by selecting the value of the field, the user can edit the properties of the selected object, and the adapted values of the existing fields in the properties window are updated accordingly in .initial and .edited,
-- to allow the user to edit the properties of the selected object, and to show the adapted values of the existing fields in the properties window
-- ***********************************************************
-- GUI Render : 
-- each rendering is based on introspection of the object's fields and their current values having gui_field_type, and the rendering is done according to the field's type and its properties, with the appropriate visual representation for each field type, and the adapted values of the existing fields in the properties window are updated accordingly in .initial and .edited,
-- to allow the user to edit the properties of the selected object, and to show the adapted
--  field_*+gui_field_type, will render in HTML like phylosophy, with the appropriate visual representation for each field type, and the adapted values of the existing fields in the properties window are updated accordingly in .initial and .edited,
-- like as follow for an property "border_style" form "field_border_style", with the appropriate visual representation for each field type, and the adapted values of the existing fields in the properties window are updated accordingly in .initial and .edited,

-- ***********************************************************
test_gui_object = OBJECTS_DEFINITIONS.new(OBJECTS_DEFINITIONS.field_type.enum.Field) -- Crée un objet de type Field, avec .default pointant vers.default[TYPE] et .initial = nil, .edited = nil
test_gui_object.field_border_style.default = OBJECTS_DEFINITIONS.field_border_style.default[test_gui_object.field_type
                                                 .initial] -- Récupère les attributs de border_style du champ
test_gui_object.field_border_style.initial = test_gui_object.field_border_style.default -- Initialise la valeur initiale à la valeur par défaut
test_gui_object.field_border_style.edited = test_gui_object.field_border_style.initial -- Initialise la valeur éditée à la valeur initiale
-- using the already defined : test_gui_object.field_border_style.gui_field_type for rendering the border style field in the properties window
myPropsToShow = test_gui_object.field_border_style -- Récupère les attributs de border_style du champ
-- using decoration (Fieldset) section is rendered like this in the properties window:
-- ======= | border style | ========
-- | kprops : [ ....kvprops... |v] |
-- | kprops : [ ....kvprops... |v] |
-- =================================
-- ***********************************************************
