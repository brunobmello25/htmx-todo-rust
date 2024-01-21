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
    <div class="p-4 md:p-6 lg:p-8">
      <div class="space-y-4">
        <h1 class="text-2xl font-bold">Todo List</h1>
        <form class="flex space-x-2">
          <input
            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 flex-1"
            placeholder="Add a task"
          />
          <button
            class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
            type="submit"
          >
            Add
          </button>
        </form>
      </div>
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
