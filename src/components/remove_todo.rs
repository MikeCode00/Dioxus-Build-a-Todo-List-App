use dioxus::prelude::*;

use crate::{ToDo, ToDoList};

#[component]
pub fn RemoveTodo(todo_list_signal: Signal<ToDoList>, id: usize) -> Element {
  rsx!(
    button {
      class: "text-slate-50 bg-rose-500 rounded p-1 ml-2 hover:bg-rose-600",
      onclick: move |_| {
          let new_list = todo_list_signal
              .read()
              .list
              .clone()
              .iter()
              .filter(|item| item.id != id)
              .map(|item| item.clone())
              .collect::<Vec<ToDo>>();
          todo_list_signal.set(ToDoList { list: new_list });
      },
      "remove"
    }
  )
}