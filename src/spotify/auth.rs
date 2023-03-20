use std::collections::HashSet;

use anyhow::{Ok, Result, anyhow};
use rspotify::prelude::OAuthClient;
use rspotify::{ClientCredsSpotify, Credentials, scopes, AuthCodeSpotify, OAuth};

use super::client_config::ClientConfig;
use super::spotify_client::SpotifyClient;

#[derive(Clone)]
pub struct SpotifyAuth {
    spotify: Option<AuthCodeSpotify>
}

impl SpotifyAuth {

    pub fn new() -> Self {
        SpotifyAuth { spotify: None }
    }

    pub async fn is_configuration_required() -> Result<bool> {
        let client_config = ClientConfig::load_config()?;
        let spotify = SpotifyAuth::get_client_creds(&client_config)?;
        let is_config_required = match spotify.read_token_cache().await? {
            Some(token) => {
                if token.is_expired() {
                    spotify.request_token().await?;
                }
                false
            },
            None => {
                spotify.request_token().await?;
                false
            },
        };
        Ok(is_config_required)
    }

    pub async fn oauth(&mut self) -> Result<String> {
        let client_config = ClientConfig::load_config()?;
        let creds = Credentials {
            id: client_config.client_id.clone(),
            secret: Some(client_config.client_secret.clone()),
        };

        let mut oauth = OAuth::default();
        oauth.scopes = SpotifyAuth::get_scopes();
        oauth.redirect_uri = ClientConfig::get_redirect_uri(client_config.port);

        let mut config = rspotify::Config::default();
        config.cache_path = ClientConfig::get_config_paths()?.token_cache_path;
        config.token_cached = true;
        config.token_refreshing = true;

        let spotify = AuthCodeSpotify::with_config(creds, oauth, config);
        let url = spotify.get_authorize_url(true)?;

        self.spotify = Some(spotify);
        Ok(url)
    }

    pub async fn continue_oauth(&self, redirect_url: String) -> Result<SpotifyClient> {
        let spotify = *&self.spotify.as_ref().unwrap();
        if let Some(code) = spotify.parse_response_code(&redirect_url) {
            spotify.request_token(&code).await?;
            let me = spotify.me().await?;
            return Ok(SpotifyClient::new(spotify.clone(), me))
        }

        Err(anyhow!("Unable to fetch token"))
    }

    fn get_client_creds(client_config: &ClientConfig) -> Result<ClientCredsSpotify> {
        let creds = Credentials {
            id: client_config.client_id.clone(),
            secret: Some(client_config.client_secret.clone()),
        };

        let mut config = rspotify::Config::default();
        config.cache_path = ClientConfig::get_config_paths()?.token_cache_path;
        config.token_cached = true;
        config.token_refreshing = true;

        Ok(ClientCredsSpotify::with_config(creds, config))
    }

    fn get_scopes() -> HashSet<String> {
        scopes!(
            "playlist-read-collaborative",
            "playlist-read-private",
            "playlist-modify-private",
            "playlist-modify-public",
            "user-follow-read",
            "user-follow-modify",
            "user-library-modify",
            "user-library-read",
            "user-modify-playback-state",
            "user-read-currently-playing",
            "user-read-playback-state",
            "user-read-playback-position",
            "user-read-private",
            "user-read-recently-played"
        )
    }
}

