use crate::spotify::spotify_client::SpotifyClient;

#[derive(Debug, Clone)]
pub struct HomeState {
    spotify_client: SpotifyClient,
}

impl HomeState {
    pub fn new(spotify_client: SpotifyClient) -> Self {
        HomeState { spotify_client }
    }

    pub fn get_playlists() {}
}
