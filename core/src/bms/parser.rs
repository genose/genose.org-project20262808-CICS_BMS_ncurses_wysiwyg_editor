use crate::bms::model::*;
use nom::{
    IResult,
    bytes::complete::{tag_no_case, take_until, take_while1},
    character::complete::{char, digit1, multispace0, space0},
    combinator::{map, opt},
    sequence::{delimited, separated_pair},
};
use std::fs;
use thiserror::Error;

/// Custom error type for BMS parsing
#[derive(Error, Debug)]
pub enum BmsParseError {
    #[error("Failed to parse BMS: {0}")]
    ParseError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Parse a BMS source string and return a BmsMap
pub fn parse_bms(input: &str) -> Result<BmsMap, BmsParseError> {
    let map = BmsMap::new("UNNAMED", "DEFAULT");
    let mut current_map: Option<BmsMap> = None;
    
    // Split into lines and process each
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue; // Skip comments and empty lines
        }
        
        // Parse each statement type
        if let Ok((_, new_map)) = parse_dfhmsd(trimmed) {
            current_map = Some(new_map);
        } else if let Ok((_, size)) = parse_dfhmdi(trimmed) {
            if let Some(ref mut m) = current_map {
                m.size = size;
            }
        } else if let Ok((_, field)) = parse_dfhmnd(trimmed) {
            if let Some(ref mut m) = current_map {
                m.fields.push(field);
            }
        } else if let Ok((_, field)) = parse_dfhmdff(trimmed) {
            if let Some(ref mut m) = current_map {
                m.fields.push(field);
            }
        }
    }
    
    Ok(current_map.unwrap_or(map))
}

/// Parse a BMS file from disk
pub fn parse_bms_file(path: &str) -> Result<BmsMap, BmsParseError> {
    let content = fs::read_to_string(path)?;
    parse_bms(&content)
}

/// Parse DFHMSD statement
/// Example: DFHMSD TYPE=MENU01,MAPSET=MAPSET1,LANG=COBOL,PHYSICAL=YES
fn parse_dfhmsd(input: &str) -> IResult<&str, BmsMap> {
    let (input, _) = tag_no_case("DFHMSD")(input)?;
    let (input, _) = space0(input)?;
    
    let (input, type_value) = parse_attribute_value("TYPE", input)?;
    let (input, mapset_value) = parse_optional_attribute("MAPSET", "DEFAULT", input)?;
    let (input, lang_value) = parse_optional_attribute("LANG", "COBOL", input)?;
    let (input, physical_value) = parse_optional_attribute("PHYSICAL", "YES", input)?;
    
    let physical = physical_value.to_uppercase() == "YES";
    
    Ok((input, BmsMap {
        name: type_value.to_uppercase(),
        mapset: mapset_value.to_uppercase(),
        size: (24, 80),
        language: Some(lang_value.to_uppercase()),
        map_type: FieldType::Map,
        fields: vec![],
        physical,
        symbolic: false,
        terminal: None,
        cursor_pos: None,
        erase: None,
        freekb: None,
        alarm: None,
        timetag: None,
    }))
}

/// Helper: Parse an optional attribute with a default value
fn parse_optional_attribute<'a>(attr_name: &str, default: &str, input: &'a str) -> IResult<&'a str, String> {
    match opt(|i: &'a str| parse_attribute_value(attr_name, i))(input) {
        Ok((remaining, Some(value))) => Ok((remaining, value)),
        Ok((remaining, None)) => Ok((remaining, default.to_string())),
        Err(e) => Err(e),
    }
}

/// Parse DFHMDI statement (map dimensions)
/// Example: DFHMDI SIZE=(24,80)
fn parse_dfhmdi(input: &str) -> IResult<&str, (u16, u16)> {
    let (input, _) = tag_no_case("DFHMDI")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag_no_case("SIZE")(input)?;
    let (input, _) = char('=')(input)?;
    let (input, (lines, cols)) = delimited(
        char('('),
        separated_pair(
            parse_u16,
            char(','),
            parse_u16,
        ),
        char(')'),
    )(input)?;
    
    Ok((input, (lines, cols)))
}

/// Parse DFHMND statement (field definition)
/// Example: DFHMND POS=(1,1),LENGTH=10,ATTRB=(PROT,NUM),COLOR=BLUE
fn parse_dfhmnd(input: &str) -> IResult<&str, BmsField> {
    let (input, _) = tag_no_case("DFHMND")(input)?;
    let (input, _) = space0(input)?;
    
    let mut field = BmsField::default();
    let mut remaining = input;
    
    // Parse all attributes
    loop {
        // Skip optional separators before each attribute
        let (input, _) = multispace0(remaining)?;
        let (input, _) = opt(char(','))(input)?;
        let (input, _) = multispace0(input)?;
        
        // POS=(line,col)
        if let Ok((new_input, pos)) = parse_pos(input) {
            field.pos = pos;
            remaining = new_input;
            continue;
        }
        
        // LENGTH=value
        if let Ok((new_input, length)) = parse_length(input) {
            field.length = length;
            remaining = new_input;
            continue;
        }
        
        // ATTRB=(attr1,attr2,...)
        if let Ok((new_input, attrs)) = parse_attrb(input) {
            field.attrb = attrs;
            remaining = new_input;
            continue;
        }
        
        // COLOR=value
        if let Ok((new_input, color)) = parse_color(input) {
            field.color = Some(color);
            remaining = new_input;
            continue;
        }
        
        // TYPE=value
        if let Ok((new_input, field_type)) = parse_field_type(input) {
            field.field_type = field_type;
            remaining = new_input;
            continue;
        }
        
        // INITIAL='value'
        if let Ok((new_input, initial)) = parse_initial(input) {
            field.initial = Some(initial);
            remaining = new_input;
            continue;
        }
        
        // PIC='value'
        if let Ok((new_input, pic)) = parse_pic(input) {
            field.pic = Some(pic);
            remaining = new_input;
            continue;
        }
        
        // If no more attributes, break
        break;
    }
    
    Ok((remaining, field))
}

/// Parse DFHMDF statement (formatted field)
fn parse_dfhmdff(input: &str) -> IResult<&str, BmsField> {
    let (input, _) = tag_no_case("DFHMDF")(input)?;
    let (input, _) = space0(input)?;
    
    // Similar to DFHMND but with FORMAT clause
    // For now, treat as DFHMND
    parse_dfhmnd(input)
}

/// Helper: Parse POS=(line,col)
fn parse_pos(input: &str) -> IResult<&str, (u16, u16)> {
    let (input, _) = tag_no_case("POS")(input)?;
    let (input, _) = char('=')(input)?;
    delimited(
        char('('),
        separated_pair(
            parse_u16,
            char(','),
            parse_u16,
        ),
        char(')'),
    )(input)
}

/// Helper: Parse LENGTH=value
fn parse_length(input: &str) -> IResult<&str, u16> {
    let (input, _) = tag_no_case("LENGTH")(input)?;
    let (input, _) = char('=')(input)?;
    parse_u16(input)
}

/// Helper: Parse ATTRB=(attr1,attr2,...)
fn parse_attrb(input: &str) -> IResult<&str, Vec<FieldAttribute>> {
    let (input, _) = tag_no_case("ATTRB")(input)?;
    let (input, _) = char('=')(input)?;
    let (input, attrs_str) = delimited(
        char('('),
        take_until(")"),
        char(')'),
    )(input)?;
    
    let attrs: Vec<FieldAttribute> = attrs_str
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            match s.to_uppercase().as_str() {
                "PROT" => FieldAttribute::Prot,
                "NORM" => FieldAttribute::Norm,
                "NUM" => FieldAttribute::Num,
                "ALPH" => FieldAttribute::Alph,
                "ALNUM" => FieldAttribute::AlphaNum,
                "BOOL" => FieldAttribute::Bool,
                "DATE" => FieldAttribute::Date,
                "TIME" => FieldAttribute::Time,
                "FLOAT" => FieldAttribute::Float,
                "SIGN" => FieldAttribute::Signed,
                "INTENS" => FieldAttribute::Intens,
                "BLINK" => FieldAttribute::Blink,
                "REVERSE" => FieldAttribute::Reverse,
                "UNDERLINE" => FieldAttribute::Underline,
                "DARK" => FieldAttribute::Dark,
                _ => FieldAttribute::Unknown(s.to_string()),
            }
        })
        .collect();
    
    Ok((input, attrs))
}

/// Helper: Parse COLOR=value
fn parse_color(input: &str) -> IResult<&str, Color> {
    // Skip optional separators
    let (input, _) = multispace0(input)?;
    let (input, _) = opt(char(','))(input)?;
    let (input, _) = multispace0(input)?;
    
    let (input, _) = tag_no_case("COLOR")(input)?;
    let (input, _) = char('=')(input)?;
    let (input, color_str) = take_while1(|c: char| c.is_alphabetic())(input)?;
    
    Ok((input, Color::from_str(color_str)))
}

/// Helper: Parse TYPE=value (for DFHMND)
fn parse_field_type(input: &str) -> IResult<&str, FieldType> {
    let (input, _) = tag_no_case("TYPE")(input)?;
    let (input, _) = char('=')(input)?;
    let (input, type_str) = take_while1(|c: char| c.is_alphabetic())(input)?;
    
    match type_str.to_uppercase().as_str() {
        "MAP" => Ok((input, FieldType::Map)),
        "FIELD" => Ok((input, FieldType::Field)),
        "LITERAL" => Ok((input, FieldType::Literal)),
        "GRP" => Ok((input, FieldType::Group)),
        "ATTRB" => Ok((input, FieldType::Attribute)),
        "SYMBOLIC" => Ok((input, FieldType::Symbolic)),
        _ => Ok((input, FieldType::Unknown(type_str.to_string()))),
    }
}

/// Helper: Parse INITIAL='value'
fn parse_initial(input: &str) -> IResult<&str, String> {
    let (input, _) = tag_no_case("INITIAL")(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value) = delimited(
        char('\''),
        take_until("'"),
        char('\''),
    )(input)?;
    
    Ok((input, value.to_string()))
}

/// Helper: Parse PIC='value'
fn parse_pic(input: &str) -> IResult<&str, String> {
    let (input, _) = tag_no_case("PIC")(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value) = delimited(
        char('\''),
        take_until("'"),
        char('\''),
    )(input)?;
    
    Ok((input, value.to_string()))
}

/// Helper: Parse a generic attribute value (TYPE=value, MAPSET=value, etc.)
fn parse_attribute_value<'a>(attr_name: &str, input: &'a str) -> IResult<&'a str, String> {
    // Skip optional separators (comma, space) before the attribute
    let (input, _) = multispace0(input)?;
    let (input, _) = opt(char(','))(input)?;
    let (input, _) = multispace0(input)?;
    
    let (input, _) = tag_no_case(attr_name)(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value) = take_while1(|c: char| c != ',' && !c.is_whitespace())(input)?;
    Ok((input, value.to_string()))
}

/// Helper: Parse unsigned 16-bit integer
fn parse_u16(input: &str) -> IResult<&str, u16> {
    map(digit1, |s: &str| s.parse().unwrap_or(0))(input)
}

/// FieldAttribute for Unknown variant
impl FieldAttribute {
    pub fn unknown(s: &str) -> Self {
        FieldAttribute::Unknown(s.to_string())
    }
}
