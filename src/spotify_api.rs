use rspotify::{prelude::*,  Credentials, ClientCredsSpotify, model::{AlbumId, PlaylistId}};


pub struct SpotifyMain {}

impl SpotifyMain {
    pub fn new() -> Self {
        SpotifyMain {}
    }

    pub async fn run(&self) {
        let creds = Credentials::from_env().unwrap();

        let spotify = ClientCredsSpotify::new(creds);

        // Obtaining the access token. Requires to be mutable because the internal
        // token will be modified. We don't need OAuth for this specific endpoint,
        // so `...` is used instead of `prompt_for_user_token`.
        spotify.request_token().await.unwrap();

        // Running the requests
        let playlist_id = PlaylistId::from_id("1PQyNaugjttLBkjJ0262NG").unwrap();
        let playlist = spotify.playlist(playlist_id, None, None).await;

        println!("Response: {playlist:#?}");
    }
}
