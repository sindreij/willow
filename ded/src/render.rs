use std::fmt::Debug;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, MouseEvent, Node, Element};

use crate::{html::Html, program::Program};

pub fn render<Msg: Debug + Clone + 'static, Model: Clone + 'static>(
    program: &Rc<Program<Model, Msg>>,
    input: &Html<Msg>,
) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let node = create_node(program, &document, input)?;

    // Manufacture the element we're gonna append
    // let val = document.create_element("p")?;
    // val.set_inner_html("Hello from Rust!");

    // Right now the class inheritance hierarchy of the DOM isn't super
    // ergonomic, so we manually cast `val: Element` to `&Node` to call the
    // `append_child` method.
    AsRef::<Element>::as_ref(&body).set_inner_html("");
    AsRef::<Node>::as_ref(&body).append_child(&node)?;

    Ok(())
}

fn create_node<Msg: Debug + Clone + 'static, Model: Clone + 'static>(
    program: &Rc<Program<Model, Msg>>,
    document: &Document,
    input: &Html<Msg>,
) -> Result<Node, JsValue> {
    match input {
        Html::Tag {
            tag,
            attrs,
            children,
        } => {
            let val: Node = document.create_element(&tag)?.into();

            for attr in attrs {
                let name_for_logging = attr.type_.clone();
                let msg_for_logging = format!("{:?}", attr.message);
                let message = attr.message.clone();
                let program = program.clone();
                let closure = Closure::wrap(Box::new(move |_: MouseEvent| {
                    console_log!("On Event {}, {}!", name_for_logging, msg_for_logging);
                    program.dispatch(&message);
                }) as Box<FnMut(_)>);

                (val.as_ref() as &web_sys::EventTarget).add_event_listener_with_callback(
                    &attr.type_,
                    closure.as_ref().unchecked_ref(),
                )?;

                closure.forget();
            }

            for child in children {
                let node = create_node(&program, document, &child)?;
                val.append_child(&node)?;
            }

            Ok(val)
        }
        Html::Text(text) => {
            let val = document.create_text_node(&text);

            Ok(val.into())
        }
    }
}
