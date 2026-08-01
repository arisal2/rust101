# Rust 101

My exercises and notes while learning Rust with [The Rust Programming Language](https://doc.rust-lang.org/book/) and [Rustlings](https://github.com/rust-lang/rustlings).

## Current progress

- `hello-world/` — a minimal program compiled directly with `rustc`
- `hello_cargo/` — a Hello World program created and run with Cargo
- `guessing_game/` — reads a guess from standard input and prints it back
- `rustlings/` — small exercises for practicing Rust concepts; currently on `intro2`

The Cargo projects use Rust edition 2024. Generated `target/` directories and other temporary Rust files are excluded through the repository's `.gitignore`.

## Running the examples

Compile and run the standalone example:

```sh
cd hello-world
rustc main.rs
./main
```

Run a Cargo project:

```sh
cd hello_cargo
cargo run
```

Replace `hello_cargo` with `guessing_game` to run that project instead.

## Practicing with Rustlings

Start the interactive Rustlings exercise runner:

```sh
cd rustlings
rustlings
```

Rustlings checks each exercise as it is edited and advances after it passes. Exercise files are under `rustlings/exercises/`.
