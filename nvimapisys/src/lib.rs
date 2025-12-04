#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(trim_prefix_suffix)]
// mod out;
use core::ops::ControlFlow;
use std::io::{Write, stdout};
use rmpv::Value;
use serde::{Deserialize, Serialize};
mod nvimrpc;
pub use nvimrpc::Nvimapi;
pub mod error;


const VALUE_SUFFIX: &str = "_wv";
const API_DOC_FILE: &str = "/usr/share/nvim/runtime/doc/api.txt";

pub fn main() {
    let data = include_bytes!("nvimapi.msgpack");
    let v = rmpv::decode::read_value(&mut data.as_slice()).unwrap();
    let root = Vec::try_from(v).unwrap();
    println!("{HEADER_COMMON}");
    println!("{HEADER_VALUE}");
    println!("{HEADER_SERDE}");
    for (key, value) in root {
        if key.as_str().unwrap() == "functions" {
            println!("impl Nvimrpc {{");
            handle_functions(value);
            println!("}}");
        }
    }
}

const HEADER_COMMON: &str = r###"
type Nil = ()    ;
type Boolean = bool;
type Integer = i64;
type Float = f64  ;
pub struct Buffer(pub Integer);
pub struct Window(pub Integer);
pub struct Tabpage(pub Integer);
use crate::Nvimrpc;
"###;
const HEADER_VALUE: &str = r###"
use rmpv::Value;
type Array  = Vec<Value>;
type Dict   = Vec<(Value,Value)>;
type Object = Value;
impl From<Buffer> for Value {
    fn from(that: Buffer) -> Self {
        Value::from(that.0)
    }
}
impl From<Window> for Value {
    fn from(that: Window) -> Self {
        Value::from(that.0)
    }
}
impl From<Tabpage> for Value {
    fn from(that: Tabpage) -> Self {
        Value::from(that.0)
    }
}
"###;
const HEADER_SERDE: &str = r###"
use serde::{Deserialize, Serialize};
impl serde::Serialize for Buffer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        i64::serialize(&self.0, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for Buffer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        Ok(Self(i64::deserialize(deserializer)?))
    }
}
impl serde::Serialize for Window {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        i64::serialize(&self.0, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for Window {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        Ok(Self(i64::deserialize(deserializer)?))
    }
}
impl serde::Serialize for Tabpage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        i64::serialize(&self.0, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for Tabpage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        Ok(Self(i64::deserialize(deserializer)?))
    }
}
"###;


fn handle_functions(value: Value) {
    let functions = Vec::<Value>::try_from(value).unwrap();
    let mut buffer = Default::default();
    let ignored_types = ["LuaRef",];
    // let generit_types = ["Array", "Dict", "Object"];
    let api_doc = std::fs::read_to_string(API_DOC_FILE).unwrap_or_default();
    let api_doc: Vec<&str> = api_doc.split("\n").collect();
    'outer: for fun in functions {
        if let ControlFlow::Break(_) = handle_fun(&mut buffer, &ignored_types, &fun, false, &api_doc) {
            continue 'outer;
        }
        println!("{buffer}");
        if buffer.contains("Serialize") || buffer.contains("Deserialize") {
            let vf = handle_fun(&mut buffer, &ignored_types, &fun, true, &api_doc);
            assert!(matches!(vf, ControlFlow::Continue(_)), "if it was fine with serde it should be fine with value.");
            println!("{buffer}");
        }
    }
}

fn handle_fun(buffer: &mut String, ignored_types: &[&str], fun: &Value, use_value: bool, api_doc: &[&str]) -> ControlFlow<()> {
    buffer.clear();
    let deprecated = value_get(&fun, "deprecated_since");
    if deprecated.is_some() { return ControlFlow::Break(()); }
    let fn_name = value_get(&fun, "name").unwrap().as_str().unwrap();
    let doc = get_doc_for_fn(fn_name, api_doc);
    buffer.push_str(&doc);
    buffer.push_str("pub async ");
    buffer.push_str("fn ");
    buffer.push_str(fn_name.trim_prefix("nvim_"));
    if use_value {
        buffer.push_str(VALUE_SUFFIX);
    }
    buffer.push_str("(&mut self, ");
    let params = value_get(&fun, "parameters").unwrap().as_array().unwrap();
    let mut pnames = Vec::with_capacity(params.len());
    for param in params {
        let param = param.as_array().unwrap();
        let p_type = param[0].as_str().unwrap();
        if ignored_types.contains(&p_type) { return ControlFlow::Break(()); }
        let p_type =
            if use_value {
                param_type_to_value(p_type)
            } else {param_type_to_serde(p_type)};
        let p_name = param[1].as_str().unwrap();
        pnames.push(p_name);
        buffer.push_str(p_name);
        buffer.push_str(": ");
        buffer.push_str(p_type);
        buffer.push_str(", ");
    }
    let ret_type = value_get(&fun, "return_type").unwrap().as_str().unwrap();
    let ret_type = if use_value {return_type_to_value(ret_type)}
        else {return_type_to_serde(ret_type)};
    buffer.push_str(") -> ");
    buffer.push_str(ret_type);
    buffer.push_str(" {\n");
    {// inside of fn, just call the call_fn with  tuple as arg.
        buffer.push_str("\t"); // indentation, remove maybe
        buffer.push_str("self.call_fn");
        if use_value { buffer.push_str(VALUE_SUFFIX); }
        buffer.push('(');
        buffer.push('"');
        buffer.push_str(fn_name);
        buffer.push('"');
        if pnames.is_empty() {
            buffer.push_str(", [();0]");
        } else {
            buffer.push_str(", (");
            for pname in pnames {
                buffer.push_str(pname);
                buffer.push_str(", ");
            }
            buffer.push_str(")");
        }
        buffer.push_str(")");
        buffer.push_str(".await");
        buffer.push_str("\n");
    }
    buffer.push_str("}");
    return ControlFlow::Continue(());
}
fn get_doc_for_fn(name: &str, api_doc: &[&str]) -> String {
    let to_search = format!("*{name}()*");
    let mut write_it = false;
    // let's say 22 lines with 50 chars each?.
    let mut rv = String::with_capacity(22 * 50);
    for &line in api_doc {
        if write_it {
            if line.ends_with("*") { return rv; }
            rv.push_str("///");
            rv.push_str(line);
            rv.push_str("\n");
        }
        else if line.contains(&to_search) {
            write_it = true;
            rv.push_str("///");
            rv.push_str(line);
            rv.push_str("\n");
        }
    }
    return rv;
}

fn value_get<'v>(map: &'v Value, key: &str) -> Option<&'v Value> {
    let map = map.as_map().unwrap();
    for (k,v) in map {
        if k.as_str().unwrap() == key {
            return Some(v);
        }
    }
    return None;
}

fn param_type_to_value(type_: &str) -> &'static str {
    match type_ {
        "Nil"              => "Nil",
        "Boolean"          => "Boolean",
        "Integer"          => "Integer",
        "Float"            => "Float",
        "String"           => "String",
        "Buffer"           => "Buffer",
        "Window"           => "Window",
        "Tabpage"          => "Tabpage",
        "void"             => "()",
        "Array"            => "Array",
        "Dict"             => "Dict",
        "Object"           => "Object",
        "ArrayOf(String)"  => "Vec<String>",
        "ArrayOf(Buffer)"  => "Vec<Buffer>",
        "ArrayOf(Window)"  => "Vec<Window>",
        "ArrayOf(Tabpage)" => "Vec<Tabpage>",
        "ArrayOf(Dict)"    => "Vec<Dict>",
        "ArrayOf(Integer)" => "Vec<Integer>",
        t if t.starts_with("ArrayOf(Integer") => "Vec<Integer>",
        t => unimplemented!("{t}")
    }
}
fn return_type_to_value(type_: &str) -> &'static str {
    param_type_to_value(type_)
}
fn param_type_to_serde(type_: &str) -> &'static str {
    match type_ {
        "Nil"              => "Nil",
        "Boolean"          => "Boolean",
        "Integer"          => "Integer",
        "Float"            => "Float",
        "Buffer"           => "Buffer",
        "Window"           => "Window",
        "Tabpage"          => "Tabpage",
        "void"             => "()",
        "String"           => "&str",
        "Array"            => "impl Serialize",
        "Dict"             => "impl Serialize",
        "Object"           => "impl Serialize",
        "ArrayOf(Dict)"    => "impl Serialize",
        "ArrayOf(String)"  => "&[&str]",
        "ArrayOf(Buffer)"  => "&[Buffer]",
        "ArrayOf(Window)"  => "&[Window]",
        "ArrayOf(Tabpage)" => "&[Tabpage]",
        "ArrayOf(Integer)" => "&[Integer]",
        t if t.starts_with("ArrayOf(Integer") => "&[Integer]",
        t => unimplemented!("{t}")
    }
}
fn return_type_to_serde(type_: &str) -> &'static str {
    match type_ {
        "Nil"              => "Nil",
        "Boolean"          => "Boolean",
        "Integer"          => "Integer",
        "Float"            => "Float",
        "Buffer"           => "Buffer",
        "Window"           => "Window",
        "Tabpage"          => "Tabpage",
        "void"             => "()",
        "String"           => "String",
        "Array"            => "impl Deserialize<'static>",
        "Dict"             => "impl Deserialize<'static>",
        "Object"           => "impl Deserialize<'static>",
        "ArrayOf(Dict)"    => "impl Deserialize<'static>",
        "ArrayOf(String)"  => "Vec<String>",
        "ArrayOf(Buffer)"  => "Vec<Buffer>",
        "ArrayOf(Window)"  => "Vec<Window>",
        "ArrayOf(Tabpage)" => "Vec<Tabpage>",
        "ArrayOf(Integer)" => "Vec<Integer>",
        t if t.starts_with("ArrayOf(Integer") => "Vec<Integer>",
        t => unimplemented!("{t}")
    }
}
