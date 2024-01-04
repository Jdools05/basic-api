use rspotify::{prelude::*,  Credentials, ClientCredsSpotify, model::{AlbumId, PlaylistId, TrackId, Market, Country}, AuthCodeSpotify, OAuth, scopes, Config};


pub struct SpotifyMain {}

impl SpotifyMain {
    pub fn new() -> Self {
        SpotifyMain {}
    }

    pub async fn run(&self) {
        let creds = Credentials::from_env().unwrap();
        let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();

        let spotify = AuthCodeSpotify::with_config(creds, oauth, Config::default());

        let url = spotify.get_authorize_url(false).unwrap();

        spotify.prompt_for_token(&url).await.unwrap();

        println!("Token: {:#?}", spotify.get_token());

        // Running the requests
        let playlist_id = TrackId::from_id("60PAzFNW3vAiAiVK6DRJfB").unwrap();
        let playlist = spotify.track(playlist_id, Some(Market::Country(Country::UnitedStates))).await;

        println!("Response: {playlist:#?}");
    }
}
