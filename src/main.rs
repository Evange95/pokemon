use axum::{extract::Path, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct PokemonResponse {
    name: String,
    description: String,
    habitat: String,
    is_legendary: bool,
}

async fn get_pokemon(_name: Path<String>) -> Json<PokemonResponse> {
    Json(PokemonResponse {
        name: "mewtwo".to_string(),
        description: "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.".to_string(),
        habitat: "rare".to_string(),
        is_legendary: true,
    })
}

fn app() -> Router {
    Router::new().route("/pokemon/:name", get(get_pokemon))
}

#[tokio::main]
async fn main() {
    let app = app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_pokemon_returns_mewtwo() {
        let app = app();

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
        assert_eq!(pokemon.is_legendary, true);
    }
}
