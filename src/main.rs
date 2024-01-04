use poem::{listener::TcpListener, Server, Route,};
use poem_openapi::OpenApiService;


mod api_routes;
mod spotify_api;


#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    let spotify = spotify_api::SpotifyMain::new();

    spotify.run().await;


    // let api_service = OpenApiService::new(api_routes::Api, "API", "1.0.0").server("https:/0.0.0.0:3000");
    // let ui = api_service.swagger_ui();
    // let app = Route::new().nest("/", api_service).nest("/docs", ui);

    // let _ = Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await;
}
