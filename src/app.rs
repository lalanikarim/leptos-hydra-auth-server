use crate::{consent::Consent, demo::Demo, login::*};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <HomePage/> }>
                        <Route path="" view=|cx| view! { cx, <Demo /> }/>
                        <Route path="login" view=|cx| view! { cx, <Login/> }/>
                        <Route path="consent" view=|cx| view! { cx, <Consent/> }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button

    view! { cx,
        <h1><A href="/">"Leptos Hydra Auth"</A></h1>
        <Outlet />
    }
}
