use dioxus::prelude::*;
use dioxus::logger;
use dioxus::logger::tracing;
use gloo::console::log;
use gloo::utils::document_element;
use gloo::storage::{LocalStorage, Storage};

const FAVICON: Asset = asset!("/assets/favicon.ico");

const CSS: Asset = asset!("/assets/tailwind.css");
const DAISY_CSS: &str = "https://cdn.jsdelivr.net/npm/daisyui@5";
const DAISY_THEMES_CSS: &str = "https://cdn.jsdelivr.net/npm/daisyui@5/themes.css";

fn main() {
    // Init logger
    logger::init(tracing::Level::DEBUG).expect("failed to init logger");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    if let Some(theme) = document_element().get_attribute("data-theme") {
        tracing::info!("theme = {}", theme);
    }
    rsx! {
        document::Stylesheet { href: CSS },
        document::Stylesheet { href: DAISY_CSS },
        document::Stylesheet { href: DAISY_THEMES_CSS },
        document::Link { href: FAVICON, rel: "icon" }

        div {
            div { "Hello, world!" },
            button {
                onclick: move |_| handle_click(),
                class: "btn btn-primary",
                "Click me!",
            }
        }
    }
}

fn handle_click() {
    log!("clicked");
    
    let value: String = LocalStorage::get("theme").unwrap_or("light".to_string());
    // log!("value = %s", value);
    tracing::debug!("value = {}", value);

}
