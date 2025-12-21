use std::sync::Arc;

use crate::{domain::{Pokemon, PokemonErrors}, ports::Pokedex};

pub struct PokemonService {
    pub pokedex: Arc<dyn Pokedex>,
}

impl PokemonService {
    pub fn new(pokedex: Arc<dyn Pokedex>) -> Self {
        Self { pokedex }
    }

    pub async fn get_pokemon(&self, name: String) -> Result<Pokemon, PokemonErrors> {
        self.pokedex.get_pokemon(name).await
    }
}
