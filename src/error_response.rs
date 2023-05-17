use leptos::*;
use leptos_router::use_query_map;
#[component]
pub fn ErrorResponse(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let error = move || query.get().get("error").cloned();
    let error_description = move || query.get().get("error_description").cloned();
    if let (Some(error), Some(error_description)) = (error(), error_description()) {
        Some(view! {
            cx,
            <pre style="text-align: left;">
                "Error: " {error}
                "\nDescription: " {error_description}
            </pre>
        })
    } else {
        None
    }
}
