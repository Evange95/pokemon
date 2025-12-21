use crate::{domain::Pokemon, ports::Pokedex};

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
    async fn get_pokemon(&self, name: String) -> Pokemon {
        let url = format!("{}/pokemon-species/{}", self.base_url, name.to_lowercase());

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .expect("Failed to fetch pokemon data");

        let api_response: PokeApiResponse =
            response.json().await.expect("Failed to parse pokemon data");

        // Find first English flavor text
        let description = api_response
            .flavor_text_entries
            .iter()
            .find(|entry| entry.language.name == "en")
            .map(|entry| entry.flavor_text.replace('\n', " ").replace('\u{c}', " "))
            .unwrap_or_else(|| "No description available".to_string());

        Pokemon {
            name: api_response.name,
            description,
            habitat: api_response
                .habitat
                .map(|h| h.name)
                .unwrap_or_else(|| "unknown".to_string()),
            is_legendary: api_response.is_legendary,
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
            _ => panic!("Unknown pokemon fixture: {}", pokemon_name),
        };

        Mock::given(method("GET"))
            .and(path(format!("/pokemon-species/{}", pokemon_name)))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&mock_server)
            .await;

        mock_server
    }

    #[tokio::test]
    async fn test_pokeapi_adapter_fetches_mewtwo() {
        let mock_server = setup_mock_server_with_pokemon("mewtwo").await;
        let adapter = PokeApiAdapter::new(mock_server.uri());
        let pokemon = adapter.get_pokemon("mewtwo".to_string()).await;

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
        let pokemon = adapter.get_pokemon("pikachu".to_string()).await;

        assert_eq!(pokemon.name, "pikachu");
        assert_eq!(pokemon.habitat, "forest");
        assert!(!pokemon.is_legendary);
        assert!(!pokemon.description.is_empty());
        assert!(pokemon.description.contains("electricity"));
    }
}
