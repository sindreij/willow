use crate::{
    elm::{button, div, on_click, text, Html},
    program::Program,
};

#[derive(Debug, Clone)]
pub enum Msg {
    Increment,
    Decrement,
}

#[derive(Debug, Clone)]
pub struct Model {
    counter: i32,
}

fn init() -> Model {
    Model { counter: 4 }
}

fn update(msg: &Msg, mut model: Model) -> Model {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
    }

    model
}

fn view(model: &Model) -> Html<Msg> {
    div(
        &[],
        &[
            button(&[on_click(Msg::Increment)], &[text("+")]),
            div(&[], &[text(&model.counter.to_string())]),
            button(&[on_click(Msg::Decrement)], &[text("-")]),
        ],
    )
}

pub fn main() -> Program<Model, Msg> {
    Program::new(view, update, init())
}
