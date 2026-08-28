//! Module pour l'edition WYSIWYG des maps BMS
//! 
//! Ce module fournit les fonctionnalites pour:
//! - Creer des maps BMS depuis zero
//! - Ajouter/supprimer/modifier des champs
//! - Deplacer/redimensionner des champs
//! - Gerer les attributs (couleurs, types, etc.)

use crate::bms::model::*;
use std::cmp::{max, min};

/// Represente une operation d'edition (pour undo/redo)
#[derive(Debug, Clone)]
pub enum EditOperation {
    /// Ajout d'un champ
    AddField { field: BmsField, index: usize },
    /// Suppression d'un champ
    RemoveField { field: BmsField, index: usize },
    /// Modification d'un champ (ancien, nouveau)
    ModifyField { old_field: BmsField, new_field: BmsField, index: usize },
    /// Deplacement d'un champ
    MoveField { field_index: usize, old_pos: (u16, u16), new_pos: (u16, u16) },
    /// Redimensionnement d'un champ
    ResizeField { field_index: usize, old_length: u16, new_length: u16 },
    /// Changement de couleur
    ChangeColor { field_index: usize, old_color: Option<Color>, new_color: Option<Color> },
    /// Changement d'attributs
    ChangeAttributes { field_index: usize, old_attrs: Vec<FieldAttribute>, new_attrs: Vec<FieldAttribute> },
    /// Creation d'une nouvelle map
    NewMap { old_map: Option<BmsMap>, new_map: BmsMap },
}

/// Historique des operations pour undo/redo
#[derive(Debug, Default)]
pub struct EditHistory {
    pub undo_stack: Vec<EditOperation>,
    pub redo_stack: Vec<EditOperation>,
    pub max_size: usize,
}

impl EditHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
        }
    }
    
    pub fn push(&mut self, op: EditOperation) {
        self.undo_stack.push(op);
        self.redo_stack.clear();
        
        // Limiter la taille
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }
    
    pub fn undo(&mut self) -> Option<EditOperation> {
        self.undo_stack.pop().map(|op| {
            self.redo_stack.push(op.clone());
            op
        })
    }
    
    pub fn redo(&mut self) -> Option<EditOperation> {
        self.redo_stack.pop().map(|op| {
            self.undo_stack.push(op.clone());
            op
        })
    }
    
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

/// Etat de l'editeur WYSIWYG
#[derive(Debug, Clone)]
pub struct BmsEditor {
    pub map: BmsMap,
    pub selected_field: Option<usize>,
    pub cursor_pos: (u16, u16),
    pub drag_start: Option<(u16, u16)>,
    pub mode: EditorMode,
    pub history: EditHistory,
    pub clipboard: Option<BmsField>,
}

/// Mode de l'editeur
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorMode {
    /// Mode navigation (selection de champs)
    Navigate,
    /// Mode ajout de champ (click pour positionner)
    AddField { field_type: FieldType, default_length: u16 },
    /// Mode deplacement de champ
    MoveField,
    /// Mode redimensionnement de champ
    ResizeField { direction: ResizeDirection },
    /// Mode edition des proprietes
    EditProperties,
    /// Mode selection rectangulaire
    SelectRect,
}

/// Direction de redimensionnement
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResizeDirection {
    Left,
    Right,
    Up,
    Down,
}

impl BmsEditor {
    /// Creer un nouvel editeur avec une map vide
    pub fn new() -> Self {
        Self {
            map: BmsMap::new("NEWMAP", "DEFAULT"),
            selected_field: None,
            cursor_pos: (1, 1),
            drag_start: None,
            mode: EditorMode::Navigate,
            history: EditHistory::new(100),
            clipboard: None,
        }
    }
    
    /// Creer un nouvel editeur a partir d'une map existante
    pub fn from_map(map: BmsMap) -> Self {
        Self {
            map,
            selected_field: None,
            cursor_pos: (1, 1),
            drag_start: None,
            mode: EditorMode::Navigate,
            history: EditHistory::new(100),
            clipboard: None,
        }
    }
    
    /// Creer une nouvelle map vide
    pub fn new_map(&mut self, name: &str, mapset: &str, size: (u16, u16)) {
        let old_map = std::mem::replace(&mut self.map, BmsMap::new(name, mapset));
        self.map.size = size;
        self.history.push(EditOperation::NewMap { old_map: Some(old_map), new_map: self.map.clone() });
        self.selected_field = None;
    }
    
    /// Ajouter un champ a la position du curseur
    pub fn add_field(&mut self, field: BmsField) -> usize {
        let index = self.map.fields.len();
        self.map.fields.push(field);
        self.history.push(EditOperation::AddField { field: field.clone(), index });
        self.selected_field = Some(index);
        index
    }
    
    /// Ajouter un champ avec position et taille par defaut
    pub fn add_field_at_cursor(&mut self, length: u16) -> usize {
        let field = BmsField {
            name: format!("FIELD{}", self.map.fields.len() + 1),
            field_type: FieldType::Field,
            pos: self.cursor_pos,
            length,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Yellow),
            initial: None,
            pic: None,
            grp_name: None,
        };
        self.add_field(field)
    }
    
    /// Supprimer le champ selectionne
    pub fn remove_selected_field(&mut self) -> Option<BmsField> {
        if let Some(index) = self.selected_field {
            let field = self.map.fields.remove(index);
            self.history.push(EditOperation::RemoveField { field: field.clone(), index });
            self.selected_field = None;
            Some(field)
        } else {
            None
        }
    }
    
    /// Deplacer le champ selectionne a une nouvelle position
    pub fn move_selected_field(&mut self, new_pos: (u16, u16)) {
        if let Some(index) = self.selected_field {
            let old_pos = self.map.fields[index].pos;
            self.map.fields[index].pos = new_pos;
            self.history.push(EditOperation::MoveField { field_index: index, old_pos, new_pos });
        }
    }
    
    /// Redimensionner le champ selectionne
    pub fn resize_selected_field(&mut self, new_length: u16) {
        if let Some(index) = self.selected_field {
            let old_length = self.map.fields[index].length;
            self.map.fields[index].length = new_length;
            self.history.push(EditOperation::ResizeField { field_index: index, old_length, new_length });
        }
    }
    
    /// Selectionner le champ a la position donnee
    pub fn select_field_at(&mut self, pos: (u16, u16)) -> Option<usize> {
        for (idx, field) in self.map.fields.iter().enumerate().rev() {
            let (row, col) = field.pos;
            let end_col = col + field.length - 1;
            
            if pos.0 == row && pos.1 >= col && pos.1 <= end_col {
                self.selected_field = Some(idx);
                return Some(idx);
            }
        }
        self.selected_field = None;
        None
    }
    
    /// Selectionner le champ suivant
    pub fn select_next_field(&mut self) {
        if self.map.fields.is_empty() {
            self.selected_field = None;
            return;
        }
        
        self.selected_field = Some(
            (self.selected_field.map_or(0, |i| (i + 1) % self.map.fields.len()))
        );
    }
    
    /// Selectionner le champ precedent
    pub fn select_prev_field(&mut self) {
        if self.map.fields.is_empty() {
            self.selected_field = None;
            return;
        }
        
        self.selected_field = Some(
            (self.selected_field.map_or(0, |i| {
                if i == 0 { self.map.fields.len() - 1 } else { i - 1 }
            }))
        );
    }
    
    /// Deplacer le curseur
    pub fn move_cursor(&mut self, direction: CursorDirection, step: u16) {
        let (mut row, mut col) = self.cursor_pos;
        
        match direction {
            CursorDirection::Up => row = row.saturating_sub(step),
            CursorDirection::Down => row = min(row + step, self.map.size.0),
            CursorDirection::Left => col = col.saturating_sub(step),
            CursorDirection::Right => col = min(col + step, self.map.size.1),
        }
        
        self.cursor_pos = (max(row, 1), max(col, 1));
    }
    
    /// Deplacer le curseur a une position absolue
    pub fn set_cursor(&mut self, pos: (u16, u16)) {
        self.cursor_pos = (
            min(max(pos.0, 1), self.map.size.0),
            min(max(pos.1, 1), self.map.size.1)
        );
    }
    
    /// Copier le champ selectionne dans le presse-papier
    pub fn copy_selected(&mut self) {
        if let Some(index) = self.selected_field {
            self.clipboard = Some(self.map.fields[index].clone());
        }
    }
    
    /// Couper le champ selectionne
    pub fn cut_selected(&mut self) -> Option<BmsField> {
        self.copy_selected();
        self.remove_selected_field()
    }
    
    /// Coller le presse-papier a la position du curseur
    pub fn paste_at_cursor(&mut self) -> Option<usize> {
        if let Some(field) = self.clipboard.clone() {
            let mut new_field = field;
            new_field.pos = self.cursor_pos;
            Some(self.add_field(new_field))
        } else {
            None
        }
    }
    
    /// Modifier le nom du champ selectionne
    pub fn set_selected_field_name(&mut self, name: &str) {
        if let Some(index) = self.selected_field {
            let old_field = self.map.fields[index].clone();
            self.map.fields[index].name = name.to_string();
            self.history.push(EditOperation::ModifyField {
                old_field,
                new_field: self.map.fields[index].clone(),
                index,
            });
        }
    }
    
    /// Modifier la couleur du champ selectionne
    pub fn set_selected_field_color(&mut self, color: Option<Color>) {
        if let Some(index) = self.selected_field {
            let old_color = self.map.fields[index].color;
            self.map.fields[index].color = color;
            self.history.push(EditOperation::ChangeColor { field_index: index, old_color, new_color: color });
        }
    }
    
    /// Modifier les attributs du champ selectionne
    pub fn set_selected_field_attributes(&mut self, attrs: Vec<FieldAttribute>) {
        if let Some(index) = self.selected_field {
            let old_attrs = std::mem::replace(&mut self.map.fields[index].attrb, attrs);
            self.history.push(EditOperation::ChangeAttributes { field_index: index, old_attrs, new_attrs: attrs });
        }
    }
    
    /// Ajouter un attribut au champ selectionne
    pub fn add_selected_field_attribute(&mut self, attr: FieldAttribute) {
        if let Some(index) = self.selected_field {
            let old_attrs = self.map.fields[index].attrb.clone();
            self.map.fields[index].attrb.push(attr);
            self.history.push(EditOperation::ChangeAttributes {
                field_index: index,
                old_attrs,
                new_attrs: self.map.fields[index].attrb.clone(),
            });
        }
    }
    
    /// Retirer un attribut du champ selectionne
    pub fn remove_selected_field_attribute(&mut self, attr: &FieldAttribute) {
        if let Some(index) = self.selected_field {
            let old_attrs = self.map.fields[index].attrb.clone();
            self.map.fields[index].attrb.retain(|a| !std::mem::discriminant(a).eq(&std::mem::discriminant(attr)));
            self.history.push(EditOperation::ChangeAttributes {
                field_index: index,
                old_attrs,
                new_attrs: self.map.fields[index].attrb.clone(),
            });
        }
    }
    
    /// Changer le type du champ selectionne
    pub fn set_selected_field_type(&mut self, field_type: FieldType) {
        if let Some(index) = self.selected_field {
            let old_field = self.map.fields[index].clone();
            self.map.fields[index].field_type = field_type;
            self.history.push(EditOperation::ModifyField {
                old_field,
                new_field: self.map.fields[index].clone(),
                index,
            });
        }
    }
    
    /// Changer la valeur INITIAL du champ selectionne
    pub fn set_selected_field_initial(&mut self, initial: Option<&str>) {
        if let Some(index) = self.selected_field {
            let old_field = self.map.fields[index].clone();
            self.map.fields[index].initial = initial.map(|s| s.to_string());
            self.history.push(EditOperation::ModifyField {
                old_field,
                new_field: self.map.fields[index].clone(),
                index,
            });
        }
    }
    
    /// Changer la valeur PIC du champ selectionne
    pub fn set_selected_field_pic(&mut self, pic: Option<&str>) {
        if let Some(index) = self.selected_field {
            let old_field = self.map.fields[index].clone();
            self.map.fields[index].pic = pic.map(|s| s.to_string());
            self.history.push(EditOperation::ModifyField {
                old_field,
                new_field: self.map.fields[index].clone(),
                index,
            });
        }
    }
    
    /// Undo la derniere operation
    pub fn undo(&mut self) {
        if let Some(op) = self.history.undo() {
            match op {
                EditOperation::AddField { field, index } => {
                    if index < self.map.fields.len() {
                        self.map.fields.remove(index);
                    }
                    self.selected_field = None;
                }
                EditOperation::RemoveField { field, index } => {
                    self.map.fields.insert(index, field);
                    self.selected_field = Some(index);
                }
                EditOperation::ModifyField { old_field, new_field, index } => {
                    if index < self.map.fields.len() {
                        self.map.fields[index] = old_field;
                    }
                }
                EditOperation::MoveField { field_index, old_pos, new_pos } => {
                    if field_index < self.map.fields.len() {
                        self.map.fields[field_index].pos = old_pos;
                    }
                }
                EditOperation::ResizeField { field_index, old_length, new_length } => {
                    if field_index < self.map.fields.len() {
                        self.map.fields[field_index].length = old_length;
                    }
                }
                EditOperation::ChangeColor { field_index, old_color, new_color } => {
                    if field_index < self.map.fields.len() {
                        self.map.fields[field_index].color = old_color;
                    }
                }
                EditOperation::ChangeAttributes { field_index, old_attrs, new_attrs } => {
                    if field_index < self.map.fields.len() {
                        self.map.fields[field_index].attrb = old_attrs;
                    }
                }
                EditOperation::NewMap { old_map, new_map: _ } => {
                    self.map = old_map.unwrap_or_default();
                    self.selected_field = None;
                }
            }
        }
    }
    
    /// Redo la derniere operation
    pub fn redo(&mut self) {
        if let Some(op) = self.history.redo() {
            match op {
                EditOperation::AddField { field, index } => {
                    self.map.fields.insert(index, field);
                    self.selected_field = Some(index);
                }
                EditOperation::RemoveField { field: _, index } => {
                    if index < self.map.fields.len() {
                        self.map.fields.remove(index);
                    }
                    self.selected_field = None;
                }
                EditOperation::ModifyField { old_field: _, new_field, index } => {
                    if index < self.map.fields.len() {
                        self.map.fields[index] = new_field;
                    }
                }
                EditOperation::MoveField { field_index, old_pos: _, new_pos } => {
                    if field_index < self.map.fields.len() {
                        self.map.fields[field_index].pos = new_pos;
                    }
                }
                EditOperation::ResizeField { field_index, old_length: _, new_length } => {
                    if field_index < self.map.fields.len() {
                        self.map.fields[field_index].length = new_length;
                    }
                }
                EditOperation::ChangeColor { field_index, old_color: _, new_color } => {
                    if field_index < self.map.fields.len() {
                        self.map.fields[field_index].color = new_color;
                    }
                }
                EditOperation::ChangeAttributes { field_index, old_attrs: _, new_attrs } => {
                    if field_index < self.map.fields.len() {
                        self.map.fields[field_index].attrb = new_attrs;
                    }
                }
                EditOperation::NewMap { old_map: _, new_map } => {
                    self.map = new_map;
                    self.selected_field = None;
                }
            }
        }
    }
    
    /// Exporter la map au format BMS
    pub fn export_to_bms(&self) -> String {
        let mut output = String::new();
        
        // DFHMSD
        output.push_str(&format!("DFHMSD TYPE={},MAPSET={}", self.map.name, self.map.mapset));
        if let Some(ref lang) = self.map.language {
            output.push_str(&format!(",LANG={}", lang));
        }
        output.push_str(&format!(",PHYSICAL={}", if self.map.physical { "YES" } else { "NO" }));
        output.push_str("\n");
        
        // DFHMDI
        output.push_str(&format!("DFHMDI SIZE=({},{})\n", self.map.size.0, self.map.size.1));
        
        // DFHMND TYPE=MAP
        output.push_str("DFHMND TYPE=MAP\n");
        
        // Champs
        for field in &self.map.fields {
            output.push_str(&self.export_field(field));
        }
        
        output
    }
    
    /// Exporter un champ au format DFHMND
    fn export_field(&self, field: &BmsField) -> String {
        let mut line = format!("DFHMND POS=({},{}),LENGTH={}", field.pos.0, field.pos.1, field.length);
        
        if !field.attrb.is_empty() {
            let attrs: Vec<String> = field.attrb.iter().map(|a| format!("{}", a)).collect();
            line.push_str(&format!(",ATTRB=({})", attrs.join(",")));
        }
        
        if let Some(ref color) = field.color {
            line.push_str(&format!(",COLOR={}", color));
        }
        
        if let Some(ref initial) = field.initial {
            line.push_str(&format!(",INITIAL='{}'", initial));
        }
        
        if let Some(ref pic) = field.pic {
            line.push_str(&format!(",PIC='{}'", pic));
        }
        
        if !field.name.is_empty() {
            // Note: DFHMND doesn't have a NAME parameter, but we can add a comment
            line.push_str(&format!(" * {}", field.name));
        }
        
        line.push_str("\n");
        line
    }
}

/// Direction du curseur
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Presets de champs pour creation rapide
pub fn create_preset_fields() -> Vec<BmsField> {
    vec![
        // Header
        BmsField {
            name: "TITLE".to_string(),
            field_type: FieldType::Field,
            pos: (1, 25),
            length: 30,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("NEW BMS SCREEN".to_string()),
            pic: None,
            grp_name: None,
        },
        // Menu option 1
        BmsField {
            name: "OPTION1".to_string(),
            field_type: FieldType::Field,
            pos: (3, 10),
            length: 2,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("1.".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "OPTION1_TEXT".to_string(),
            field_type: FieldType::Field,
            pos: (3, 13),
            length: 20,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Option 1".to_string()),
            pic: None,
            grp_name: None,
        },
        // Input field
        BmsField {
            name: "INPUT1".to_string(),
            field_type: FieldType::Field,
            pos: (5, 10),
            length: 20,
            attrb: vec![FieldAttribute::Norm, FieldAttribute::Alph],
            color: Some(Color::Yellow),
            initial: None,
            pic: None,
            grp_name: None,
        },
        // Status line
        BmsField {
            name: "STATUS".to_string(),
            field_type: FieldType::Field,
            pos: (23, 1),
            length: 80,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Reverse],
            color: Some(Color::Blue),
            initial: Some("F1=Help F3=Exit F12=Save".to_string()),
            pic: None,
            grp_name: None,
        },
    ]
}

/// Creer une nouvelle map avec des champs par defaut
pub fn create_default_map(name: &str, mapset: &str) -> BmsMap {
    let mut map = BmsMap::new(name, mapset);
    map.size = (24, 80);
    map.fields = create_preset_fields();
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_editor() {
        let editor = BmsEditor::new();
        assert_eq!(editor.map.name, "NEWMAP");
        assert_eq!(editor.map.fields.len(), 0);
    }
    
    #[test]
    fn test_add_field() {
        let mut editor = BmsEditor::new();
        let index = editor.add_field_at_cursor(10);
        assert_eq!(index, 0);
        assert_eq!(editor.map.fields.len(), 1);
        assert_eq!(editor.map.fields[0].pos, (1, 1));
        assert_eq!(editor.map.fields[0].length, 10);
    }
    
    #[test]
    fn test_remove_field() {
        let mut editor = BmsEditor::new();
        editor.add_field_at_cursor(10);
        assert_eq!(editor.map.fields.len(), 1);
        
        let removed = editor.remove_selected_field();
        assert!(removed.is_some());
        assert_eq!(editor.map.fields.len(), 0);
    }
    
    #[test]
    fn test_export_to_bms() {
        let mut editor = BmsEditor::new();
        editor.new_map("TEST", "TESTSET", (5, 10));
        editor.add_field_at_cursor(5);
        
        let bms = editor.export_to_bms();
        assert!(bms.contains("DFHMSD TYPE=TEST,MAPSET=TESTSET"));
        assert!(bms.contains("DFHMDI SIZE=(5,10)"));
        assert!(bms.contains("DFHMND POS=(1,1),LENGTH=5"));
    }
    
    #[test]
    fn test_undo_redo() {
        let mut editor = BmsEditor::new();
        editor.new_map("TEST", "TESTSET", (5, 10));
        
        // Add field
        editor.add_field_at_cursor(5);
        assert_eq!(editor.map.fields.len(), 1);
        
        // Undo
        editor.undo();
        assert_eq!(editor.map.fields.len(), 0);
        
        // Redo
        editor.redo();
        assert_eq!(editor.map.fields.len(), 1);
    }
}
