//! OBJECTS_DEFINITIONS Properties Handler
//!
//! This module provides property handling using the new OBJECTS_DEFINITIONS system.
//! It serves as a bridge between the old PropertyType system and the new comprehensive
//! property system from OBJECTS_DEFINITIONS.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use ratatui::text::{Line, Span, Text};
use ratatui::style::{Style, Color as TuiColor};
use ratatui::widgets::{Scrollbar, ScrollbarOrientation};

use crate::App;
use crate::types::{get_object_definitions_properties_for_field, to_bms_field_type, InsertableObject};
use cobol_bms_core::bms::objects::{PropertyCategory, GuiPropertyInfo, PropertyValue, GuiFieldType, ControlType};
use cobol_bms_core::model::BmsField;

/// Property navigation state for OBJECTS_DEFINITIONS system
#[derive(Debug, Clone, Default)]
pub struct ObjectDefinitionsPropertyState {
    pub selected_category: Option<PropertyCategory>,
    pub selected_property_index: usize,
    pub scroll_offset: usize,
    pub properties_by_category: std::collections::HashMap<PropertyCategory, Vec<GuiPropertyInfo>>,
    pub flat_properties: Vec<GuiPropertyInfo>,
    pub show_all_categories: bool, // When false, only show essential categories
}

impl ObjectDefinitionsPropertyState {
    /// Create new state for a field
    pub fn new(field: &BmsField) -> Self {
        let gui_properties = get_object_definitions_properties_for_field(field);
        
        // Organize by category
        let mut properties_by_category: std::collections::HashMap<PropertyCategory, Vec<GuiPropertyInfo>> = std::collections::HashMap::new();
        let mut flat_properties = Vec::new();
        
        for prop in gui_properties {
            properties_by_category.entry(prop.category).or_default().push(prop.clone());
            flat_properties.push(prop);
        }
        
        // Set first category as selected if available
        let selected_category = properties_by_category.keys().next().cloned();
        
        Self {
            selected_category,
            selected_property_index: 0,
            scroll_offset: 0,
            properties_by_category,
            flat_properties,
            show_all_categories: false, // Default to showing only essential categories
        }
    }
    
    /// Get current property
    pub fn current_property(&self) -> Option<&GuiPropertyInfo> {
        self.flat_properties.get(self.selected_property_index)
    }
    
    /// Get all categories in order
    pub fn categories(&self) -> Vec<PropertyCategory> {
        let mut categories: Vec<PropertyCategory> = self.properties_by_category.keys().cloned().collect();
        // Sort categories for consistent display
        categories.sort_by(|a, b| {
            // Define category display order
            let order_a = category_display_order(a);
            let order_b = category_display_order(b);
            order_a.cmp(&order_b)
        });
        categories
    }
    
    /// Get properties for current category
    pub fn current_category_properties(&self) -> Option<&Vec<GuiPropertyInfo>> {
        self.selected_category.and_then(|cat| self.properties_by_category.get(&cat))
    }
    
    /// Handle navigation key
    pub fn handle_navigation(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                if self.selected_property_index > 0 {
                    self.selected_property_index -= 1;
                    if self.selected_property_index < self.scroll_offset {
                        self.scroll_offset = self.selected_property_index;
                    }
                }
            }
            KeyCode::Down => {
                if self.selected_property_index + 1 < self.flat_properties.len() {
                    self.selected_property_index += 1;
                    // Scroll if needed
                    let visible_height = 18; // Approximate visible lines
                    if self.selected_property_index >= self.scroll_offset + visible_height {
                        self.scroll_offset = self.selected_property_index.saturating_sub(visible_height - 1);
                    }
                }
            }
            KeyCode::PageUp => {
                if self.scroll_offset > 0 {
                    let step = 10.min(self.scroll_offset);
                    self.scroll_offset -= step;
                    if self.selected_property_index < self.scroll_offset {
                        self.selected_property_index = self.scroll_offset;
                    }
                }
            }
            KeyCode::PageDown => {
                let visible_height = 18;
                let max_scroll = self.flat_properties.len().saturating_sub(visible_height);
                if self.scroll_offset < max_scroll {
                    let step = 10.min(max_scroll - self.scroll_offset);
                    self.scroll_offset += step;
                    if self.selected_property_index < self.scroll_offset {
                        self.selected_property_index = self.scroll_offset;
                    }
                }
            }
            KeyCode::Home => {
                self.selected_property_index = 0;
                self.scroll_offset = 0;
            }
            KeyCode::End => {
                if !self.flat_properties.is_empty() {
                    self.selected_property_index = self.flat_properties.len() - 1;
                    let visible_height = 18;
                    self.scroll_offset = self.selected_property_index.saturating_sub(visible_height - 1);
                }
            }
            _ => {}
        }
    }
}

/// Display order for categories
fn category_display_order(category: &PropertyCategory) -> u8 {
    match category {
        PropertyCategory::Values => 0,      // Name, type at top
        PropertyCategory::Dimensions => 1, // Size next
        PropertyCategory::Position => 2,   // Position
        PropertyCategory::Colors => 3,     // Appearance
        PropertyCategory::Style => 4,      // Text styling
        PropertyCategory::Alignment => 5,  // Alignment
        PropertyCategory::Font => 6,       // Font
        PropertyCategory::Borders => 7,    // Borders
        PropertyCategory::Fill => 8,       // Fill characters
        PropertyCategory::Markers => 9,     // Required/error markers
        PropertyCategory::Attributes => 10, // Field attributes
        PropertyCategory::Children => 11,   // Child fields
        PropertyCategory::Visual => 12,    // Visual representation
        PropertyCategory::Other => 13,     // Other properties
    }
}

/// Handle OBJECTS_DEFINITIONS property mode input
pub fn handle_object_definitions_properties_mode(app: &mut App, key: KeyEvent) {
    if app.edit_properties_field.is_none() {
        return;
    }
    
    // Initialize property state if not exists
    if app.object_definitions_property_state.is_none() {
        if let Some(field) = &app.edit_properties_field {
            app.object_definitions_property_state = Some(ObjectDefinitionsPropertyState::new(field));
        }
    }
    
    if app.edit_properties_field.is_none() || app.object_definitions_property_state.is_none() {
        return;
    }
    
    // We need to handle the borrows carefully to avoid conflicts
    if key.code == KeyCode::Esc {
        app.mode = crate::AppMode::Edit;
        app.edit_properties_field = None;
        app.object_definitions_property_state = None;
        return;
    }
    
    // Handle navigation without borrow conflicts
    if let Some(state) = &mut app.object_definitions_property_state {
        match key.code {
            KeyCode::Up | KeyCode::Down | KeyCode::PageUp | KeyCode::PageDown | KeyCode::Home | KeyCode::End => {
                state.handle_navigation(key.code);
            }
            KeyCode::Enter => {
                // For Enter, we need to handle property editing - get the property first
                if let Some(property) = state.current_property() {
                    let property_name = property.name.clone();
                    let gui_name = property.gui_name.clone();
                    
                    // Drop the state borrow before we modify app
                    let _ = state;
                    
                    // Now we can safely borrow app mutably
                    // We'll use the edit_properties_field directly by index
                    if app.edit_properties_field.is_some() {
                        handle_property_edit(app, &property_name, &gui_name);
                    }
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                // Toggle show all categories
                if let Some(state) = &mut app.object_definitions_property_state {
                    let new_value = !state.show_all_categories;
                    let message = if new_value {
                        "Showing all property categories"
                    } else {
                        "Showing essential categories only"
                    };
                    state.show_all_categories = new_value;
                    app.set_message(message);
                }
            }
            _ => {}
        }
    }
}

/// Handle editing of a specific property
fn handle_property_edit(app: &mut App, property_name: &str, gui_name: &str) {
    // Extract data from the field before borrowing app mutably
    let field_data = match &app.edit_properties_field {
        Some(f) => {
            Some((f.name.clone(), f.length, f.text_color.clone(), f.initial.clone(), f.attrb.clone()))
        }
        None => None,
    };
    
    let (field_name, field_length, field_text_color, field_initial, _field_attrb) = match field_data {
        Some(data) => data,
        None => return,
    };
    
    match property_name {
        "field_name" => {
            app.start_text_input("Enter field name:", &field_name, crate::TextInputAction::SetFieldName);
        }
        "field_pos" => {
            // For position, we could open a position dialog
            app.set_message("Position editing: Use arrow keys to move field");
        }
        "field_width" | "field_length" => {
            app.start_text_input("Enter length:", &field_length.to_string(), crate::TextInputAction::SetFieldLength);
        }
        "field_text_color" => {
            app.mode = crate::AppMode::ColorPicker;
            app.selected_color = field_text_color;
        }
        "field_attrb" => {
            app.mode = crate::AppMode::AttributePicker;
        }
        "field_initial" => {
            let initial = field_initial.unwrap_or_default();
            app.start_text_input("Enter INITIAL value:", &initial, crate::TextInputAction::SetFieldInitial);
        }
        "field_type" => {
            app.set_message("Field type cannot be changed after creation");
        }
        _ => {
            // For other properties, show a generic text input
            app.start_text_input(&format!("Enter {}:", gui_name), "", crate::TextInputAction::Custom(property_name.to_string()));
        }
    }
}

/// Render properties using OBJECTS_DEFINITIONS system
pub fn render_object_definitions_properties_panel(f: &mut Frame, app: &App, area: Rect) {
    let panel_width = (area.width * 3 / 4).min(50).max(40);
    let panel_area = Rect {
        x: (area.width - panel_width) / 2 + area.x,
        y: area.y + 2,
        width: panel_width,
        height: area.height.saturating_sub(4),
    };
    
    let block = Block::default()
        .title(" Edit Properties [↑↓:Nav|Enter:Edit|Esc:Close] ")
        .borders(Borders::ALL);
    f.render_widget(block, panel_area);
    
    let inner = Rect {
        x: panel_area.x + 1,
        y: panel_area.y + 1,
        width: panel_area.width.saturating_sub(2),
        height: panel_area.height.saturating_sub(2),
    };
    
    if let (Some(field), Some(state)) = (&app.edit_properties_field, &app.object_definitions_property_state) {
        let mut lines = Vec::new();
        
        // Show categories
        for category in state.categories() {
            // Only show essential categories by default unless show_all_categories is true
            if !state.show_all_categories {
                let essential_categories = [
                    PropertyCategory::Values,
                    PropertyCategory::Dimensions,
                    PropertyCategory::Position,
                    PropertyCategory::Colors,
                    PropertyCategory::Attributes,
                ];
                if !essential_categories.contains(&category) {
                    continue;
                }
            }
            
            let category_name = format_category_name(&category);
            let is_selected = Some(category) == state.selected_category;
            let style = if is_selected {
                Style::default().fg(TuiColor::Black).bg(TuiColor::Yellow)
            } else {
                Style::default().fg(TuiColor::Cyan)
            };
            
            lines.push(Line::from(Span::styled(category_name, style)));
            
            // Show properties in this category
            if let Some(props) = state.properties_by_category.get(&category) {
                for (i, prop) in props.iter().enumerate() {
                    // Only show editable/important properties
                    let should_show = match prop.name.as_str() {
                        "field_name" | "field_type" | "field_width" | "field_length" | "field_height" | 
                        "field_pos" | "field_text_color" | "field_attrb" | "field_initial" | 
                        "field_border_color" | "field_title_color" | "field_fill_char" | 
                        "field_border_style" => true,
                        _ => false, // Skip less important properties
                    };
                    
                    if !should_show {
                        continue;
                    }
                    
                    // Find global index of this property
                    let global_index = state.flat_properties.iter()
                        .position(|p| p.name == prop.name)
                        .unwrap_or(0);
                    
                    let is_selected = state.selected_property_index == global_index;
                    let style = if is_selected {
                        Style::default().fg(TuiColor::Black).bg(TuiColor::Green)
                    } else {
                        Style::default()
                    };
                    
                    // Get current value
                    let value = get_property_value(field, &prop.name);
                    lines.push(Line::from(Span::styled(format!("    {}: {}", prop.gui_name, value), style)));
                }
            }
            // Only add empty line between categories, not after last one
            if lines.last() != Some(&Line::from("")) {
                lines.push(Line::from(""));
            }
        }
        
        let has_lines = !lines.is_empty();
        let line_count = lines.len();
        let has_scrollbar = line_count > inner.height as usize;
        let text = Text::from(lines);
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::NONE))
            .scroll((state.scroll_offset as u16, 0));
        f.render_widget(paragraph, inner);
        
        // Add scrollbar if content is scrollable
        if has_scrollbar {
            let mut scrollbar_state = ratatui::widgets::ScrollbarState::new(line_count.saturating_sub(inner.height as usize))
                .position(state.scroll_offset);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(None)
                .thumb_symbol("█");
            f.render_stateful_widget(
                scrollbar,
                Rect {
                    x: inner.x + inner.width - 1,
                    y: inner.y,
                    width: 1,
                    height: inner.height,
                },
                &mut scrollbar_state,
            );
        }
        
        // Show navigation help
        if has_lines {
            let help_line = Line::from("[↑↓:Navigate | Enter:Edit | A:Toggle All Categories | Esc:Close]".dim());
            let help_text = Text::from(vec![help_line]);
            let help_paragraph = Paragraph::new(help_text).block(Block::default().borders(Borders::NONE));
            let help_area = Rect {
                x: inner.x,
                y: inner.y + inner.height.saturating_sub(1),
                width: inner.width,
                height: 1,
            };
            f.render_widget(help_paragraph, help_area);
        }
    }
}

/// Get current value for a property from a field
fn get_property_value(field: &BmsField, property_name: &str) -> String {
    match property_name {
        "field_name" => field.name.clone(),
        "field_pos" => format!("({}, {})", field.pos.0, field.pos.1),
        "field_width" | "field_length" => field.length.to_string(),
        "field_text_color" => format!("{:?}", field.text_color),
        "field_attrb" => format!("{:?}", field.attrb),
        "field_type" => format!("{:?}", field.field_type),
        "field_initial" => field.initial.clone().unwrap_or_else(|| "".to_string()),
        _ => "N/A".to_string(),
    }
}

/// Format category name for display
fn format_category_name(category: &PropertyCategory) -> String {
    let name = match category {
        PropertyCategory::Dimensions => "Dimensions",
        PropertyCategory::Colors => "Colors",
        PropertyCategory::Font => "Font",
        PropertyCategory::Style => "Style",
        PropertyCategory::Alignment => "Alignment",
        PropertyCategory::Position => "Position",
        PropertyCategory::Borders => "Borders",
        PropertyCategory::Fill => "Fill",
        PropertyCategory::Markers => "Markers",
        PropertyCategory::Attributes => "Attributes",
        PropertyCategory::Values => "Values",
        PropertyCategory::Children => "Children",
        PropertyCategory::Visual => "Visual",
        PropertyCategory::Other => "Other",
    };
    format!("> {} ", name)
}