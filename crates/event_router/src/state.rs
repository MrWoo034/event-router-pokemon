use std::cell::RefCell;
use candid::Principal;
use event_model::Event;
use pokemon_model::Pokemon;
use std::collections::LinkedList;

pub type CanisterId = Principal;

#[derive(Default)]
pub struct State {
    poke_events: LinkedList<Event<Pokemon>>,
}

impl State {
    thread_local! {
        pub static STATE: RefCell<State> = RefCell::default();
    }

    pub fn read_state<F: FnOnce(&Self) -> R, R>(f: F) -> R {
        State::STATE.with(|s| f(&s.borrow()))
    }

    pub fn mutate_state<F: FnOnce(&mut Self) -> R, R>(f: F) -> R {
        State::STATE.with(|s| f(&mut s.borrow_mut()))
    }

    pub fn store_event(&mut self, event: Event<Pokemon>) {
        self.poke_events.push_back(event);
    }

    pub fn check_events(&mut self,) -> Vec<Event<Pokemon>> {
        let mut result = Vec::new();
        let now = ic_cdk::api::time();
        while !self.poke_events.is_empty() {
            let current = self.poke_events.pop_front().unwrap();
            if current.consume_at_timestamp.is_some() && current.consume_at_timestamp.unwrap() <= now {
                result.push(current);
            } else {
                self.poke_events.push_front(current);
                break
            }
        }
        result
    }
}