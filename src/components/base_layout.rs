use leptos::*;

#[component]
#[allow(non_snake_case)]
pub fn BaseLayout(children: Children) -> impl IntoView {
    view! {
        <html>
            <head>
                <title>"Rust + HTMX todo-list"</title>
                <script src="https://cdn.tailwindcss.com"></script>
                <script src="https://unpkg.com/htmx.org@1.9.10" integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC" crossorigin="anonymous"></script>
            </head>
            <body>
                {children().nodes.collect_view()}
            </body>
        </html>
    }
}
