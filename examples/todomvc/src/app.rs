use serde_derive::{Deserialize, Serialize};

use willow::{
    attributes::{
        autofocus, checked, class, class_list, for_, hidden, href, id, key, name, placeholder,
        style, type_, value,
    },
    events::{on_blur, on_click, on_double_click, on_enter, on_input, on_input2},
    html::{
        a, button, div, footer, h1, header, input, label, li, p, section, span, strong, text, ul,
        Html,
    },
    Cmd, Program,
};

use crate::storage::{self, SetStorage};

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    UpdateField(String),
    UpdateEntry(i32, String),
    EditingEntry(i32, bool),
    Add,
    CheckAll(bool),
    Check(i32, bool),
    Delete(i32),
    ChangeVisibility(&'static str),
    DeleteCompleted,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    entries: Vec<Entry>,
    field: String,
    uid: i32,
    visibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    description: String,
    completed: bool,
    editing: bool,
    id: i32,
}

fn init() -> Model {
    storage::get_model()

    // Model {
    //     entries,
    //     visibility: "All".to_string(),
    //     field: "".to_string(),
    //     uid: 0,
    // }
}

fn update(msg: &Msg, model: &mut Model) -> Box<Cmd<Msg>> {
    match msg {
        Msg::UpdateField(val) => model.field = val.to_owned(),
        Msg::Add => {
            if !model.field.is_empty() {
                model.entries.push(Entry {
                    description: model.field.clone(),
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
        Msg::UpdateEntry(id, task) => {
            for entry in &mut model.entries {
                if entry.id == *id {
                    entry.description = task.to_owned();
                }
            }
        }
        Msg::EditingEntry(id, is_editing) => {
            for entry in &mut model.entries {
                if entry.id == *id {
                    entry.editing = *is_editing;
                }
            }
        }
        Msg::ChangeVisibility(visibility) => {
            model.visibility = visibility.to_string();
        }
        Msg::DeleteCompleted => model.entries.retain(|entry| !entry.completed),
    };
    Box::new(SetStorage(model.clone()))
}

fn view(model: &Model) -> Html<Msg> {
    div(
        &[class("todomvc-wrapper"), style("visibility", "hidden")],
        &[
            section(
                &[class("todoapp")],
                &[
                    view_input(&model.field),
                    view_entries(&model.visibility, &model.entries),
                    view_controls(&model.visibility, &model.entries),
                ],
            ),
            info_footer(),
        ],
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
                &entries
                    .iter()
                    .filter(|entry| match visibility.as_str() {
                        "Completed" => entry.completed,
                        "Active" => !entry.completed,
                        _ => true,
                    })
                    .map(view_entry)
                    .collect::<Vec<_>>(),
            ),
        ],
    )
}

fn view_entry(todo: &Entry) -> Html<Msg> {
    li(
        &[
            key(todo.id.to_string()),
            class_list(&[("completed", todo.completed), ("editing", todo.editing)]),
        ],
        &[
            div(
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
                    label(
                        &[on_double_click(Msg::EditingEntry(todo.id, true))],
                        &[text(&todo.description)],
                    ),
                    button(&[class("destroy"), on_click(Msg::Delete(todo.id))], &[]),
                ],
            ),
            input(
                &[
                    class("edit"),
                    value(&todo.description),
                    name("title"),
                    id(&format!("todo-{}", todo.id.to_string())),
                    on_input2(todo.id, |todo_id, val| Msg::UpdateEntry(todo_id, val)),
                    on_blur(Msg::EditingEntry(todo.id, false)),
                    on_enter(Msg::EditingEntry(todo.id, false)),
                ],
                &[],
            ),
        ],
    )
}

fn view_controls(visibility: &String, entries: &Vec<Entry>) -> Html<Msg> {
    let entries_completed = entries.iter().filter(|e| e.completed).count();
    let entries_left = entries.len() - entries_completed;

    footer(
        &[class("footer"), hidden(entries.is_empty())],
        &[
            view_controls_count(entries_left),
            view_controls_filters(visibility),
            view_controls_clear(entries_completed),
        ],
    )
}

fn view_controls_count(entries_left: usize) -> Html<Msg> {
    let item_ = if entries_left == 1 { " item" } else { " items" };

    span(
        &[class("todo-count")],
        &[
            strong(&[], &[text(&entries_left.to_string())]),
            text(&format!("{} left", item_)),
        ],
    )
}

fn view_controls_filters(visibility: &str) -> Html<Msg> {
    ul(
        &[class("filters")],
        &[
            visibility_swap("#/", "All", visibility),
            text(" "),
            visibility_swap("#/active", "Active", visibility),
            text(" "),
            visibility_swap("#/completed", "Completed", visibility),
        ],
    )
}

fn visibility_swap(uri: &str, visibility: &'static str, actual_visibility: &str) -> Html<Msg> {
    li(
        &[on_click(Msg::ChangeVisibility(visibility))],
        &[a(
            &[
                href(uri),
                class_list(&[("selected", visibility == actual_visibility)]),
            ],
            &[text(visibility)],
        )],
    )
}

fn view_controls_clear(entries_completed: usize) -> Html<Msg> {
    button(
        &[
            class("clear-completed"),
            hidden(entries_completed == 0),
            on_click(Msg::DeleteCompleted),
        ],
        &[text(&format!("Clear completed ({})", entries_completed))],
    )
}

fn info_footer() -> Html<Msg> {
    footer(
        &[class("info")],
        &[
            p(&[], &[text("Double-click to edit a todo")]),
            p(
                &[],
                &[
                    text("Written by "),
                    a(
                        &[href("https://github.com/sindreij")],
                        &[text("Sindre Johansen")],
                    ),
                ],
            ),
            p(
                &[],
                &[
                    text("Part of "),
                    a(&[href("http://todomvc.com")], &[text("TodoMVC")]),
                ],
            ),
        ],
    )
}

pub fn main() -> Program<Model, Msg> {
    Program::new(view, update, init())
}
