use dioxus::prelude::*;
use crate::{ToDoList, Route};

use crate::components::{remove_todo::RemoveTodo, toggle_completed::ToggleCompleted};

#[component]
pub fn List() -> Element {
  let todo_list_signal = use_context::<Signal<ToDoList>>();
  rsx!(
    if todo_list_signal.read().list.len() == 0 {
      div { class: "text-center text-xl", "No todos" }
    } else {
      div { class: "h-screen w-full flex justify-center",
        ul { class: "w-1/2",
          {
              todo_list_signal
                  .read()
                  .list
                  .iter()
                  .map(|item| {
                      let id = item.id;
                      let completed = item.completed;
                      rsx! {
                        li { class: "mb-2 flex justify-between items-center bg-slate-100 px-2 py-2 rounded hover:bg-slate-200",
                          div { class: "flex",
                            Link { to: Route::SingleTodo { id: id }, "{item.content}" }
                            if item.completed {
                              div { class: "text-lime-500 ml-2", "√" }
                            } else {
                              div { class: "text-rose-500 ml-2", "X" }
                            }
                          }
                          div {
                            ToggleCompleted { todo_list_signal, id, completed }
                            RemoveTodo { todo_list_signal, id }
                          }
                        }
                      }
                  })
          }
        }
      }
    }
  )
}