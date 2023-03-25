#[derive(Debug, PartialEq)]
pub struct SplashState;

impl SplashState {
    pub fn is_auth_required() -> bool {
        true
    }
}
