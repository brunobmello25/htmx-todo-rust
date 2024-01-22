mod components;
mod models;

use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Form, Router,
};
use components::*;
use leptos::*;
use models::*;
use serde::Deserialize;

#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<Vec<Todo>>>,
}

#[derive(Debug, Deserialize)]
struct CreateTask {
    task: String,
}

async fn create_task(State(state): State<AppState>, Form(body): Form<CreateTask>) -> Html<String> {
    let max_id = state
        .todos
        .lock()
        .unwrap()
        .iter()
        .max_by_key(|t| t.id)
        .map(|t| t.id)
        .unwrap_or(0);

    state.todos.lock().unwrap().push(Todo {
        id: max_id + 1,
        content: body.task,
        done: false,
    });

    let html = ssr::render_to_string(|| {
        view! {
            <TodoList todos={state.todos} />
        }
    });

    return Html(html.to_string());
}

async fn index(State(state): State<AppState>) -> Html<String> {
    let html = ssr::render_to_string(|| {
        view! {
            <BaseLayout>
                <div class="space-y-4">
                    <h1 class="text-2xl font-bold">Todo List</h1>
                    <form hx-post="/tasks" hx-target="#todo-list" class="flex space-x-2">
                      <input
                        class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 flex-1"
                        placeholder="Add a task"
                        name="task"
                      />
                      <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
                        type="submit"
                      >
                        Add
                      </button>
                    </form>
                </div>
                <TodoList todos={state.todos.clone()} />
            </BaseLayout>
        }
    })
    .to_string();

    return Html(html);
}

#[tokio::main]
async fn main() {
    let state = AppState {
        todos: Arc::new(Mutex::new(vec![])),
    };

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/tasks", post(create_task))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
