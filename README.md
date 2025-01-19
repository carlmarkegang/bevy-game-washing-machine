# bevy-game-washing-machine
A prototype game to test the functionality of Bevy and Rust

## Commands
 * Setup new folder: cd bevy_game_creature + cargo init + cargo add bevy + cargo add rand
 * For exe: cargo run
 * For web: cargo run --target wasm32-unknown-unknown 
 * Remove dependencies and lock file: cargo clean
 * cargo build --release
 * cargo build --release --target wasm32-unknown-unknown
 * cargo fix  - Auto clean project

## install web
 * rustup target install wasm32-unknown-unknown
 * cargo install wasm-server-runner
 * cargo run --target wasm32-unknown-unknown

## Auto build on update
cargo watch -x run

## Improve compile during development using dynamic linking
cargo run --features bevy/dynamic_linking