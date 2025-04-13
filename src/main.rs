use dioxus::prelude::*;
use dioxus::desktop::{Config, WindowBuilder};

mod components;
use crate::components::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {dioxus::LaunchBuilder::desktop()
    .with_cfg(Config::new().with_window(WindowBuilder::new()
        .with_resizable(false)
        .with_title("Document manager")))
    .launch(App)
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}