use std::fmt::Debug;
use std::rc::Rc;

use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{self, Document, Element, HtmlElement, MouseEvent, Node};

use crate::{
    html::{Attribute, Html, PropertyValue},
    program::Program,
};

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
            let val: HtmlElement = document.create_element(&tag)?.dyn_into()?;

            for attr in attrs {
                match attr {
                    Attribute::Property(key, value) => {
                        Reflect::set(
                            val.as_ref(),
                            &JsValue::from_str(&key),
                            &property_value_to_json_value(value),
                        )?;
                    }
                    Attribute::Style(property, value) => {
                        val.style().set_property(property, value)?;
                    }
                    Attribute::Event {
                        type_,
                        to_message,
                        stop_propagation,
                        prevent_default,
                    } => {
                        let name_for_logging = type_.clone();
                        let to_message = to_message.clone();
                        let program = program.clone();
                        let stop_propagation = *stop_propagation;
                        let prevent_default = *prevent_default;
                        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                            let event: web_sys::Event = event.into();
                            if prevent_default {
                                event.prevent_default();
                            }
                            if stop_propagation {
                                event.stop_propagation();
                            }
                            let generated_message = to_message(event);
                            console_log!("On Event {}, {:?}!", name_for_logging, generated_message);
                            if let Some(message) = generated_message {
                                program.dispatch(&message);
                            }
                        }) as Box<FnMut(_)>);

                        (val.as_ref() as &web_sys::EventTarget).add_event_listener_with_callback(
                            &type_,
                            closure.as_ref().unchecked_ref(),
                        )?;

                        // TODO: Cleanup
                        closure.forget();
                    }
                }
            }

            let val: Node = val.into();

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

fn property_value_to_json_value(val: &PropertyValue) -> JsValue {
    match val {
        PropertyValue::String(ref value) => JsValue::from_str(value),
        PropertyValue::Bool(value) => JsValue::from_bool(*value),
    }
}
