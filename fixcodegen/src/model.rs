
extern crate serde;

pub trait FixComposite {
    fn entries(&self) -> &Vec<MessageEntry>;
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Message {
    pub name: String,
    pub msg_type : String,
    pub is_admin : bool,
    pub entries : Vec<MessageEntry>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct FixHeader {
    pub fields : Vec<FlatField>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct FlatMessage {
    pub name: String,
    pub msg_type : String,
    pub is_admin : bool,
    pub rust_type: String, // fields struct name
    pub fields : Vec<FlatField>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct FlatComponent {
    // pub name: String,
    pub rust_type: String,
    pub fields : Vec<FlatField> 
}

#[derive(Serialize,Deserialize,Debug)]
pub struct FlatGroup {
    pub name: String,
    pub rust_type: String,
    pub fields : Vec<FlatField> 
}

#[derive(Serialize,Deserialize,Debug,Default,Clone)]
pub struct FlatField {
    pub name : String,
    pub fld_tag: i32,
    pub is_top_level : bool, // top level or the field within a component?
    pub vname : String, // variable name: snake cased, prefixed
    pub is_simple : bool,
    pub is_component : bool,
    pub is_group : bool,
    pub rust_type : String, // u32, String, something else?
    pub rust_type_converter : String, // .to_string? parse?
    pub fix_type_converter : String, 
    pub is_required : bool,
    pub group_builder_fn_name : String,
    // pub requires_init : bool, // if component, adds init code before trying to assign to member
    pub parent : Option<Box<FlatField>>, // not flat, but sadly required by components
}

#[derive(Serialize,Deserialize,PartialEq,Eq,Hash,Debug)]
pub enum MessageEntry {
    FieldRef(String, bool), // Name + required
    ComponentRef(String, bool),
    Group(String, bool, FixGroup),
}

#[derive(Serialize,Deserialize,Debug)]
pub struct FixField {
    pub tag : i32,
    pub name : String,
    pub fld_type : String,
    pub enum_vals : Vec<FixFieldEnum>,
    pub is_enum : bool,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct FixFieldEnum {
    pub enum_val : String,
    pub description : String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct FixComponent {
    pub name : String,
    pub entries : Vec<MessageEntry>,
}

#[derive(Serialize,Deserialize,PartialEq,Eq,Hash,Debug)]
pub struct FixGroup {
    pub name: String,
    pub entries : Vec<MessageEntry>,
}

use std::cmp::{PartialOrd, Ord, Ordering};

impl PartialOrd<FixGroup> for FixGroup {  
     
    fn partial_cmp(&self, other: &FixGroup) -> Option<Ordering> {
        self.name.partial_cmp( &other.name )
    }
}
impl Ord for FixGroup {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp( &other.name )
    }
}

impl FixComposite for Message {
    fn entries(&self) -> &Vec<MessageEntry> {
        &self.entries
    }
}
impl FixComposite for FixGroup {
    fn entries(&self) -> &Vec<MessageEntry> {
        &self.entries
    }
}
impl FixComposite for FixComponent {
    fn entries(&self) -> &Vec<MessageEntry> {
        &self.entries
    }
}