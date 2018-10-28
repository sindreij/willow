use crate::elm::{button, div, on_click, text, Html};

#[derive(Debug, Clone)]
pub enum Msg {
    Increment,
    Decrement,
}

#[derive(Debug)]
pub struct Model {
    counter: i32,
}

pub fn init() -> Model {
    Model { counter: 4 }
}

pub fn update(msg: Msg, mut model: Model) -> Model {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
    }

    model
}

pub fn view(model: &Model) -> Html<Msg> {
    div(
        &[],
        &[
            button(&[on_click(Msg::Increment)], &[text("-")]),
            div(&[], &[text(&model.counter.to_string())]),
            button(&[on_click(Msg::Decrement)], &[text("+")]),
        ],
    )
}
