use crate::domain::{Pokemon, PokemonErrors, PokemonEvent};

#[async_trait::async_trait]
pub trait Pokedex: Send + Sync {
    async fn get_pokemon(&self, name: String) -> Result<Pokemon, PokemonErrors>;
}

/// Logger port for domain event observation
/// Receives domain events and delegates to infrastructure for actual logging
pub trait Logger: Send + Sync {
    fn log(&self, event: &PokemonEvent);
}
