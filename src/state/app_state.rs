use super::{
    home_state::HomeState, splash_state::SplashState, spotify_auth_state::SpotifyAuthState,
};

#[derive(Debug, Clone)]
pub enum AppState {
    Splash(SplashState),
    SpotifyAuth(SpotifyAuthState),
    Home(HomeState),
}

impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Splash(l0), Self::Splash(r0)) => l0 == r0,
            (Self::SpotifyAuth(l0), Self::SpotifyAuth(r0)) => l0 == r0,
            (Self::Home(_), Self::Home(_)) => true,
            _ => false,
        }
    }
}
