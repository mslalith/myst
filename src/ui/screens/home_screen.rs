use dioxus::prelude::*;

pub fn HomeScreen(cx: Scope) -> Element {
    render! {
        div { class: "w-screen h-screen flex flex-col justify-center items-center",
          div { class: "flex flex-col gap-4 px-12",
            p {
              class: "text-white",
              "Albums"
            }
            p {
              class: "text-white",
              "Artists"
            }
          }
        }
    }
}
