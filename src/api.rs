use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PokemonResponse {
    pub name: String,
    pub description: String,
    pub habitat: String,
    pub is_legendary: bool,
}

// Driving Port: application-facing API used by driving adapters (HTTP, CLI, etc.)
pub trait Pokedex: Send + Sync {
    fn get_pokemon(&self, name: String) -> PokemonResponse;
}

// Simple adapter that always returns Mewtwo
pub struct StaticPokemonApi;

impl Pokedex for StaticPokemonApi {
    fn get_pokemon(&self, _name: String) -> PokemonResponse {
        PokemonResponse {
            name: "mewtwo".to_string(),
            description: "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.".to_string(),
            habitat: "rare".to_string(),
            is_legendary: true,
        }
    }
}
