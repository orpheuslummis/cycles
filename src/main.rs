#![allow(non_snake_case)]

use dioxus::{
    desktop::{Config, LogicalSize, WindowBuilder},
    prelude::*,
};
use dioxus_logger::tracing::{info, Level};

mod cycles;
use cycles::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Cycles {},
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    LaunchBuilder::new()
        .with_cfg(desktop!(Config::new().with_window(
            WindowBuilder::new()
                .with_title("ooo")
                .with_inner_size(LogicalSize::new(500.0, 500.0))
                .with_resizable(false)
                .with_always_on_top(true) // .with_decorations(false)
        )))
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div { Router::<Route> {} }
    }
}
