#![allow(non_snake_case)]

use dioxus::prelude::*;
use components::{list::List, single_todo::SingleTodo, add_todo::AddTodo};
mod components;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    Home{},
    #[route("/todo/:id")]
    SingleTodo {id: usize}
}

fn main() {
    launch(App);
}

#[component]
fn Title() -> Element {
    rsx!(
        h1 { class: "my-5 text-center text-orange-500 text-5xl", "ToDo List" }
    )
}

#[component]
fn App() -> Element {
    let todo_list = ToDoList {
        list: vec![
            ToDo {
                id: 0,
                content: "Jog".to_string(),
                completed: true,
            },
            ToDo {
                id: 1,
                content: "Walk".to_string(),
                completed: false
            }
        ]
    };
    use_context_provider(|| Signal::new(todo_list));
    rsx!(
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        Title {}
        Router::<Route> {}
    )
}

#[component]
fn Home() -> Element {
    rsx!(
        AddTodo {}
        List {}
    )
}

#[derive(PartialEq, Clone)]
struct ToDo {
    id: usize,
    content: String,
    completed: bool
}
#[derive(PartialEq, Clone)]
struct ToDoList {
    list: Vec<ToDo>
}