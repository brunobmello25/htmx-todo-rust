use leptos::*;

use crate::models::*;

#[allow(non_snake_case)]
#[component]
pub fn Todo<'a>(todo: &'a Todo) -> impl IntoView {
    view! {
    <tr
      class="border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted"
    >
      <td class="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">
        <button
        type="button"
        role="checkbox"
        aria-checked="false"
        data-state="unchecked"
        value="on"
        class="peer h-4 w-4 shrink-0 rounded-sm border border-primary ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground"
        ></button>
      </td>
      <td
        class="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0 font-medium"
      >
        {todo.content.clone()}
      </td>
      <td
      class="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0 text-right"
      >
        <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
          Delete
        </button>
      </td>
    </tr>
    }
}
