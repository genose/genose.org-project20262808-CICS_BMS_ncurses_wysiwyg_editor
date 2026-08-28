use crate::bms::model::*;
use std::fmt::Write;

/// Generate COBOL/CICS code for a BMS map
pub fn generate_cobol(map: &BmsMap) -> String {
    let mut cobol = String::new();
    
    // Header
    writeln!(cobol, "IDENTIFICATION DIVISION.").unwrap();
    writeln!(cobol, "PROGRAM-ID. {}.", map.name).unwrap();
    writeln!(cobol, "AUTHOR. BMS-GENERATOR.").unwrap();
    writeln!(cobol, "DATE-WRITTEN. TODAY.").unwrap();
    writeln!(cobol, "*------------------------------------------------------------*").unwrap();
    writeln!(cobol, "* Auto-generated from BMS map: {} (Mapset: {})", map.name, map.mapset).unwrap();
    writeln!(cobol, "*------------------------------------------------------------*").unwrap();
    writeln!(cobol).unwrap();
    
    // Environment Division
    writeln!(cobol, "ENVIRONMENT DIVISION.").unwrap();
    writeln!(cobol, "CONFIGURATION SECTION.").unwrap();
    writeln!(cobol, "SOURCE-COMPUTER. IBM-Z.").unwrap();
    writeln!(cobol, "OBJECT-COMPUTER. IBM-Z.").unwrap();
    writeln!(cobol).unwrap();
    
    // Data Division
    writeln!(cobol, "DATA DIVISION.").unwrap();
    writeln!(cobol, "WORKING-STORAGE SECTION.").unwrap();
    
    // CICS response fields
    writeln!(cobol, "01  WS-EIBRESP      PIC S9(8) COMP VALUE 0.").unwrap();
    writeln!(cobol, "01  WS-EIBRESP2     PIC S9(8) COMP VALUE 0.").unwrap();
    writeln!(cobol, "01  WS-EIBFN       PIC X(4) VALUE SPACES.").unwrap();
    writeln!(cobol).unwrap();
    
    // Generate 01 level for the map
    writeln!(cobol, "01  {}.", map.name).unwrap();
    
    // Generate fields from BMS
    for field in &map.fields {
        generate_cobol_field(&mut cobol, field);
    }
    writeln!(cobol).unwrap();
    
    // Procedure Division
    writeln!(cobol, "PROCEDURE DIVISION.").unwrap();
    writeln!(cobol, "MAIN-PARAGRAPH.").unwrap();
    writeln!(cobol, "    EXEC CICS").unwrap();
    writeln!(cobol, "        RECEIVE MAP('{}')", map.name).unwrap();
    writeln!(cobol, "              MAPSET('{}')", map.mapset).unwrap();
    writeln!(cobol, "        INTO({})", map.name).unwrap();
    writeln!(cobol, "        RESP(WS-EIBRESP)").unwrap();
    writeln!(cobol, "        RESP2(WS-EIBRESP2)").unwrap();
    writeln!(cobol, "    END-EXEC.").unwrap();
    writeln!(cobol).unwrap();
    
    // Check response
    writeln!(cobol, "    IF WS-EIBRESP = DFHRESP(NORMAL)").unwrap();
    writeln!(cobol, "        CONTINUE").unwrap();
    writeln!(cobol, "    ELSE").unwrap();
    writeln!(cobol, "        EXEC CICS").unwrap();
    writeln!(cobol, "            ABEND").unwrap();
    writeln!(cobol, "        END-EXEC").unwrap();
    writeln!(cobol, "    END-IF.").unwrap();
    writeln!(cobol).unwrap();
    
    // Business logic placeholder
    writeln!(cobol, "    *------------------------------------------------------------*").unwrap();
    writeln!(cobol, "    * Add your business logic here").unwrap();
    writeln!(cobol, "    *------------------------------------------------------------*").unwrap();
    writeln!(cobol).unwrap();
    
    // Send map
    writeln!(cobol, "    EXEC CICS").unwrap();
    writeln!(cobol, "        SEND MAP('{}')", map.name).unwrap();
    writeln!(cobol, "              MAPSET('{}')", map.mapset).unwrap();
    writeln!(cobol, "              FROM({})", map.name).unwrap();
    writeln!(cobol, "        RESP(WS-EIBRESP)").unwrap();
    writeln!(cobol, "        RESP2(WS-EIBRESP2)").unwrap();
    writeln!(cobol, "    END-EXEC.").unwrap();
    writeln!(cobol).unwrap();
    
    // Return
    writeln!(cobol, "    EXEC CICS").unwrap();
    writeln!(cobol, "        RETURN").unwrap();
    writeln!(cobol, "    END-EXEC.").unwrap();
    writeln!(cobol, "    GOBACK.").unwrap();
    
    cobol
}

/// Generate COBOL field definition from BMS field
fn generate_cobol_field(cobol: &mut String, field: &BmsField) {
    let field_name = if field.name.is_empty() {
        format!("FILLER")
    } else {
        field.name.clone()
    };
    
    // Determine PIC clause
    let pic_clause = if let Some(ref pic) = field.pic {
        format!(" PIC {}", pic)
    } else {
        match field.attrb.iter().find(|a| matches!(a, FieldAttribute::Num)) {
            Some(_) => String::from(" PIC 9(8)"),
            None => match field.attrb.iter().find(|a| matches!(a, FieldAttribute::Alph | FieldAttribute::AlphaNum)) {
                Some(_) => String::from(" PIC X(8)"),
                None => String::from(" PIC X(1)"),
            },
        }
    };
    
    // Check for protected field (output only)
    let is_prot = field.attrb.iter().any(|a| matches!(a, FieldAttribute::Prot));
    
    // Generate field
    if is_prot {
        // Protected fields are typically for output
        writeln!(cobol, "    05  {}       {}.", field_name, pic_clause).unwrap();
    } else {
        // Input fields
        writeln!(cobol, "    05  {}       {}.", field_name, pic_clause).unwrap();
    }
}

/// Render a BMS map as text (for CLI preview)
pub fn render_bms_text(map: &BmsMap) -> String {
    let mut output = String::new();
    
    // Header
    writeln!(output, "+{}+", "-".repeat(map.size.1 as usize)).unwrap();
    writeln!(output, "| Map: {} (Mapset: {}) [{:?}] |", map.name, map.mapset, map.size).unwrap();
    writeln!(output, "+{}+", "-".repeat(map.size.1 as usize)).unwrap();
    
    // Create a grid
    let mut grid = vec![vec![' '; map.size.1 as usize]; map.size.0 as usize];
    
    // Place fields on grid
    for field in &map.fields {
        let (line, col) = field.pos;
        let line_idx = line as usize - 1;
        let col_idx = col as usize - 1;
        
        if line_idx < grid.len() {
            for i in 0..field.length as usize {
                if col_idx + i < grid[line_idx].len() {
                    // Use different characters based on field type
                    let c = match field.field_type {
                        FieldType::Map => '#',
                        FieldType::Field => {
                            if field.attrb.iter().any(|a| matches!(a, FieldAttribute::Prot)) {
                                'P'
                            } else if field.attrb.iter().any(|a| matches!(a, FieldAttribute::Num)) {
                                '0'
                            } else {
                                'F'
                            }
                        },
                        FieldType::Literal => 'L',
                        FieldType::Group => 'G',
                        _ => 'X',
                    };
                    grid[line_idx][col_idx + i] = c;
                }
            }
        }
    }
    
    // Render grid
    for row in &grid {
        output.write_char('|').unwrap();
        for &c in row {
            output.write_char(c).unwrap();
        }
        output.write_str("|\n").unwrap();
    }
    
    // Footer
    writeln!(output, "+{}+", "-".repeat(map.size.1 as usize)).unwrap();
    
    // Field legend
    writeln!(output, "\nLegend:").unwrap();
    writeln!(output, "  # : Map definition").unwrap();
    writeln!(output, "  F : Field (input)").unwrap();
    writeln!(output, "  P : Protected field (output)").unwrap();
    writeln!(output, "  0 : Numeric field").unwrap();
    writeln!(output, "  L : Literal").unwrap();
    writeln!(output, "  G : Group").unwrap();
    
    // Field details
    writeln!(output, "\nFields:").unwrap();
    for (i, field) in map.fields.iter().enumerate() {
        writeln!(output, "  {}. {} at ({},{}) len={} attr={:?} color={:?}",
            i + 1,
            field.name,
            field.pos.0, field.pos.1,
            field.length,
            field.attrb,
            field.color
        ).unwrap();
    }
    
    output
}

/// Generate HTML for VSCode webview preview
pub fn render_bms_html(map: &BmsMap) -> String {
    let mut html = String::new();
    
    html.push_str("<div class=\"bms-preview\">");
    html.push_str(&format!("<div class=\"header\">Map: {} (Mapset: {}) [{:?}]</div>", map.name, map.mapset, map.size));
    html.push_str("<div class=\"grid\" style=\"grid-template-columns: repeat(");
    html.push_str(&map.size.1.to_string());
    html.push_str(", 1fr); grid-template-rows: repeat(");
    html.push_str(&map.size.0.to_string());
    html.push_str(", 1fr);\">");
    
    // Create grid cells
    for row in 0..map.size.0 {
        for col in 0..map.size.1 {
            // Check if any field covers this cell
            let mut field_class = "empty";
            let mut field_tooltip = "";
            
            for field in &map.fields {
                let (field_row, field_col) = field.pos;
                let field_row = field_row as usize - 1;
                let field_col = field_col as usize - 1;
                
                if row as usize == field_row && col as usize >= field_col && col as usize < field_col + field.length as usize {
                    field_class = match field.field_type {
                        FieldType::Map => "map-def",
                        FieldType::Field => {
                            if field.attrb.iter().any(|a| matches!(a, FieldAttribute::Prot)) {
                                "field-prot"
                            } else if field.attrb.iter().any(|a| matches!(a, FieldAttribute::Num)) {
                                "field-num"
                            } else {
                                "field"
                            }
                        },
                        FieldType::Literal => "literal",
                        FieldType::Group => "group",
                        _ => "other",
                    };
                    
                    field_tooltip = format!(
                        "Name: {}|Type: {:?}|Pos: ({},{})|Len: {}",
                        field.name, field.field_type, field.pos.0, field.pos.1, field.length
                    );
                    break;
                }
            }
            
            html.push_str(&format!(
                "<div class=\"{}\" title=\"{}\"></div>",
                field_class, field_tooltip
            ));
        }
    }
    
    html.push_str("</div>");
    html.push_str("<div class=\"legend\">");
    html.push_str("<span class=\"legend-item\"><span class=\"legend-color field\"></span> Field (Input)</span>");
    html.push_str("<span class=\"legend-item\"><span class=\"legend-color field-prot\"></span> Protected Field</span>");
    html.push_str("<span class=\"legend-item\"><span class=\"legend-color field-num\"></span> Numeric Field</span>");
    html.push_str("</div>");
    html.push_str("</div>");
    
    // CSS
    html.push_str("<style>");
    html.push_str(".bms-preview { font-family: monospace; }");
    html.push_str(".header { font-weight: bold; margin-bottom: 10px; }");
    html.push_str(".grid { display: grid; gap: 1px; }");
    html.push_str(".grid > div { width: 20px; height: 20px; border: 1px solid #ddd; }");
    html.push_str(".empty { background: white; }");
    html.push_str(".field { background: #e6f7ff; }");
    html.push_str(".field-prot { background: #e0f2fe; }");
    html.push_str(".field-num { background: #f0fff4; }");
    html.push_str(".literal { background: #fff2cc; }");
    html.push_str(".group { background: #f9f9f9; }");
    html.push_str(".map-def { background: #f3e5f5; }");
    html.push_str(".legend { margin-top: 10px; }");
    html.push_str(".legend-item { margin-right: 20px; }");
    html.push_str(".legend-color { display: inline-block; width: 16px; height: 16px; margin-right: 5px; border: 1px solid #000; }");
    html.push_str("</style>");
    
    html
}
