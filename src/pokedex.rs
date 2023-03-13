use crate::ip::IpDetails;
use crate::pokemon;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Default, Serialize, Deserialize)]
pub struct IpPokedex {
    pokemon: Vec<(IpDetails, MatchingPokemon)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MatchingPokemon {
    Pokemon(pokemon::Pokemon),
    FailedToLoad(pokemon::PokemonId),
    Loading(pokemon::PokemonId),
    Missing,
}

impl IpPokedex {
    pub fn seed_exists(&self, seed: pokemon::PokemonSeed) -> bool {
        self.pokemon.iter().any(|(_, poke)| {
            if let MatchingPokemon::Pokemon(p) = poke {
                p.seed == seed
            } else {
                false
            }
        })
    }

    pub fn catch_pokemon(&mut self, ip: IpDetails, poke: MatchingPokemon) {
        self.pokemon.push((ip, poke))
    }

    pub fn iter(&self) -> Iter<'_, (IpDetails, MatchingPokemon)> {
        self.pokemon.iter()
    }
}
