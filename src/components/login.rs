use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::*;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use ory_hydra_client::apis::*;
        use ory_hydra_client::models::{AcceptOAuth2LoginRequest, OAuth2RedirectTo};
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LoginResponse {
    RedirectTo(String),
    ErrorFetchingRequest(String),
    ErrorAcceptingRequest(String),
}

#[server(LoginRequest, "/api")]
async fn login_request(
    cx: Scope,
    email: String,
    password: String,
    login_challenge: String,
) -> Result<LoginResponse, ServerFnError> {
    log!("Email: {email}, Password: {password}, Login challenge: {login_challenge}");
    let configuration = crate::get_config();
    let resp: LoginResponse = match o_auth2_api::get_o_auth2_login_request(
        &configuration,
        login_challenge.as_ref(),
    )
    .await
    {
        Ok(_request) => {
            let accept_request = AcceptOAuth2LoginRequest::new(email);
            match o_auth2_api::accept_o_auth2_login_request(
                &configuration,
                login_challenge.as_str(),
                Some(&accept_request),
            )
            .await
            {
                Ok(OAuth2RedirectTo { redirect_to }) => LoginResponse::RedirectTo(redirect_to),
                Err(accept_request_error) => LoginResponse::ErrorAcceptingRequest(format!(
                    "Login Request Accept Error: {accept_request_error:?}"
                )),
            }
        }
        Err(error) => LoginResponse::ErrorFetchingRequest(format!("Login Request Error {error:?}")),
    };
    Ok(resp)
}
#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let login_challenge = move || query.get().get("login_challenge").cloned();

    if let Some(login_challenge) = login_challenge() {
        let login_challenge = login_challenge.clone();
        let email_element = create_node_ref::<Input>(cx);
        let password_element = create_node_ref::<Input>(cx);

        let on_submit = move |ev: SubmitEvent| {
            ev.prevent_default();
            let login_challenge = login_challenge.clone();
            let email = email_element
                .get()
                .expect("email should be present")
                .value();
            let password = password_element
                .get()
                .expect("password should be present")
                .value();
            log!("Email: {email}");
            log!("Password: {password}");
            log!("Login Challenge: {login_challenge}");
            spawn_local(async move {
                if let Ok(response) =
                    login_request(cx, email, password, login_challenge.clone()).await
                {
                    match response {
                        LoginResponse::RedirectTo(redirect_to) => {
                            let location = leptos::window().location();
                            log!("{location:?}");
                            _ = location.assign(&redirect_to);
                        }
                        LoginResponse::ErrorFetchingRequest(error) => panic!("{error}"),
                        LoginResponse::ErrorAcceptingRequest(error) => panic!("{error}"),
                    }
                } else {
                    panic!("Unexpected error encountered")
                }
            });
        };

        view! {
            cx,
            <h2>"Login"</h2>
            <div class="container">
                <div class="row">
                    <div class="column column-33 column-offset-33">
                        <form on:submit=on_submit>
                            <fieldset>
                                <label for="email" class="float-left">"Email"</label>
                                <input id="email" _ref=email_element type="text"/>
                                <label for="password" class="float-left">"Password"</label>
                                <input id="password" _ref=password_element type="password"/>
                                <button class="button" type="submit">"Login"</button>
                                <button class="button button-clear">"Forgot password?"</button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        }
        .into_view(cx)
    } else {
        view! {
            cx,
            <h3>"Invalid Request"</h3>
        }
        .into_view(cx)
    }
}
