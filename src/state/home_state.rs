use crate::spotify::spotify_client::SpotifyClient;

#[derive(Debug)]
pub struct HomeState {
    spotify_client: SpotifyClient,
}

impl HomeState {
    pub fn get_playlists() {}
}
