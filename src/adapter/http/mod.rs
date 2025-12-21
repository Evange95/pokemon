mod pokemon;
mod translated;

use std::sync::Arc;

use axum::Router;

use crate::application::PokemonService;

pub fn router(pokemon_service: Arc<PokemonService>) -> Router {
    Router::new()
        .merge(pokemon::routes(pokemon_service.clone()))
        .merge(translated::routes(pokemon_service))
}
