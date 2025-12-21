use crate::domain::{Pokemon, PokemonErrors};

#[async_trait::async_trait]
pub trait Pokedex: Send + Sync {
    async fn get_pokemon(&self, name: String) -> Result<Pokemon, PokemonErrors>;
}
