use wasm_bindgen::JsValue;

pub trait Cmd<Msg> {
    fn run(&self) -> Result<(), JsValue>;

    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub struct None;

impl<Msg> Cmd<Msg> for None {
    fn run(&self) -> Result<(), JsValue> {
        Ok(())
    }
}
