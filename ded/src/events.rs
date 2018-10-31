use std::rc::Rc;

use wasm_bindgen::JsCast;

use crate::html::Attribute;

pub fn on_click<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
    Attribute::Event {
        type_: "click".to_owned(),
        to_message: Rc::new(move |_| Some(message.clone())),
        stop_propagation: false,
        prevent_default: false,
    }
}

pub fn on_input<Msg: 'static>(message: impl Fn(String) -> Msg + 'static + Clone) -> Attribute<Msg> {
    Attribute::Event {
        type_: "input".to_owned(),
        to_message: Rc::new(move |event| {
            Some(message(
                event
                    .target()
                    .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                    .map(|el| el.value())
                    .unwrap_or_default(),
            ))
        }),
        stop_propagation: true,
        prevent_default: false,
    }
}

pub fn on_enter<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
    Attribute::Event {
        type_: "keydown".to_owned(),
        to_message: Rc::new(move |event| {
            let keycode = event
                .dyn_into::<web_sys::KeyboardEvent>()
                .ok()
                .map(|ev| ev.key_code())
                .unwrap_or(0);
            if keycode == 13 {
                Some(message.clone())
            } else {
                None
            }
        }),
        prevent_default: false,
        stop_propagation: false,
    }
}
