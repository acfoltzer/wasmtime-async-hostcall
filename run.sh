#!/usr/bin/env bash

cargo build
(cd guest && cargo build --target wasm32-wasi)
cargo run
