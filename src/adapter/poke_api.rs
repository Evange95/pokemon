use crate::{
    domain::{Pokemon, PokemonErrors},
    ports::Pokedex,
};

pub struct PokeApiAdapter {
    client: reqwest::Client,
    base_url: String,
}

impl PokeApiAdapter {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
struct PokeApiResponse {
    name: String,
    habitat: Option<Habitat>,
    is_legendary: bool,
    flavor_text_entries: Vec<FlavorTextEntry>,
}

#[derive(serde::Deserialize, Debug)]
struct Habitat {
    name: String,
}

#[derive(serde::Deserialize, Debug)]
struct FlavorTextEntry {
    flavor_text: String,
    language: Language,
}

#[derive(serde::Deserialize, Debug)]
struct Language {
    name: String,
}

#[async_trait::async_trait]
impl Pokedex for PokeApiAdapter {
    async fn get_pokemon(&self, name: String) -> Result<Pokemon, PokemonErrors> {
        let url = format!("{}/pokemon-species/{}", self.base_url, name.to_lowercase());

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|_| PokemonErrors::PokedexUnavailable)?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let api_response: PokeApiResponse = response
                    .json()
                    .await
                    .map_err(|e| PokemonErrors::InvalidPokemonData(e.to_string()))?;

                // Find first English flavor text
                let description = api_response
                    .flavor_text_entries
                    .iter()
                    .find(|entry| entry.language.name == "en")
                    .map(|entry| entry.flavor_text.replace(['\n', '\u{c}'], " "))
                    .unwrap_or_else(|| "No description available".to_string());

                Ok(Pokemon {
                    name: api_response.name,
                    description,
                    habitat: api_response
                        .habitat
                        .map(|h| h.name)
                        .unwrap_or_else(|| "unknown".to_string()),
                    is_legendary: api_response.is_legendary,
                })
            }
            reqwest::StatusCode::NOT_FOUND => Err(PokemonErrors::PokemonNotFound(name)),
            _ => Err(PokemonErrors::PokedexUnavailable),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    async fn setup_mock_server_with_pokemon(pokemon_name: &str) -> MockServer {
        let mock_server = MockServer::start().await;

        let fixture = match pokemon_name {
            "mewtwo" => include_str!("fixtures/mewtwo.json"),
            "pikachu" => include_str!("fixtures/pikachu.json"),
            _ => panic!("Unknown pokemon fixture: {pokemon_name}"),
        };

        Mock::given(method("GET"))
            .and(path(format!("/pokemon-species/{pokemon_name}")))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&mock_server)
            .await;

        mock_server
    }

    #[tokio::test]
    async fn test_pokeapi_adapter_fetches_mewtwo() {
        let mock_server = setup_mock_server_with_pokemon("mewtwo").await;
        let adapter = PokeApiAdapter::new(mock_server.uri());
        let pokemon = adapter.get_pokemon("mewtwo".to_string()).await.unwrap();

        assert_eq!(pokemon.name, "mewtwo");
        assert_eq!(pokemon.habitat, "rare");
        assert!(pokemon.is_legendary);
        assert!(!pokemon.description.is_empty());
        assert!(pokemon
            .description
            .contains("created by a scientist after years"));
    }

    #[tokio::test]
    async fn test_pokeapi_adapter_fetches_pikachu() {
        let mock_server = setup_mock_server_with_pokemon("pikachu").await;
        let adapter = PokeApiAdapter::new(mock_server.uri());
        let pokemon = adapter.get_pokemon("pikachu".to_string()).await.unwrap();

        assert_eq!(pokemon.name, "pikachu");
        assert_eq!(pokemon.habitat, "forest");
        assert!(!pokemon.is_legendary);
        assert!(!pokemon.description.is_empty());
        assert!(pokemon.description.contains("electricity"));
    }

    #[tokio::test]
    async fn test_pokeapi_adapter_returns_not_found_for_unknown_pokemon() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/pokemon-species/unknown"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let adapter = PokeApiAdapter::new(mock_server.uri());
        let result = adapter.get_pokemon("unknown".to_string()).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            PokemonErrors::PokemonNotFound("unknown".to_string())
        );
    }

    #[tokio::test]
    async fn test_pokeapi_adapter_returns_unavailable_on_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/pokemon-species/mewtwo"))
            .respond_with(ResponseTemplate::new(503))
            .mount(&mock_server)
            .await;

        let adapter = PokeApiAdapter::new(mock_server.uri());
        let result = adapter.get_pokemon("mewtwo".to_string()).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PokemonErrors::PokedexUnavailable);
    }

    #[tokio::test]
    async fn test_pokeapi_adapter_returns_invalid_data_on_parse_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/pokemon-species/malformed"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&mock_server)
            .await;

        let adapter = PokeApiAdapter::new(mock_server.uri());
        let result = adapter.get_pokemon("malformed".to_string()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PokemonErrors::InvalidPokemonData(_) => (),
            _ => panic!("Expected InvalidPokemonData error"),
        }
    }
}
