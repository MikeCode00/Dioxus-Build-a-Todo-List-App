use dioxus::prelude::*;

use crate::ToDoList;

#[component]
pub fn SingleTodo(id: usize) -> Element {
  let todo_list_signal = use_context::<Signal<ToDoList>>();
  let todo_op = todo_list_signal.read().list.iter().find(|item| item.id == id).cloned();
  let navigator = use_navigator();
  match todo_op {
      None => rsx!(
        div { class: "flex justify-center my-5",
          button {
            class: "bg-slate-50 p-1 rounded hover:bg-slate-100",
            onclick: move |_| navigator.go_back(),
            "go back"
          }
          h1 { class: "text-5xl", "No todo id : {id} Found" }
        }
      ),
      Some(todo) => rsx!(
        div { class: "flex justify-center my-5",
          div { class: "border-solid border-slate-100 border-2 rounded p-1",
            button {
              class: "bg-slate-50 p-1 rounded hover:bg-slate-100",
              onclick: move |_| navigator.go_back(),
              "go back"
            }
            div { class: "text-3xl",
              h1 { "id : {todo.id}" }
              h1 { "content : {todo.content}" }
              h1 { "completed : {todo.completed}" }
            }
          }
        }
      )
  }
}