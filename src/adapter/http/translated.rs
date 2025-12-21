use std::sync::Arc;

use axum::{Json, Router, extract::Path, extract::State, routing::get};

use crate::application::PokemonService;
use crate::domain::PokemonErrors;

use super::pokemon::PokemonResponse;

async fn get_translated_pokemon_handler(
    State(pokemon_service): State<Arc<PokemonService>>,
    Path(name): Path<String>,
) -> Result<Json<PokemonResponse>, PokemonErrors> {
    let pokemon = pokemon_service.get_translated_pokemon(name).await?;
    Ok(Json(pokemon.into()))
}

pub fn routes(pokemon_service: Arc<PokemonService>) -> Router {
    Router::new()
        .route(
            "/pokemon/translated/{name}",
            get(get_translated_pokemon_handler),
        )
        .with_state(pokemon_service)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Pokemon, PokemonEvent, TranslationErrors, TranslationStyle};
    use crate::ports::{Logger, Pokedex, Translator};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use std::sync::Arc;
    use tower::util::ServiceExt;

    struct MockLogger;

    impl Logger for MockLogger {
        fn log(&self, _event: &PokemonEvent) {}
    }

    struct StaticPokemonApi {
        habitat: String,
        is_legendary: bool,
    }

    #[async_trait::async_trait]
    impl Pokedex for StaticPokemonApi {
        async fn get_pokemon(&self, _name: String) -> Result<Pokemon, PokemonErrors> {
            Ok(Pokemon {
                name: "mewtwo".to_string(),
                description: "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.".to_string(),
                habitat: self.habitat.clone(),
                is_legendary: self.is_legendary,
            })
        }
    }

    struct YodaTranslator;

    #[async_trait::async_trait]
    impl Translator for YodaTranslator {
        async fn translate(
            &self,
            _description: String,
            style: TranslationStyle,
        ) -> Result<String, TranslationErrors> {
            match style {
                TranslationStyle::Yoda => Ok("Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was.".to_string()),
                TranslationStyle::Shakespeare => Ok("'twas did create by a scientist after years of horrific gene splicing and dna engineering experiments.".to_string()),
            }
        }
    }

    struct FailingTranslator;

    #[async_trait::async_trait]
    impl Translator for FailingTranslator {
        async fn translate(
            &self,
            _description: String,
            _style: TranslationStyle,
        ) -> Result<String, TranslationErrors> {
            Err(TranslationErrors::TranslationRateLimited)
        }
    }

    #[tokio::test]
    async fn test_translated_pokemon_uses_yoda_for_legendary() {
        let api = Arc::new(StaticPokemonApi {
            habitat: "rare".to_string(),
            is_legendary: true,
        });
        let logger = Arc::new(MockLogger);
        let translator = Arc::new(YodaTranslator);
        let service = Arc::new(PokemonService::new(api, logger).with_translator(translator));
        let app = routes(service);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/pokemon/translated/mewtwo")
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
        assert_eq!(
            pokemon.description,
            "Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was."
        );
    }

    #[tokio::test]
    async fn test_translated_pokemon_uses_yoda_for_cave_habitat() {
        let api = Arc::new(StaticPokemonApi {
            habitat: "cave".to_string(),
            is_legendary: false,
        });
        let logger = Arc::new(MockLogger);
        let translator = Arc::new(YodaTranslator);
        let service = Arc::new(PokemonService::new(api, logger).with_translator(translator));
        let app = routes(service);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/pokemon/translated/zubat")
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

        assert_eq!(
            pokemon.description,
            "Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was."
        );
    }

    #[tokio::test]
    async fn test_translated_pokemon_uses_shakespeare_for_normal() {
        let api = Arc::new(StaticPokemonApi {
            habitat: "forest".to_string(),
            is_legendary: false,
        });
        let logger = Arc::new(MockLogger);
        let translator = Arc::new(YodaTranslator);
        let service = Arc::new(PokemonService::new(api, logger).with_translator(translator));
        let app = routes(service);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/pokemon/translated/pikachu")
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

        assert_eq!(
            pokemon.description,
            "'twas did create by a scientist after years of horrific gene splicing and dna engineering experiments."
        );
    }

    #[tokio::test]
    async fn test_translated_pokemon_falls_back_on_translation_error() {
        let api = Arc::new(StaticPokemonApi {
            habitat: "rare".to_string(),
            is_legendary: true,
        });
        let logger = Arc::new(MockLogger);
        let translator = Arc::new(FailingTranslator);
        let service = Arc::new(PokemonService::new(api, logger).with_translator(translator));
        let app = routes(service);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/pokemon/translated/mewtwo")
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

        // Should fall back to original description
        assert_eq!(
            pokemon.description,
            "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments."
        );
    }
}
