use ded::{
    attributes::{
        autofocus, checked, class, classList, for_, name, placeholder, style, type_, value,
    },
    events::{on_click, on_enter, on_input},
    html::{div, h1, header, input, label, li, section, text, ul, Html, button},
    Program,
};

#[derive(Debug, Clone)]
pub enum Msg {
    UpdateField(String),
    Add,
    CheckAll(bool),
    Check(i32, bool),
    Delete(i32),
}

#[derive(Debug, Clone)]
pub struct Model {
    entries: Vec<Entry>,
    field: String,
    uid: i32,
    visibility: String,
}

#[derive(Debug, Clone)]
pub struct Entry {
    description: String,
    completed: bool,
    editing: bool,
    id: i32,
}

fn init() -> Model {
    Model {
        entries: vec![],
        visibility: "All".to_string(),
        field: "".to_string(),
        uid: 0,
    }
}

fn update(msg: &Msg, mut model: Model) -> Model {
    match msg {
        Msg::UpdateField(val) => model.field = val.to_owned(),
        Msg::Add => {
            if !model.field.is_empty() {
                model.entries.push(Entry {
                    description: model.field,
                    completed: false,
                    editing: false,
                    id: model.uid,
                });
                model.uid += 1;
                model.field = String::new();
            }
        }
        Msg::CheckAll(is_completed) => {
            for entry in &mut model.entries {
                entry.completed = *is_completed
            }
        }
        Msg::Check(id, is_completed) => {
            for entry in &mut model.entries {
                if entry.id == *id {
                    entry.completed = *is_completed
                }
            }
        }
        Msg::Delete(id) => {
            model.entries.retain(|entry| entry.id != *id);
        }
    }
    model
}

fn view(model: &Model) -> Html<Msg> {
    div(
        &[class("todomvc-wrapper"), style("visibility", "hidden")],
        &[section(
            &[class("todoapp")],
            &[
                view_input(&model.field),
                view_entries(&model.visibility, &model.entries),
                view_controls(&model.visibility, &model.entries),
            ],
        )],
    )
}

fn view_input(task: &str) -> Html<Msg> {
    header(
        &[class("header")],
        &[
            h1(&[], &[text("todos")]),
            input(
                &[
                    class("new-todo"),
                    placeholder("What needs to be done?"),
                    autofocus(true),
                    value(task),
                    name("newTodo"),
                    on_input(Msg::UpdateField),
                    on_enter(Msg::Add),
                ],
                &[],
            ),
        ],
    )
}

fn view_entries(visibility: &String, entries: &Vec<Entry>) -> Html<Msg> {
    let css_visibility = if entries.is_empty() {
        "hidden"
    } else {
        "visible"
    };

    let all_completed = entries.iter().all(|e| e.completed);

    section(
        &[class("main"), style("visibility", css_visibility)],
        &[
            input(
                &[
                    class("toggle-all"),
                    type_("checkbox"),
                    name("toggle"),
                    checked(all_completed),
                    on_click(Msg::CheckAll(!all_completed)),
                ],
                &[],
            ),
            label(&[for_("toggle-all")], &[text("Mark all as complete")]),
            ul(
                &[class("todo-list")],
                &entries.iter().map(view_entry).collect::<Vec<_>>(),
            ),
        ],
    )
}

fn view_entry(todo: &Entry) -> Html<Msg> {
    li(
        &[classList(&[
            ("completed", todo.completed),
            ("editing", todo.editing),
        ])],
        &[div(
            &[class("view")],
            &[
                input(
                    &[
                        class("toggle"),
                        type_("checkbox"),
                        checked(todo.completed),
                        on_click(Msg::Check(todo.id, !todo.completed)),
                    ],
                    &[],
                ),
                label(&[], &[text(&todo.description)]),
                button(&[class("destroy"), on_click(Msg::Delete(todo.id))], &[]),
            ],
        )],
    )
}

fn view_controls(visibility: &String, entires: &Vec<Entry>) -> Html<Msg> {
    div(&[], &[])
}

pub fn main() -> Program<Model, Msg> {
    Program::new(view, update, init())
}
