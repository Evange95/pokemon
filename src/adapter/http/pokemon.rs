use std::sync::Arc;

use axum::{extract::Path, extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{application::PokemonService, domain::Pokemon};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PokemonResponse {
    pub name: String,
    pub description: String,
    pub habitat: String,
    pub is_legendary: bool,
}

impl From<Pokemon> for PokemonResponse {
    fn from(pokemon: Pokemon) -> Self {
        PokemonResponse {
            name: pokemon.name,
            description: pokemon.description,
            habitat: pokemon.habitat,
            is_legendary: pokemon.is_legendary,
        }
    }
}

async fn get_pokemon_handler(
    State(pokedex_service): State<Arc<PokemonService>>,
    Path(name): Path<String>,
) -> Json<PokemonResponse> {
    let resp: Pokemon = pokedex_service.get_pokemon(name);
    Json(resp.into())
}

pub fn routes(pokedex_service: Arc<PokemonService>) -> Router {
    Router::new()
        .route("/pokemon/:name", get(get_pokemon_handler))
        .with_state(pokedex_service)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use std::sync::Arc;
    use tower::util::ServiceExt; // oneshot

    use crate::adapter::StaticPokemonApi;

    #[tokio::test]
    async fn test_get_pokemon_returns_mewtwo() {
        let api = Arc::new(StaticPokemonApi);
        let service = Arc::new(PokemonService::new(api));
        let app = routes(service);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/pokemon/pikachu")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let pokemon: PokemonResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(pokemon.name, "mewtwo");
        assert_eq!(pokemon.description, "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.");
        assert_eq!(pokemon.habitat, "rare");
        assert!(pokemon.is_legendary);
    }

    #[tokio::test]
    async fn test_response_json_format() {
        let api = Arc::new(StaticPokemonApi);
        let service = Arc::new(PokemonService::new(api));
        let app = routes(service);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/pokemon/mew")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json_value: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert!(json_value.get("name").is_some());
        assert!(json_value.get("description").is_some());
        assert!(json_value.get("habitat").is_some());
        assert!(json_value.get("isLegendary").is_some());
    }
}
