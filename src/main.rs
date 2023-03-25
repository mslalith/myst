#![allow(non_snake_case)]
mod hooks;
mod spotify;
mod state;
mod ui;
mod utils;

use dioxus::prelude::*;
use dioxus_desktop::tao::menu::{MenuBar, MenuItem};

use dioxus_desktop::{tao::dpi::Size, Config, PhysicalSize, WindowBuilder};
use dioxus_router::{Route, Router};

use crate::state::app::App;
use crate::ui::screens::menu_screen::MenuScreen;
use crate::ui::screens::splash_screen::SplashScreen;
use crate::ui::screens::spotify_config_screen::SpotifyConfigScreen;

static STYLES: &'static str = include_str!("../dist/output.css");

fn main() {
    // hot_reload_init!(dioxus_hot_reload::Config::new().with_paths(&["src"]));
    dioxus_desktop::launch_with_props(App, (), get_launch_config());
}

fn get_launch_config() -> Config {
    let size = Size::Physical(PhysicalSize::new(800, 1400));

    Config::default().with_window(
        WindowBuilder::new()
            .with_title("myst")
            .with_min_inner_size(size)
            .with_max_inner_size(size)
            .with_menu({
                let mut menu = MenuBar::new();

                let mut app_menu = MenuBar::new();
                app_menu.add_native_item(MenuItem::Quit);

                menu.add_submenu("&myst", true, app_menu);
                menu
            }),
    )
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || App::new());

    render! {
        style { STYLES }
        div {
            class: "w-screen h-screen bg-appBlack",
            Router {
                Route { to: "/", SplashScreen {} }
                Route { to: "/spotify-config", SpotifyConfigScreen {} }
                Route { to: "/menu", MenuScreen {} }
            }
        }
    }
}
