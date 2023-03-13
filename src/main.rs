#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::{tao::dpi::Size, Config, PhysicalSize, WindowBuilder};

static STYLES: &'static str = include_str!("../dist/output.css");

fn main() {
    // hot_reload_init!(Config::new().with_paths(&["src"]));
    dioxus_desktop::launch_with_props(
        App,
        (),
        Config::default().with_window(
            WindowBuilder::new()
                .with_title("myst")
                .with_inner_size(Size::Physical(PhysicalSize::new(800, 1200))),
        ),
    );
}

fn App(cx: Scope) -> Element {
    let name = use_state(cx, || "bob".to_string());

    cx.render(rsx! {
        style { STYLES }
        div { class: "flex flex-col",
            input {
                class: "border-2 rounded-sm",
                value: "{name}",
                oninput: move |evt| name.set(evt.value.clone()),
                placeholder: "Token"
            }
            button {
                class: "w-auto px-8 py-2 border-2 text-red-500",
                onclick: |event| {
                    println!("button clicked {event:?}");
                },
                "Authenticate"
            }
        }
    })
}
