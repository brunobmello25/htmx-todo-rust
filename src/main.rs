mod components;
mod models;

use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    response::Html,
    routing::{delete, get, post},
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

async fn delete_task(State(state): State<AppState>, Path(task_id): Path<u32>) {
    let mut todos = state.todos.lock().unwrap();

    let index = todos.iter().position(|t| t.id == task_id).unwrap();

    todos.remove(index);
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

    let new_todo = Todo {
        done: false,
        content: body.task,
        id: max_id + 1,
    };

    state.todos.lock().unwrap().insert(0, new_todo.clone());

    let html = ssr::render_to_string(move || {
        view! {
            <Todo todo={&new_todo} />
        }
    });
    let html = html.to_string();

    return Html(html);
}

async fn index(State(state): State<AppState>) -> Html<String> {
    let html = ssr::render_to_string(|| {
        view! {
            <BaseLayout>
                <div class="space-y-4">
                    <h1 class="text-2xl font-bold">Todo List</h1>
                    <AddTodoForm />
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
        .route("/tasks/:task_id", delete(delete_task))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
