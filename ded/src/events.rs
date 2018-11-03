use std::rc::Rc;

use wasm_bindgen::JsCast;

use crate::html::Attribute;

// pub fn on_click<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
//     Attribute::Event {
//         type_: "click".to_owned(),
//         message: message.clone(),
//         stop_propagation: false,
//         prevent_default: false,
//         js_closure: <_>::default(),
//     }
// }

// pub fn on_double_click<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
//     Attribute::Event {
//         type_: "dblclick".to_owned(),
//         message: message.clone(),
//         stop_propagation: false,
//         prevent_default: false,
//         js_closure: <_>::default(),
//     }
// }

// pub fn on_blur<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
//     Attribute::Event {
//         type_: "blur".to_owned(),
//         message: message.clone(),
//         stop_propagation: false,
//         prevent_default: false,
//         js_closure: <_>::default(),
//     }
// }

// // TODO: Ensure that when we start using animationFrame, on_input gets special treatement
// pub fn on_input<Msg: 'static>(message: fn(String) -> Msg) -> Attribute<Msg> {
//     Attribute::Event {
//         type_: "input".to_owned(),
//         to_message: Callback::new(Rc::new(move |event| {
//             Some(message(
//                 event
//                     .target()
//                     .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
//                     .map(|el| el.value())
//                     .unwrap_or_default(),
//             ))
//         })),
//         stop_propagation: true,
//         prevent_default: false,
//     }
// }

// pub fn on_enter<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
//     Attribute::Event {
//         type_: "keydown".to_owned(),
//         to_message: Callback::new(Rc::new(move |event| {
//             let keycode = event
//                 .dyn_into::<web_sys::KeyboardEvent>()
//                 .ok()
//                 .map(|ev| ev.key_code())
//                 .unwrap_or(0);
//             if keycode == 13 {
//                 Some(message.clone())
//             } else {
//                 None
//             }
//         })),
//         prevent_default: false,
//         stop_propagation: false,
//     }
// }
