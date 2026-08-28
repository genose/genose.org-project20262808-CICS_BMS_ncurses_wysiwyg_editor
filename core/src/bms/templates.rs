//! Module pour les templates BMS predefinis
//!
//! Ce module fournit des templates de maps BMS pour:
//! - Menus
//! - Formulaires de saisie
//! - Listes
//! - Ecrans de confirmation
//! - Ecrans d'erreur
//! - Ecran de login
//! - Ecran de recherche
//! - Tableau de bord
//!
//! Les templates peuvent etre utilises comme point de depart
//! pour la creation rapide d'ecrans BMS.

use crate::bms::model::*;
use crate::bms::editor::BmsEditor;
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
    
    /// Creer un BmsEditor a partir du template
    pub fn to_editor(&self) -> BmsEditor {
        BmsEditor::from_map(self.to_map())
    }
}

/// Liste de tous les templates disponibles
pub fn get_all_templates() -> Vec<BmsTemplate> {
    vec![
        template_menu(),
        template_data_entry_form(),
        template_list(),
        template_confirmation(),
        template_error(),
        template_login(),
        template_search(),
        template_dashboard(),
    ]
}

/// Trouver un template par son nom
pub fn get_template_by_name(name: &str) -> Option<BmsTemplate> {
    get_all_templates().into_iter().find(|t| t.name == name)
}

/// Liste des noms de templates
pub fn get_template_names() -> Vec<&'static str> {
    get_all_templates().into_iter().map(|t| t.name).collect()
}

/// Creer un editeur a partir d'un template par nom
pub fn create_editor_from_template(template_name: &str) -> Option<BmsEditor> {
    get_template_by_name(template_name).map(|t| t.to_editor())
}

// ==================== TEMPLATE FUNCTIONS ====================

/// Template: Menu principal
fn template_menu() -> BmsTemplate {
    BmsTemplate {
        name: "menu",
        description: "Menu principal avec options numerotees",
        map_name: "MENU01".to_string(),
        mapset: "APPSET".to_string(),
        size: (24, 80),
        fields: vec![
            BmsField { name: "TITLE".to_string(), field_type: FieldType::Field, pos: (1, 28), length: 24, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::White), initial: Some("MAIN MENU".to_string()), pic: None, grp_name: None },
            BmsField { name: "OPTION1_KEY".to_string(), field_type: FieldType::Field, pos: (3, 10), length: 2, attrb: vec![FieldAttribute::Prot], color: Some(Color::Green), initial: Some("1.".to_string()), pic: None, grp_name: None },
            BmsField { name: "OPTION1_TEXT".to_string(), field_type: FieldType::Field, pos: (3, 13), length: 30, attrb: vec![FieldAttribute::Prot], color: Some(Color::Green), initial: Some("Customer Maintenance".to_string()), pic: None, grp_name: None },
            BmsField { name: "OPTION2_KEY".to_string(), field_type: FieldType::Field, pos: (5, 10), length: 2, attrb: vec![FieldAttribute::Prot], color: Some(Color::Green), initial: Some("2.".to_string()), pic: None, grp_name: None },
            BmsField { name: "OPTION2_TEXT".to_string(), field_type: FieldType::Field, pos: (5, 13), length: 30, attrb: vec![FieldAttribute::Prot], color: Some(Color::Green), initial: Some("Order Processing".to_string()), pic: None, grp_name: None },
            BmsField { name: "OPTION3_KEY".to_string(), field_type: FieldType::Field, pos: (7, 10), length: 2, attrb: vec![FieldAttribute::Prot], color: Some(Color::Green), initial: Some("3.".to_string()), pic: None, grp_name: None },
            BmsField { name: "OPTION3_TEXT".to_string(), field_type: FieldType::Field, pos: (7, 13), length: 30, attrb: vec![FieldAttribute::Prot], color: Some(Color::Green), initial: Some("Product Catalog".to_string()), pic: None, grp_name: None },
            BmsField { name: "OPTION4_KEY".to_string(), field_type: FieldType::Field, pos: (9, 10), length: 2, attrb: vec![FieldAttribute::Prot], color: Some(Color::Green), initial: Some("4.".to_string()), pic: None, grp_name: None },
            BmsField { name: "OPTION4_TEXT".to_string(), field_type: FieldType::Field, pos: (9, 13), length: 30, attrb: vec![FieldAttribute::Prot], color: Some(Color::Green), initial: Some("Reports".to_string()), pic: None, grp_name: None },
            BmsField { name: "STATUS".to_string(), field_type: FieldType::Field, pos: (24, 1), length: 80, attrb: vec![FieldAttribute::Prot, FieldAttribute::Reverse], color: Some(Color::Blue), initial: Some("F1=Help  F3=Exit  F12=Cancel".to_string()), pic: None, grp_name: None },
        ],
    }
}

/// Template: Formulaire de saisie de donnees
fn template_data_entry_form() -> BmsTemplate {
    BmsTemplate {
        name: "data_entry_form",
        description: "Formulaire de saisie avec champs editable",
        map_name: "DATAENT".to_string(),
        mapset: "APPSET".to_string(),
        size: (24, 80),
        fields: vec![
            BmsField { name: "TITLE".to_string(), field_type: FieldType::Field, pos: (1, 25), length: 30, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::White), initial: Some("CUSTOMER DATA ENTRY".to_string()), pic: None, grp_name: None },
            BmsField { name: "CUSTID_LBL".to_string(), field_type: FieldType::Field, pos: (3, 5), length: 12, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("Customer ID:".to_string()), pic: None, grp_name: None },
            BmsField { name: "CUSTID".to_string(), field_type: FieldType::Field, pos: (3, 18), length: 10, attrb: vec![FieldAttribute::Norm, FieldAttribute::Alph, FieldAttribute::Intens], color: Some(Color::Yellow), initial: None, pic: Some("X(10)".to_string()), grp_name: None },
            BmsField { name: "NAME_LBL".to_string(), field_type: FieldType::Field, pos: (5, 5), length: 12, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("Name:".to_string()), pic: None, grp_name: None },
            BmsField { name: "NAME".to_string(), field_type: FieldType::Field, pos: (5, 18), length: 30, attrb: vec![FieldAttribute::Norm, FieldAttribute::Alph], color: Some(Color::Yellow), initial: None, pic: Some("X(30)".to_string()), grp_name: None },
            BmsField { name: "STATUS".to_string(), field_type: FieldType::Field, pos: (24, 1), length: 80, attrb: vec![FieldAttribute::Prot, FieldAttribute::Reverse], color: Some(Color::Blue), initial: Some("F1=Help  F3=Exit  F12=Save".to_string()), pic: None, grp_name: None },
        ],
    }
}

/// Template: Liste de donnees
fn template_list() -> BmsTemplate {
    BmsTemplate {
        name: "list",
        description: "Affichage de liste avec en-tetes",
        map_name: "LIST01".to_string(),
        mapset: "APPSET".to_string(),
        size: (24, 80),
        fields: vec![
            BmsField { name: "TITLE".to_string(), field_type: FieldType::Field, pos: (1, 30), length: 20, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::White), initial: Some("CUSTOMER LIST".to_string()), pic: None, grp_name: None },
            BmsField { name: "HDR_ID".to_string(), field_type: FieldType::Field, pos: (3, 1), length: 10, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::White), initial: Some("Cust ID".to_string()), pic: None, grp_name: None },
            BmsField { name: "HDR_NAME".to_string(), field_type: FieldType::Field, pos: (3, 12), length: 25, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::White), initial: Some("Name".to_string()), pic: None, grp_name: None },
            BmsField { name: "SEPARATOR".to_string(), field_type: FieldType::Field, pos: (4, 1), length: 80, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("-".repeat(80)), pic: None, grp_name: None },
            BmsField { name: "ROW1_ID".to_string(), field_type: FieldType::Field, pos: (5, 1), length: 10, attrb: vec![FieldAttribute::Prot], color: Some(Color::Green), initial: Some("CUST001".to_string()), pic: None, grp_name: None },
            BmsField { name: "ROW1_NAME".to_string(), field_type: FieldType::Field, pos: (5, 12), length: 25, attrb: vec![FieldAttribute::Prot], color: Some(Color::Green), initial: Some("John Doe".to_string()), pic: None, grp_name: None },
            BmsField { name: "STATUS".to_string(), field_type: FieldType::Field, pos: (24, 1), length: 80, attrb: vec![FieldAttribute::Prot, FieldAttribute::Reverse], color: Some(Color::Blue), initial: Some("F1=Help  F3=Exit  F7=Back  F8=Fwd".to_string()), pic: None, grp_name: None },
        ],
    }
}

/// Template: Ecran de confirmation
fn template_confirmation() -> BmsTemplate {
    BmsTemplate {
        name: "confirmation",
        description: "Ecran de confirmation Oui/Non",
        map_name: "CONFIRM".to_string(),
        mapset: "APPSET".to_string(),
        size: (10, 80),
        fields: vec![
            BmsField { name: "MSG_LINE1".to_string(), field_type: FieldType::Field, pos: (3, 10), length: 60, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("Are you sure you want to".to_string()), pic: None, grp_name: None },
            BmsField { name: "MSG_LINE2".to_string(), field_type: FieldType::Field, pos: (4, 10), length: 60, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("delete this record?".to_string()), pic: None, grp_name: None },
            BmsField { name: "SEPARATOR".to_string(), field_type: FieldType::Field, pos: (5, 1), length: 80, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("-".repeat(80)), pic: None, grp_name: None },
            BmsField { name: "YES_KEY".to_string(), field_type: FieldType::Field, pos: (7, 30), length: 2, attrb: vec![FieldAttribute::Norm], color: Some(Color::Green), initial: Some("Y".to_string()), pic: None, grp_name: Some("CONFIRM".to_string()) },
            BmsField { name: "YES_TEXT".to_string(), field_type: FieldType::Field, pos: (7, 33), length: 3, attrb: vec![FieldAttribute::Prot], color: Some(Color::Green), initial: Some("= Yes".to_string()), pic: None, grp_name: None },
            BmsField { name: "NO_KEY".to_string(), field_type: FieldType::Field, pos: (7, 40), length: 2, attrb: vec![FieldAttribute::Norm], color: Some(Color::Green), initial: Some("N".to_string()), pic: None, grp_name: Some("CONFIRM".to_string()) },
            BmsField { name: "NO_TEXT".to_string(), field_type: FieldType::Field, pos: (7, 43), length: 3, attrb: vec![FieldAttribute::Prot], color: Some(Color::Green), initial: Some("= No".to_string()), pic: None, grp_name: None },
        ],
    }
}

/// Template: Ecran d'erreur
fn template_error() -> BmsTemplate {
    BmsTemplate {
        name: "error",
        description: "Ecran d'affichage d'erreur",
        map_name: "ERROR01".to_string(),
        mapset: "APPSET".to_string(),
        size: (10, 80),
        fields: vec![
            BmsField { name: "ERROR_TITLE".to_string(), field_type: FieldType::Field, pos: (2, 30), length: 20, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens, FieldAttribute::Reverse], color: Some(Color::Red), initial: Some("ERROR".to_string()), pic: None, grp_name: None },
            BmsField { name: "ERROR_MSG1".to_string(), field_type: FieldType::Field, pos: (4, 10), length: 60, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::Red), initial: Some("An error has occurred:".to_string()), pic: None, grp_name: None },
            BmsField { name: "ERROR_MSG2".to_string(), field_type: FieldType::Field, pos: (5, 10), length: 60, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::Red), initial: Some("Invalid data entered".to_string()), pic: None, grp_name: None },
            BmsField { name: "OK_KEY".to_string(), field_type: FieldType::Field, pos: (7, 50), length: 3, attrb: vec![FieldAttribute::Norm], color: Some(Color::Green), initial: Some("OK".to_string()), pic: None, grp_name: None },
        ],
    }
}

/// Template: Ecran de login
fn template_login() -> BmsTemplate {
    BmsTemplate {
        name: "login",
        description: "Ecran de connexion utilisateur",
        map_name: "LOGIN".to_string(),
        mapset: "APPSET".to_string(),
        size: (12, 80),
        fields: vec![
            BmsField { name: "TITLE".to_string(), field_type: FieldType::Field, pos: (2, 30), length: 20, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::White), initial: Some("USER LOGIN".to_string()), pic: None, grp_name: None },
            BmsField { name: "USER_LBL".to_string(), field_type: FieldType::Field, pos: (5, 25), length: 10, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("Username:".to_string()), pic: None, grp_name: None },
            BmsField { name: "USERNAME".to_string(), field_type: FieldType::Field, pos: (5, 36), length: 20, attrb: vec![FieldAttribute::Norm, FieldAttribute::Alph, FieldAttribute::Intens], color: Some(Color::Yellow), initial: None, pic: Some("X(20)".to_string()), grp_name: None },
            BmsField { name: "PASS_LBL".to_string(), field_type: FieldType::Field, pos: (7, 25), length: 10, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("Password:".to_string()), pic: None, grp_name: None },
            BmsField { name: "PASSWORD".to_string(), field_type: FieldType::Field, pos: (7, 36), length: 20, attrb: vec![FieldAttribute::Norm, FieldAttribute::Alph, FieldAttribute::Intens], color: Some(Color::Yellow), initial: None, pic: Some("X(20)".to_string()), grp_name: None },
            BmsField { name: "STATUS".to_string(), field_type: FieldType::Field, pos: (12, 1), length: 80, attrb: vec![FieldAttribute::Prot, FieldAttribute::Reverse], color: Some(Color::Blue), initial: Some("F1=Help  F3=Exit  Enter=Login".to_string()), pic: None, grp_name: None },
        ],
    }
}

/// Template: Ecran de recherche
fn template_search() -> BmsTemplate {
    BmsTemplate {
        name: "search",
        description: "Ecran de recherche avec criteres",
        map_name: "SEARCH01".to_string(),
        mapset: "APPSET".to_string(),
        size: (15, 80),
        fields: vec![
            BmsField { name: "TITLE".to_string(), field_type: FieldType::Field, pos: (1, 28), length: 24, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::White), initial: Some("CUSTOMER SEARCH".to_string()), pic: None, grp_name: None },
            BmsField { name: "CUSTID_LBL".to_string(), field_type: FieldType::Field, pos: (5, 10), length: 12, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("Customer ID:".to_string()), pic: None, grp_name: None },
            BmsField { name: "CUSTID".to_string(), field_type: FieldType::Field, pos: (5, 23), length: 10, attrb: vec![FieldAttribute::Norm, FieldAttribute::Alph], color: Some(Color::Yellow), initial: None, pic: Some("X(10)".to_string()), grp_name: None },
            BmsField { name: "NAME_LBL".to_string(), field_type: FieldType::Field, pos: (7, 10), length: 12, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("Name:".to_string()), pic: None, grp_name: None },
            BmsField { name: "NAME".to_string(), field_type: FieldType::Field, pos: (7, 23), length: 30, attrb: vec![FieldAttribute::Norm, FieldAttribute::Alph], color: Some(Color::Yellow), initial: None, pic: Some("X(30)".to_string()), grp_name: None },
            BmsField { name: "SEARCH_BTN".to_string(), field_type: FieldType::Field, pos: (11, 30), length: 10, attrb: vec![FieldAttribute::Norm, FieldAttribute::Intens], color: Some(Color::Green), initial: Some("SEARCH".to_string()), pic: None, grp_name: None },
            BmsField { name: "STATUS".to_string(), field_type: FieldType::Field, pos: (15, 1), length: 80, attrb: vec![FieldAttribute::Prot, FieldAttribute::Reverse], color: Some(Color::Blue), initial: Some("F1=Help  F3=Exit  F12=Search".to_string()), pic: None, grp_name: None },
        ],
    }
}

/// Template: Tableau de bord
fn template_dashboard() -> BmsTemplate {
    BmsTemplate {
        name: "dashboard",
        description: "Tableau de bord avec statistiques",
        map_name: "DASH01".to_string(),
        mapset: "APPSET".to_string(),
        size: (24, 80),
        fields: vec![
            BmsField { name: "TITLE".to_string(), field_type: FieldType::Field, pos: (1, 25), length: 30, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::White), initial: Some("OPERATIONS DASHBOARD".to_string()), pic: None, grp_name: None },
            BmsField { name: "STATS_TITLE".to_string(), field_type: FieldType::Field, pos: (3, 5), length: 20, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::Cyan), initial: Some("Today's Statistics".to_string()), pic: None, grp_name: None },
            BmsField { name: "ORDERS_LBL".to_string(), field_type: FieldType::Field, pos: (5, 5), length: 15, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("Orders Processed:".to_string()), pic: None, grp_name: None },
            BmsField { name: "ORDERS_VAL".to_string(), field_type: FieldType::Field, pos: (5, 21), length: 10, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::Green), initial: Some("245".to_string()), pic: None, grp_name: None },
            BmsField { name: "CUSTOMERS_LBL".to_string(), field_type: FieldType::Field, pos: (7, 5), length: 15, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("New Customers:".to_string()), pic: None, grp_name: None },
            BmsField { name: "CUSTOMERS_VAL".to_string(), field_type: FieldType::Field, pos: (7, 21), length: 10, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::Green), initial: Some("42".to_string()), pic: None, grp_name: None },
            BmsField { name: "REVENUE_LBL".to_string(), field_type: FieldType::Field, pos: (9, 5), length: 15, attrb: vec![FieldAttribute::Prot], color: Some(Color::White), initial: Some("Total Revenue:".to_string()), pic: None, grp_name: None },
            BmsField { name: "REVENUE_VAL".to_string(), field_type: FieldType::Field, pos: (9, 21), length: 15, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::Green), initial: Some("$125,485.63".to_string()), pic: None, grp_name: None },
            BmsField { name: "ACTIONS_TITLE".to_string(), field_type: FieldType::Field, pos: (12, 5), length: 20, attrb: vec![FieldAttribute::Prot, FieldAttribute::Intens], color: Some(Color::Cyan), initial: Some("Quick Actions".to_string()), pic: None, grp_name: None },
            BmsField { name: "ACTION1".to_string(), field_type: FieldType::Field, pos: (14, 10), length: 20, attrb: vec![FieldAttribute::Norm, FieldAttribute::Intens], color: Some(Color::Green), initial: Some("1. New Order".to_string()), pic: None, grp_name: None },
            BmsField { name: "ACTION2".to_string(), field_type: FieldType::Field, pos: (16, 10), length: 20, attrb: vec![FieldAttribute::Norm, FieldAttribute::Intens], color: Some(Color::Green), initial: Some("2. View Reports".to_string()), pic: None, grp_name: None },
            BmsField { name: "ACTION3".to_string(), field_type: FieldType::Field, pos: (18, 10), length: 20, attrb: vec![FieldAttribute::Norm, FieldAttribute::Intens], color: Some(Color::Green), initial: Some("3. Customer List".to_string()), pic: None, grp_name: None },
            BmsField { name: "STATUS".to_string(), field_type: FieldType::Field, pos: (24, 1), length: 80, attrb: vec![FieldAttribute::Prot, FieldAttribute::Reverse], color: Some(Color::Blue), initial: Some("F1=Help  F3=Exit  F12=Refresh".to_string()), pic: None, grp_name: None },
        ],
    }
}
