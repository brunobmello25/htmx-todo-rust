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
                <div class="flex flex-col min-h-screen bg-gray-100">
                    <header class="p-4 bg-white shadow-md">
                      <div class="container mx-auto flex items-center justify-between">
                        <h1 class="text-2xl font-bold text-gray-800">HTMX todo</h1>
                      </div>
                    </header>
                    <main class="flex-1 p-4 md:p-6 lg:p-8">
                        {children().nodes.collect_view()}
                    </main>
                    <footer class="p-4 bg-white shadow-md mt-auto">
                      <div class="container mx-auto text-center text-gray-600">
                        <p>"Hehe I don't know what to put here."</p>
                      </div>
                    </footer>
                </div>
            </body>
        </html>
    }
}
