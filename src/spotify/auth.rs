use std::collections::{HashSet, HashMap};

use anyhow::{Ok, Result};
use dioxus_desktop::wry::webview::Url;
use rspotify::prelude::OAuthClient;
use rspotify::{ClientCredsSpotify, Credentials, scopes, AuthCodeSpotify, OAuth};

use crate::spotify::redirect_uri::redirect_uri_web_server;

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

    pub async fn authenticate() -> Result<SpotifyClient> {
        let client_config = ClientConfig::load_config()?;
        let spotify = SpotifyAuth::get_client_creds(&client_config)?;
        spotify.request_token().await?;
        Ok(SpotifyClient::new(spotify))
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

        let spotify = AuthCodeSpotify::new(creds, oauth);

        let url = spotify.get_authorize_url(true)?;
        println!("auth url: {}", url);
        let redirect_url = redirect_uri_web_server(&spotify, client_config.port)?;
        println!("redirect url: {}", redirect_url);
        if let Some(code) = spotify.parse_response_code(&redirect_url) {
            println!("CODE ================================================== {}", code);
            spotify.request_token(&code).await?;
            let me = spotify.me().await?;
            println!("me: {me:?}");
        }

        self.spotify = Some(spotify);
        Ok(url)
    }

    pub async fn continue_oauth(self, redirect_url: String) -> Result<()> {
        let spotify = self.spotify.unwrap();
        if let Some(code) = spotify.parse_response_code(&redirect_url) {
            println!("CODE ================================================== {}", code);
            spotify.request_token(&code).await?;
            let me = spotify.me().await?;
            println!("me: {me:?}");
        }

        Ok(())
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

fn parse_response_code(url: &str, expected_state: String) -> Option<String> {
    let url = Url::parse(url).ok()?;
    let params = url.query_pairs().collect::<HashMap<_, _>>();
    eprintln!("params: {params:?}");

    let code = params.get("code")?;

    let state = params.get("state")?.as_ref();
    if state != expected_state {
        eprintln!("Request state doesn't match the callback state");
        return None;
    }

    Some(code.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let code = "AQCV1WWdE7lKbafbDuiFwbUhHhVIJn9VBbCu4jIBPWfUYvKgLVdZqWI12aCAFHRr1hRJQnMNOouoQkHCIywh1vY47Du5jMda99q7nsfObCPQG0FNlyweirXfLlxeWmOOlJS63Ggr88aCAwQGN0X8WK12Y40eJGx6h1DG5adRMnruHQUN3vTgGZU4qSgx1oJ9yaOA1f3pJHso9JaqQ4n3_wJus5DmN4BsGv6jip1ieKBc6dfXt2XGzZkKGg71m2B5jL3x7TiDbvShHoL0BbW6jaUum2UyAuSNp32wrnHuJnwuzZgA6vWGsrXAv9d8PyPVhVlgiDtayWvIBo07lbXmcdkJQvrqZrA8BGd6MneBRwJl2cEFiWZazCh4YsCn7YgTcGURuOrO-A-TiCWQyHoC7GZL8Em_USR58LyfkAivj8KKHKOYriyyAalP0vrHonN791xwmX1d1opmnRaspLQhAqWfhFlkNIw54ueuLuVH5Dqbujt5En61zeZlUFpXeWmXRSeVQW1YsrQdV3bY_H4yqmGOs1FzUizNGCHXdU1vuOqUcAqv9r3OVsD6cLDnEguGwyxE9Po4ozzVNhROHLOf2KfbG3cghVmTxg8xvB8tgbNr4HI".to_string();
        let state = "qe6UMma77J4rPhDD".to_string();
        let url = format!("127.0.0.1:8888/callback?code={}&state={}", code, state);
        let code1 = parse_response_code(url.as_str(), state).unwrap();
        eprintln!("CODE: {}", code1);
        assert_eq!(code, code1);
    }
}
