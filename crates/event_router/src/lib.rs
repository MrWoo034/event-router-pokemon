mod state;

use candid::encode_one;
use event_model::Event;
use ic_cdk_macros::{query, update, heartbeat};
use pokemon_model::Pokemon;
use crate::state::State;

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
fn register_event(event: Event<Pokemon>) -> String {
    State::mutate_state(|state| {
        state.store_event(event.clone())
    });
    format!("Stored event: {:?}", event)
}

#[heartbeat]
async fn check_events() {
    let events = State::mutate_state(|state| {
        state.check_events()
    });

    ic_cdk::println!("These events need to resolve! {:?}", events);
    for event in events {
        resolve_event(event).await;
    }
}

async fn resolve_event(event: Event<Pokemon>) {
    let arg = encode_one(event.payload);

    let canister_call = State::read_state(|state| {
        ic_cdk::api::call::call_raw(event.consumer_address, &event.consumer_method, &arg.unwrap(), 0)
    });

    let result = canister_call.await;
    ic_cdk::println!("{:?}", result);
}


