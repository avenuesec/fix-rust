
#[macro_use] extern crate serde_derive;

extern crate fixcodegen;
extern crate handlebars;
extern crate xmltree;
extern crate argparse;

use handlebars::{Handlebars, Helper, Renderable, RenderContext, RenderError, to_json};
use argparse::{ArgumentParser, Store};

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path,PathBuf};
use std::ffi::OsStr;

use fixcodegen::*;


fn main() {

    let mut tpl_path = String::new();
    let mut out_path = String::new();
    let mut fix_path = String::new();

    {
        let desc= format!("Generates the fix message model, builder and parser code from a fix xml dictionary");
        let mut ap = ArgumentParser::new(); // <- https://github.com/tailhook/rust-argparse
        ap.set_description( desc.as_str() );
        ap.refer(&mut fix_path)
            .add_argument("fix", Store, "Fix dictionary file such as FIX44.xml").required();
        ap.refer(&mut tpl_path)
            .add_argument("tpl", Store, "Template path").required();
        ap.refer(&mut out_path)
            .add_argument("out", Store, "Output path").required();
        ap.parse_args_or_exit();
    }

    let model_output  = combine(&out_path, "fixmodel.rs");
    let parser_output = combine(&out_path, "parsing.rs");
    let gen_output    = combine(&out_path, "gen.rs");

    println!("generated templates will go to {}", &out_path );
    println!("{}", model_output);
    println!("{}", parser_output);
    println!("{}", gen_output);

    let gen = FixCodeGen::new(&fix_path);
    gen.render("parsing",  &combine(&tpl_path, "parsing.hbs"), &parser_output ).unwrap();
    gen.render("fixmodel", &combine(&tpl_path, "model.hbs"), &model_output ).unwrap();
    gen.render("gen",      &combine(&tpl_path, "gen.hbs"), &gen_output ).unwrap();
    
}

//fn combine<'a>(path : &'a mut PathBuf, a: &'a str) -> String {
//    path.set_file_name(a);
//    path.to_str().unwrap().to_string()
//}

fn combine(path : &str, with: &str) -> String {
    let mut builder = String::new();
    builder.push_str(path);
    if !path.ends_with('/') {
        builder.push_str("/");
    }
    builder.push_str(with);
    builder
}