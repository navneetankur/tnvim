#![feature(anonymous_lifetime_in_impl_trait)]
use core::ops::ControlFlow;
use std::io::{Write, stdout};

use rmpv::Value;

pub fn main() {
    let data = include_bytes!("nvimapi.msgpack");
    let v = rmpv::decode::read_value(&mut data.as_slice()).unwrap();
    let root = Vec::try_from(v).unwrap();
    for (key, value) in root {
        if key.as_str().unwrap() == "functions" {
            handle_functions(value);
        }
    }
}
impl From<A> for Value {
    fn from(that: A) -> Self {
        Value::from(that.0)
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
"###;
const HEADER_VALUE: &str = r###"
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
type ArrayP = impl Serialize;
type DictP = impl Serialize;
type ObjectP = impl Serialize;
type ArrayDictP = impl Serialize;
type ArrayR = impl Deserialize;
type DictR = impl Deserialize;
type ObjectR = impl Deserialize;
type ArrayDictR = impl Deserialize;
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

struct A(i64);
impl serde::Serialize for A {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        i64::serialize(&self.0, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for A {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        Ok(Self(i64::deserialize(deserializer)?))
    }
}

fn handle_functions(value: Value) {
    let functions = Vec::<Value>::try_from(value).unwrap();
    let mut buffer = Vec::new();
    let ignored_types = ["LuaRef",];
    'outer: for fun in functions {
        if let ControlFlow::Break(_) = handle_fun(&mut buffer, &ignored_types, fun) {
            continue 'outer;
        }
        stdout().write_all(&buffer).unwrap();
    }
}

fn handle_fun(buffer: &mut Vec<u8>, ignored_types: &[&str], fun: Value) -> ControlFlow<()> {
    buffer.clear();
    let deprecated = value_get(&fun, "deprecated_since");
    if deprecated.is_some() { return ControlFlow::Break(()); }
    let fn_name = value_get(&fun, "name").unwrap().as_str().unwrap();
    write!(buffer, "fn {fn_name}(").unwrap();
    let params = value_get(&fun, "parameters").unwrap().as_array().unwrap();
    for param in params {
        let param = param.as_array().unwrap();
        let p_type = param[0].as_str().unwrap();
        if ignored_types.contains(&p_type) { return ControlFlow::Break(()); }
        let p_type = param_type_to_value(p_type);
        let p_name = param[1].as_str().unwrap();
        write!(buffer, "{p_name}: {p_type}, ").unwrap();
    }
    let ret_type = value_get(&fun, "return_type").unwrap().as_str().unwrap();
    let ret_type = return_type_to_value(ret_type);
    write!(buffer, "): -> {ret_type} {{").unwrap();
    writeln!(buffer, "}}").unwrap();
    ControlFlow::Continue(())
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
        "Object"           => "Value",
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
        "String"           => "&str",
        "Buffer"           => "Buffer",
        "Window"           => "Window",
        "Tabpage"          => "Tabpage",
        "void"             => "()",
        "Array"            => "ArrayP",
        "Dict"             => "DictP",
        "Object"           => "ObjectP",
        "ArrayOf(String)"  => "&[&str]",
        "ArrayOf(Buffer)"  => "&[Buffer]",
        "ArrayOf(Window)"  => "&[Window]",
        "ArrayOf(Tabpage)" => "&[Tabpage]",
        "ArrayOf(Dict)"    => "ArrayDictP",
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
        "Array"            => "ArrayR",
        "Dict"             => "DictR",
        "Object"           => "ObjectR",
        "String"           => "String",
        "ArrayOf(String)"  => "Vec<String>",
        "ArrayOf(Buffer)"  => "Vec<Buffer>",
        "ArrayOf(Window)"  => "Vec<Window>",
        "ArrayOf(Tabpage)" => "Vec<Tabpage>",
        "ArrayOf(Dict)"    => "ArrayDictR",
        "ArrayOf(Integer)" => "Vec<Integer>",
        t if t.starts_with("ArrayOf(Integer") => "Vec<Integer>",
        t => unimplemented!("{t}")
    }
}
