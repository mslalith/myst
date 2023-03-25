use super::{app_state::AppState, splash_state::SplashState};

pub struct App {
    current_state: AppState,
}

impl App {
    pub fn new() -> App {
        App {
            current_state: AppState::Splash(SplashState),
        }
    }

    pub fn move_app_state_to(&mut self, state: AppState) {
        self.current_state = state;
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{
        app::App, app_state::AppState, splash_state::SplashState,
        spotify_auth_state::SpotifyAuthState,
    };

    #[test]
    fn create_new_app() {
        let app = App::new();
        assert_eq!(app.current_state, AppState::Splash(SplashState));
    }

    #[test]
    fn move_state() {
        let mut app = App::new();
        app.move_app_state_to(AppState::SpotifyAuth(SpotifyAuthState));
        assert_eq!(app.current_state, AppState::SpotifyAuth(SpotifyAuthState));
    }
}
