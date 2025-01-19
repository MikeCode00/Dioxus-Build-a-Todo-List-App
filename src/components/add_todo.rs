use dioxus::prelude::*;

use crate::{ToDoList, ToDo};


#[component]
pub fn AddTodo() -> Element {
  let mut todo_content = use_signal(|| String::new());
  let mut todo_list_signal = use_context::<Signal<ToDoList>>();

  rsx!(
    div { class: "flex justify-center w-full my-2",
      div { class: "w-1/2",
        input {
          class: "w-3/4 p-1 border-2 border-slate-100 border-solid rounded",
          r#type: "text",
          value: todo_content,
          oninput: move |e| todo_content.set(e.value()),
        }
        button {
          class: "text-slate-50 bg-sky-500 p-1 rounded w-1/4 hover:bg-sky-600",
          onclick: move |e| {
              e.stop_propagation();
              let mut todo_list = (*todo_list_signal.read()).clone();
              let todo = ToDo {
                  id: if todo_list.list.len() == 0 {
                      0
                  } else {
                      todo_list.list.last().unwrap().id + 1
                  },
                  content: (*todo_content.read()).clone(),
                  completed: false,
              };
              todo_list.list.push(todo);
              todo_list_signal.set(todo_list);
              todo_content.set(String::new());
          },
          // disabled: if todo_content.to_string().trim() == "" { true } else { false },
          "Add"
        }
      }
    }
  )
}