use leptos::*;
use leptos_router::use_query_map;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use ory_hydra_client::apis::o_auth2_api::*;
        use ory_hydra_client::models::*;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsentResponse {
    RedirectTo(String),
    PromptConsent(Option<Vec<String>>),
    NotReady,
}

#[server(AcceptConsentRequest, "/api")]
async fn accept_consent_request(
    consent_challenge: String,
    accept_consent: bool,
) -> Result<ConsentResponse, ServerFnError> {
    let configuration = crate::get_config();
    if accept_consent {
        let accept_consent_request = AcceptOAuth2ConsentRequest::new();
        accept_o_auth2_consent_request(
            &configuration,
            consent_challenge.as_ref(),
            Some(&accept_consent_request),
        )
        .await
        .map(|OAuth2RedirectTo { redirect_to }| ConsentResponse::RedirectTo(redirect_to))
        .map_err(|err| ServerFnError::ServerError(format!("Error accepting consent: {err:?}")))
    } else {
        let reject_consent_request = RejectOAuth2Request::new();
        reject_o_auth2_consent_request(
            &configuration,
            consent_challenge.as_ref(),
            Some(&reject_consent_request),
        )
        .await
        .map(|OAuth2RedirectTo { redirect_to }| ConsentResponse::RedirectTo(redirect_to))
        .map_err(|err| ServerFnError::ServerError(format!("Error accepting consent: {err:?}")))
    }
}

#[server(ConsentRequest, "/api")]
async fn get_consent(
    cx: Scope,
    consent_challenge: String,
) -> Result<ConsentResponse, ServerFnError> {
    let configuration = crate::get_config();
    match get_o_auth2_consent_request(&configuration, consent_challenge.as_ref()).await {
        Ok(request) => {
            if let Some(skip) = request.skip {
                if skip {
                    return accept_consent_request(consent_challenge, true).await;
                }
            }
            Ok(ConsentResponse::PromptConsent(request.requested_scope))
        }
        Err(err) => Err(ServerFnError::ServerError(format!(
            "Error fetching consent: {err:?}"
        ))),
    }
}

#[component]
pub fn Consent(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let consent_challenge = move || query.get().get("consent_challenge").cloned();
    let (accept, set_accept) = create_signal(cx, None);
    let consent_prompt = create_resource(
        cx,
        move || (consent_challenge(), accept.get()),
        move |(consent_challenge, accept)| async move {
            if let Some(consent_challenge) = consent_challenge {
                if let Some(accept) = accept {
                    accept_consent_request(consent_challenge, accept).await
                } else {
                    get_consent(cx, consent_challenge).await
                }
            } else {
                Result::Err(ServerFnError::Args("Consent challenge missing".to_string()))
            }
        },
    );
    let handle_consent_response = move |response: ConsentResponse| {
        if let ConsentResponse::RedirectTo(redirect_to) = response.clone() {
            _ = window().location().assign(&redirect_to);
        }
        match response {
            ConsentResponse::PromptConsent(scopes) => view! {cx,
                <h3>"Do you consent?"</h3>
                {
                    if let Some(scopes) = scopes {
                        view!{cx,
                            <ul>{
                                scopes
                                .into_iter()
                                .map(|s| view!{cx, <li>{s}</li>})
                                .collect::<Vec<_>>()
                            }</ul>
                        }.into_view(cx)
                    } else {
                        view!{cx, <div />}.into_view(cx)
                    }
                }
                <button class="button" on:click=move|_| { set_accept.set(Some(true)) } >"Yes"</button>
                <button class="button" on:click=move|_| { set_accept.set(Some(false)) } >"No"</button>
            }
            .into_view(cx),
            _ => view! {cx, <div>"Loading..."</div>}.into_view(cx),
        }
    };
    if let Some(_) = consent_challenge() {
        view! {
            cx,
            <h2>"Consent"</h2>
            <Suspense fallback=move|| view!{cx, <div>"Loading..."</div>}>
            {
                consent_prompt.read(cx).map(|response|{
                    handle_consent_response(response.unwrap())
                })
            }
            </Suspense>
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
