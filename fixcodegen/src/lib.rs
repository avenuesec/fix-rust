
// From http://forum.fixtrading.org/t/body-fields-ordering/209
// 
// The rule-of-thumb is this: When formatting an outgoing message, 
// order fields and components exactly as specified in the standard 
// or as extended in your ROE. When parsing an incoming message expect and allow discrepancies.
// Even though the official rules require strict field ordering for 
// repeating groups you can require just this: The NumInGroup field 
// gives the count of instances of the first tag in the component. 
// 
// If there are to be three instances then the fields between the 
// first appearance of the first tag and the second appearance belong 
// in the first instance, between the second appearance and the third 
// appearance in the second instance. After the third appearance is 
// the challenge - the first tag in sequence that is not defined in 
// the repeating group - not necessarily in proper order - ends the third instance.
// 
// The fields in a non-repeating component at the base of a message, 
// e.g. Instrument in NewOrderSingle often appear randomly throughout 
// the base - all expectations are off. The fields of a non-repeating 
// component embedded in a repeating component, e.g. InstrumentLeg 
// within LegOrdGrp, OUGHT to appear in standard order but the incoming-message 
// parser should not require it.

#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate handlebars;
extern crate xmltree;
extern crate serde_json;
extern crate time;

use std::collections::*;
use std::io::{Error};
use std::io::{Read};
use std::str::{FromStr};
use std::default::Default;
use std::ascii::AsciiExt;

use handlebars::{Handlebars, Helper, RenderContext, RenderError};
use xmltree::Element;

pub mod model;
use model::*;

pub struct FixCodeGen {
    data : BTreeMap<String, serde_json::Value>,
}

impl FixCodeGen {

    pub fn new(fixfilename: &str) -> FixCodeGen {
        let mut full_fix = String::new();

        std::fs::File::open(fixfilename)
            .expect( &format!("Failed to open {}", fixfilename) )
            .read_to_string(&mut full_fix)
            .expect( &format!("Failed to read {}", fixfilename) );

        let fix_xml = Element::parse(full_fix.as_bytes()).unwrap();

        // TODO: transitive closure to keep fields and components that were referenced by the messages. remove all else.

        let fields          : Vec<FixField>     = fix_xml.get_child("fields").unwrap().children.iter().map(build_field).collect();
        let mut header_flds : Vec<MessageEntry> = build_message_refs( fix_xml.get_child("header").unwrap() );
        let messages        : Vec<Message>      = fix_xml.get_child("messages").unwrap().children.iter().map(|m| { build_message(m) }).collect();
        let cptns           : Vec<FixComponent> = fix_xml.get_child("components").unwrap().children.iter().map(|c| { build_fix_component(c) } ).collect();
        let mut field_names : HashSet<String>   = HashSet::new(); // &fields.iter().map(|f| { *f.name.clone() } ).collect();
        {
            for f in &fields {
                field_names.insert( f.name.clone() );
            }
        }

        // remove fields that are kind of specially handled by code gen
        remove_fieldref( &mut header_flds, "BeginString" );
        remove_fieldref( &mut header_flds, "BodyLength" );

        // flat list of groups found on messages, components and groups (nested)
        let mut all_groups : HashSet<&FixGroup> = HashSet::new();
        collect_groups_recursive(&mut messages.iter(), &mut all_groups);
        collect_groups_recursive(&mut cptns.iter(),    &mut all_groups);

        // one-to-one unique group to struct/fn to decode it
        // note that groups may have the same "name" like NoRelatedSym
        let unique_group_ids   = uniquefy_group(&all_groups);
        let components_map     : HashMap<&String, &FixComponent> = cptns.iter().map(|c| { (&c.name, c) } ).collect();
        let fields_map_by_name : HashMap<&String, &FixField>     = fields.iter().map(|c| { (&c.name, c) } ).collect();
        let groups_map_by_name : HashMap<&String, &FixGroup>     = all_groups.iter().map(|c| { (&c.name, *c) } ).collect();

        let flat_parser_msgs : Vec<FlatMessage> = 
            // fix_xml.get_child("messages").unwrap().children.iter()
            messages.iter()
            .map(|m| { build_flat_message(m, &unique_group_ids, &groups_map_by_name, &components_map, &fields_map_by_name, &mut field_names) })
            .collect();
        let flat_parser_comps : Vec<FlatComponent> = cptns.iter()
            .map(|m| { build_flat_parser_component(m, &unique_group_ids, &groups_map_by_name, &components_map, &fields_map_by_name, &mut field_names) })
            .collect();
        let flatgroups : Vec<FlatGroup> = unique_group_ids.iter()
            .map(|kv| build_flat_group(kv, &unique_group_ids, &groups_map_by_name, &components_map, &fields_map_by_name, &mut field_names))
            .collect();
        let header = FixHeader {
            fields: entries_to_flat_fields(None, &header_flds, &unique_group_ids, &groups_map_by_name, &components_map, &fields_map_by_name, &mut field_names),
        };

        let mut data : BTreeMap<String, serde_json::Value> = BTreeMap::new();

        let flatmessages  = serde_json::to_value(&flat_parser_msgs).unwrap();
        let fields_json   = serde_json::to_value(&fields).unwrap();
        let cptns_json    = serde_json::to_value(flat_parser_comps).unwrap();
        let groups_json   = serde_json::to_value(flatgroups).unwrap();
        let header_json   = serde_json::to_value(header).unwrap();

        // let ioi_message = flat_parser_msgs.iter().find(|f| f.name == "IOI").unwrap();
        // println!(" {:#}", serde_json::to_value(ioi_message).unwrap() );

        // println!("fields_map {:#}", fields_map );
        data.insert("flatmessages".to_string(), flatmessages);
        data.insert("fields".to_string(),       fields_json);
        data.insert("components".to_string(),   cptns_json);
        data.insert("flatgroups".to_string(),   groups_json);
        data.insert("header".to_owned(),        header_json);

        // println!("unused fields:");
        // {
        //     for f in field_names.iter().sorted() {
        //         println!("\t{}", f);
        //     }
        // }
        println!("Total unused: {}", field_names.len());

        FixCodeGen { data }
    }

    pub fn render(&self, tpl_name: &str, tpl_filename: &str, output: &str) -> Result<(), Error> {
        let mut full_tpl = String::new();
        
        std::fs::File::open(tpl_filename)
            .expect( &format!("Failed to open {}", tpl_filename) )
            .read_to_string(&mut full_tpl)
            .expect( &format!("Failed to read {}", tpl_filename) );

        let mut out = std::fs::File::create(output).expect( &format!("Failed to create file {}", output) );

        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(handlebars::no_escape);
        handlebars.register_helper("camel",         Box::new(camel_helper));
        handlebars.register_helper("snake",         Box::new(snake_helper));
        handlebars.register_helper("upper",         Box::new(upper_helper));
        handlebars.register_helper("capitalize",    Box::new(capitalize_helper));
        handlebars.register_helper("chainvname",    Box::new(chainvname_helper));
        handlebars.register_helper("mutchainvname", Box::new(mutchainvname_helper));

        handlebars.register_template_string(tpl_name, full_tpl).expect("Failed to register full template");

        // writeln!(out, "{}", handlebars.render(tpl_name, &self.data).expect("Failed to render full template")).expect("Failed to write generated.rs");
        println!("rendering {} ...", output);
        let start = time::precise_time_s();
        handlebars.renderw(tpl_name, &self.data, &mut out).expect("Failed to render template");
        let end = time::precise_time_s();
        println!("done in {}", (end - start) );

        Ok( () ) 
    }
}

fn uniquefy_group<'a>(groups: &HashSet<&'a FixGroup>) -> HashMap<&'a FixGroup, String> {
    let mut map = HashMap::<&FixGroup, String>::new();
    let mut counter = 1;

    let mut sorted : Vec<&&'a FixGroup> = groups.iter().collect();
    sorted.as_mut_slice().sort();

    for g in sorted {
        let name = &g.name;
        map.insert(g, format!("{}{}", name, counter));
        counter = counter + 1;
    };

    map
}

fn collect_groups_recursive<'a, T : FixComposite>(source: & mut Iterator<Item=&'a T>, groups: &mut HashSet<&'a FixGroup>)-> () {

    while let Some(item) = source.next() {
        // Item is FixComposite: Message or Component
        collect_groups(item, groups);
    }
}

fn collect_groups<'a, T : FixComposite>(source: &'a T, groups: &mut HashSet<&'a FixGroup>) -> () {

    for e in source.entries().iter() {
        match e {
            &MessageEntry::Group(_, _, ref gr) => { 
                groups.insert( gr );
                collect_groups(gr, groups);
            },
            _ => {  }
        }
    }
}

fn build_flat_parser_component(c: &FixComponent, 
                               group_to_uniqueid : &HashMap<&FixGroup, String>, 
                               groups: &HashMap<&String, &FixGroup>, 
                               comps:  &HashMap<&String, &FixComponent>,
                               fields_map: &HashMap<&String, &FixField>, field_names: &mut HashSet<String>) -> FlatComponent {
    
    let flat_fields = entries_to_flat_fields(None, &c.entries, group_to_uniqueid, groups, comps, fields_map, field_names );

    FlatComponent {
        rust_type: format!("{}Fields", c.name),
        // name: name.clone(),
        fields: flat_fields
    }
}

fn build_flat_group( kv:(&&FixGroup, &String), 
                     group_to_uniqueid : &HashMap<&FixGroup, String>, 
                     groups: &HashMap<&String, &FixGroup>, 
                     comps:  &HashMap<&String, &FixComponent>,
                     fields_map: &HashMap<&String, &FixField>, field_names: &mut HashSet<String>) -> FlatGroup {
    let group = kv.0;
    // let name = kv.1;

    let unique_id = group_to_uniqueid.get(group).unwrap();

    field_names.remove( &group.name );

    let fields = entries_to_flat_fields(None, &group.entries, group_to_uniqueid, groups, comps, fields_map, field_names );

    FlatGroup {
        name: group.name.clone(),
        rust_type: format!("{}Fields", unique_id),
        fields
    }
}

fn build_flat_message(msg : &Message, 
                      group_to_uniqueid : &HashMap<&FixGroup, String>, 
                      groups: &HashMap<&String, &FixGroup>, 
                      comps:  &HashMap<&String, &FixComponent>,
                      fields_map: &HashMap<&String, &FixField>, field_names: &mut HashSet<String> ) -> FlatMessage {
    
    let fields = entries_to_flat_fields(None, &msg.entries, group_to_uniqueid, groups, comps, fields_map, field_names);

    FlatMessage {
        name: msg.name.clone(),
        rust_type: format!("{}Fields", msg.name), 
        msg_type: msg.msg_type.clone(),
        is_admin: msg.is_admin,
        fields
    }
}

fn entries_to_flat_fields(parent: Option<FlatField>,
                          entries: &Vec<MessageEntry>, 
                          group_to_uniqueid : &HashMap<&FixGroup, String>, 
                          groups: &HashMap<&String, &FixGroup>, 
                          comps:  &HashMap<&String, &FixComponent>,
                          fields_map: &HashMap<&String, &FixField>, 
                          // requires_init: bool, 
                          field_names: &mut HashSet<String>) -> Vec<FlatField> {

    let is_top_level = parent.is_none();
    let mut flat_fields = Vec::new();

    for entry in entries {
        match entry {
            &MessageEntry::FieldRef(ref name, is_required) => {

                let field_def = fields_map.get(name).unwrap();
                let rust_type = fix_type_to_rust(field_def);
                let conv      = fld_type_to_conv(field_def);
                let inv_conv  = rust_type_to_conv(field_def);
                field_names.remove(&field_def.name);

                let flat_fld = FlatField {
                    name : name.clone(),
                    fld_tag: field_def.tag,
                    is_top_level, // top level or the field within a component?
                    vname : snake_case(name), // format!("{}{}", parent_name, snake_case(name)), // variable name: snake cased, prefixed
                    is_simple : true,  // simple, group, component
                    rust_type, // u32, String, something else?
                    rust_type_converter : conv, // .to_string? parse?
                    fix_type_converter: inv_conv,
                    is_required,
                    parent: match &parent { &Some(ref p) => Some(Box::new(p.clone())), _ => None },
                    .. Default::default()
                };
                flat_fields.push(flat_fld);
            },
            
            &MessageEntry::ComponentRef(ref name, is_required) => {
                
                let flat_fld = FlatField {
                    name: name.clone(),
                    is_top_level, // top level or the field within a component?
                    vname : snake_case(name), // format!("{}{}", parent_name, snake_case(name)), // variable name: snake cased, prefixed
                    is_component : true,  // simple, group, component
                    rust_type : format!("{}Fields", name),  // ""rust_type.to_string(), // u32, String, something else?
                    rust_type_converter : "".to_string(), // .to_string? parse?
                    is_required,
                    parent: match &parent { &Some(ref p) => Some(Box::new(p.clone())), _ => None },
                    .. Default::default()
                };
                flat_fields.push(flat_fld.clone());

                // let composed_name = format!("{}{}", parent_name, snake_case(name));
                let sub_comp = comps.get(name).unwrap();

                let mut subfields = 
                    entries_to_flat_fields(
                        Some(flat_fld.clone()), &sub_comp.entries, 
                        group_to_uniqueid, groups, comps, fields_map, field_names);

                flat_fields.append(&mut subfields);
            }, 
            
            &MessageEntry::Group(ref name, is_required, ref group_def) => {
                
                let unique_id = group_to_uniqueid.get(group_def).unwrap();
                let rust_type = format!("Vec<{}Fields>", unique_id);

                let flat_fld = FlatField {
                    name: name.clone(),
                    is_top_level, // top level or the field within a component?
                    vname : snake_case(name), // format!("{}{}", parent_name, snake_case(name)), // variable name: snake cased, prefixed
                    is_group : true,  // simple, group, component
                    rust_type : rust_type.to_string(), // u32, String, something else?
                    rust_type_converter : "".to_string(), // .to_string? parse?
                    is_required,
                    parent: match &parent { &Some(ref p) => Some(Box::new(p.clone())), _ => None },
                    group_builder_fn_name : format!("group_{}_fields", snake_case(unique_id)),
                    .. Default::default()
                };

                flat_fields.push(flat_fld);
            },
        }
    }

    flat_fields
}

fn build_message(el: &Element) -> Message {
    let name = el.attributes.get( "name" ).unwrap();
    let msg_type = el.attributes.get( "msgtype" );
    let is_admin = el.attributes.get( "msgcat" ).map_or(false, |v| v == "admin");

    let entries = build_message_refs(el);

    let msg_type2 = match msg_type {
        Some(s) => { s.to_owned() },
        None => { String::new() }
    };

    Message { name: name.clone(), msg_type: msg_type2, entries, is_admin }
}

fn build_message_refs(el : &Element) -> Vec<MessageEntry> {
    let mut entries = Vec::new();

    for c in &el.children {
        let name = c.attributes.get( "name" ).unwrap();
        let is_required = c.attributes.get( "required" ).unwrap() == "Y";

        let entry = match c.name.as_str() {
            "field" => {
                MessageEntry::FieldRef(name.clone(), is_required)
            },
            "component" => {
                MessageEntry::ComponentRef(name.clone(), is_required)
            },
            "group" => {
                let group = build_fix_group(c);
                MessageEntry::Group(name.clone(), is_required, group)
            },
            _ => { panic!("unsupported {}", c.name); }
        };

        entries.push(entry);
    }
    entries
}

fn build_fix_group(el: &Element) -> FixGroup {
    // let name = el.attributes.get( "name" ).unwrap();
    let message = build_message( el );

    FixGroup { name: message.name, entries: message.entries }
}

fn build_fix_component(el: &Element) -> FixComponent {
    // let name = el.attributes.get( "name" ).unwrap();
    let message = build_message( el );

    FixComponent { name: message.name.clone(), entries: message.entries }
}

fn build_field(el: &Element) -> FixField {
    let tag = i32::from_str(el.attributes.get( "number" ).unwrap()).unwrap();
    let name = el.attributes.get( "name" ).unwrap();
    let fld_type = el.attributes.get( "type" ).unwrap();
    let children = &el.children;
    let mut enum_vals : Vec<FixFieldEnum> = vec![];

    let is_enum = fld_type != "BOOLEAN" && children.len() != 0;

    if is_enum {
        enum_vals.append(&mut children.iter().map(build_enum_val).collect() );
    }

    FixField { 
        tag,
        name: name.clone(), 
        fld_type: fld_type.clone(), 
        enum_vals,
        is_enum
    }
}

fn build_enum_val(el: &Element) -> FixFieldEnum {
    // <value enum='0' description='INVALIDTAGNUMBER' />
    let val = el.attributes.get("enum").unwrap();
    let mut desc = el.attributes.get("description").unwrap().clone();
    // let mut desc = match el.attributes.get("description") {
    //     Some(s) => { s.to_owned() },
    //     None => { String::new() }
    // };
    if (&desc.as_str().chars().nth(0).unwrap()).is_digit(10)  {
        desc.insert(0, '_');
    }
    FixFieldEnum {
        enum_val: val.clone(), 
        description: desc
    }
}

pub fn chainvname_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let value = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"chainvname\""))?;
    let param = value.value();

    let mut name = String::new();
    rec_chain_name(param, &mut name, false);

    rc.writer.write_all(name.as_bytes())?;
    Ok(())
}

pub fn mutchainvname_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let value = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"chainvname\""))?;
    let param = value.value();

    let mut name = String::new();
    rec_chain_name(param, &mut name, true);

    rc.writer.write_all(name.as_bytes())?;
    Ok(())
}

fn rec_chain_name(fld: &serde_json::Value, writer: &mut String, for_mut: bool) {
    match fld {
        &serde_json::Value::Object(ref map) => {
            // rec on parent first
            match map.get("parent") {
                Some(p) => { rec_chain_name(p, writer, for_mut); },
                _ => { }
            }
            let name = map.get("vname").unwrap().as_str().unwrap();
            let is_not_simple = map.get("is_component").unwrap().as_bool().unwrap();
            let is_empty = writer.len() == 0;

            if !is_empty { writer.push_str("."); }
            writer.push_str(name); 
            if for_mut && is_not_simple { writer.push_str(".as_mut().unwrap()"); }
        },
        _ => { }
    }
}

pub fn fix_type_to_rust (fld : &FixField) -> String {

    if fld.is_enum {
        format!("Field{0}Enum", &fld.name)
    } else {
        match fld.fld_type.as_str() {
            "STRING"      => { "String" },
            "DATA"        => { "String" },
            "EXCHANGE"    => { "String" },
            "COUNTRY"     => { "String" },
            "MULTIPLEVALUESTRING" => { "String" },
            "CHAR"        => { "char" },
            "PRICEOFFSET" => { "f32" },
            "PRICE"       => { "f32" },
            "FLOAT"       => { "f32" },
            "PERCENTAGE"  => { "f32" },
            "AMT"         => { "f32" },
            "CURRENCY"    => { "f32" },
            "QTY"         => { "f32" },
            "SEQNUM"      => { "u32" },
            "INT"         => { "i32" },
            "LENGTH"      => { "usize" },
            "NUMINGROUP"  => { "usize" },
            "LOCALMKTDATE" => { "UtcDateTime" }, // should we represent local?
            "UTCTIMESTAMP" => { "UtcDateTime" },
            "UTCDATE" |
            "UTCDATEONLY" => { "UtcDate" },
            "UTCTIME" |
            "UTCTIMEONLY" => { "UtcTime" },
            "MONTHYEAR"   => { "UtcDate" },
            "DAYOFMONTH"  => { "i32" },
            "BOOLEAN"     => { "bool" },
            _ => { panic!(format!("Could not map type {} to rust type", fld.fld_type)); }
        }.to_string()
    }
}

pub fn fld_type_to_conv (fld : &FixField) -> String {

    if fld.is_enum {
        format!("Field{0}Enum::from_str(v).unwrap()", &fld.name)
    } else {
         match fld.fld_type.as_str() {
            "STRING"      |
            "DATA"        |
            "EXCHANGE"    |
            "COUNTRY"     |
            "MULTIPLEVALUESTRING" => { "v.to_string()" },
            "CHAR"        => { "char::from_str(v).unwrap()" },
            "PRICEOFFSET" |
            "PRICE"       |
            "FLOAT"       |
            "PERCENTAGE"  |
            "AMT"         |
            "CURRENCY"    |
            "QTY"         => { "f32::from_str(v).unwrap()" },
            "SEQNUM"      => { "u32::from_str(v).unwrap()" },
            "INT"         => { "i32::from_str(v).unwrap()" },
            "DAYOFMONTH"  => { "i32::from_str(v).unwrap()" },
            "LENGTH"      => { "usize::from_str(v).unwrap()" },
            "NUMINGROUP"  => { "usize::from_str(v).unwrap()" },
            "UTCTIMESTAMP" => { "UtcDateTime::from_str(v).unwrap()" },
            "LOCALMKTDATE" => { "UtcDateTime::from_str(v).unwrap()" },
            "UTCDATE"     |
            "UTCDATEONLY" => { "UtcDate::from_str(v).unwrap()" },
            "UTCTIME"     |
            "UTCTIMEONLY" => { "UtcTime::from_str(v).unwrap()" },
            "MONTHYEAR"   => { "UtcDate::from_str(v).unwrap()" },
            "BOOLEAN"     => { "boolconv(v)" },
            _ => { panic!(format!("Could not map type {} to rust conversion", fld.fld_type)); }
        }.to_string()
    }
}

pub fn rust_type_to_conv (fld : &FixField) -> String {

    match fld.fld_type.as_str() {
        "BOOLEAN"     => { "to_boolconv(val)" },
        _ => { "val" }
    }.to_string()
}

// From lapin https://github.com/sozu-proxy/lapin

pub fn camel_helper (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let value = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"camel\""))?;
    let param = value.value().as_str().ok_or_else(|| RenderError::new("Non-string param given to helper \"camel\""))?;
    rc.writer.write_all(camel_case(param).as_bytes())?;
    Ok(())
}

pub fn snake_helper (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let value = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"snake\""))?;
    let param = value.value().as_str().ok_or_else(|| RenderError::new("Non-string param given to helper \"snake\""))?;
    rc.writer.write_all(snake_case(param).as_bytes())?;
    Ok(())
}

pub fn upper_helper (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let value = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"upper\""))?;
    let param = value.value().as_str().ok_or_else(|| RenderError::new("Non-string param given to helper \"upper\""))?;
    rc.writer.write_all(upper_case(param).as_bytes())?;
    Ok(())
}

pub fn capitalize_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let value = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"capitalize\""))?;
    let param = value.value().as_str().ok_or_else(|| RenderError::new("Non-string param given to helper \"capitalize\""))?;
    rc.writer.write_all(capitalize(param.trim()).as_bytes())?;
    Ok(())
}

pub fn upper_case(name: &str) -> String {
    name.to_ascii_uppercase()
}

pub fn camel_case(name: &str) -> String {
    let mut new_word = true;
    name.chars().fold("".to_string(), |mut result, ch| {
        if ch == '-' || ch == '_' || ch == ' ' {
            new_word = true;
            result
        } else {
            result.push(if new_word { ch.to_ascii_uppercase() } else { ch });
            new_word = false;
            result
        }
    })
}

pub fn capitalize(name: &str) -> String {
    let mut new_word = true;
    name.chars().fold("".to_string(), |mut result, ch| {
        if result.len() == 0 && ch.is_digit(10) {
            result.push('A'); // arbitrary prefix
        }
        if ch == '-' || ch == '_' || ch == ' ' {
            new_word = true;
        } else {
            result.push(if new_word { ch.to_ascii_uppercase() } else { ch.to_ascii_lowercase() });
            new_word = false;
        }
        result
    })
}

pub fn snake_case(name: &str) -> String {
    match name {
        "type"   => "type_".to_string(),
        "Yield"  => "yield_".to_string(),
        "return" => "return_".to_string(),
        name     => {
            let mut new_word       = false;
            let mut last_was_upper = false;
            name.chars().fold("".to_string(), |mut result, ch| {
                if ch == '-' || ch == '_' || ch == ' ' {
                    new_word = true;
                    result
                } else {
                    let uppercase = ch.is_uppercase();
                    if new_word || (!last_was_upper && !result.is_empty() && uppercase) {
                        result.push('_');
                        new_word = false;
                    }
                    last_was_upper = uppercase;
                    result.push(if uppercase { ch.to_ascii_lowercase() } else { ch });
                    result
                }
            })
        }
    }
}

fn remove_fieldref( fields: &mut Vec<MessageEntry>, name : &str ) {
    let to_remove = fields.iter().enumerate().find(move |t| {
        match t.1 {
            &MessageEntry::FieldRef(ref name_, _) => {
                name_ == name
            },
            _ => false
        }
    } ).unwrap().0 ;

    fields.remove( to_remove );
}

mod tests {

    #[test]
    fn test_capitalize() {
        assert_eq!("Description", capitalize("DESCRIPTION"));
        assert_eq!("DescriptionOfSomething", capitalize("DESCRIPTION_OF_SOMETHING"));
    }

    #[test]
    fn test_capitalize_with_invalid_starts() {
        assert_eq!("A5yr", capitalize("5YR"));
    }

}