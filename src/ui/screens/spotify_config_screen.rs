use std::{sync::{Arc, Mutex, RwLock}, rc::Rc};

use arboard::Clipboard;
use dioxus::prelude::*;
use dioxus_desktop::use_window;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fi_icons::FiPc;
use dioxus_router::use_router;

use crate::state::{app_state::AppState, home_state::HomeState};
use crate::ui::components::{
    button::Button,
    input::Input,
    text::Text,
    link_text::{LinkText, LinkTextType}
};
use crate::hooks::use_app::use_app;
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

    let app = use_app(cx).read().clone();

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
                LinkText {
                    types: vec![
                        LinkTextType::Text("1. Open ".to_string()),
                        LinkTextType::Link("Spotify Dashboard".to_string(), Box::new(|| {
                            open::that("https://developer.spotify.com/dashboard/").unwrap();
                        })),
                        LinkTextType::Text(" and create an app".to_string()),
                    ]
                }

                Text { value: "2. Enter Client ID & Client Secret below".to_string() }
                Input {
                    placeholder: "Client ID",
                    value: "{client_id}",
                    on_change: move |evt: Event<FormData>| client_id.set(evt.value.clone()),
                }
                Input {
                    placeholder: "Client Secret",
                    value: "{client_secret}",
                    on_change: move |evt: Event<FormData>| client_secret.set(evt.value.clone()),
                }

                Text { value: "3. Enter port (Optional)".to_string() }
                Input {
                    placeholder: "Port",
                    value: "{port}",
                    on_change: move |evt: Event<FormData>| port.set(evt.value.clone()),
                }

                Text { value: "4. Click Edit Settings & add below URL to Redirect URIs".to_string() }
                CopyRedirectUrl { port: port }

                Button {
                    text: "Authenticate",
                    on_click: move |_| {
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
                            to_owned![router, window, app];
    
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
                                                    to_owned![router, on_complete_auth, app];
                                                    async move {
                                                        if let Ok(spotify_client) = on_complete_auth.lock().unwrap().continue_oauth(redirect_url).await {
                                                            app.move_app_state_to(AppState::Home(HomeState::new(spotify_client)));
                                                            router.navigate_to("/home")
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
                }
            }
        }
    }
}

#[inline_props]
fn CopyRedirectUrl<'a>(cx: Scope, port: &'a str) -> Element {
    render! {
        div { class: "rounded-md border-[1px] border-white border-opacity-30 flex flex-row px-2 py-1.5 hover:bg-white hover:bg-opacity-30 hover:cursor-pointer",
            onclick: move |_| {
                if let Ok(mut clipboard) = Clipboard::new() {
                    let text = ClientConfig::get_redirect_uri(port.parse::<u16>().unwrap_or(DEFAULT_CONFIG_PORT));
                    let _ = clipboard.set_text(text);
                }
            },
            Text {
                value: ClientConfig::get_redirect_uri(port.parse::<u16>().unwrap_or(DEFAULT_CONFIG_PORT)),
                class: "flex-auto",
            }
            Icon {
                class: "text-white",
                width: 24,
                height: 24,
                icon: FiPc
            }
        }
    }
}
