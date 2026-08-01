# Rust 101

My exercises and notes while learning Rust with [The Rust Programming Language](https://doc.rust-lang.org/book/).

## Current progress

- `hello-world/` — a minimal program compiled directly with `rustc`
- `hello_cargo/` — a Hello World program created and run with Cargo
- `guessing_game/` — an interactive number-guessing game
  - Generates a random secret number from 1 to 100
  - Uses a `loop` to accept guesses until the player wins
  - Uses `match` to handle parsing errors
  - Uses `match` with `Ordering` to report whether a guess is too small, too big, or correct

The Cargo projects use Rust edition 2024. Generated `target/` directories and temporary Rust files are excluded through the repository's `.gitignore`.

## Running the examples

Compile and run the standalone example:

```sh
cd hello-world
rustc main.rs
./main
```

Run the Cargo Hello World project:

```sh
cd hello_cargo
cargo run
```

Run the guessing game from the repository root:

```sh
cd guessing_game
cargo run
```
