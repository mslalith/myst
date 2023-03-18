use rspotify::{model::{AlbumId}, ClientCredsSpotify, prelude::{BaseClient}};

pub struct SpotifyClient {
    spotify: ClientCredsSpotify,
}

impl SpotifyClient {
    pub fn new(client_creds: ClientCredsSpotify) -> SpotifyClient {
        SpotifyClient {
            spotify: client_creds,
        }
    }

    pub fn tracks(self) {
        // self.spotify.user_playlists(None);
        // let a = AuthCodeSpotify::new(creds, oauth);
        // let url = a.get_authorize_url(false).unwrap();
        // a.me();
    }

    pub async fn test(self) {
        let birdy_uri = AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap();
        let albums = self.spotify.album(birdy_uri).await;
        println!("Response: {albums:#?}");
    }
}
