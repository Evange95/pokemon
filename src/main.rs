mod adapter;
mod api;
mod config;

use adapter::http;
use api::StaticPokemonApi;
use config::Config;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Load .env file if present (optional, falls back to system env vars)
    let _ = dotenvy::dotenv();

    let config = Config::from_env().expect("Failed to load configuration");
    let address = config.address();

    // Driving adapter (HTTP) using driving port (PokemonApi)
    let api = Arc::new(StaticPokemonApi);
    let app = http::router(api);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to {address}"));

    println!("Server listening on http://{address}");
    axum::serve(listener, app).await.unwrap();
}
