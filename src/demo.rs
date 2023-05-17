use leptos::*;

use crate::{auth_flow::AuthFlow, error_response::ErrorResponse, token_flow::TokenFlow};

#[component]
pub fn Demo(cx: Scope) -> impl IntoView {
    let client_id = dotenv!("CLIENT_ID");
    let response_type = dotenv!("RESPONSE_TYPE");
    let scope = dotenv!("SCOPE");
    let state = "qwertyuiopasdfghjklzxcvbnm";
    let redirect_uri = dotenv!("REDIRECT_URI");
    let auth_endpoint = dotenv!("AUTH_ENDPOINT");
    let auth_url = format!(
        "{}?client_id={}&response_type={}&scope={}&state={}&redirect_uri={}",
        auth_endpoint, client_id, response_type, scope, state, redirect_uri
    );

    let (token_trigger, set_token_trigger) = create_signal(cx, false);

    view! {
        cx,
        <h3>"Demo OAuth2 Client"</h3>
        <div class="container">
            <div class="row">
                <div class="column column-50 column-offset-25">
                    <pre style="text-align: left;">
                        "client_id: " {client_id}
                        "\nresponse_type: " {response_type}
                        "\nscope: " {scope}
                    </pre>
                    <a href=auth_url>"Start"</a>
                    <AuthFlow set_token_trigger />
                    <TokenFlow trigger=token_trigger/>
                    <ErrorResponse />
                </div>
            </div>
        </div>
    }
}
