#[derive(Debug)]
pub struct Pokemon {
    pub name: PokemonName,
    pub description: Description,
    pub habitat: Habitat,
    pub is_legendary: IsLegendary,
}

type PokemonName = String;
pub type Description = String;
pub type Habitat = String;
pub type IsLegendary = bool;

#[derive(Debug, Clone, PartialEq)]
pub enum PokemonErrors {
    PokemonNotFound(String),
    PokedexUnavailable,
    InvalidPokemonData(String),
}

/// Domain events for observability
/// These represent business-significant occurrences in the domain
#[derive(Debug, Clone)]
pub enum PokemonEvent {
    FetchRequested {
        name: String,
    },
    FetchSucceeded {
        name: String,
        habitat: String,
        is_legendary: bool,
    },
    FetchFailed {
        name: String,
        reason: PokemonFailureReason,
    },
}

#[derive(Debug, Clone)]
pub enum PokemonFailureReason {
    NotFound,
    ServiceUnavailable,
    DataCorrupted,
}
