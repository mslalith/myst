use dioxus::prelude::{use_shared_state, ScopeState, UseSharedState};

use crate::App;

pub fn use_app(cx: &ScopeState) -> UseSharedState<App> {
    use_shared_state::<App>(cx).expect("App not provided.")
}
