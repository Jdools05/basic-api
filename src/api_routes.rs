use std::fs;

use poem::{http::StatusCode, Error};
use poem_openapi::{
    auth::Basic,
    payload::{Html, PlainText},
    OpenApi, SecurityScheme,
};

use crate::spotify_api;

pub struct Api;

#[derive(SecurityScheme)]
#[oai(ty = "basic")]
struct ApiSecurity(Basic);

#[OpenApi]
impl Api {
    #[oai(path = "/api/v1/health", method = "get")]
    async fn health(&self, auth: ApiSecurity) -> Result<Html<String>, Error> {
        if auth.0.username != "admin" || auth.0.password != "admin" {
            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
        }

        let index_html: Result<String, std::io::Error> =
            fs::read_to_string("src/front_end/index.html");
        match index_html {
            Ok(html) => Ok(Html(html)),
            Err(_err) => Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR)),
        }
    }

    #[oai(path = "/api/v1/open", method = "get")]
    // this will just show the open.html file
    async fn open(&self) -> Result<Html<String>, Error> {
        let open_html: Result<String, std::io::Error> =
            fs::read_to_string("src/front_end/open.html");
        match open_html {
            Ok(html) => Ok(Html(html)),
            Err(_err) => Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR)),
        }
    }

    #[oai(path = "/api/v1/callback", method = "get")]
    async fn callback(&self) -> Result<Html<String>, Error> {
        println!("callback");
        let callback_html: Result<String, std::io::Error> =
            fs::read_to_string("src/front_end/callback.html");
        match callback_html {
            Ok(html) => Ok(Html(html)),
            Err(_err) => Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR)),
        }
    }

    #[oai(path = "/api/v1/spotify", method = "get")]
    async fn spotify(&self) -> Result<PlainText<String>, Error> {
        let spotify = spotify_api::SpotifyMain::new();

        spotify.get_auth_url().await;
        Ok(PlainText("Hello, world!".to_string()))
    }

    #[oai(path = "/api/v1/spotify/token", method = "get")]
    async fn spotify_token(&self) -> Result<PlainText<String>, Error> {
        let spotify = spotify_api::SpotifyMain::new();

        spotify.get_token("".to_string()).await;
        Ok(PlainText("Hello, world!".to_string()))
    }
}
