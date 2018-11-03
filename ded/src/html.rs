// TODO: Attributes
// TODO: Are we able to convert Html<A> to Html<B>?

use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::{self, Debug};
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[derive(Debug)]
pub struct HtmlTag<Msg> {
    pub tag: String,
    pub attrs: Vec<Box<Attribute<Msg>>>,
    pub children: Vec<Html<Msg>>,
}

#[derive(Debug)]
pub enum Html<Msg> {
    Tag(HtmlTag<Msg>),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropertyValue {
    String(String),
    Bool(bool),
}

pub trait Attribute<Msg>: Debug + PartialEq {
    fn add_to_node(&self, dispatch: Box<Fn(Msg)>, node: &HtmlElement) -> Result<(), JsValue>;
    fn remove_from_node(&self, node: &HtmlElement) -> Result<(), JsValue>;
}

// #[derive(Clone, Default)]
// pub struct JsClosure(pub Rc<RefCell<Option<wasm_bindgen::closure::Closure<Fn(web_sys::Event)>>>>);

// impl Debug for JsClosure {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         if self.0.borrow().is_some() {
//             write!(f, "HAS A CLOSURE")
//         } else {
//             write!(f, "NO CLOSURE")
//         }
//     }
// }

// impl<Msg> PartialEq for JsClosure {
//     fn eq(&self, other: &JsClosure) -> bool {
//         // This is not good enough to implent Eq, i think
//         // And its a bit weird. But it's to ignore this in the Attribute enum
//         true
//     }
// }
// #[derive(Clone, Debug, PartialEq)]
// pub enum Attribute<Msg> {
//     // Event where the message depends on the event data
//     EventDyn {
//         type_: String,
//         to_message: fn(web_sys::Event) -> Option<Msg>,
//         js_closure: JsClosure,
//         stop_propagation: bool,
//         prevent_default: bool,
//     },
//     // Event where the message is always the same
//     Event {
//         type_: String,
//         message: Msg,
//         js_closure: JsClosure,
//         stop_propagation: bool,
//         prevent_default: bool,
//     },
//     // TODO: Value should be JsValue or something like that, not String
//     Property(&'static str, PropertyValue),
//     Style(String, String),
// }

macro_rules! create_node {
    ($x:ident) => {
        pub fn $x<Msg: Clone>(
            attrs: Vec<Box<Attribute<Msg>>>,
            children: Vec<Html<Msg>>,
        ) -> Html<Msg> {
            Html::Tag(HtmlTag {
                tag: stringify!($x).to_owned(),
                children: children,
                attrs: attrs,
            })
        }
    };
}

create_node!(div);
create_node!(button);
create_node!(section);
create_node!(header);
create_node!(h1);
create_node!(h2);
create_node!(h3);
create_node!(h4);
create_node!(input);
create_node!(label);
create_node!(ul);
create_node!(li);
create_node!(footer);
create_node!(span);
create_node!(strong);
create_node!(a);
create_node!(p);

pub fn text<Msg>(inner: &str) -> Html<Msg> {
    Html::Text(inner.to_owned())
}
