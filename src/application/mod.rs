use std::sync::Arc;

use crate::{
    domain::{
        Pokemon, PokemonErrors, PokemonEvent, PokemonFailureReason, TranslationFailureReason,
        TranslationStyle,
    },
    ports::{Logger, Pokedex, Translator},
};

pub struct PokemonService {
    pub pokedex: Arc<dyn Pokedex>,
    pub logger: Arc<dyn Logger>,
    pub translator: Option<Arc<dyn Translator>>,
}

impl PokemonService {
    pub fn new(pokedex: Arc<dyn Pokedex>, logger: Arc<dyn Logger>) -> Self {
        Self {
            pokedex,
            logger,
            translator: None,
        }
    }

    pub fn with_translator(mut self, translator: Arc<dyn Translator>) -> Self {
        self.translator = Some(translator);
        self
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

    pub async fn get_translated_pokemon(&self, name: String) -> Result<Pokemon, PokemonErrors> {
        // First, fetch the pokemon
        let mut pokemon = self.get_pokemon(name.clone()).await?;

        // If no translator is available, return the pokemon as-is
        let translator = match &self.translator {
            Some(t) => t,
            None => return Ok(pokemon),
        };

        // Determine translation style based on habitat and legendary status
        let style = if pokemon.habitat == "cave" || pokemon.is_legendary {
            TranslationStyle::Yoda
        } else {
            TranslationStyle::Shakespeare
        };

        // Log translation request
        self.logger.log(&PokemonEvent::TranslationRequested {
            name: name.clone(),
            style: style.clone(),
        });

        // Attempt translation
        match translator
            .translate(pokemon.description.clone(), style.clone())
            .await
        {
            Ok(translated_description) => {
                pokemon.description = translated_description;
                self.logger.log(&PokemonEvent::TranslationSucceeded {
                    name: name.clone(),
                    style,
                });
                Ok(pokemon)
            }
            Err(err) => {
                // Map translation errors to failure reasons
                let reason = match err {
                    crate::domain::TranslationErrors::TranslationRateLimited => {
                        TranslationFailureReason::RateLimited
                    }
                    crate::domain::TranslationErrors::TranslationUnavailable => {
                        TranslationFailureReason::ServiceUnavailable
                    }
                    crate::domain::TranslationErrors::InvalidTranslationData(_) => {
                        TranslationFailureReason::DataCorrupted
                    }
                };

                // Log failure with fallback indicator
                self.logger.log(&PokemonEvent::TranslationFailed {
                    name: name.clone(),
                    reason,
                    fallback_used: true,
                });

                // Return pokemon with original description (graceful fallback)
                Ok(pokemon)
            }
        }
    }
}
