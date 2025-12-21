mod pokemon;

use std::sync::Arc;

use axum::Router;

use crate::application::PokemonService;

pub fn router(pokedex_service: Arc<PokemonService>) -> Router {
    Router::new().merge(pokemon::routes(pokedex_service))
}
