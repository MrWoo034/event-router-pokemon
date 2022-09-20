#!/bin/bash

dfx canister call pokemon_box retrieve_pokemon --type idl "(${1})"