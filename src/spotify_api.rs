use rspotify::{prelude::*,  Credentials, AuthCodeSpotify, OAuth, scopes, Config};


pub struct SpotifyMain {}

impl SpotifyMain {
    pub fn new() -> Self {
        SpotifyMain {}
    }

    pub async fn get_auth_url(&self) {
        let creds = Credentials::from_env().unwrap();
        let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();

        let spotify = AuthCodeSpotify::with_config(creds, oauth, Config::default());

        let url = spotify.get_authorize_url(false).unwrap();

        let code = spotify.get_code_from_user(&url);
        println!("URL: {}", url);
        println!("Code: {:#?}", code);
    }

    pub async fn get_token(&self, code: String) {
        let creds = Credentials::from_env().unwrap();
        let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();

        let spotify = AuthCodeSpotify::with_config(creds, oauth, Config::default());

        let token = spotify.request_token(&code).await.unwrap();

        println!("Token: {:?}", token);
    }
}
