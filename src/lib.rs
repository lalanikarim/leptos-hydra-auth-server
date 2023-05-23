use cfg_if::cfg_if;
pub mod components;
pub mod error_template;
pub mod fileserv;

#[macro_use]
extern crate dotenv_codegen;
cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::components::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move |cx| {
            view! { cx, <App/> }
        });
    }
}}

cfg_if! {
    if #[cfg(feature = "ssr")] {

        use ory_hydra_client::apis::configuration::Configuration;
        pub fn get_config() -> Configuration {
            let mut config = Configuration::new();
            config.base_path = "http://127.0.0.1:4445".to_string();
            config.client = reqwest::Client::new();
            config
        }
    }
}
