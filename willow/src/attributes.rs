use crate::html::{Attribute, PropertyValue};

macro_rules! string_property {
    ($x:ident, $tag:expr) => {
        pub fn $x<Msg>(value: &str) -> Attribute<Msg> {
            Attribute::Property($tag, PropertyValue::String(value.to_owned()))
        }
    };
    ($x:ident) => {
        string_property!($x, stringify!($x));
    };
}

macro_rules! bool_property {
    ($x:ident, $tag:expr) => {
        pub fn $x<Msg>(value: bool) -> Attribute<Msg> {
            Attribute::Property($tag, PropertyValue::Bool(value.to_owned()))
        }
    };
    ($x:ident) => {
        bool_property!($x, stringify!($x));
    };
}

pub fn style<Msg>(property: &str, value: &str) -> Attribute<Msg> {
    Attribute::Style(property.to_owned(), value.to_owned())
}

pub fn class_list<Msg>(classes: &[(&str, bool)]) -> Attribute<Msg> {
    let active = classes
        .iter()
        .filter(|(_, active)| *active)
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();

    // TODO: Change `class` to use Into<Cow> and use it here
    Attribute::Property("className", PropertyValue::String(active.join(" ")))
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
