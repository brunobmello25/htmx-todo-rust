use leptos::*;

use crate::components::Todo;
use crate::models;

use std::sync::{Arc, Mutex};

#[allow(non_snake_case)]
#[component]
pub fn TodoList(todos: Arc<Mutex<Vec<models::todo::Todo>>>) -> impl IntoView {
    let todos = todos.lock().unwrap();

    let todos = todos
        .iter()
        .map(|todo| {
            view! {
                <Todo todo={todo} />
            }
        })
        .collect_view();

    view! {
    <div id="todo-list" class="p-4 md:p-6 lg:p-8">
      <div class="border rounded-lg mt-4">
        <div class="relative w-full overflow-auto">
          <div class="relative w-full overflow-auto">
            <table class="w-full caption-bottom text-sm">
              <tbody class="[&amp;_tr:last-child]:border-0">
                {todos}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
    }
}
