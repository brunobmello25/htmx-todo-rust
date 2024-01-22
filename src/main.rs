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
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
