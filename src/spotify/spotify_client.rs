use rspotify::model::PrivateUser;
use rspotify::{prelude::BaseClient, AuthCodeSpotify};

#[derive(Debug, Clone)]
pub struct SpotifyClient {
    spotify: AuthCodeSpotify,
    me: PrivateUser,
}

impl SpotifyClient {
    pub fn new(spotify: AuthCodeSpotify, me: PrivateUser) -> SpotifyClient {
        SpotifyClient { spotify, me }
    }

    pub fn tracks(self) {
        self.spotify.user_playlists(self.me.id);
    }
}
