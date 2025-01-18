use dioxus::prelude::*;

use crate::{ToDo,  ToDoList};

#[component]
pub fn ToggleCompleted(todo_list_signal: Signal<ToDoList>, id: usize, completed: bool) -> Element {
  rsx!(
    button {
      class: if completed { "text-slate-50 bg-orange-500 p-1 rounded hover:bg-orange-600" } else { "text-slate-50 bg-lime-500 p-1 rounded hover:bg-lime-600" },
      onclick: move |_| {
          let new_list = todo_list_signal
              .read()
              .list
              .clone()
              .iter()
              .map(|item| {
                  if item.id == id {
                      ToDo {
                          id: id,
                          content: item.content.clone(),
                          completed: !completed,
                      }
                  } else {
                      item.clone()
                  }
              })
              .collect::<Vec<ToDo>>();
          todo_list_signal.set(ToDoList { list: new_list });
      },
      if completed {
        "uncomplete"
      } else {
        "complete"
      }
    }
  )
}