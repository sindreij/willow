use wasm_bindgen::JsValue;
use web_sys::{Document, Node, Element};

use crate::{
    elm::{Html}
};

pub fn render<Msg>(input: &Html<Msg>) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let node = create_node(&document, input)?;

    // Manufacture the element we're gonna append
    // let val = document.create_element("p")?;
    // val.set_inner_html("Hello from Rust!");

    // Right now the class inheritance hierarchy of the DOM isn't super
    // ergonomic, so we manually cast `val: Element` to `&Node` to call the
    // `append_child` method.
    AsRef::<web_sys::Node>::as_ref(&body).append_child(&node)?;

    Ok(())
}

fn create_node<Msg>(document: &Document, input: &Html<Msg>) -> Result<Node, JsValue> {
    match input {
        Html::Tag {
            tag, attrs, children
        } => {
            let val:Node = document.create_element(&tag)?.into();

            for child in children {
                let node = create_node(document, &child)?;
                val.append_child(&node);
            }

            Ok(val)
        },
        Html::Text(text) => {
            let val = document.create_text_node(&text);

            Ok(val.into())
        }
    }
}