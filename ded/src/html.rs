// TODO: Attributes
// TODO: Are we able to convert Html<A> to Html<B>?

use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::{self, Debug};
use std::rc::Rc;

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

impl<Msg> Html<Msg> {
    pub fn to_html_text(&self, indent: u32) -> String {
        let indent_s = "  ".repeat(indent as usize);
        match self {
            Html::Text(text) => format!("{}{}", indent_s, text),
            Html::Tag(tag) => {
                if tag.children.is_empty() {
                    return format!("{}<{} />", indent_s, tag.tag);
                }
                let children = tag
                    .children
                    .iter()
                    .map(|child| child.to_html_text(indent + 1))
                    .collect::<Vec<_>>()
                    .join("\n");;
                format!(
                    "{}<{}>\n{}\n{}</{}>",
                    indent_s, tag.tag, children, indent_s, tag.tag,
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropertyValue {
    String(String),
    Bool(bool),
}

#[derive(Clone, Default)]
pub struct JsClosure(pub Rc<RefCell<Option<wasm_bindgen::closure::Closure<Fn(web_sys::Event)>>>>);

impl Debug for JsClosure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.borrow().is_some() {
            write!(f, "HAS A CLOSURE")
        } else {
            write!(f, "NO CLOSURE")
        }
    }
}

impl PartialEq for JsClosure {
    fn eq(&self, _: &JsClosure) -> bool {
        // This is not good enough to implent Eq, i think
        // And its a bit weird. But it's to ignore this in the Attribute enum
        true
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute<Msg> {
    // Event where the message depends on the event data
    Event {
        js_closure: JsClosure,
        type_: String,
        stop_propagation: bool,
        prevent_default: bool,
        to_message: EventToMessage<Msg>,
    },
    // TODO: Value should be JsValue or something like that, not String
    Property(&'static str, PropertyValue),
    Style(String, String),
}

impl<Msg> Attribute<Msg> {
    pub fn is_event(&self) -> bool {
        match self {
            Attribute::Event { .. } => true,
            _ => false,
        }
    }

    /// Panics if self is not an event
    pub fn get_js_closure(&self) -> JsClosure {
        match self {
            Attribute::Event { js_closure, .. } => js_closure.clone(),
            _ => panic!("get_js_closure called with something that is not an event"),
        }
    }

    /// Panics if self is not an event
    pub fn set_js_closure(&self, closure: wasm_bindgen::closure::Closure<Fn(web_sys::Event)>) {
        match self {
            Attribute::Event { js_closure, .. } => {
                let ret = js_closure.0.replace(Some(closure));

                if ret.is_some() {
                    console_log!("set_js_closure called, but event did already have a closure???");
                }
            }
            _ => panic!("set_js_closure called with something that is not an event"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventToMessage<Msg> {
    StaticMsg(Msg),
    Input(fn(String) -> Msg),
    WithFilter {
        msg: Msg,
        filter: fn(web_sys::Event) -> bool,
    },
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
