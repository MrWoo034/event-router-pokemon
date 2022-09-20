mod state;

use serde::{Deserialize, Serialize};
use candid::{encode_one};
use ic_cdk_macros::{query, update};
use ic_cdk::export::candid::CandidType;
use event_model::{Event};
use pokemon_model::{Pokemon, PokemonSpecies};
use crate::state::State;

#[derive(Deserialize, Serialize, Debug, CandidType)]
pub struct StoreRequest {
    species: PokemonSpecies,
    nickname: Option<String>,
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


#[update]
async fn store_pokemon(request: StoreRequest) -> Pokemon {
    let pokemon = State::mutate_state(|state| {
        let pokemon = Pokemon::new(request.species, state.get_next_index(), request.nickname);
        state.store_pokemon(pokemon.clone());
        pokemon
    });

    register_event(pokemon.clone()).await;
    pokemon
}

#[query]
fn retrieve_pokemon(id: u32) -> Option<Pokemon> {
    State::read_state(|state| {
        state.retrieve_pokemon(id)
    })
}

#[update]
async fn level_up(pokemon: Pokemon) -> Option<Pokemon> {
    let pokemon = State::mutate_state(|state| {
        state.level_up_pokemon(pokemon.id)
    });

    match &pokemon {
        Some(p) => {
            register_event(p.clone()).await;
        },
        None => { },
    };

    pokemon

}

async fn register_event(pokemon: Pokemon) {
    let now = ic_cdk::api::time();

    let event: Event<Pokemon> = Event {
        consumer_address: ic_cdk::id(),
        consumer_method: "level_up".to_string(),
        consume_at_timestamp: Some(now + 60_000_000_000),
        payload: pokemon,
        source_timestamp: now,
    };

    let arg = encode_one(event);

    let canister_call = State::read_state(|state| {
        ic_cdk::api::call::call_raw(state.router_id, "register_event", &arg.unwrap(), 0)
    });

    let result = canister_call.await;
    ic_cdk::println!("{:?}", result);
}