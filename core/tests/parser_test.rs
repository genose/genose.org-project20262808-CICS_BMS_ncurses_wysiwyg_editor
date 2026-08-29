use cobol_bms_core::{parse_bms, parse_bms_file, BmsMap, BmsField, FieldType, FieldAttribute, Color, generate_cobol, render_bms_text};

#[test]
fn test_parse_basic_map() {
    let source = r#"
        DFHMSD TYPE=MENU01,MAPSET=MAPSET1,LANG=COBOL
        DFHMDI SIZE=(24,80)
    "#;
    
    let map = parse_bms(source).unwrap();
    assert_eq!(map.name, "MENU01");
    assert_eq!(map.mapset, "MAPSET1");
    assert_eq!(map.size, (24, 80));
    assert_eq!(map.language, Some("COBOL".to_string()));
}

#[test]
fn test_parse_map_with_fields() {
    let source = r#"
        DFHMSD TYPE=MENU01,MAPSET=MAPSET1
        DFHMDI SIZE=(24,80)
        DFHMND POS=(1,1),LENGTH=10,ATTRB=(PROT,NUM),COLOR=BLUE
        DFHMND POS=(2,1),LENGTH=20,ATTRB=(NORM,ALPH)
    "#;
    
    let map = parse_bms(source).unwrap();
    assert_eq!(map.name, "MENU01");
    assert_eq!(map.fields.len(), 2);
    
    // First field
    assert_eq!(map.fields[0].pos, (1, 1));
    assert_eq!(map.fields[0].length, 10);
    assert!(map.fields[0].attrb.contains(&FieldAttribute::Prot));
    assert!(map.fields[0].attrb.contains(&FieldAttribute::Num));
    assert_eq!(map.fields[0].text_color, Some(Color::Blue));
    
    // Second field
    assert_eq!(map.fields[1].pos, (2, 1));
    assert_eq!(map.fields[1].length, 20);
    assert!(map.fields[1].attrb.contains(&FieldAttribute::Norm));
    assert!(map.fields[1].attrb.contains(&FieldAttribute::Alph));
    assert_eq!(map.fields[1].text_color, None);
}

#[test]
fn test_parse_field_with_type() {
    let source = r#"
        DFHMSD TYPE=MENU01
        DFHMDI SIZE=(24,80)
        DFHMND TYPE=GRP,POS=(1,1),LENGTH=10
    "#;
    
    let map = parse_bms(source).unwrap();
    assert_eq!(map.fields[0].field_type, FieldType::Group);
}

#[test]
fn test_parse_field_with_initial() {
    let source = r#"
        DFHMSD TYPE=MENU01
        DFHMDI SIZE=(24,80)
        DFHMND POS=(1,1),LENGTH=10,INITIAL='DEFAULT'
    "#;
    
    let map = parse_bms(source).unwrap();
    assert_eq!(map.fields[0].initial, Some("DEFAULT".to_string()));
}

#[test]
fn test_parse_field_with_pic() {
    let source = r#"
        DFHMSD TYPE=MENU01
        DFHMDI SIZE=(24,80)
        DFHMND POS=(1,1),LENGTH=5,PIC='9(5)'
    "#;
    
    let map = parse_bms(source).unwrap();
    assert_eq!(map.fields[0].pic, Some("9(5)".to_string()));
}

#[test]
fn test_parse_physical_map() {
    let source = r#"
        DFHMSD TYPE=MENU01,PHYSICAL=NO
        DFHMDI SIZE=(24,80)
    "#;
    
    let map = parse_bms(source).unwrap();
    assert!(!map.physical);
}

#[test]
fn test_parse_comments_and_empty_lines() {
    let source = r#"
        * This is a comment
        DFHMSD TYPE=MENU01
        
        DFHMDI SIZE=(24,80)
        * Another comment
        DFHMND POS=(1,1),LENGTH=10
    "#;
    
    let map = parse_bms(source).unwrap();
    assert_eq!(map.name, "MENU01");
    assert_eq!(map.fields.len(), 1);
}

#[test]
fn test_case_insensitive_parsing() {
    let source = r#"
        dfhmsd type=menu01,mapset=mapset1
        DFHMDI size=(24,80)
        dfhmnd pos=(1,1),length=10,attrb=(prot,num),color=blue
    "#;
    
    let map = parse_bms(source).unwrap();
    assert_eq!(map.name, "MENU01");
    assert_eq!(map.mapset, "MAPSET1");
    assert_eq!(map.fields[0].text_color, Some(Color::Blue));
}

#[test]
fn test_generate_cobol_basic() {
    let source = r#"
        DFHMSD TYPE=MENU01,MAPSET=MAPSET1
        DFHMDI SIZE=(24,80)
        DFHMND POS=(1,1),LENGTH=10,ATTRB=(PROT,NUM)
    "#;
    
    let map = parse_bms(source).unwrap();
    let cobol = generate_cobol(&map);
    
    assert!(cobol.contains("PROGRAM-ID. MENU01."));
    assert!(cobol.contains("RECEIVE MAP('MENU01')"));
    assert!(cobol.contains("SEND MAP('MENU01')"));
    assert!(cobol.contains("MAPSET('MAPSET1')"));
}

#[test]
fn test_render_bms_text() {
    let source = r#"
        DFHMSD TYPE=MENU01,MAPSET=MAPSET1
        DFHMDI SIZE=(5,10)
        DFHMND POS=(1,1),LENGTH=3,ATTRB=(PROT)
        DFHMND POS=(2,1),LENGTH=5,ATTRB=(NUM)
    "#;
    
    let map = parse_bms(source).unwrap();
    let text = render_bms_text(&map);
    
    assert!(text.contains("Map: MENU01"));
    assert!(text.contains("(5, 10)")); // Size is displayed as tuple
    assert!(text.contains("PPP")); // Protected field (3 P's together)
    assert!(text.contains("00000")); // Numeric field (5 0's together)
}
