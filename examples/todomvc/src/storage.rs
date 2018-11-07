use std::default::Default;

use wasm_bindgen::JsValue;
use web_sys;

use willow::{console_log, Cmd};

use crate::app::{Model, Msg};

pub struct SetStorage(pub Model);

impl Cmd<Msg> for SetStorage {
    fn run(&self) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let local_storage = window.local_storage();
        if let Ok(Some(local_storage)) = local_storage {
            let data = serde_json::to_string(&self.0)
                .map_err(|err| JsValue::from_str(&err.to_string()))?;
            if let Err(err) = local_storage.set_item("todomvc::data", &data) {
                console_log!("Could not write to local storage, {:?}", err);
            }
        }

        Ok(())
    }
}

pub fn get_model() -> Model {
    let window = web_sys::window().expect("no global `window` exists");
    let local_storage = window.local_storage();

    if let Ok(Some(local_storage)) = local_storage {
        if let Ok(Some(s)) = local_storage.get_item("todomvc::data") {
            serde_json::from_str(&s).ok().unwrap_or_default()
        } else {
            Default::default()
        }
    } else {
        Default::default()
    }
}
