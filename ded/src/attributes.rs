use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::html::{Attribute, PropertyValue};

#[derive(Clone, Debug)]
struct PropertyAttribute {
    key: &'static str,
    value: PropertyValue,
}

#[derive(Clone, Debug)]
struct StyleAttribute {
    property: String,
    value: String,
}

fn property_value_to_json_value(val: &PropertyValue) -> JsValue {
    match val {
        PropertyValue::String(ref value) => JsValue::from_str(value),
        PropertyValue::Bool(value) => JsValue::from_bool(*value),
    }
}

impl<Msg> Attribute<Msg> for PropertyAttribute {
    fn add_to_node(&self, dispatch: Box<Fn(Msg)>, node: &HtmlElement) -> Result<(), JsValue> {
        Reflect::set(
            node.as_ref(),
            &JsValue::from_str(&self.key),
            &property_value_to_json_value(&self.value),
        )?;
        Ok(())
    }

    fn remove_from_node(&self, node: &HtmlElement) -> Result<(), JsValue> {
        Reflect::delete_property(node.as_ref(), &JsValue::from_str(&self.key))?;
        Ok(())
    }
}

impl<Msg> Attribute<Msg> for StyleAttribute {
    fn add_to_node(&self, dispatch: Box<Fn(Msg)>, node: &HtmlElement) -> Result<(), JsValue> {
        node.style().set_property(&self.property, &self.value)?;;
        Ok(())
    }

    fn remove_from_node(&self, node: &HtmlElement) -> Result<(), JsValue> {
        node.style().remove_property(&self.property)?;
        Ok(())
    }
}

macro_rules! string_property {
    ($x:ident, $tag:expr) => {
        pub fn $x<Msg>(value: &str) -> Box<Attribute<Msg>> {
            Box::new(PropertyAttribute {
                key: $tag,
                value: PropertyValue::String(value.to_owned()),
            })
        }
    };
    ($x:ident) => {
        string_property!($x, stringify!($x));
    };
}

macro_rules! bool_property {
    ($x:ident, $tag:expr) => {
        pub fn $x<Msg>(value: bool) -> Box<Attribute<Msg>> {
            Box::new(PropertyAttribute {
                key: $tag,
                value: PropertyValue::Bool(value.to_owned()),
            })
        }
    };
    ($x:ident) => {
        bool_property!($x, stringify!($x));
    };
}

pub fn style<Msg>(property: &str, value: &str) -> Box<Attribute<Msg>> {
    Box::new(StyleAttribute {
        property: property.to_owned(),
        value: value.to_owned(),
    })
}

pub fn class_list<Msg>(classes: &[(&str, bool)]) -> Box<Attribute<Msg>> {
    let active = classes
        .iter()
        .filter(|(_, active)| *active)
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();

    // TODO: Change `class` to use Into<Cow> and use it here
    Box::new(PropertyAttribute {
        key: "className",
        value: PropertyValue::String(active.join(" ")),
    })
}

string_property!(placeholder);
string_property!(name);
string_property!(value);
string_property!(id);
string_property!(href);
string_property!(class, "className");
string_property!(type_, "type");
string_property!(for_, "htmlFor");

bool_property!(autofocus);
bool_property!(checked);
bool_property!(hidden);
