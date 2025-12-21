use std::sync::Arc;

use crate::{
    domain::{Pokemon, PokemonErrors, PokemonEvent, PokemonFailureReason},
    ports::{Logger, Pokedex},
};

pub struct PokemonService {
    pub pokedex: Arc<dyn Pokedex>,
    pub logger: Arc<dyn Logger>,
}

impl PokemonService {
    pub fn new(pokedex: Arc<dyn Pokedex>, logger: Arc<dyn Logger>) -> Self {
        Self { pokedex, logger }
    }

    pub async fn get_pokemon(&self, name: String) -> Result<Pokemon, PokemonErrors> {
        self.logger
            .log(&PokemonEvent::FetchRequested { name: name.clone() });

        let result = self.pokedex.get_pokemon(name.clone()).await;

        match &result {
            Ok(pokemon) => {
                self.logger.log(&PokemonEvent::FetchSucceeded {
                    name: pokemon.name.clone(),
                    habitat: pokemon.habitat.clone(),
                    is_legendary: pokemon.is_legendary,
                });
            }
            Err(err) => {
                let reason = match err {
                    PokemonErrors::PokemonNotFound(_) => PokemonFailureReason::NotFound,
                    PokemonErrors::PokedexUnavailable => PokemonFailureReason::ServiceUnavailable,
                    PokemonErrors::InvalidPokemonData(_) => PokemonFailureReason::DataCorrupted,
                };
                self.logger.log(&PokemonEvent::FetchFailed {
                    name: name.clone(),
                    reason,
                });
            }
        }

        result
    }
}
