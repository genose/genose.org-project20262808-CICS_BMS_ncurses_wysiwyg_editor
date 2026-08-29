//! Tests pour le module editor (BMS WYSIWYG Editor)

use cobol_bms_core::bms::editor::*;
use cobol_bms_core::bms::model::*;

// ==================== TESTS: BmsEditor ====================

#[test]
fn test_new_editor() {
    let editor = BmsEditor::new();
    assert_eq!(editor.map.name, "NEWMAP");
    assert_eq!(editor.map.mapset, "DEFAULT");
    assert_eq!(editor.map.size, (24, 80));
    assert_eq!(editor.map.fields.len(), 0);
    assert_eq!(editor.selected_field, None);
    assert_eq!(editor.cursor_pos, (1, 1));
    assert_eq!(editor.mode, EditorMode::Navigate);
}

#[test]
fn test_new_editor_from_map() {
    let mut map = BmsMap::new("TEST", "TESTSET");
    map.size = (10, 20);
    map.fields.push(BmsField {
        name: "FIELD1".to_string(),
        field_type: FieldType::Field,
        pos: (1, 1),
        length: 10,
        attrb: vec![FieldAttribute::Norm],
        text_color: Some(Color::Yellow),
        initial: None,
        pic: None,
        grp_name: None, ..Default::default()
    });
    
    let editor = BmsEditor::from_map(map);
    assert_eq!(editor.map.name, "TEST");
    assert_eq!(editor.map.mapset, "TESTSET");
    assert_eq!(editor.map.size, (10, 20));
    assert_eq!(editor.map.fields.len(), 1);
}

#[test]
fn test_new_map() {
    let mut editor = BmsEditor::new();
    editor.new_map("NEWMAP", "MYSET", (30, 100));
    
    assert_eq!(editor.map.name, "NEWMAP");
    assert_eq!(editor.map.mapset, "MYSET");
    assert_eq!(editor.map.size, (30, 100));
    assert_eq!(editor.map.fields.len(), 0);
    assert_eq!(editor.selected_field, None);
    
    // Vérifier que l'opération est dans l'historique
    assert_eq!(editor.history.undo_stack.len(), 1);
}

// ==================== TESTS: Field Operations ====================

#[test]
fn test_add_field_at_cursor() {
    let mut editor = BmsEditor::new();
    editor.set_cursor((5, 10));
    
    let idx = editor.add_field_at_cursor(15);
    
    assert_eq!(idx, 0);
    assert_eq!(editor.map.fields.len(), 1);
    assert_eq!(editor.map.fields[0].pos, (5, 10));
    assert_eq!(editor.map.fields[0].length, 15);
    assert_eq!(editor.map.fields[0].attrb, vec![FieldAttribute::Norm]);
    assert_eq!(editor.map.fields[0].text_color, Some(Color::Yellow));
    assert_eq!(editor.selected_field, Some(0));
    
    // Vérifier l'historique
    assert_eq!(editor.history.undo_stack.len(), 1);
}

#[test]
fn test_add_multiple_fields() {
    let mut editor = BmsEditor::new();
    
    editor.add_field_at_cursor(10);
    editor.set_cursor((2, 5));
    editor.add_field_at_cursor(20);
    editor.set_cursor((3, 8));
    editor.add_field_at_cursor(5);
    
    assert_eq!(editor.map.fields.len(), 3);
    assert_eq!(editor.map.fields[0].pos, (1, 1));
    assert_eq!(editor.map.fields[1].pos, (2, 5));
    assert_eq!(editor.map.fields[2].pos, (3, 8));
}

#[test]
fn test_remove_selected_field() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.add_field_at_cursor(10);
    
    assert_eq!(editor.map.fields.len(), 2);
    editor.selected_field = Some(0);
    
    let removed = editor.remove_selected_field();
    
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().length, 10);
    assert_eq!(editor.map.fields.len(), 1);
    assert_eq!(editor.selected_field, None);
    
    // Vérifier l'historique
    assert_eq!(editor.history.undo_stack.len(), 3); // new_map + 2 adds + 1 remove
}

#[test]
fn test_remove_none_selected() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.selected_field = None;
    
    let removed = editor.remove_selected_field();
    assert!(removed.is_none());
    assert_eq!(editor.map.fields.len(), 1); // Rien n'est supprimé
}

// ==================== TESTS: Field Selection ====================

#[test]
fn test_select_field_at() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(5); // POS=(1,1), LENGTH=5
    
    // Sélectionner un point dans le champ
    let idx = editor.select_field_at((1, 3));
    assert_eq!(idx, Some(0));
    assert_eq!(editor.selected_field, Some(0));
    
    // Sélectionner un point hors du champ
    let idx = editor.select_field_at((5, 5));
    assert_eq!(idx, None);
    assert_eq!(editor.selected_field, None);
}

#[test]
fn test_select_field_at_multiple_fields() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(5); // POS=(1,1), LENGTH=5
    editor.set_cursor((2, 10));
    editor.add_field_at_cursor(8); // POS=(2,10), LENGTH=8
    
    // Sélectionner le premier champ
    editor.select_field_at((1, 2));
    assert_eq!(editor.selected_field, Some(0));
    
    // Sélectionner le deuxième champ
    editor.select_field_at((2, 12));
    assert_eq!(editor.selected_field, Some(1));
}

#[test]
fn test_select_next_field() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(5);
    editor.add_field_at_cursor(5);
    editor.add_field_at_cursor(5);
    
    editor.selected_field = None;
    editor.select_next_field();
    assert_eq!(editor.selected_field, Some(0));
    
    editor.select_next_field();
    assert_eq!(editor.selected_field, Some(1));
    
    editor.select_next_field();
    assert_eq!(editor.selected_field, Some(2));
    
    editor.select_next_field();
    assert_eq!(editor.selected_field, Some(0)); // Boucle
}

#[test]
fn test_select_prev_field() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(5);
    editor.add_field_at_cursor(5);
    editor.add_field_at_cursor(5);
    
    editor.selected_field = None;
    editor.select_prev_field();
    assert_eq!(editor.selected_field, Some(2)); // Dernier champ
    
    editor.select_prev_field();
    assert_eq!(editor.selected_field, Some(1));
    
    editor.select_prev_field();
    assert_eq!(editor.selected_field, Some(0));
}

// ==================== TESTS: Cursor Movement ====================

#[test]
fn test_move_cursor() {
    let mut editor = BmsEditor::new();
    editor.new_map("TEST", "DEFAULT", (24, 80));
    
    // Position initiale
    assert_eq!(editor.cursor_pos, (1, 1));
    
    // Déplacer à droite
    editor.move_cursor(CursorDirection::Right, 5);
    assert_eq!(editor.cursor_pos, (1, 6));
    
    // Déplacer en bas
    editor.move_cursor(CursorDirection::Down, 3);
    assert_eq!(editor.cursor_pos, (4, 6));
    
    // Déplacer à gauche (limité à 1)
    editor.move_cursor(CursorDirection::Left, 10);
    assert_eq!(editor.cursor_pos, (4, 1));
    
    // Déplacer en haut (limité à 1)
    editor.move_cursor(CursorDirection::Up, 10);
    assert_eq!(editor.cursor_pos, (1, 1));
}

#[test]
fn test_set_cursor_bounds() {
    let mut editor = BmsEditor::new();
    editor.new_map("TEST", "DEFAULT", (10, 20));
    
    // Essayer de dépasser les limites
    editor.set_cursor((0, 0));
    assert_eq!(editor.cursor_pos, (1, 1));
    
    editor.set_cursor((100, 100));
    assert_eq!(editor.cursor_pos, (10, 20));
    
    editor.set_cursor((5, 15));
    assert_eq!(editor.cursor_pos, (5, 15));
}

// ==================== TESTS: Field Movement ====================

#[test]
fn test_move_selected_field() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(5); // POS=(1,1)
    editor.selected_field = Some(0);
    
    editor.move_selected_field((3, 10));
    
    assert_eq!(editor.map.fields[0].pos, (3, 10));
    
    // Vérifier l'historique
    assert_eq!(editor.history.undo_stack.len(), 2); // new_map + add + move
}

#[test]
fn test_move_field_without_selection() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(5);
    editor.selected_field = None;
    
    let old_pos = editor.map.fields[0].pos;
    editor.move_selected_field((5, 5));
    
    // Rien ne doit changer
    assert_eq!(editor.map.fields[0].pos, old_pos);
}

// ==================== TESTS: Field Resizing ====================

#[test]
fn test_resize_selected_field() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.selected_field = Some(0);
    
    editor.resize_selected_field(20);
    
    assert_eq!(editor.map.fields[0].length, 20);
}

#[test]
fn test_resize_field_without_selection() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.selected_field = None;
    
    let old_length = editor.map.fields[0].length;
    editor.resize_selected_field(20);
    
    assert_eq!(editor.map.fields[0].length, old_length);
}

// ==================== TESTS: Clipboard ====================

#[test]
fn test_copy_cut_paste() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.selected_field = Some(0);
    editor.set_selected_field_color(Some(Color::Blue));
    
    // Copier
    editor.copy_selected();
    assert!(!editor.clipboard.is_empty());
    assert_eq!(editor.clipboard[0].text_color, Some(Color::Blue));
    assert_eq!(editor.map.fields.len(), 1);
    
    // Couper
    editor.selected_field = Some(0);
    let cut_field = editor.cut_selected();
    assert!(cut_field.is_some());
    assert_eq!(editor.map.fields.len(), 0);
    assert_eq!(editor.clipboard[0].text_color, Some(Color::Blue));
    
    // Coller
    editor.set_cursor((5, 5));
    let new_idx = editor.paste_at_cursor();
    assert!(new_idx.is_some());
    assert_eq!(editor.map.fields.len(), 1);
    assert_eq!(editor.map.fields[0].pos, (5, 5));
    assert_eq!(editor.map.fields[0].text_color, Some(Color::Blue));
}

#[test]
fn test_paste_empty_clipboard() {
    let mut editor = BmsEditor::new();
    editor.clipboard.clear();
    
    let result = editor.paste_at_cursor();
    assert!(result.is_none());
    assert_eq!(editor.map.fields.len(), 0);
}

#[test]
fn test_multiple_field_clipboard() {
    let mut editor = BmsEditor::new();
    
    // Add multiple fields
    editor.add_field_at_cursor(10); // Field 0
    editor.set_cursor((2, 1));
    editor.add_field_at_cursor(10); // Field 1
    editor.set_cursor((3, 1));
    editor.add_field_at_cursor(10); // Field 2
    
    assert_eq!(editor.map.fields.len(), 3);
    
    // Copy multiple fields
    editor.copy_fields(&[0, 1, 2]);
    assert_eq!(editor.clipboard_count(), 3);
    
    // Move cursor and paste
    editor.set_cursor((10, 1));
    let count = editor.paste_fields_at((10, 1));
    assert_eq!(count, 3);
    assert_eq!(editor.map.fields.len(), 6);
    
    // Check pasted positions
    assert_eq!(editor.map.fields[3].pos, (10, 1));
    assert_eq!(editor.map.fields[4].pos, (11, 1));
    assert_eq!(editor.map.fields[5].pos, (12, 1));
}

#[test]
fn test_clipboard_clear() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.copy_selected();
    assert_eq!(editor.clipboard_count(), 1);
    
    editor.clear_clipboard();
    assert_eq!(editor.clipboard_count(), 0);
}

// ==================== TESTS: Properties ====================

#[test]
fn test_set_selected_field_name() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.selected_field = Some(0);
    
    editor.set_selected_field_name("CUSTOMER");
    
    assert_eq!(editor.map.fields[0].name, "CUSTOMER");
}

#[test]
fn test_set_selected_field_color() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.selected_field = Some(0);
    
    editor.set_selected_field_color(Some(Color::Red));
    assert_eq!(editor.map.fields[0].text_color, Some(Color::Red));
    
    editor.set_selected_field_color(None);
    assert_eq!(editor.map.fields[0].text_color, None);
}

#[test]
fn test_set_selected_field_attributes() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.selected_field = Some(0);
    
    editor.set_selected_field_attributes(vec![
        FieldAttribute::Prot,
        FieldAttribute::Intens,
    ]);
    
    assert_eq!(editor.map.fields[0].attrb.len(), 2);
    assert!(editor.map.fields[0].attrb.contains(&FieldAttribute::Prot));
    assert!(editor.map.fields[0].attrb.contains(&FieldAttribute::Intens));
}

#[test]
fn test_add_remove_field_attribute() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.selected_field = Some(0);
    
    // Le champ a deja Norm par defaut
    assert_eq!(editor.map.fields[0].attrb.len(), 1);
    assert!(editor.map.fields[0].attrb.contains(&FieldAttribute::Norm));
    
    // Ajouter un attribut
    editor.add_selected_field_attribute(FieldAttribute::Prot);
    assert_eq!(editor.map.fields[0].attrb.len(), 2);
    
    // Ajouter un autre
    editor.add_selected_field_attribute(FieldAttribute::Num);
    assert_eq!(editor.map.fields[0].attrb.len(), 3);
    
    // Retirer un attribut
    editor.remove_selected_field_attribute(&FieldAttribute::Prot);
    assert_eq!(editor.map.fields[0].attrb.len(), 2);
    assert!(editor.map.fields[0].attrb.contains(&FieldAttribute::Num));
    assert!(editor.map.fields[0].attrb.contains(&FieldAttribute::Norm));
}

#[test]
fn test_set_selected_field_initial() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.selected_field = Some(0);
    
    editor.set_selected_field_initial(Some("DEFAULT"));
    assert_eq!(editor.map.fields[0].initial, Some("DEFAULT".to_string()));
}

#[test]
fn test_set_selected_field_pic() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.selected_field = Some(0);
    
    editor.set_selected_field_pic(Some("9(5)"));
    assert_eq!(editor.map.fields[0].pic, Some("9(5)".to_string()));
}

// ==================== TESTS: Undo/Redo ====================

#[test]
fn test_undo_add_field() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    assert_eq!(editor.map.fields.len(), 1);
    
    editor.undo();
    assert_eq!(editor.map.fields.len(), 0);
}

#[test]
fn test_redo_add_field() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.undo();
    assert_eq!(editor.map.fields.len(), 0);
    
    editor.redo();
    assert_eq!(editor.map.fields.len(), 1);
}

#[test]
fn test_undo_remove_field() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.selected_field = Some(0);
    editor.remove_selected_field();
    assert_eq!(editor.map.fields.len(), 0);
    
    editor.undo();
    assert_eq!(editor.map.fields.len(), 1);
}

#[test]
fn test_undo_move_field() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.selected_field = Some(0);
    editor.move_selected_field((5, 5));
    assert_eq!(editor.map.fields[0].pos, (5, 5));
    
    editor.undo();
    assert_eq!(editor.map.fields[0].pos, (1, 1));
}

#[test]
fn test_undo_clear_redo_stack() {
    let mut editor = BmsEditor::new();
    editor.add_field_at_cursor(10);
    editor.undo();
    
    // Faire une nouvelle action doit vider le redo stack
    editor.add_field_at_cursor(10);
    assert_eq!(editor.history.redo_stack.len(), 0);
}

#[test]
fn test_undo_new_map() {
    let mut editor = BmsEditor::new();
    editor.new_map("NEW", "SET", (10, 10));
    assert_eq!(editor.map.name, "NEW");
    
    editor.undo();
    assert_eq!(editor.map.name, "NEWMAP"); // Retour à l'état initial
}

// ==================== TESTS: Export ====================

#[test]
fn test_export_to_bms_basic() {
    let mut editor = BmsEditor::new();
    editor.new_map("EXPORT", "TESTSET", (24, 80));
    editor.add_field_at_cursor(10);
    
    let bms = editor.export_to_bms();
    
    assert!(bms.contains("DFHMSD TYPE=EXPORT,MAPSET=TESTSET"));
    assert!(bms.contains("DFHMDI SIZE=(24,80)"));
    assert!(bms.contains("DFHMND TYPE=MAP"));
    assert!(bms.contains("DFHMND POS=(1,1),LENGTH=10"));
}

#[test]
fn test_export_field_with_attributes() {
    let mut editor = BmsEditor::new();
    editor.new_map("EXPORT", "TESTSET", (24, 80));
    
    let mut field = BmsField::default();
    field.pos = (2, 5);
    field.length = 15;
    field.attrb = vec![FieldAttribute::Prot, FieldAttribute::Num];
    field.text_color = Some(Color::Blue);
    field.initial = Some("TEST".to_string());
    field.name = "INPUT1".to_string();
    
    editor.add_field(field);
    
    let bms = editor.export_to_bms();
    
    assert!(bms.contains("POS=(2,5)"));
    assert!(bms.contains("LENGTH=15"));
    assert!(bms.contains("ATTRB=(PROT,NUM)"));
    assert!(bms.contains("COLOR=BLUE"));
    assert!(bms.contains("INITIAL='TEST'"));
    assert!(bms.contains("* INPUT1")); // Nom dans le commentaire
}

#[test]
fn test_export_empty_map() {
    let mut editor = BmsEditor::new();
    editor.new_map("EMPTY", "TESTSET", (10, 10));
    
    let bms = editor.export_to_bms();
    
    assert!(bms.contains("DFHMSD TYPE=EMPTY,MAPSET=TESTSET"));
    assert!(bms.contains("DFHMDI SIZE=(10,10)"));
    assert!(!bms.contains("DFHMND POS=")); // Aucun champ
}

// ==================== TESTS: EditHistory ====================

#[test]
fn test_history_push_and_undo() {
    let mut history = EditHistory::new(10);
    
    history.push(EditOperation::AddField {
        field: BmsField::default(),
        index: 0,
    });
    
    assert_eq!(history.undo_stack.len(), 1);
    assert_eq!(history.redo_stack.len(), 0);
    
    let op = history.undo();
    assert!(op.is_some());
    assert_eq!(history.undo_stack.len(), 0);
    assert_eq!(history.redo_stack.len(), 1);
}

#[test]
fn test_history_redo() {
    let mut history = EditHistory::new(10);
    
    history.push(EditOperation::AddField {
        field: BmsField::default(),
        index: 0,
    });
    
    history.undo();
    assert_eq!(history.redo_stack.len(), 1);
    
    let op = history.redo();
    assert!(op.is_some());
    assert_eq!(history.undo_stack.len(), 1);
    assert_eq!(history.redo_stack.len(), 0);
}

#[test]
fn test_history_max_size() {
    let mut history = EditHistory::new(3);
    
    for i in 0..5 {
        history.push(EditOperation::AddField {
            field: BmsField::default(),
            index: i,
        });
    }
    
    // Seul les 3 dernieres operations doivent rester
    assert_eq!(history.undo_stack.len(), 3);
}

#[test]
fn test_history_clear() {
    let mut history = EditHistory::new(10);
    
    history.push(EditOperation::AddField {
        field: BmsField::default(),
        index: 0,
    });
    history.push(EditOperation::AddField {
        field: BmsField::default(),
        index: 1,
    });
    
    history.clear();
    assert_eq!(history.undo_stack.len(), 0);
    assert_eq!(history.redo_stack.len(), 0);
}

// ==================== TESTS: Templates ====================

#[test]
fn test_create_preset_fields() {
    let fields = create_preset_fields();
    
    assert!(fields.len() > 0);
    
    // Vérifier que tous les champs ont des positions valides
    for field in &fields {
        assert!(field.pos.0 >= 1);
        assert!(field.pos.1 >= 1);
        assert!(field.length >= 1);
    }
}

#[test]
fn test_create_default_map() {
    let map = create_default_map("TEMPLATE", "DEFAULT");
    
    assert_eq!(map.name, "TEMPLATE");
    assert_eq!(map.mapset, "DEFAULT");
    assert_eq!(map.size, (24, 80));
    assert!(map.fields.len() > 0);
}

// ==================== TESTS: JSON Serialization ====================

#[test]
fn test_export_import_map_to_json() {
    let mut map = BmsMap::new("JSONTEST", "TESTSET");
    map.size = (25, 80);
    map.fields.push(BmsField {
        name: "TESTFIELD".to_string(),
        field_type: FieldType::Field,
        pos: (5, 10),
        length: 20,
        attrb: vec![FieldAttribute::Norm, FieldAttribute::Intens],
        text_color: Some(Color::Green),
        initial: Some("DEFAULT VALUE".to_string()),
        pic: Some("X(20)".to_string()),
        grp_name: None, ..Default::default()
    });
    
    // Export to JSON
    let json = map.to_json().unwrap();
    assert!(json.contains("JSONTEST"));
    assert!(json.contains("TESTSET"));
    assert!(json.contains("TESTFIELD"));
    
    // Import from JSON
    let imported_map = BmsMap::from_json(&json).unwrap();
    assert_eq!(imported_map.name, "JSONTEST");
    assert_eq!(imported_map.mapset, "TESTSET");
    assert_eq!(imported_map.size, (25, 80));
    assert_eq!(imported_map.fields.len(), 1);
    assert_eq!(imported_map.fields[0].name, "TESTFIELD");
    assert_eq!(imported_map.fields[0].pos, (5, 10));
    assert_eq!(imported_map.fields[0].length, 20);
}

#[test]
fn test_export_import_editor_to_json() {
    let mut editor = BmsEditor::new();
    editor.new_map("EDITORTEST", "MYSET", (30, 100));
    editor.add_field_at_cursor(15);
    editor.set_cursor((10, 20));
    
    // Export to JSON
    let json = editor.export_to_json().unwrap();
    assert!(json.contains("EDITORTEST"));
    assert!(json.contains("MYSET"));
    
    // Import from JSON
    let mut new_editor = BmsEditor::new();
    new_editor.import_from_json(&json).unwrap();
    assert_eq!(new_editor.map.name, "EDITORTEST");
    assert_eq!(new_editor.map.mapset, "MYSET");
    assert_eq!(new_editor.map.size, (30, 100));
    assert_eq!(new_editor.map.fields.len(), 1);
}

#[test]
fn test_export_import_editor_full_state() {
    let mut editor = BmsEditor::new();
    editor.new_map("FULLTEST", "FULLSET", (25, 80));
    editor.add_field_at_cursor(10);
    editor.select_next_field();
    editor.set_cursor((5, 5));
    
    // Export full editor state
    let json = editor.export_editor_to_json().unwrap();
    
    // Import full editor state
    let imported_editor = BmsEditor::import_editor_from_json(&json).unwrap();
    assert_eq!(imported_editor.map.name, "FULLTEST");
    assert_eq!(imported_editor.cursor_pos, (5, 5));
    assert_eq!(imported_editor.map.fields.len(), 1);
}

#[test]
fn test_mapset_json_serialization() {
    let mut mapset = BmsMapSet::new("TESTSET");
    
    let mut map1 = BmsMap::new("MAP1", "TESTSET");
    map1.fields.push(BmsField {
        name: "FIELD1".to_string(),
        field_type: FieldType::Field,
        pos: (1, 1),
        length: 10,
        attrb: vec![FieldAttribute::Norm],
        text_color: None,
        initial: None,
        pic: None,
        grp_name: None, ..Default::default()
    });
    
    mapset.maps.insert("MAP1".to_string(), map1);
    
    // Export to JSON
    let json = mapset.to_json().unwrap();
    assert!(json.contains("TESTSET"));
    assert!(json.contains("MAP1"));
    
    // Import from JSON
    let imported_mapset = BmsMapSet::from_json(&json).unwrap();
    assert_eq!(imported_mapset.name, "TESTSET");
    assert_eq!(imported_mapset.maps.len(), 1);
    assert!(imported_mapset.maps.contains_key("MAP1"));
}

// ==================== TESTS: Validation ====================

#[test]
fn test_validate_empty_map() {
    let map = BmsMap::new("TEST", "TESTSET");
    let errors = map.validate();
    assert_eq!(errors.len(), 0); // Empty map is valid
}

#[test]
fn test_validate_field_out_of_bounds() {
    let mut map = BmsMap::new("TEST", "TESTSET");
    map.size = (24, 80);
    
    // Add field that exceeds column limit
    let mut field = BmsField::default();
    field.pos = (1, 75);
    field.length = 10; // Would go to column 84, but map is only 80 wide
    map.fields.push(field);
    
    let errors = map.validate();
    assert!(errors.len() > 0);
    assert!(errors.iter().any(|e| e.contains("extends beyond")));
}

#[test]
fn test_validate_overlapping_fields() {
    let mut map = BmsMap::new("TEST", "TESTSET");
    map.size = (24, 80);
    
    // Add two fields on the same row that overlap
    let mut field1 = BmsField::default();
    field1.pos = (1, 1);
    field1.length = 10;
    map.fields.push(field1);
    
    let mut field2 = BmsField::default();
    field2.pos = (1, 5);
    field2.length = 10; // Overlaps with field1
    map.fields.push(field2);
    
    let errors = map.validate();
    assert!(errors.len() > 0);
    assert!(errors.iter().any(|e| e.contains("overlap")));
}

#[test]
fn test_validate_zero_position() {
    let mut map = BmsMap::new("TEST", "TESTSET");
    map.size = (24, 80);
    
    // Add field with zero position
    let mut field = BmsField::default();
    field.pos = (0, 1); // Row 0 is invalid
    field.length = 10;
    map.fields.push(field);
    
    let errors = map.validate();
    assert!(errors.len() > 0);
    assert!(errors.iter().any(|e| e.contains("Position must be greater than 0")));
}

#[test]
fn test_validate_zero_length() {
    let mut map = BmsMap::new("TEST", "TESTSET");
    map.size = (24, 80);
    
    // Add field with zero length
    let mut field = BmsField::default();
    field.pos = (1, 1);
    field.length = 0;
    map.fields.push(field);
    
    let errors = map.validate();
    assert!(errors.len() > 0);
    assert!(errors.iter().any(|e| e.contains("Length must be greater than 0")));
}

#[test]
fn test_is_valid_field_position() {
    let mut map = BmsMap::new("TEST", "TESTSET");
    map.size = (24, 80);
    
    // Valid position
    assert!(map.is_valid_field_position((1, 1), 10));
    
    // Invalid: out of bounds
    assert!(!map.is_valid_field_position((25, 1), 10)); // Row 25 > 24
    assert!(!map.is_valid_field_position((1, 80), 10)); // Would extend beyond column 80
    
    // Invalid: zero values
    assert!(!map.is_valid_field_position((0, 1), 10));
    assert!(!map.is_valid_field_position((1, 0), 10));
    assert!(!map.is_valid_field_position((1, 1), 0));
    
    // Add a field and test overlap
    let mut field = BmsField::default();
    field.pos = (1, 1);
    field.length = 10;
    map.fields.push(field);
    
    // Overlapping position
    assert!(!map.is_valid_field_position((1, 5), 10));
    
    // Non-overlapping position
    assert!(map.is_valid_field_position((1, 15), 10));
}

// ==================== TESTS: Multi-Selection ====================

#[test]
fn test_select_all_fields() {
    let mut editor = BmsEditor::new();
    
    // Add 3 fields
    editor.add_field(BmsField {
        name: "FIELD1".to_string(),
        pos: (1, 1),
        length: 10,
        ..Default::default()
    });
    editor.add_field(BmsField {
        name: "FIELD2".to_string(),
        pos: (2, 1),
        length: 10,
        ..Default::default()
    });
    editor.add_field(BmsField {
        name: "FIELD3".to_string(),
        pos: (3, 1),
        length: 10,
        ..Default::default()
    });
    
    editor.select_all_fields();
    
    assert_eq!(editor.selected_fields.len(), 3);
    assert_eq!(editor.selected_field, Some(0));
    assert_eq!(editor.selected_count(), 3);
}

#[test]
fn test_selected_count() {
    let mut editor = BmsEditor::new();
    
    // No selection
    assert_eq!(editor.selected_count(), 0);
    
    // Add a field and select it
    let idx = editor.add_field(BmsField {
        name: "FIELD1".to_string(),
        pos: (1, 1),
        length: 10,
        ..Default::default()
    });
    editor.select_field(idx);
    assert_eq!(editor.selected_count(), 1);
    
    // Multi-select
    editor.add_field(BmsField {
        name: "FIELD2".to_string(),
        pos: (2, 1),
        length: 10,
        ..Default::default()
    });
    editor.toggle_field_selection(1);
    assert_eq!(editor.selected_count(), 2);
}

#[test]
fn test_extend_selection_to() {
    let mut editor = BmsEditor::new();
    
    // Add 3 fields
    let idx0 = editor.add_field(BmsField {
        name: "FIELD1".to_string(),
        pos: (1, 1),
        length: 10,
        ..Default::default()
    });
    let idx1 = editor.add_field(BmsField {
        name: "FIELD2".to_string(),
        pos: (2, 1),
        length: 10,
        ..Default::default()
    });
    let idx2 = editor.add_field(BmsField {
        name: "FIELD3".to_string(),
        pos: (3, 1),
        length: 10,
        ..Default::default()
    });
    
    // Select first field
    editor.select_field(idx0);
    assert_eq!(editor.selected_count(), 1);
    
    // Extend to third field
    editor.extend_selection_to(idx2);
    assert_eq!(editor.selected_fields.len(), 3);
    assert!(editor.selected_fields.contains(&idx0));
    assert!(editor.selected_fields.contains(&idx1));
    assert!(editor.selected_fields.contains(&idx2));
}

#[test]
fn test_select_range() {
    let mut editor = BmsEditor::new();
    
    // Add 5 fields
    for i in 0..5 {
        editor.add_field(BmsField {
            name: format!("FIELD{}", i+1),
            pos: ((i+1) as u16, 1),
            length: 10,
            ..Default::default()
        });
    }
    
    // Select range from 1 to 3
    editor.select_range(1, 3);
    
    assert_eq!(editor.selected_fields.len(), 3);
    assert!(editor.selected_fields.contains(&1));
    assert!(editor.selected_fields.contains(&2));
    assert!(editor.selected_fields.contains(&3));
    assert!(!editor.selected_fields.contains(&0));
    assert!(!editor.selected_fields.contains(&4));
}

#[test]
fn test_field_at() {
    let mut editor = BmsEditor::new();
    
    // Add a field at (5, 10) with length 5
    editor.add_field(BmsField {
        name: "FIELD1".to_string(),
        pos: (5, 10),
        length: 5,
        ..Default::default()
    });
    
    // Field covers columns 10-14 on row 5
    assert_eq!(editor.field_at((5, 10)), Some(0));
    assert_eq!(editor.field_at((5, 12)), Some(0));
    assert_eq!(editor.field_at((5, 14)), Some(0));
    assert_eq!(editor.field_at((5, 9)), None);
    assert_eq!(editor.field_at((5, 15)), None);
    assert_eq!(editor.field_at((6, 10)), None);
}

// ==================== TESTS: Grid Snap ====================

#[test]
fn test_snap_to_grid() {
    let mut editor = BmsEditor::new();
    
    // Initially disabled - snapping should return original position
    assert!(!editor.is_snap_to_grid_enabled());
    assert_eq!(editor.get_grid_size(), 1);
    assert_eq!(editor.snap_to_grid((3, 3)), (3, 3));
    
    // Enable with grid size 5
    editor.enable_snap_to_grid(5);
    assert!(editor.is_snap_to_grid_enabled());
    assert_eq!(editor.get_grid_size(), 5);
    
    // Test snapping (rounds to nearest multiple of grid size)
    // Positions are 1-indexed, so 1-2 snap to 1, 3-7 snap to 5, 8-12 snap to 10, etc.
    assert_eq!(editor.snap_to_grid((1, 1)), (1, 1));  // 1 is closest to 1 (0 would be 0, but we clamp to 1)
    assert_eq!(editor.snap_to_grid((2, 2)), (1, 1));  // 2 is closer to 1 than to 5
    assert_eq!(editor.snap_to_grid((3, 3)), (5, 5));  // 3 is closer to 5 than to 1
    assert_eq!(editor.snap_to_grid((7, 7)), (5, 5));  // 7 is closer to 5 than to 10
    assert_eq!(editor.snap_to_grid((8, 8)), (10, 10)); // 8 is closer to 10 than to 5
    assert_eq!(editor.snap_to_grid((5, 5)), (5, 5));  // 5 stays 5
    
    // Disable
    editor.disable_snap_to_grid();
    assert!(!editor.is_snap_to_grid_enabled());
    assert_eq!(editor.snap_to_grid((3, 3)), (3, 3));
}

#[test]
fn test_align_selected_to_grid() {
    let mut editor = BmsEditor::new();
    
    // Add fields at non-grid positions
    editor.add_field(BmsField {
        name: "FIELD1".to_string(),
        pos: (2, 3),
        length: 10,
        ..Default::default()
    });
    editor.add_field(BmsField {
        name: "FIELD2".to_string(),
        pos: (7, 12),
        length: 10,
        ..Default::default()
    });
    
    // Select both fields
    editor.select_all_fields();
    
    // Enable grid snap with size 5
    editor.enable_snap_to_grid(5);
    
    // Align to grid
    let count = editor.align_selected_to_grid();
    assert_eq!(count, 2);
    
    // Check that fields are now aligned to grid
    // (2, 3) with grid 5: row 2 -> 1 (2/5=0.4 rounds to 0, *5=0, clamp to 1), col 3 -> 5 (3/5=0.6 rounds to 1, *5=5)
    // (7, 12) with grid 5: row 7 -> 5 (7/5=1.4 rounds to 1, *5=5), col 12 -> 10 (12/5=2.4 rounds to 2, *5=10)
    assert_eq!(editor.map.fields[0].pos, (1, 5));
    assert_eq!(editor.map.fields[1].pos, (5, 10));
}

#[test]
fn test_set_grid_size() {
    let mut editor = BmsEditor::new();
    
    editor.set_grid_size(10);
    assert_eq!(editor.get_grid_size(), 10);
    
    // Grid size cannot be 0
    editor.set_grid_size(0);
    assert_eq!(editor.get_grid_size(), 1);
}

#[test]
fn test_toggle_snap_to_grid() {
    let mut editor = BmsEditor::new();
    
    assert!(!editor.is_snap_to_grid_enabled());
    
    editor.toggle_snap_to_grid();
    assert!(editor.is_snap_to_grid_enabled());
    
    editor.toggle_snap_to_grid();
    assert!(!editor.is_snap_to_grid_enabled());
}
