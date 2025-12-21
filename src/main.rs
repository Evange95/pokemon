mod adapter;
mod application;
mod config;
mod domain;
mod ports;

use adapter::http;
use adapter::tracing_logger::TracingLogger;
use config::Config;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::{
    adapter::fun_translations::FunTranslationsAdapter, adapter::poke_api::PokeApiAdapter,
    application::PokemonService,
};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = Config::from_env().expect("Failed to load configuration");
    let address = config.address();
    let poke_api_base_url = config.poke_api_base_url();
    let fun_translations_base_url = config.fun_translations_base_url();

    tracing::info!("Starting Pokemon API server");
    tracing::info!(
        "Configuration loaded: host={} port={}",
        config.host,
        config.port
    );

    let logger = Arc::new(TracingLogger::new());
    let pokedex = Arc::new(PokeApiAdapter::new(poke_api_base_url.to_string()));
    let translator = Arc::new(FunTranslationsAdapter::new(
        fun_translations_base_url.to_string(),
    ));
    let pokemon_service =
        Arc::new(PokemonService::new(pokedex, logger).with_translator(translator));
    let app = http::router(pokemon_service).layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to {address}"));

    tracing::info!("Server listening on http://{}", address);
    axum::serve(listener, app).await.unwrap();
}
