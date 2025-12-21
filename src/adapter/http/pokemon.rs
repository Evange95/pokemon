use std::sync::Arc;

use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    application::PokemonService,
    domain::{Pokemon, PokemonErrors},
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PokemonResponse {
    pub name: String,
    pub description: String,
    pub habitat: String,
    pub is_legendary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for PokemonErrors {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            PokemonErrors::PokemonNotFound(name) => (
                StatusCode::NOT_FOUND,
                format!("Pokemon '{}' not found", name),
            ),
            PokemonErrors::PokedexUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Pokedex service is currently unavailable".to_string(),
            ),
            PokemonErrors::InvalidPokemonData(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Invalid pokemon data: {}", msg),
            ),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
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
) -> Result<Json<PokemonResponse>, PokemonErrors> {
    let pokemon = pokedex_service.get_pokemon(name).await?;
    Ok(Json(pokemon.into()))
}

pub fn routes(pokedex_service: Arc<PokemonService>) -> Router {
    Router::new()
        .route("/pokemon/:name", get(get_pokemon_handler))
        .with_state(pokedex_service)
}

#[cfg(test)]
mod tests {
    use crate::ports::{Logger, Pokedex};

    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use std::sync::Arc;
    use tower::util::ServiceExt; // oneshot

    pub struct MockLogger;

    impl Logger for MockLogger {
        fn log(&self, _event: &crate::domain::PokemonEvent) {}
    }

    pub struct StaticPokemonApi;

    #[async_trait::async_trait]
    impl Pokedex for StaticPokemonApi {
        async fn get_pokemon(&self, _name: String) -> Result<Pokemon, PokemonErrors> {
            Ok(Pokemon {
                name: "mewtwo".to_string(),
                description: "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.".to_string(),
                habitat: "rare".to_string(),
                is_legendary: true,
            })
        }
    }

    #[tokio::test]
    async fn test_get_pokemon_returns_mewtwo() {
        let api = Arc::new(StaticPokemonApi);
        let logger = Arc::new(MockLogger);
        let service = Arc::new(PokemonService::new(api, logger));
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
        let logger = Arc::new(MockLogger);
        let service = Arc::new(PokemonService::new(api, logger));
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

    pub struct ErrorPokemonApi;

    #[async_trait::async_trait]
    impl Pokedex for ErrorPokemonApi {
        async fn get_pokemon(&self, name: String) -> Result<Pokemon, PokemonErrors> {
            Err(PokemonErrors::PokemonNotFound(name))
        }
    }

    #[tokio::test]
    async fn test_get_pokemon_returns_404_when_not_found() {
        let api = Arc::new(ErrorPokemonApi);
        let logger = Arc::new(MockLogger);
        let service = Arc::new(PokemonService::new(api, logger));
        let app = routes(service);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/pokemon/unknown")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let error: ErrorResponse = serde_json::from_slice(&body).unwrap();

        assert!(error.error.contains("unknown"));
        assert!(error.error.contains("not found"));
    }
}
