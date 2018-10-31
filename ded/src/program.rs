use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

use crate::{html::Html, render};

pub struct Program<Model, Msg> {
    pub view: Box<Fn(&Model) -> Html<Msg>>,
    pub update: Box<Fn(&Msg, Model) -> Model>,
    pub current_model: RefCell<Model>,
}

impl<Model, Msg> Program<Model, Msg>
where
    Model: Clone + 'static,
    Msg: Debug + Clone + 'static,
{
    pub fn new<ViewFn, UpdateFn>(view: ViewFn, update: UpdateFn, initial: Model) -> Self
    where
        ViewFn: Fn(&Model) -> Html<Msg> + 'static,
        UpdateFn: Fn(&Msg, Model) -> Model + 'static,
    {
        Self {
            view: Box::new(view),
            update: Box::new(update),
            current_model: RefCell::new(initial),
        }
    }

    pub fn dispatch(self: &Rc<Self>, message: &Msg) {
        let old_model = self.current_model.borrow().clone();

        let new_model = (self.update)(message, old_model);

        self.current_model.replace(new_model);

        self.render()
    }

    pub fn render(self: &Rc<Self>) {
        let tree = (self.view)(&self.current_model.borrow());

        console_log!("View: {:#?}", tree);

        if let Err(err) = render::render(self, &tree) {
            console_log!("Got error: {:?}", err);
        }
    }

    pub fn start(self: &Rc<Self>) {
        self.render()
    }
}
