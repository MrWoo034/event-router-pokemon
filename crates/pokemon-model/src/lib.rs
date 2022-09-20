use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use ic_cdk::export::candid::CandidType;

pub const MAX_BOXES: usize = 10;
pub const BOX_SIZE: usize = 16;

#[derive(Serialize, Deserialize, Debug, Clone, CandidType)]
pub struct Pokemon {
    pub id: u32,
    pokedex_id: u8,
    species: PokemonSpecies,
    evolvable: bool,
    nickname: Option<String>,
    pub level: u8,
}

impl Pokemon {
    pub fn new(species: PokemonSpecies, id: u32, nickname: Option<String>) -> Self {
        let evolvable = match &species {
            PokemonSpecies::Bulbasaur => true,
            PokemonSpecies::Ivysaur => true,
            PokemonSpecies::Charmander => true,
            PokemonSpecies::Charmeleon => true,
            PokemonSpecies::Squirtle => true,
            PokemonSpecies::Wartortle => true,
            PokemonSpecies::Caterpie => true,
            PokemonSpecies::Metapod => true,
            PokemonSpecies::Weedle => true,
            PokemonSpecies::Kakuna => true,
            PokemonSpecies::Pidgey => true,
            PokemonSpecies::Pidgeotto => true,
            PokemonSpecies::Rattata => true,
            _ => false,
        };
        Self {
            id,
            pokedex_id: species.clone() as u8,
            species,
            evolvable,
            nickname,
            level: (ic_cdk::api::time() % 17) as u8,
        }
    }

    pub fn level_up(&mut self) {
        if self.level < 100 {
            self.level += 1;
        }
        if self.evolvable {
            self.evolve();
        }
    }

    fn evolve(&mut self) {
        match self.species {
            PokemonSpecies::Bulbasaur => {
                self.evolve_by_level(16, PokemonSpecies::Ivysaur)
            }
            PokemonSpecies::Ivysaur => {
                self.evolve_by_level(32, PokemonSpecies::Venasaur)
            }
            PokemonSpecies::Charmander => {
                self.evolve_by_level(16, PokemonSpecies::Charmeleon)
            }
            PokemonSpecies::Charmeleon => {
                self.evolve_by_level(36, PokemonSpecies::Charizard)
            }
            PokemonSpecies::Squirtle => {
                self.evolve_by_level(16, PokemonSpecies::Wartortle)
            }
            PokemonSpecies::Wartortle => {
                self.evolve_by_level(32, PokemonSpecies::Blastoise)
            }
            PokemonSpecies::Caterpie => {
                self.evolve_by_level(7, PokemonSpecies::Metapod)
            }
            PokemonSpecies::Metapod => {
                self.evolve_by_level(10, PokemonSpecies::Butterfree)
            }
            PokemonSpecies::Weedle => {
                self.evolve_by_level(7, PokemonSpecies::Kakuna)
            }
            PokemonSpecies::Kakuna => {
                self.evolve_by_level(10, PokemonSpecies::Beedrill)
            }
            PokemonSpecies::Pidgey => {
                self.evolve_by_level(18, PokemonSpecies::Pidgeotto)
            }
            PokemonSpecies::Pidgeotto => {
                self.evolve_by_level(36, PokemonSpecies::Pidgeot)
            }
            PokemonSpecies::Rattata => {
                self.evolve_by_level(20, PokemonSpecies::Raticate)
            }
            _ => { return; }
        }
    }

    fn evolve_by_level(&mut self, evolution_level: u8, evolution_species: PokemonSpecies) {
        if self.level >= evolution_level {
            self.pokedex_id = evolution_species.clone() as u8;
            self.species = evolution_species;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, CandidType)]
pub enum PokemonSpecies {
    Bulbasaur = 1,
    Ivysaur = 2,
    Venasaur = 3,
    Charmander = 4,
    Charmeleon = 5,
    Charizard = 6,
    Squirtle = 7,
    Wartortle = 8,
    Blastoise = 9,
    Caterpie = 10,
    Metapod = 11,
    Butterfree = 12,
    Weedle = 13,
    Kakuna = 14,
    Beedrill = 15,
    Pidgey = 16,
    Pidgeotto = 17,
    Pidgeot = 18,
    Rattata = 19,
    Raticate = 20,
}


pub type StorageBox = HashMap<u32, Pokemon>;

pub trait Storage {
    fn new_storage() -> HashMap<u32, Pokemon> {
        HashMap::with_capacity(BOX_SIZE)
    }

    fn store_pokemon(&mut self, pokemon: Pokemon);
}

impl Storage for StorageBox {
    fn store_pokemon(&mut self, pokemon: Pokemon) {
        self.insert(pokemon.id, pokemon);
    }
}