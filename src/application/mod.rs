use std::sync::Arc;

use crate::{domain::Pokemon, ports::Pokedex};

pub struct PokemonService {
    pub pokedex: Arc<dyn Pokedex>,
}

impl PokemonService {
    pub fn new(pokedex: Arc<dyn Pokedex>) -> Self {
        Self { pokedex }
    }

    pub fn get_pokemon(&self, name: String) -> Pokemon {
        self.pokedex.get_pokemon(name)
    }
}
