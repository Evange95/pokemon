use crate::{domain::Pokemon, ports::Pokedex};

pub mod http;

pub struct StaticPokemonApi;

impl Pokedex for StaticPokemonApi {
    fn get_pokemon(&self, _name: String) -> Pokemon {
        Pokemon {
            name: "mewtwo".to_string(),
            description: "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.".to_string(),
            habitat: "rare".to_string(),
            is_legendary: true,
        }
    }
}
