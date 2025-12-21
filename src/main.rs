mod adapter;
mod application;
mod config;
mod domain;
mod ports;

use adapter::http;
use config::Config;
use std::sync::Arc;

use crate::adapter::StaticPokemonApi;
use crate::application::PokemonService;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let config = Config::from_env().expect("Failed to load configuration");
    let address = config.address();

    let pokedex = Arc::new(StaticPokemonApi);
    let pokemon_service = Arc::new(PokemonService::new(pokedex));
    let app = http::router(pokemon_service);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to {address}"));

    println!("Server listening on http://{address}");
    axum::serve(listener, app).await.unwrap();
}
