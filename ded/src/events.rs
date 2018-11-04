use std::default::Default;

use wasm_bindgen::JsCast;

use crate::html::{Attribute, EventToMessage};

pub fn on_click<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
    Attribute::Event {
        type_: "click".to_owned(),
        to_message: EventToMessage::StaticMsg(message),
        stop_propagation: false,
        prevent_default: false,
        js_closure: Default::default(),
    }
}

pub fn on_double_click<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
    Attribute::Event {
        type_: "dblclick".to_owned(),
        to_message: EventToMessage::StaticMsg(message),
        stop_propagation: false,
        prevent_default: false,
        js_closure: Default::default(),
    }
}

pub fn on_blur<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
    Attribute::Event {
        type_: "blur".to_owned(),
        to_message: EventToMessage::StaticMsg(message),
        stop_propagation: false,
        prevent_default: false,
        js_closure: Default::default(),
    }
}

// TODO: Ensure that when we start using animationFrame, on_input gets special treatement
pub fn on_input<Msg: 'static>(message: fn(String) -> Msg) -> Attribute<Msg> {
    Attribute::Event {
        type_: "input".to_owned(),
        to_message: EventToMessage::Input(message),
        stop_propagation: true,
        prevent_default: false,
        js_closure: Default::default(),
    }
}

pub fn on_enter<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
    Attribute::Event {
        type_: "keydown".to_owned(),
        to_message: EventToMessage::WithFilter {
            msg: message,
            filter: |event| {
                let key_code = event
                    .dyn_into::<web_sys::KeyboardEvent>()
                    .ok()
                    .map(|ev| ev.key_code())
                    .unwrap_or(0);
                key_code == 13
            },
        },
        prevent_default: false,
        stop_propagation: false,
        js_closure: Default::default(),
    }
}
