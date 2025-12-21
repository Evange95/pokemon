mod adapter;
mod api;
mod config;

use adapter::http;
use api::StaticPokemonApi;
use config::Config;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let config = Config::from_env().expect("Failed to load configuration");
    let address = config.address();

    let pokemon_service = Arc::new(StaticPokemonApi);
    let app = http::router(pokemon_service);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to {address}"));

    println!("Server listening on http://{address}");
    axum::serve(listener, app).await.unwrap();
}
