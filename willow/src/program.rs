use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use web_sys;

use crate::{html::Html, render};

pub struct Program<Model, Msg> {
    pub view: Box<Fn(&Model) -> Html<Msg>>,
    pub update: Box<Fn(&Msg, &mut Model)>,
    pub current_model: RefCell<Model>,
    pub last_tree: RefCell<Option<Html<Msg>>>,
}

impl<Model, Msg> Program<Model, Msg>
where
    Model: Debug + Clone + 'static,
    Msg: PartialEq + Debug + Clone + 'static,
{
    pub fn new<ViewFn, UpdateFn>(view: ViewFn, update: UpdateFn, initial: Model) -> Self
    where
        ViewFn: Fn(&Model) -> Html<Msg> + 'static,
        UpdateFn: Fn(&Msg, &mut Model) + 'static,
    {
        Self {
            view: Box::new(view),
            update: Box::new(update),
            current_model: RefCell::new(initial),
            last_tree: RefCell::new(None),
        }
    }

    pub fn dispatch(self: &Rc<Self>, message: &Msg) {
        let mut model = self.current_model.borrow().clone();

        let window = web_sys::window().expect("no global `window` exists");
        let performance = window
            .performance()
            .expect("should have performance on window");
        let start_time = performance.now();
        (self.update)(message, &mut model);
        let end_time = performance.now();
        console_log!("Update took {} ms", end_time - start_time);

        // console_log!("Model: {:?}", model);

        self.current_model.replace(model);

        self.render()
    }

    pub fn render(self: &Rc<Self>) {
        let tree = (self.view)(&self.current_model.borrow());

        // console_log!("New view: {}", tree.to_html_text(0));

        let window = web_sys::window().expect("no global `window` exists");
        let performance = window
            .performance()
            .expect("should have performance on window");
        let start_time = performance.now();

        if let Err(err) = render::render(self, &tree, &self.last_tree.borrow()) {
            console_log!("Got error: {:?}", err);
        }
        let end_time = performance.now();
        console_log!("Rendering took {} ms", end_time - start_time);

        self.last_tree.replace(Some(tree));
    }

    pub fn start(self: &Rc<Self>) {
        self.render()
    }
}
