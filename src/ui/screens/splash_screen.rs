use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fi_icons::FiMusicNote;
use dioxus_router::{use_router, RouterService};

use crate::hooks::use_app::use_app;
use crate::state::app_state::AppState;
use crate::utils::timer_utils::{timeout_ms, delay_ms};

pub fn SplashScreen(cx: Scope) -> Element {
    let app = use_app(cx);
    let loading_text = use_state(&cx, || "".to_string());
    let router = use_router(&cx);

    let app_state = &app.read().current_state;

    use_effect(&cx, (), |_| {
        to_owned![loading_text, router, app_state];

        async move {
            let _ = timeout_ms(3000, animate_loading_text(&loading_text)).await;
            navigate_to_next_screen(&app_state, &router).await;
        }
    });

    async fn animate_loading_text(loading_text: &UseState<String>) {
        let mut dot_count = 0;
        loop {
            loading_text.set(format!("{}", ".".repeat(2 * dot_count + 1)));
            delay_ms(500).await;
            if dot_count < 3 {
                dot_count += 1;
            } else {
                dot_count = 0;
            }
        }
    }

    async fn navigate_to_next_screen(app_state: &AppState, router: &Rc<RouterService>) {
        if let AppState::Splash(splash_state) = app_state {
            if let Ok(is_authorized) = splash_state.is_authorized().await {
                if is_authorized {
                    router.navigate_to("menu");
                    return;
                }
            }
        }

        router.navigate_to("spotify-config");
    }

    render! {
        div {
            class: "flex flex-col h-screen justify-center items-center",
            div { class: "flex flex-col gap-4 h-5/6 items-center",
                div {
                    class: "flex flex-col flex-grow justify-center items-center",
                    div {
                        class: "w-8/12 flex flex-row gap-5 items-center",
                        Icon {
                            class: "text-appGreen",
                            width: 120,
                            height: 120,
                            icon: FiMusicNote,
                        }
                        p {
                            class: "text-appGreen text-[36px]",
                            "myst"
                        }
                    }
                    p {
                        class: "text-appWhite",
                        "A lightweight Spotify client"
                    }
                }
                p {
                    class: "text-appWhite",
                    "{loading_text}"
                }
            }
        }
    }
}
