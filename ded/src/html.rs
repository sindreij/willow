// TODO: Attributes
// TODO: Are we able to convert Html<A> to Html<B>?

use std::rc::Rc;

use derivative::Derivative;

#[derive(Clone, Debug)]
pub enum Html<Msg> {
    Tag {
        tag: String,
        attrs: Vec<Attribute<Msg>>,
        children: Vec<Html<Msg>>,
    },
    Text(String),
}

#[derive(Clone, Debug)]
pub enum PropertyValue {
    String(String),
    Bool(bool),
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub enum Attribute<Msg> {
    Event {
        type_: String,
        #[derivative(Debug = "ignore")]
        to_message: Rc<Fn(web_sys::Event) -> Option<Msg>>,
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
            Html::Tag {
                tag: stringify!($x).to_owned(),
                children: children.to_vec(),
                attrs: attrs.to_vec(),
            }
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
