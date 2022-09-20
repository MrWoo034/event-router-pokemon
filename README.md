# Event Router - Pokemon Box

This project consists of two canisters, `event-router` and `pokemon-box`.  

The `pokemon_box` takes a `Pokemon` and stores it in its `State`.  It also registers the `Pokemon` as an `Event<Pokemon>`
with the `event-router`.  

By default, `Pokemon` will level up every one minute.  The `event-router` will check for new events that need to resolve
using its `#[heartbeat]` function.  Those events will be passed back to the `pokemon_box` canisters `level_up` endpoint.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --clean --background

# Deploys the pokemon_box and event_router canisters.
./scripts deploy_canisters.sh 

# Stores a new Bulbasaur in the pokemon_box.
# You can use the returned ID from this function with
# ./sctipts retrieve_pokemon.sh ${ID}
./scripts store_pokemon.sh
```

## Troubleshooting

The `event-router` canister id is currently a hardcoded constant address.  You may have to update this address to match 
the address assigned when the `event-router` built on your local environment.