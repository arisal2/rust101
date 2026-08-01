# Rust setup

Install Rust using [`rustup`](https://rustup.rs/). It includes the Rust compiler (`rustc`) and Cargo:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Windows, download and run `rustup-init.exe` from the [`rustup` website](https://rustup.rs/). Restart the terminal after installation.

Verify the installation:

```sh
rustc --version
cargo --version
```

Useful Cargo commands:

```sh
cargo build  # Download dependencies and compile
cargo run    # Compile and run
cargo check  # Check code without producing a binary
cargo test   # Run tests
```

Update Rust with:

```sh
rustup update
```
