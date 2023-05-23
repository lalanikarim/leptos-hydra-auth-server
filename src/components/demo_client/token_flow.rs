use leptos::*;
use leptos_router::use_query_map;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequest {
    pub code: String,
    pub grant_type: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    scope: String,
    token_type: String,
    expires_in: usize,
}

#[component]
pub fn TokenFlow(cx: Scope, trigger: ReadSignal<bool>) -> impl IntoView {
    let query = use_query_map(cx);
    let client_id = dotenv!("CLIENT_ID");
    let client_secret = dotenv!("CLIENT_SECRET");
    let grant_type = dotenv!("GRANT_TYPE");
    let redirect_uri = "http://127.0.0.1:3000";
    let auth_code = move || query.get().get("code").cloned();
    let token_request = move || {
        auth_code().map(|code| TokenRequest {
            code,
            redirect_uri: redirect_uri.to_string(),
            grant_type: grant_type.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
        })
    };
    let token_resource = create_resource(
        cx,
        move || trigger.get(),
        move |trigger| {
            let form = token_request.clone();
            let form = form();
            async move {
                if trigger && form.is_some() {
                    let token_url = String::from("http://127.0.0.1:4444/oauth2/token");
                    let client = reqwest::Client::new();
                    let response = client.post(token_url).form(&form).send().await;
                    let response = response.unwrap();
                    let response = response.json::<TokenResponse>().await;
                    response.map_err(|err: reqwest::Error| {
                        ServerFnError::ServerError(format!("Reqwest Error: {err:?}"))
                    })
                } else {
                    Err(ServerFnError::ServerError("Not ready".to_string()))
                }
            }
        },
    );

    view! {
        cx,
        <Suspense fallback=move ||view!{cx,<div>"Loading..."</div>}>
            {
                token_resource.read(cx).map(|result| result.map(|token| {
                    view!{
                        cx,
                        <h3>"Token Flow Response"</h3>
                        <div style="overflow-wrap: anywhere">"Access token: " {token.access_token} </div>
                    }
                }))
            }
        </Suspense>
    }
}
