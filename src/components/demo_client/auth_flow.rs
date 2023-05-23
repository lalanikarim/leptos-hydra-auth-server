use leptos::*;
use leptos_router::use_query_map;

#[component]
pub fn AuthFlow(cx: Scope, set_token_trigger: WriteSignal<bool>) -> impl IntoView {
    let query = use_query_map(cx);
    let auth_code = move || query.get().get("code").cloned();
    let client_id = dotenv!("CLIENT_ID");
    let client_secret = dotenv!("CLIENT_SECRET");
    let grant_type = dotenv!("GRANT_TYPE");
    let scope = dotenv!("SCOPE");
    let redirect_uri = dotenv!("REDIRECT_URI");
    if let Some(auth_code) = auth_code() {
        Some(view! {
            cx,
            <h3>"Auth Flow Response"</h3>
            <table>
                <thead>
                    <tr><th>"Param"</th><th>"Value"</th></tr>
                </thead>
                <tbody>
                    <tr><td>"code"</td><td style="overflow-wrap: anywhere">{auth_code}</td></tr>
                    <tr><td>"grant_type"</td><td>{grant_type}</td></tr>
                    <tr><td>"redirect_uri"</td><td>{redirect_uri}</td></tr>
                    <tr><td>"scope"</td><td>{scope}</td></tr>
                    <tr><td>"client_id"</td><td>{client_id}</td></tr>
                    <tr><td>"client_secret"</td><td>{client_secret}</td></tr>
                </tbody>
            </table>
            <button on:click=move|_|set_token_trigger.set(true) >"Exchange Token"</button>
        })
    } else {
        None
    }
}
