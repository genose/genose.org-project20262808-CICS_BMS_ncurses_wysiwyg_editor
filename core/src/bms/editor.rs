//! Module pour l'edition WYSIWYG des maps BMS
//! 
//! Ce module fournit les fonctionnalites pour:
//! - Creer des maps BMS depuis zero
//! - Ajouter/supprimer/modifier des champs
//! - Deplacer/redimensionner des champs
//! - Gerer les attributs (couleurs, types, etc.)
//! - Exporter/Importer au format JSON

use crate::bms::model::*;
use std::cmp::{max, min};
use serde::{Serialize, Deserialize};

/// Represente une operation d'edition (pour undo/redo)
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmsEditor {
    pub map: BmsMap,
    pub selected_field: Option<usize>,
    pub selected_fields: Vec<usize>,
    pub cursor_pos: (u16, u16),
    pub drag_start: Option<(u16, u16)>,
    pub mode: EditorMode,
    pub history: EditHistory,
    pub clipboard: Vec<BmsField>,
    /// Grid size for snap-to-grid functionality (0 = no grid snapping)
    pub grid_size: u16,
    /// Snap to grid during field operations
    pub snap_to_grid: bool,
}

/// Mode de l'editeur
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
            selected_fields: Vec::new(),
            cursor_pos: (1, 1),
            drag_start: None,
            mode: EditorMode::Navigate,
            history: EditHistory::new(100),
            clipboard: Vec::new(),
            grid_size: 1, // Default grid size of 1 (no snapping if snap_to_grid is false)
            snap_to_grid: false,
        }
    }
    
    /// Creer un nouvel editeur a partir d'une map existante
    pub fn from_map(map: BmsMap) -> Self {
        Self {
            map,
            selected_field: None,
            selected_fields: Vec::new(),
            cursor_pos: (1, 1),
            drag_start: None,
            mode: EditorMode::Navigate,
            history: EditHistory::new(100),
            clipboard: Vec::new(),
            grid_size: 1,
            snap_to_grid: false,
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
        self.map.fields.push(field.clone());
        self.history.push(EditOperation::AddField { field: field.clone(), index });
        self.selected_field = Some(index);
        index
    }
    
    /// Ajouter un champ avec position et taille par defaut
    pub fn add_field_at_cursor(&mut self, length: u16) -> usize {
        let mut field = BmsField::default();
        field.name = format!("FIELD{}", self.map.fields.len() + 1);
        field.field_type = FieldType::Field;
        field.pos = self.cursor_pos;
        field.length = length;
        field.attrb = vec![FieldAttribute::Norm];
        field.text_color = Some(Color::Yellow);
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
            let field_type = self.map.fields[index].field_type.clone();
            let field_name = self.map.fields[index].name.clone();
            
            // Calculate the offset
            let offset_row = new_pos.0 as i32 - old_pos.0 as i32;
            let offset_col = new_pos.1 as i32 - old_pos.1 as i32;
            
            // If this is a Fieldset (Group), move all child fields with matching grp_name
            if field_type == FieldType::Group {
                for (child_idx, child_field) in self.map.fields.iter_mut().enumerate() {
                    if child_idx != index && child_field.grp_name.as_ref() == Some(&field_name) {
                        let old_child_pos = child_field.pos;
                        let new_child_pos = (
                            (old_child_pos.0 as i32 + offset_row).max(1) as u16,
                            (old_child_pos.1 as i32 + offset_col).max(1) as u16,
                        );
                        child_field.pos = new_child_pos;
                        // Record history for each child move
                        self.history.push(EditOperation::MoveField { 
                            field_index: child_idx, 
                            old_pos: old_child_pos, 
                            new_pos: new_child_pos 
                        });
                    }
                }
            }
            
            // Move the selected field
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
    
    /// Trouver le champ a la position donnee (sans selectionner)
    pub fn field_at(&self, pos: (u16, u16)) -> Option<usize> {
        for (idx, field) in self.map.fields.iter().enumerate().rev() {
            let (row, col) = field.pos;
            let end_col = col + field.length - 1;
            
            if pos.0 == row && pos.1 >= col && pos.1 <= end_col {
                return Some(idx);
            }
        }
        None
    }
    
    /// Selectionner le champ a la position donnee
    pub fn select_field_at(&mut self, pos: (u16, u16)) -> Option<usize> {
        if let Some(idx) = self.field_at(pos) {
            self.selected_field = Some(idx);
            self.selected_fields = vec![idx];
            Some(idx)
        } else {
            self.selected_field = None;
            self.selected_fields.clear();
            None
        }
    }
    
    /// Selectionner le champ suivant
    pub fn select_next_field(&mut self) {
        if self.map.fields.is_empty() {
            self.selected_field = None;
            return;
        }
        
        self.selected_field = Some(
            self.selected_field.map_or(0, |i| (i + 1) % self.map.fields.len())
        );
    }
    
    /// Selectionner le champ precedent
    pub fn select_prev_field(&mut self) {
        if self.map.fields.is_empty() {
            self.selected_field = None;
            return;
        }
        
        self.selected_field = Some(
            self.selected_field.map_or(self.map.fields.len() - 1, |i| {
                if i == 0 { self.map.fields.len() - 1 } else { i - 1 }
            })
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
            self.clipboard = vec![self.map.fields[index].clone()];
        }
    }
    
    /// Couper le champ selectionne
    pub fn cut_selected(&mut self) -> Option<BmsField> {
        self.copy_selected();
        self.remove_selected_field()
    }
    
    /// Coller le presse-papier a la position du curseur
    /// Si plusieurs champs sont dans le clipboard, ils sont colles avec un decalage
    pub fn paste_at_cursor(&mut self) -> Option<usize> {
        if self.clipboard.is_empty() {
            return None;
        }
        
        let clipboard = self.clipboard.clone();
        let first_index = if let Some(first) = clipboard.first() {
            let mut new_field = first.clone();
            new_field.pos = self.cursor_pos;
            Some(self.add_field(new_field))
        } else {
            None
        };
        
        // Paste remaining fields with offset
        for (i, field) in clipboard.iter().skip(1).enumerate() {
            let mut new_field = field.clone();
            new_field.pos = (self.cursor_pos.0 + i as u16 + 1, self.cursor_pos.1);
            self.add_field(new_field);
        }
        
        first_index
    }
    
    /// Copier plusieurs champs selectionnes (par indices)
    pub fn copy_fields(&mut self, indices: &[usize]) {
        self.clipboard = indices.iter()
            .filter_map(|&idx| self.map.fields.get(idx).cloned())
            .collect();
    }
    
    /// Coller plusieurs champs avec un decalage
    pub fn paste_fields_at(&mut self, pos: (u16, u16)) -> usize {
        let clipboard = self.clipboard.clone();
        for (i, field) in clipboard.iter().enumerate() {
            let mut new_field = field.clone();
            new_field.pos = (pos.0 + i as u16, pos.1);
            self.add_field(new_field);
        }
        clipboard.len()
    }
    
    /// Effacer le presse-papier
    pub fn clear_clipboard(&mut self) {
        self.clipboard.clear();
    }
    
    /// Nombre d'elements dans le presse-papier
    pub fn clipboard_count(&self) -> usize {
        self.clipboard.len()
    }
    
    /// Selectionner un champ (remplace la selection actuelle)
    pub fn select_field(&mut self, index: usize) {
        self.selected_field = Some(index);
        self.selected_fields = vec![index];
    }
    
    /// Ajouter un champ a la selection multiple
    pub fn toggle_field_selection(&mut self, index: usize) {
        if let Some(pos) = self.selected_fields.iter().position(|&x| x == index) {
            self.selected_fields.remove(pos);
            if self.selected_fields.is_empty() {
                self.selected_field = None;
            } else {
                self.selected_field = Some(self.selected_fields[0]);
            }
        } else {
            self.selected_fields.push(index);
            self.selected_field = Some(index);
        }
    }
    
    /// Selectionner tous les champs
    pub fn select_all_fields(&mut self) {
        self.selected_fields = (0..self.map.fields.len()).collect();
        self.selected_field = self.selected_fields.first().copied();
    }
    
    /// Deslectionner tous les champs
    pub fn clear_selection(&mut self) {
        self.selected_field = None;
        self.selected_fields.clear();
    }
    
    /// Copier les champs selectionnes
    pub fn copy_selected_fields(&mut self) {
        if !self.selected_fields.is_empty() {
            let fields = self.selected_fields.clone();
            self.copy_fields(&fields);
        } else if let Some(_idx) = self.selected_field {
            self.copy_selected();
        }
    }
    
    /// Nombre de champs selectionnes
    pub fn selected_count(&self) -> usize {
        if !self.selected_fields.is_empty() {
            self.selected_fields.len()
        } else if self.selected_field.is_some() {
            1
        } else {
            0
        }
    }
    
    /// Obtenir les indices des champs selectionnes
    pub fn get_selected_indices(&self) -> Vec<usize> {
        if !self.selected_fields.is_empty() {
            self.selected_fields.clone()
        } else if let Some(idx) = self.selected_field {
            vec![idx]
        } else {
            Vec::new()
        }
    }
    
    /// Snap a position to the grid
    /// Uses rounding to nearest grid point, ensuring positions are at least 1
    pub fn snap_to_grid(&self, pos: (u16, u16)) -> (u16, u16) {
        if !self.snap_to_grid || self.grid_size == 0 || self.grid_size == 1 {
            return pos;
        }
        
        let grid = self.grid_size as f32;
        let row = ((pos.0 as f32 / grid).round() * grid) as u16;
        let col = ((pos.1 as f32 / grid).round() * grid) as u16;
        
        // Ensure positions are at least 1 (BMS is 1-indexed)
        (row.max(1), col.max(1))
    }
    
    /// Enable snap to grid
    pub fn enable_snap_to_grid(&mut self, grid_size: u16) {
        self.grid_size = grid_size.max(1);
        self.snap_to_grid = true;
    }
    
    /// Disable snap to grid
    pub fn disable_snap_to_grid(&mut self) {
        self.snap_to_grid = false;
    }
    
    /// Toggle snap to grid
    pub fn toggle_snap_to_grid(&mut self) {
        self.snap_to_grid = !self.snap_to_grid;
    }
    
    /// Set grid size
    pub fn set_grid_size(&mut self, size: u16) {
        self.grid_size = size.max(1);
    }
    
    /// Get current grid size
    pub fn get_grid_size(&self) -> u16 {
        self.grid_size
    }
    
    /// Check if snap to grid is enabled
    pub fn is_snap_to_grid_enabled(&self) -> bool {
        self.snap_to_grid
    }
    
    /// Align selected fields to a grid
    pub fn align_selected_to_grid(&mut self) -> usize {
        if !self.snap_to_grid || self.grid_size == 0 {
            return 0;
        }
        
        let indices = self.get_selected_indices();
        for idx in &indices {
            let old_field = self.map.fields[*idx].clone();
            let snapped_pos = self.snap_to_grid(self.map.fields[*idx].pos);
            self.map.fields[*idx].pos = snapped_pos;
            self.history.push(EditOperation::MoveField {
                field_index: *idx,
                old_pos: old_field.pos,
                new_pos: snapped_pos,
            });
        }
        indices.len()
    }
    
    /// Selectionner une plage de champs de start a end (inclus)
    pub fn select_range(&mut self, start: usize, end: usize) {
        let start = min(start, end);
        let end = max(start, end);
        self.selected_fields = (start..=end).collect();
        self.selected_field = if !self.selected_fields.is_empty() {
            Some(self.selected_fields[0])
        } else {
            None
        };
    }
    
    /// Etendre la selection jusqu'a un index
    pub fn extend_selection_to(&mut self, index: usize) {
        if let Some(anchor) = self.selected_fields.first().copied() {
            self.select_range(anchor, index);
        } else if let Some(anchor) = self.selected_field {
            self.selected_fields = vec![anchor];
            self.select_range(anchor, index);
        } else {
            self.select_field(index);
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
            let old_color = self.map.fields[index].text_color.clone();
            self.map.fields[index].text_color = color.clone();
            self.history.push(EditOperation::ChangeColor { field_index: index, old_color, new_color: color });
        }
    }
    
    /// Modifier les attributs du champ selectionne
    pub fn set_selected_field_attributes(&mut self, attrs: Vec<FieldAttribute>) {
        if let Some(index) = self.selected_field {
            let old_attrs = std::mem::replace(&mut self.map.fields[index].attrb, attrs.clone());
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
                EditOperation::AddField { field: _, index } => {
                    if index < self.map.fields.len() {
                        self.map.fields.remove(index);
                    }
                    self.selected_field = None;
                }
                EditOperation::RemoveField { field, index } => {
                    self.map.fields.insert(index, field);
                    self.selected_field = Some(index);
                }
                EditOperation::ModifyField { old_field, new_field: _, index } => {
                    if index < self.map.fields.len() {
                        self.map.fields[index] = old_field;
                    }
                }
                EditOperation::MoveField { field_index, old_pos, new_pos: _ } => {
                    if field_index < self.map.fields.len() {
                        self.map.fields[field_index].pos = old_pos;
                    }
                }
                EditOperation::ResizeField { field_index, old_length, new_length: _ } => {
                    if field_index < self.map.fields.len() {
                        self.map.fields[field_index].length = old_length;
                    }
                }
                EditOperation::ChangeColor { field_index, old_color, new_color: _ } => {
                    if field_index < self.map.fields.len() {
                        self.map.fields[field_index].text_color = old_color;
                    }
                }
                EditOperation::ChangeAttributes { field_index, old_attrs, new_attrs: _ } => {
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
                        self.map.fields[field_index].text_color = new_color;
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
        
        if let Some(ref color) = field.text_color {
            line.push_str(&format!(",COLOR={}", color));
        }
        
        if let Some(ref hlight_color) = field.border_color {
            line.push_str(&format!(",HLIGHT={}", hlight_color));
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
    
    /// Exporter la map au format JSON
    pub fn export_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.map)
    }
    
    /// Importer une map depuis du JSON
    pub fn import_from_json(&mut self, json: &str) -> Result<(), serde_json::Error> {
        let map: BmsMap = serde_json::from_str(json)?;
        let old_map = std::mem::replace(&mut self.map, map);
        self.history.push(EditOperation::NewMap { old_map: Some(old_map), new_map: self.map.clone() });
        self.selected_field = None;
        Ok(())
    }
    
    /// Exporter l'editeur complet (incluant l'historique) au format JSON
    pub fn export_editor_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// Importer l'editeur complet depuis du JSON
    pub fn import_editor_from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Direction du curseur
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
            text_color: Some(Color::White),
            initial: Some("NEW BMS SCREEN".to_string()),
            pic: None,
            grp_name: None,
            ..Default::default()
        },
        // Menu option 1
        BmsField {
            name: "OPTION1".to_string(),
            field_type: FieldType::Field,
            pos: (3, 10),
            length: 2,
            attrb: vec![FieldAttribute::Prot],
            text_color: Some(Color::Green),
            initial: Some("1.".to_string()),
            pic: None,
            grp_name: None,
            ..Default::default()
        },
        BmsField {
            name: "OPTION1_TEXT".to_string(),
            field_type: FieldType::Field,
            pos: (3, 13),
            length: 20,
            attrb: vec![FieldAttribute::Prot],
            text_color: Some(Color::Green),
            initial: Some("Option 1".to_string()),
            pic: None,
            grp_name: None,
            ..Default::default()
        },
        // Input field
        BmsField {
            name: "INPUT1".to_string(),
            field_type: FieldType::Field,
            pos: (5, 10),
            length: 20,
            attrb: vec![FieldAttribute::Norm, FieldAttribute::Alph],
            text_color: Some(Color::Yellow),
            initial: None,
            pic: None,
            grp_name: None,
            ..Default::default()
        },
        // Status line
        BmsField {
            name: "STATUS".to_string(),
            field_type: FieldType::Field,
            pos: (23, 1),
            length: 80,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Reverse],
            text_color: Some(Color::Blue),
            initial: Some("F1=Help F3=Exit F12=Save".to_string()),
            pic: None,
            grp_name: None,
            ..Default::default()
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
