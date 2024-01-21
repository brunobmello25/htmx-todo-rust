mod components;
mod models;

use std::sync::{Arc, Mutex};

use axum::{extract::State, response::Html, routing::get, Router};
use components::*;
use leptos::*;
use models::*;

#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<Vec<Todo>>>,
}

async fn index(State(state): State<AppState>) -> Html<String> {
    let html = ssr::render_to_string(|| {
        view! {
            <BaseLayout>
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
    let app = Router::new().route("/", get(index)).with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
