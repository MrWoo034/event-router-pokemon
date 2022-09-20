use std::cell::RefCell;
use candid::Principal;
use pokemon_model::{BOX_SIZE, MAX_BOXES, Pokemon, Storage, StorageBox};

pub type CanisterId = Principal;

pub struct State {
    index: u32,
    boxes: Vec<StorageBox>,
    pub router_id: CanisterId,
}

impl Default for State {
    fn default() -> Self {
        let mut boxes = Vec::with_capacity(MAX_BOXES);
        let empty_box = StorageBox::new_storage();
        boxes.push(empty_box);
        Self {
            index: 0,
            boxes,
            router_id: Principal::from_text(event_model::PRINCIPAL_ID).unwrap(),
        }
    }
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

    pub fn get_next_index(&mut self) -> u32 {
        self.index += 1;
        self.index
    }

    pub fn store_pokemon(&mut self, pokemon: Pokemon) -> bool {
        let max_boxes_allocated: bool = self.boxes.len() == MAX_BOXES;
        if max_boxes_allocated {
            let last_box = self.boxes.last_mut().unwrap();
            if last_box.len() == BOX_SIZE {
                false
            } else {
                last_box.store_pokemon(pokemon);
                true
            }
        } else {
            for storage_box in &mut self.boxes {
                if storage_box.len() == BOX_SIZE {
                    continue;
                } else {
                    storage_box.store_pokemon(pokemon);
                    return true;
                }
            }
            let mut new_box = StorageBox::new_storage();
            new_box.store_pokemon(pokemon);
            self.boxes.push(new_box);
            true
        }
    }

    pub fn retrieve_pokemon(&self, id: u32) -> Option<Pokemon> {
        for storage_box in &self.boxes {
            if storage_box.contains_key(&id) {
                return storage_box.get(&id).map(|p| p.clone())
            }
        }
        None
    }

    pub fn level_up_pokemon(&mut self, id: u32) -> Option<Pokemon> {
        for storage_box in &mut self.boxes {
            if storage_box.contains_key(&id) {
                return storage_box.get_mut(&id).map(|p| {
                    p.level_up();
                    p.clone()
                })
            }
        }
        None
    }
}