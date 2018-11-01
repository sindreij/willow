// TODO: Attributes
// TODO: Are we able to convert Html<A> to Html<B>?

use std::cmp::PartialEq;
use std::fmt::{self, Debug};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct HtmlTag<Msg> {
    pub tag: String,
    pub attrs: Vec<Attribute<Msg>>,
    pub children: Vec<Html<Msg>>,
}

#[derive(Clone, Debug)]
pub enum Html<Msg> {
    Tag(HtmlTag<Msg>),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropertyValue {
    String(String),
    Bool(bool),
}

#[derive(Clone)]
pub struct Callback<Msg>{
    pub closure: Rc<Fn(web_sys::Event) -> Option<Msg>>,
    pub js_closure: Rc<RefCell<Option<wasm_bindgen::closure::Closure<Fn(web_sys::Event)>>>>,
}

impl<Msg> Callback<Msg> {
    pub fn new(inner: Rc<Fn(web_sys::Event) -> Option<Msg>>) -> Self {
        Callback {
            closure: inner,
            js_closure: Rc::new(RefCell::new(None)),
        }
    }
}

impl<Msg> Debug for Callback<Msg> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A CALLBACK")?;
        if self.js_closure.borrow().is_some() {
            write!(f, " (WITH CLOSURE)")?;
        }
        Ok(())
    }
}

impl<Msg> PartialEq for Callback<Msg> {
    fn eq(&self, _: &Callback<Msg>) -> bool {
        // NOTE: This means we can not derive Eq, because a != a
        false
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute<Msg> {
    Event {
        type_: String,
        to_message: Callback<Msg>,
        stop_propagation: bool,
        prevent_default: bool,
    },
    // TODO: Value should be JsValue or something like that, not String
    Property(&'static str, PropertyValue),
    Style(String, String),
}

macro_rules! create_node {
    ($x:ident) => {
        pub fn $x<Msg: Clone>(attrs: &[Attribute<Msg>], children: &[Html<Msg>]) -> Html<Msg> {
            Html::Tag(HtmlTag {
                tag: stringify!($x).to_owned(),
                children: children.to_vec(),
                attrs: attrs.to_vec(),
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
