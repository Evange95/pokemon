use crate::domain::{
    Description, Pokemon, PokemonErrors, PokemonEvent, TranslationErrors, TranslationStyle,
};

#[async_trait::async_trait]
pub trait Pokedex: Send + Sync {
    async fn get_pokemon(&self, name: String) -> Result<Pokemon, PokemonErrors>;
}

/// Translator port for translating text to different styles
#[async_trait::async_trait]
pub trait Translator: Send + Sync {
    async fn translate(
        &self,
        description: Description,
        style: TranslationStyle,
    ) -> Result<Description, TranslationErrors>;
}

/// Logger port for domain event observation
/// Receives domain events and delegates to infrastructure for actual logging
pub trait Logger: Send + Sync {
    fn log(&self, event: &PokemonEvent);
}
