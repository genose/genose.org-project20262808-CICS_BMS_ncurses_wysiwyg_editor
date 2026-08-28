//! Module pour les templates BMS predefinis
//!
//! Ce module fournit des templates de maps BMS pour:
//! - Menus
//! - Formulaires de saisie
//! - Listes
//! - Ecrans de confirmation
//! - Ecrans d'erreur
//!
//! Les templates peuvent etre utilises comme point de depart
//! pour la creation rapide d'ecrans BMS.

use crate::bms::model::*;
use serde::{Serialize, Deserialize};

/// Represente un template de map BMS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmsTemplate {
    pub name: &'static str,
    pub description: &'static str,
    pub map_name: String,
    pub mapset: String,
    pub size: (u16, u16),
    pub fields: Vec<BmsField>,
}

impl BmsTemplate {
    /// Creer une BmsMap a partir du template
    pub fn to_map(&self) -> BmsMap {
        let mut map = BmsMap::new(&self.map_name, &self.mapset);
        map.size = self.size;
        map.fields = self.fields.clone();
        map
    }
}

/// Liste de tous les templates disponibles
pub fn get_all_templates() -> Vec<&'static BmsTemplate> {
    vec![
        &TEMPLATE_MENU,
        &TEMPLATE_DATA_ENTRY_FORM,
        &TEMPLATE_LIST,
        &TEMPLATE_CONFIRMATION,
        &TEMPLATE_ERROR,
        &TEMPLATE_LOGIN,
        &TEMPLATE_SEARCH,
        &TEMPLATE_DASHBOARD,
    ]
}

/// Trouver un template par son nom
pub fn get_template_by_name(name: &str) -> Option<&'static BmsTemplate> {
    get_all_templates().into_iter().find(|t| t.name == name)
}

/// Liste des noms de templates
pub fn get_template_names() -> Vec<&'static str> {
    get_all_templates().into_iter().map(|t| t.name).collect()
}

// ==================== TEMPLATES ====================

/// Template: Menu principal
pub static TEMPLATE_MENU: BmsTemplate = BmsTemplate {
    name: "menu",
    description: "Menu principal avec options numerotees",
    map_name: "MENU01".to_string(),
    mapset: "APPSET".to_string(),
    size: (24, 80),
    fields: vec![
        // Titre
        BmsField {
            name: "TITLE".to_string(),
            field_type: FieldType::Field,
            pos: (1, 28),
            length: 24,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("MAIN MENU".to_string()),
            pic: None,
            grp_name: None,
        },
        // Option 1
        BmsField {
            name: "OPTION1_KEY".to_string(),
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
            length: 30,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Customer Maintenance".to_string()),
            pic: None,
            grp_name: None,
        },
        // Option 2
        BmsField {
            name: "OPTION2_KEY".to_string(),
            field_type: FieldType::Field,
            pos: (5, 10),
            length: 2,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("2.".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "OPTION2_TEXT".to_string(),
            field_type: FieldType::Field,
            pos: (5, 13),
            length: 30,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Order Processing".to_string()),
            pic: None,
            grp_name: None,
        },
        // Option 3
        BmsField {
            name: "OPTION3_KEY".to_string(),
            field_type: FieldType::Field,
            pos: (7, 10),
            length: 2,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("3.".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "OPTION3_TEXT".to_string(),
            field_type: FieldType::Field,
            pos: (7, 13),
            length: 30,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Inventory Management".to_string()),
            pic: None,
            grp_name: None,
        },
        // Option 4
        BmsField {
            name: "OPTION4_KEY".to_string(),
            field_type: FieldType::Field,
            pos: (9, 10),
            length: 2,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("4.".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "OPTION4_TEXT".to_string(),
            field_type: FieldType::Field,
            pos: (9, 13),
            length: 30,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Reports".to_string()),
            pic: None,
            grp_name: None,
        },
        // Option 5
        BmsField {
            name: "OPTION5_KEY".to_string(),
            field_type: FieldType::Field,
            pos: (11, 10),
            length: 2,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("5.".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "OPTION5_TEXT".to_string(),
            field_type: FieldType::Field,
            pos: (11, 13),
            length: 30,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Exit".to_string()),
            pic: None,
            grp_name: None,
        },
        // Selection
        BmsField {
            name: "SELECTION".to_string(),
            field_type: FieldType::Field,
            pos: (13, 10),
            length: 1,
            attrb: vec![FieldAttribute::Num, FieldAttribute::Intens],
            color: Some(Color::Yellow),
            initial: None,
            pic: None,
            grp_name: None,
        },
        // Instructions
        BmsField {
            name: "INSTRUCTIONS".to_string(),
            field_type: FieldType::Field,
            pos: (15, 10),
            length: 40,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: Some("Enter selection (1-5) and press ENTER".to_string()),
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
    ],
};

/// Template: Formulaire de saisie de donnees
pub static TEMPLATE_DATA_ENTRY_FORM: BmsTemplate = BmsTemplate {
    name: "data_entry",
    description: "Formulaire de saisie avec labels et champs",
    map_name: "DATAENT".to_string(),
    mapset: "APPSET".to_string(),
    size: (24, 80),
    fields: vec![
        // Header
        BmsField {
            name: "HEADER".to_string(),
            field_type: FieldType::Field,
            pos: (1, 1),
            length: 80,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("DATA ENTRY FORM - PLEASE FILL ALL FIELDS".to_string()),
            pic: None,
            grp_name: None,
        },
        // Customer label
        BmsField {
            name: "CUSTOMER_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (3, 1),
            length: 10,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Customer:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "CUSTOMER_INPUT".to_string(),
            field_type: FieldType::Field,
            pos: (3, 12),
            length: 20,
            attrb: vec![FieldAttribute::Norm, FieldAttribute::Alph],
            color: Some(Color::Yellow),
            initial: None,
            pic: None,
            grp_name: None,
        },
        // Order label
        BmsField {
            name: "ORDER_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (5, 1),
            length: 10,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Order #:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ORDER_INPUT".to_string(),
            field_type: FieldType::Field,
            pos: (5, 12),
            length: 10,
            attrb: vec![FieldAttribute::Norm, FieldAttribute::Num],
            color: Some(Color::Yellow),
            initial: None,
            pic: None,
            grp_name: None,
        },
        // Amount label
        BmsField {
            name: "AMOUNT_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (7, 1),
            length: 10,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Amount:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "AMOUNT_INPUT".to_string(),
            field_type: FieldType::Field,
            pos: (7, 12),
            length: 15,
            attrb: vec![FieldAttribute::Norm, FieldAttribute::Num],
            color: Some(Color::Yellow),
            initial: None,
            pic: Some("9(10)V99".to_string()),
            grp_name: None,
        },
        // Date label
        BmsField {
            name: "DATE_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (9, 1),
            length: 10,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Date:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "DATE_INPUT".to_string(),
            field_type: FieldType::Field,
            pos: (9, 12),
            length: 10,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Yellow),
            initial: None,
            pic: Some("XXXXXXXX".to_string()),
            grp_name: None,
        },
        // Action buttons
        BmsField {
            name: "HELP_BUTTON".to_string(),
            field_type: FieldType::Field,
            pos: (15, 30),
            length: 10,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("[F1=Help]".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "EXIT_BUTTON".to_string(),
            field_type: FieldType::Field,
            pos: (15, 45),
            length: 10,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("[F3=Exit]".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "SAVE_BUTTON".to_string(),
            field_type: FieldType::Field,
            pos: (15, 60),
            length: 10,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("[F12=Save]".to_string()),
            pic: None,
            grp_name: None,
        },
        // Error message area
        BmsField {
            name: "ERROR_AREA".to_string(),
            field_type: FieldType::Field,
            pos: (17, 1),
            length: 80,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Red),
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
    ],
};

/// Template: Liste de donnees
pub static TEMPLATE_LIST: BmsTemplate = BmsTemplate {
    name: "list",
    description: "Liste scrollable de donnees",
    map_name: "LIST01".to_string(),
    mapset: "APPSET".to_string(),
    size: (24, 80),
    fields: vec![
        // Header
        BmsField {
            name: "HEADER".to_string(),
            field_type: FieldType::Field,
            pos: (1, 1),
            length: 80,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("DATA LIST - Use PAGE UP/PAGE DOWN to scroll".to_string()),
            pic: None,
            grp_name: None,
        },
        // Column headers
        BmsField {
            name: "COL_ID".to_string(),
            field_type: FieldType::Field,
            pos: (3, 1),
            length: 8,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("ID".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "COL_NAME".to_string(),
            field_type: FieldType::Field,
            pos: (3, 10),
            length: 20,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("Name".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "COL_VALUE".to_string(),
            field_type: FieldType::Field,
            pos: (3, 35),
            length: 15,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("Value".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "COL_STATUS".to_string(),
            field_type: FieldType::Field,
            pos: (3, 55),
            length: 15,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("Status".to_string()),
            pic: None,
            grp_name: None,
        },
        // Separator line
        BmsField {
            name: "SEPARATOR".to_string(),
            field_type: FieldType::Field,
            pos: (4, 1),
            length: 80,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: Some("-".repeat(80)),
            pic: None,
            grp_name: None,
        },
        // Row 1
        BmsField {
            name: "ROW1_ID".to_string(),
            field_type: FieldType::Field,
            pos: (5, 1),
            length: 8,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: None,
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ROW1_NAME".to_string(),
            field_type: FieldType::Field,
            pos: (5, 10),
            length: 20,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: None,
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ROW1_VALUE".to_string(),
            field_type: FieldType::Field,
            pos: (5, 35),
            length: 15,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: None,
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ROW1_STATUS".to_string(),
            field_type: FieldType::Field,
            pos: (5, 55),
            length: 15,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: None,
            pic: None,
            grp_name: None,
        },
        // More rows...
        BmsField {
            name: "ROW2_ID".to_string(),
            field_type: FieldType::Field,
            pos: (6, 1),
            length: 8,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: None,
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ROW2_NAME".to_string(),
            field_type: FieldType::Field,
            pos: (6, 10),
            length: 20,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: None,
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ROW2_VALUE".to_string(),
            field_type: FieldType::Field,
            pos: (6, 35),
            length: 15,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: None,
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ROW2_STATUS".to_string(),
            field_type: FieldType::Field,
            pos: (6, 55),
            length: 15,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
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
            initial: Some("PAGE UP/DOWN=Scroll F3=Exit".to_string()),
            pic: None,
            grp_name: None,
        },
    ],
};

/// Template: Ecran de confirmation
pub static TEMPLATE_CONFIRMATION: BmsTemplate = BmsTemplate {
    name: "confirmation",
    description: "Ecran de confirmation Yes/No",
    map_name: "CONFIRM".to_string(),
    mapset: "APPSET".to_string(),
    size: (10, 40),
    fields: vec![
        // Message
        BmsField {
            name: "MESSAGE".to_string(),
            field_type: FieldType::Field,
            pos: (2, 1),
            length: 40,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: Some("Are you sure?".to_string()),
            pic: None,
            grp_name: None,
        },
        // Yes button
        BmsField {
            name: "YES_BUTTON".to_string(),
            field_type: FieldType::Field,
            pos: (5, 15),
            length: 5,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Green),
            initial: Some("[YES]".to_string()),
            pic: None,
            grp_name: None,
        },
        // No button
        BmsField {
            name: "NO_BUTTON".to_string(),
            field_type: FieldType::Field,
            pos: (5, 25),
            length: 5,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Red),
            initial: Some("[NO]".to_string()),
            pic: None,
            grp_name: None,
        },
        // Status line
        BmsField {
            name: "STATUS".to_string(),
            field_type: FieldType::Field,
            pos: (9, 1),
            length: 40,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Reverse],
            color: Some(Color::Blue),
            initial: Some("ENTER=Select F3=Cancel".to_string()),
            pic: None,
            grp_name: None,
        },
    ],
};

/// Template: Ecran d'erreur
pub static TEMPLATE_ERROR: BmsTemplate = BmsTemplate {
    name: "error",
    description: "Ecran d'affichage d'erreur",
    map_name: "ERROR01".to_string(),
    mapset: "APPSET".to_string(),
    size: (10, 60),
    fields: vec![
        // Error title
        BmsField {
            name: "ERROR_TITLE".to_string(),
            field_type: FieldType::Field,
            pos: (1, 1),
            length: 60,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::Red),
            initial: Some("ERROR".to_string()),
            pic: None,
            grp_name: None,
        },
        // Error message (multi-line area)
        BmsField {
            name: "ERROR_MSG1".to_string(),
            field_type: FieldType::Field,
            pos: (3, 1),
            length: 60,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: None,
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ERROR_MSG2".to_string(),
            field_type: FieldType::Field,
            pos: (4, 1),
            length: 60,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: None,
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ERROR_MSG3".to_string(),
            field_type: FieldType::Field,
            pos: (5, 1),
            length: 60,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: None,
            pic: None,
            grp_name: None,
        },
        // OK button
        BmsField {
            name: "OK_BUTTON".to_string(),
            field_type: FieldType::Field,
            pos: (7, 25),
            length: 5,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Green),
            initial: Some("[OK]".to_string()),
            pic: None,
            grp_name: None,
        },
        // Status line
        BmsField {
            name: "STATUS".to_string(),
            field_type: FieldType::Field,
            pos: (9, 1),
            length: 60,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Reverse],
            color: Some(Color::Blue),
            initial: Some("ENTER=Continue".to_string()),
            pic: None,
            grp_name: None,
        },
    ],
};

/// Template: Ecran de login
pub static TEMPLATE_LOGIN: BmsTemplate = BmsTemplate {
    name: "login",
    description: "Ecran de connexion utilisateur",
    map_name: "LOGIN".to_string(),
    mapset: "APPSET".to_string(),
    size: (12, 40),
    fields: vec![
        // Title
        BmsField {
            name: "TITLE".to_string(),
            field_type: FieldType::Field,
            pos: (1, 10),
            length: 20,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("LOGIN".to_string()),
            pic: None,
            grp_name: None,
        },
        // User label
        BmsField {
            name: "USER_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (3, 5),
            length: 10,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("User:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "USER_INPUT".to_string(),
            field_type: FieldType::Field,
            pos: (3, 16),
            length: 20,
            attrb: vec![FieldAttribute::Norm, FieldAttribute::Alph],
            color: Some(Color::Yellow),
            initial: None,
            pic: None,
            grp_name: None,
        },
        // Password label
        BmsField {
            name: "PASS_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (5, 5),
            length: 12,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Password:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "PASS_INPUT".to_string(),
            field_type: FieldType::Field,
            pos: (5, 18),
            length: 20,
            attrb: vec![FieldAttribute::Norm, FieldAttribute::Alph],
            color: Some(Color::Yellow),
            initial: None,
            pic: None,
            grp_name: None,
        },
        // Login button
        BmsField {
            name: "LOGIN_BUTTON".to_string(),
            field_type: FieldType::Field,
            pos: (8, 15),
            length: 10,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Green),
            initial: Some("[Login]".to_string()),
            pic: None,
            grp_name: None,
        },
        // Status line
        BmsField {
            name: "STATUS".to_string(),
            field_type: FieldType::Field,
            pos: (11, 1),
            length: 40,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Reverse],
            color: Some(Color::Blue),
            initial: Some("F3=Cancel".to_string()),
            pic: None,
            grp_name: None,
        },
    ],
};

/// Template: Ecran de recherche
pub static TEMPLATE_SEARCH: BmsTemplate = BmsTemplate {
    name: "search",
    description: "Ecran de recherche avec criteres",
    map_name: "SEARCH".to_string(),
    mapset: "APPSET".to_string(),
    size: (15, 80),
    fields: vec![
        // Title
        BmsField {
            name: "TITLE".to_string(),
            field_type: FieldType::Field,
            pos: (1, 25),
            length: 30,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("SEARCH CRITERIA".to_string()),
            pic: None,
            grp_name: None,
        },
        // Search field
        BmsField {
            name: "SEARCH_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (3, 1),
            length: 10,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Search:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "SEARCH_INPUT".to_string(),
            field_type: FieldType::Field,
            pos: (3, 12),
            length: 50,
            attrb: vec![FieldAttribute::Norm, FieldAttribute::Alph],
            color: Some(Color::Yellow),
            initial: None,
            pic: None,
            grp_name: None,
        },
        // Date from
        BmsField {
            name: "DATE_FROM_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (5, 1),
            length: 12,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Date From:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "DATE_FROM_INPUT".to_string(),
            field_type: FieldType::Field,
            pos: (5, 14),
            length: 10,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Yellow),
            initial: None,
            pic: Some("XXXXXXXX".to_string()),
            grp_name: None,
        },
        // Date to
        BmsField {
            name: "DATE_TO_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (5, 30),
            length: 10,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Date To:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "DATE_TO_INPUT".to_string(),
            field_type: FieldType::Field,
            pos: (5, 41),
            length: 10,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Yellow),
            initial: None,
            pic: Some("XXXXXXXX".to_string()),
            grp_name: None,
        },
        // Search button
        BmsField {
            name: "SEARCH_BUTTON".to_string(),
            field_type: FieldType::Field,
            pos: (8, 30),
            length: 10,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Green),
            initial: Some("[Search]".to_string()),
            pic: None,
            grp_name: None,
        },
        // Clear button
        BmsField {
            name: "CLEAR_BUTTON".to_string(),
            field_type: FieldType::Field,
            pos: (8, 45),
            length: 10,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Green),
            initial: Some("[Clear]".to_string()),
            pic: None,
            grp_name: None,
        },
        // Results area
        BmsField {
            name: "RESULTS_AREA".to_string(),
            field_type: FieldType::Field,
            pos: (10, 1),
            length: 80,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: Some("Results will appear here...".to_string()),
            pic: None,
            grp_name: None,
        },
        // Status line
        BmsField {
            name: "STATUS".to_string(),
            field_type: FieldType::Field,
            pos: (14, 1),
            length: 80,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Reverse],
            color: Some(Color::Blue),
            initial: Some("F1=Help F3=Exit F12=Search".to_string()),
            pic: None,
            grp_name: None,
        },
    ],
};

/// Template: Dashboard
pub static TEMPLATE_DASHBOARD: BmsTemplate = BmsTemplate {
    name: "dashboard",
    description: "Tableau de bord avec indicateurs",
    map_name: "DASH01".to_string(),
    mapset: "APPSET".to_string(),
    size: (24, 80),
    fields: vec![
        // Title
        BmsField {
            name: "TITLE".to_string(),
            field_type: FieldType::Field,
            pos: (1, 28),
            length: 24,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("DASHBOARD".to_string()),
            pic: None,
            grp_name: None,
        },
        // Stats section
        BmsField {
            name: "STATS_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (3, 1),
            length: 20,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("Statistics:".to_string()),
            pic: None,
            grp_name: None,
        },
        // Customers
        BmsField {
            name: "CUSTOMERS_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (5, 5),
            length: 15,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Customers:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "CUSTOMERS_VALUE".to_string(),
            field_type: FieldType::Field,
            pos: (5, 21),
            length: 10,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: Some("000000".to_string()),
            pic: None,
            grp_name: None,
        },
        // Orders
        BmsField {
            name: "ORDERS_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (7, 5),
            length: 15,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Orders:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ORDERS_VALUE".to_string(),
            field_type: FieldType::Field,
            pos: (7, 21),
            length: 10,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: Some("000000".to_string()),
            pic: None,
            grp_name: None,
        },
        // Revenue
        BmsField {
            name: "REVENUE_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (9, 5),
            length: 15,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::Green),
            initial: Some("Revenue:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "REVENUE_VALUE".to_string(),
            field_type: FieldType::Field,
            pos: (9, 21),
            length: 15,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: Some("$000,000.00".to_string()),
            pic: None,
            grp_name: None,
        },
        // Quick actions
        BmsField {
            name: "ACTIONS_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (12, 1),
            length: 20,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("Quick Actions:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ACTION1".to_string(),
            field_type: FieldType::Field,
            pos: (14, 5),
            length: 20,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Green),
            initial: Some("[New Customer]".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ACTION2".to_string(),
            field_type: FieldType::Field,
            pos: (14, 30),
            length: 20,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Green),
            initial: Some("[New Order]".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "ACTION3".to_string(),
            field_type: FieldType::Field,
            pos: (14, 55),
            length: 20,
            attrb: vec![FieldAttribute::Norm],
            color: Some(Color::Green),
            initial: Some("[View Reports]".to_string()),
            pic: None,
            grp_name: None,
        },
        // Messages area
        BmsField {
            name: "MESSAGES_LABEL".to_string(),
            field_type: FieldType::Field,
            pos: (17, 1),
            length: 20,
            attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens],
            color: Some(Color::White),
            initial: Some("Messages:".to_string()),
            pic: None,
            grp_name: None,
        },
        BmsField {
            name: "MESSAGES_AREA".to_string(),
            field_type: FieldType::Field,
            pos: (18, 1),
            length: 80,
            attrb: vec![FieldAttribute::Prot],
            color: Some(Color::White),
            initial: Some("No new messages".to_string()),
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
            initial: Some("F1=Help F3=Exit".to_string()),
            pic: None,
            grp_name: None,
        },
    ],
};

// ==================== FONCTIONS UTILITAIRES ====================

/// Exporter un template au format BMS
pub fn export_template_to_bms(template: &BmsTemplate) -> String {
    let mut editor = BmsEditor::new();
    editor.map = template.to_map();
    editor.export_to_bms()
}

/// Creer un editeur a partir d'un template
pub fn create_editor_from_template(template_name: &str) -> Option<BmsEditor> {
    get_template_by_name(template_name).map(|template| {
        BmsEditor::from_map(template.to_map())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_templates() {
        let templates = get_all_templates();
        assert!(templates.len() >= 7); // Au moins 7 templates
    }

    #[test]
    fn test_get_template_by_name() {
        let template = get_template_by_name("menu");
        assert!(template.is_some());
        assert_eq!(template.unwrap().name, "menu");

        let template = get_template_by_name("nonexistent");
        assert!(template.is_none());
    }

    #[test]
    fn test_template_to_map() {
        let template = get_template_by_name("menu").unwrap();
        let map = template.to_map();

        assert_eq!(map.name, "MENU01");
        assert_eq!(map.mapset, "APPSET");
        assert_eq!(map.size, (24, 80));
        assert!(map.fields.len() > 0);
    }

    #[test]
    fn test_export_template_to_bms() {
        let template = get_template_by_name("menu").unwrap();
        let bms = export_template_to_bms(template);

        assert!(bms.contains("DFHMSD TYPE=MENU01"));
        assert!(bms.contains("DFHMDI SIZE=(24,80)"));
        assert!(bms.contains("DFHMND POS="));
    }

    #[test]
    fn test_create_editor_from_template() {
        let editor = create_editor_from_template("menu");
        assert!(editor.is_some());
        assert_eq!(editor.unwrap().map.name, "MENU01");
    }
}
