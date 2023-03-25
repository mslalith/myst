use anyhow::{Result, Ok};

use crate::spotify::auth::SpotifyAuth;

#[derive(Debug, PartialEq, Clone)]
pub struct SplashState;

impl SplashState {
    pub async fn is_authorized(&self) -> Result<bool> {
        let is_config_required = SpotifyAuth::is_configuration_required().await?;
        Ok(!is_config_required)
    }
}
