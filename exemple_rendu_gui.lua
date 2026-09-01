-- ***********************************************************
-- Project : CICS BMS ncurses WYSIWYG Editor
-- File    : exemple_rendu_gui.lua
-- Designed-by : Sebastien Genose.org
-- Date    : 2024-08-31
-- Description : Example script demonstrating GUI rendering with OBJECT-GUI-RENDERING.lua
-- Description : Shows how to create and render various GUI field types with position structure
-- ***********************************************************

-- ===== LOAD MODULES =====
dofile('OBJECT-GUI-RENDERING.lua')

print("=" .. string.rep("=", 78))
print("  EXEMPLE DE RENDU GUI - CICS BMS ncurses WYSIWYG Editor")
print("=" .. string.rep("=", 78))
print()

-- ===== EXAMPLE 1: Simple Text Field with Label =====
print("--- EXEMPLE 1: Champ texte avec label ---\n")

local simple_text_field = create_gui_object('Field', {
    label = "Nom d'utilisateur",
    gui_field_type = "gui_textornum_with_label_field",
    is_required = true,
    position = {row = 1, col = 1, rowend = 3, colend = 40}
})

print("Simple Text Field:")
print(simple_text_field:render_gui())
print()

-- ===== EXEMPLE 2: Select Field (Checkbox-like) =====
print("--- EXEMPLE 2: Champ de selection (Case a cocher) ---\n")

local select_field = create_gui_object('BooleanField', {
    label = "Se souvenir de moi",
    gui_field_type = "gui_select_field",
    is_required = false,
    options = {"Oui", "Non"},
    selected_index = 2,
    position = {row = 1, col = 1, rowend = 5, colend = 30}
})

print("Select Field:")
print(select_field:render_gui())
print()

-- ===== EXEMPLE 3: List Field with Items =====
print("--- EXEMPLE 3: Liste d'elements ---\n")

local list_field = create_gui_object('Field', {
    label = "Pays",
    gui_field_type = "gui_list_field",
    is_required = true,
    items = {"France", "Belgique", "Suisse", "Canada", "Allemagne"},
    position = {row = 1, col = 1, rowend = 7, colend = 25}
})

print("List Field:")
print(list_field:render_gui())
print()

-- ===== EXEMPLE 4: Fieldset with Nested Fields =====
print("--- EXEMPLE 4: Conteneur Fieldset avec champs imbriques ---\n")

local address_fieldset = create_gui_object('Fieldset', {
    label = "Adresse",
    gui_field_type = "gui_fieldset_field",
    is_required = false,
    children = {
        create_gui_object('Field', {
            label = "Rue",
            position = {row = 1, col = 1, rowend = 1, colend = 40}
        }),
        create_gui_object('Field', {
            label = "Ville",
            position = {row = 2, col = 1, rowend = 2, colend = 40}
        }),
        create_gui_object('Field', {
            label = "Code postal",
            position = {row = 3, col = 1, rowend = 3, colend = 40}
        })
    },
    position = {row = 1, col = 1, rowend = 6, colend = 50}
})

print("Fieldset with Children:")
print(address_fieldset:render_gui())
print()

-- ===== EXEMPLE 5: Complete Form with Multiple Fields =====
print("--- EXEMPLE 5: Formulaire complet avec plusieurs champs ---\n")

local login_form = create_gui_form({
    -- Username field
    create_gui_object('Field', {
        label = "Nom d'utilisateur",
        gui_field_type = "gui_textornum_with_label_field",
        is_required = true,
        position = {row = 1, col = 1, rowend = 3, colend = 40}
    }),
    
    -- Password field
    create_gui_object('Field', {
        label = "Mot de passe",
        gui_field_type = "gui_textornum_with_label_field",
        is_required = true,
        position = {row = 5, col = 1, rowend = 7, colend = 40}
    }),
    
    -- Remember me checkbox
    create_gui_object('BooleanField', {
        label = "Se souvenir de moi",
        gui_field_type = "gui_select_field",
        is_required = false,
        options = {"Oui", "Non"},
        selected_index = 2,
        position = {row = 9, col = 1, rowend = 11, colend = 20}
    }),
    
    -- Country selection
    create_gui_object('Field', {
        label = "Pays",
        gui_field_type = "gui_list_field",
        is_required = true,
        items = {"France", "Belgique", "Suisse"},
        position = {row = 9, col = 25, rowend = 14, colend = 45}
    }),
    
    -- Address fieldset
    create_gui_object('Fieldset', {
        label = "Adresse",
        gui_field_type = "gui_fieldset_field",
        is_required = false,
        children = {
            create_gui_object('Field', {
                label = "Rue",
                position = {row = 1, col = 1, rowend = 1, colend = 30}
            }),
            create_gui_object('Field', {
                label = "Ville",
                position = {row = 2, col = 1, rowend = 2, colend = 30}
            }),
            create_gui_object('Field', {
                label = "Code postal",
                position = {row = 3, col = 1, rowend = 3, colend = 30}
            })
        },
        position = {row = 15, col = 1, rowend = 20, colend = 50}
    })
}, {
    title = "Formulaire d'inscription",
    width = 60,
    height = 24,
    border_style = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double
})

print("Complete Form Rendering:")
print(login_form:render())
print()

-- ===== EXEMPLE 6: Form with Error States =====
print("--- EXEMPLE 6: Formulaire avec champs en erreur ---\n")

local error_form = create_gui_form({
    create_gui_object('Field', {
        label = "Nom d'utilisateur",
        gui_field_type = "gui_textornum_with_label_field",
        is_required = true,
        has_error = true,
        position = {row = 1, col = 1, rowend = 3, colend = 40}
    }),
    
    create_gui_object('Field', {
        label = "Mot de passe",
        gui_field_type = "gui_textornum_with_label_field",
        is_required = true,
        has_error = false,
        position = {row = 5, col = 1, rowend = 7, colend = 40}
    }),
    
    create_gui_object('Field', {
        label = "Email",
        gui_field_type = "gui_textornum_with_label_field",
        is_required = false,
        has_error = true,
        position = {row = 9, col = 1, rowend = 11, colend = 40}
    })
}, {
    title = "Formulaire avec erreurs",
    width = 50,
    height = 15,
    border_style = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single
})

print("Form with Error States:")
print(error_form:render())
print()

-- ===== EXEMPLE 7: Different Border Styles =====
print("--- EXEMPLE 7: Different styles de bordure ---\n")

print("Style: NONE")
local no_border_field = create_gui_object('Field', {
    label = "Sans bordure",
    gui_field_type = "gui_textornum_with_label_field",
    position = {row = 1, col = 1, rowend = 1, colend = 25},
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.none}
})
print(no_border_field:render_gui())
print()

print("Style: SINGLE")
local single_border_field = create_gui_object('Field', {
    label = "Bordure simple",
    gui_field_type = "gui_textornum_with_label_field",
    position = {row = 1, col = 1, rowend = 3, colend = 25},
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.single}
})
print(single_border_field:render_gui())
print()

print("Style: DOUBLE")
local double_border_field = create_gui_object('Field', {
    label = "Bordure double",
    gui_field_type = "gui_textornum_with_label_field",
    position = {row = 1, col = 1, rowend = 3, colend = 25},
    field_border_style = {initial = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double}
})
print(double_border_field:render_gui())
print()

print("Style: DASHED")
local dashed_border_field = create_gui_object('Field', {
    label = "Bordure pointillee",
    gui_field_type = "gui_textornum_with_label_field",
    position = {row = 1, col = 1, rowend = 3, colend = 25},
    field_border_style = {initial = "dashed"}
})
print(dashed_border_field:render_gui())
print()

-- ===== EXEMPLE 8: Complex Form with All Field Types =====
print("--- EXEMPLE 8: Formulaire complexe avec tous les types ---\n")

local complex_form = create_gui_form({
    -- Section: Personal Information
    create_gui_object('Fieldset', {
        label = "Informations personnelles",
        gui_field_type = "gui_fieldset_field",
        children = {
            create_gui_object('Field', {
                label = "Prenom",
                gui_field_type = "gui_textornum_with_label_field",
                is_required = true,
                position = {row = 1, col = 1}
            }),
            create_gui_object('Field', {
                label = "Nom",
                gui_field_type = "gui_textornum_with_label_field",
                is_required = true,
                position = {row = 2, col = 1}
            }),
            create_gui_object('Field', {
                label = "Date de naissance",
                gui_field_type = "gui_textornum_with_label_field",
                position = {row = 3, col = 1}
            })
        },
        position = {row = 1, col = 1, rowend = 7, colend = 40}
    }),
    
    -- Section: Contact Information
    create_gui_object('Fieldset', {
        label = "Coordonnees",
        gui_field_type = "gui_fieldset_field",
        children = {
            create_gui_object('Field', {
                label = "Email",
                gui_field_type = "gui_textornum_with_label_field",
                is_required = true,
                position = {row = 1, col = 1}
            }),
            create_gui_object('Field', {
                label = "Telephone",
                gui_field_type = "gui_textornum_with_label_field",
                position = {row = 2, col = 1}
            })
        },
        position = {row = 1, col = 45, rowend = 7, colend = 80}
    }),
    
    -- Section: Preferences
    create_gui_object('Fieldset', {
        label = "Preferences",
        gui_field_type = "gui_fieldset_field",
        children = {
            create_gui_object('BooleanField', {
                label = "Newsletter",
                gui_field_type = "gui_select_field",
                options = {"Oui", "Non"},
                selected_index = 1,
                position = {row = 1, col = 1}
            }),
            create_gui_object('BooleanField', {
                label = "Notifications",
                gui_field_type = "gui_select_field",
                options = {"Oui", "Non"},
                selected_index = 2,
                position = {row = 2, col = 1}
            }),
            create_gui_object('Field', {
                label = "Langue",
                gui_field_type = "gui_list_field",
                items = {"Francais", "Anglais", "Espagnol"},
                position = {row = 1, col = 25, rowend = 4, colend = 40}
            })
        },
        position = {row = 9, col = 1, rowend = 15, colend = 80}
    }),
    
    -- Submit button area
    create_gui_object('Fieldset', {
        label = "Actions",
        gui_field_type = "gui_fieldset_field",
        children = {
            create_gui_object('BooleanField', {
                label = "Soumettre",
                gui_field_type = "gui_select_field",
                options = {"Oui"},
                selected_index = 1,
                position = {row = 1, col = 1}
            })
        },
        position = {row = 17, col = 1, rowend = 20, colend = 80}
    })
}, {
    title = "Formulaire utilisateur complet",
    width = 82,
    height = 24,
    border_style = OBJECTS_DEFINITIONS_DEFAULTS.border_style.enum.double
})

print("Complex Form with All Types:")
print(complex_form:render())
print()

-- ===== SUMMARY =====
print("=" .. string.rep("=", 78))
print("  SUMMARY: Tous les exemples de rendu GUI ont ete executes avec succes")
print("=" .. string.rep("=", 78))
print()
print("Types de champs demontres:")
print("  ✓ Champ texte avec label (gui_textornum_with_label_field)")
print("  ✓ Champ de selection (gui_select_field)")
print("  ✓ Liste d'elements (gui_list_field)")
print("  ✓ Conteneur Fieldset avec enfants (gui_fieldset_field)")
print("  ✓ Formulaire complet avec positionnement")
print("  ✓ Gestion des champs requis et en erreur")
print("  ✓ Different styles de bordure (none, single, double, dashed)")
print()
print("Structure de position utilisee:")
print("  position = {row, col, rowend, colend}")
print()
print("Le module OBJECT-GUI-RENDERING.lua est operationnel !")
