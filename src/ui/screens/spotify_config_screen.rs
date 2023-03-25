use std::{sync::{Arc, Mutex, RwLock}, rc::Rc};

use arboard::Clipboard;
use dioxus::prelude::*;
use dioxus_desktop::use_window;
use dioxus_router::use_router;

use crate::ui::screens::request_authorization_screen::{RequestAuthorizationScreen, RequestAuthorizationScreenProps};
use crate::spotify::{
    spotify_auth::SpotifyAuth,
    client_config::{ClientConfig, DEFAULT_CONFIG_PORT},
};

pub fn SpotifyConfigScreen(cx: Scope) -> Element {
    let window = use_window(&cx);
    let router = use_router(&cx);
    let client_id = use_state(&cx, || "".to_string());
    let client_secret = use_state(&cx, || "".to_string());
    let port = use_state(&cx, || DEFAULT_CONFIG_PORT.to_string());
    let spotify_auth = Arc::new(Mutex::new(SpotifyAuth::new()));

    use_effect(&cx, (), |_| {
        to_owned![client_id, client_secret];
        async move {
            let config = ClientConfig::load_config().unwrap_or(ClientConfig::new());
            client_id.set(config.client_id);
            client_secret.set(config.client_secret);
        }
    });

    render! {
        div { class: "w-screen h-screen flex flex-col justify-center items-center",
          div { class: "flex flex-col gap-4 px-12",
            p {
                class: "text-appWhite",
                span { "1. Open " }
                span {
                    class: "text-appGreen underline underline-offset-2 hover:cursor-pointer",
                    onclick: |_| {
                        open::that("https://developer.spotify.com/dashboard/").unwrap();
                    },
                    "Spotify Dashboard",
                }
                span { " and create an app" }
            }
            p {
                class: "text-appWhite",
                "2. Enter Client ID & Client Secret below"
            }
            input {
                class: "rounded-md px-2 py-1 border-2 border-slate-500",
                placeholder: "Client ID",
                value: "{client_id}",
                oninput: move |evt| client_id.set(evt.value.clone()),
            }
            input {
                class: "rounded-md px-2 py-1 border-2 border-slate-500",
                placeholder: "Client Secret",
                value: "{client_secret}",
                oninput: move |evt| client_secret.set(evt.value.clone()),
            }
            p {
                class: "text-appWhite",
                "3. Enter port (Optional)"
            }
            input {
                class: "rounded-md px-2 py-1 border-2 border-slate-500",
                placeholder: "Port",
                value: "{port}",
                oninput: move |evt| port.set(evt.value.clone()),
            }
            p {
                class: "text-appWhite",
                "4. Click Edit Settings & add below URL to Redirect URIs"
            }
            div { class: "rounded-md border-[1px] border-appWhite border-opacity-30 flex flex-row px-2 py-1.5 hover:bg-appWhite hover:bg-opacity-30 hover:cursor-pointer",
                onclick: move |_| {
                    if let Ok(mut clipboard) = Clipboard::new() {
                        let text = ClientConfig::get_redirect_uri(port.parse::<u16>().unwrap_or(DEFAULT_CONFIG_PORT));
                        let _ = clipboard.set_text(text);
                    }
                },
                p {
                    class: "text-appWhite flex-auto",
                    ClientConfig::get_redirect_uri(port.parse::<u16>().unwrap_or(DEFAULT_CONFIG_PORT))
                }
                div {
                    class: "text-appWhite",
                    dangerous_inner_html: "{COPY_SVG}"
                }
            }
            button {
                class: "rounded-md bg-appGreen px-4 py-1.5 mt-2 text-appWhite",
                onclick: move |_| {
                    let client_config = ClientConfig {
                        client_id: client_id.to_string(),
                        client_secret: client_secret.to_string(),
                        port: match port.parse::<u16>() {
                            Ok(port_u16) => port_u16,
                            Err(_) => {
                                port.set(DEFAULT_CONFIG_PORT.to_string());
                                DEFAULT_CONFIG_PORT
                            }
                        },
                    };
                    let _ = client_config.save_config();

                    cx.spawn({
                        to_owned![router, window];

                        let initial_auth = Arc::clone(&spotify_auth);
                        let on_complete_auth = Arc::clone(&spotify_auth);
                        async move {
                            let mut initial_auth = initial_auth.lock().unwrap();
                            match initial_auth.oauth().await {
                                Ok(url) => {
                                    let dom = VirtualDom::new_with_props(
                                        RequestAuthorizationScreen,
                                        RequestAuthorizationScreenProps {
                                            url: url,
                                            on_complete: Rc::new(RwLock::new(move |redirect_url| {
                                                println!("callback url: {}", redirect_url);
                                                to_owned![router, on_complete_auth];
                                                async move {
                                                    if let Ok(spotify_client) = on_complete_auth.lock().unwrap().continue_oauth(redirect_url).await {
                                                        router.navigate_to("/menu")
                                                    }
                                                }
                                            }))
                                        },
                                    );
                                    window.new_window(dom, Default::default());
                                    drop(initial_auth);
                                },
                                Err(e) => {
                                    println!("OAuth failed: {}", e);
                                },
                            };
                        }
                    });
                },
                "Authenticate",
            }
          }
        }
    }
}

const COPY_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
  <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 01-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 011.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 00-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 01-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 00-3.375-3.375h-1.5a1.125 1.125 0 01-1.125-1.125v-1.5a3.375 3.375 0 00-3.375-3.375H9.75" />
</svg>
"#;
