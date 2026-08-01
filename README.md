# Rust 101

My exercises and notes while learning Rust with [The Rust Programming Language](https://doc.rust-lang.org/book/).

## Projects

- `hello-world/` — a minimal program compiled directly with `rustc`
- `hello_cargo/` — a Hello World program created and run with Cargo
- `guessing_game/` — an interactive game demonstrating loops, parsing, and pattern matching
- `common concepts/` — standalone examples of variables, mutability, constants, scopes, and shadowing

The Cargo projects use Rust edition 2024. See [Rust setup](RUST_SETUP.md) to install Rust and Cargo.

## Run the examples

Standalone examples use `rustc`:

```sh
cd hello-world
rustc main.rs
./main

cd "../common concepts"
rustc variables.rs
./variables
```

Cargo projects use `cargo run`:

```sh
cd hello_cargo
cargo run

cd ../guessing_game
cargo run
```
