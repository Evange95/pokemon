use crate::domain::Pokemon;

pub trait Pokedex: Send + Sync {
    fn get_pokemon(&self, name: String) -> Pokemon;
}
