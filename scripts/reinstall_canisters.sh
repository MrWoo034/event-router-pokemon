#!/bin/bash

dfx build --all
dfx canister install event_router --mode reinstall
dfx canister install pokemon_box --mode reinstall
