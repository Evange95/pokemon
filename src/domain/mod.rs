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
