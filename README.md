# URL Shortener

A simple URL shortener written in Rust as a learning project. It explore different approaches, different libs and usages. The focus is less on the business logic and more on understand the Rust language.

## How to run

Requirements: [Rust toolchain](https://rustup.rs/) (edition 2024, stable).

```bash
cargo run
```

## How to test

```bash
cargo test
```

## Configuration

The application reads a `config.toml` file in the project root at startup. The following possibilities for configurations are:

```toml
shortenen_algorithm="counter" #options hash and counter
mode="File" #options File and Interactive
file_path="./operations.csv"
```

## Architecture

The shortening logic is exposed through a single trait:

```rust
pub trait UrlShortener {
    fn shorten(&mut self, url: &str) -> bool;
    fn get(&mut self, key: &str) -> Option<&String>;
    fn list_values(&mut self) -> Vec<&String>;
    fn list_keys(&mut self) -> Vec<&String>;
}
```

## Dependencies

- [`inquire`](https://crates.io/crates/inquire) — interactive CLI prompts.
- [`sha2`](https://crates.io/crates/sha2) — SHA-256 implementation for the hash strategy.
- [`hex`](https://crates.io/crates/hex) — hex encoding of the digest.

## Next steps

* **Create a simple shortener without validations and using counter as keys**
* **Create a variant using hash for keys**
* **Add a simple CLI menu**
* **Create configurations to allow different strategies**
* **Do batch operations with files**
* Add capability to store and retrieve from SQLite
* Add continuous backups with threads into a SQLite
* Add capability to remove a key
* Add capability to do operations over REST
* Create a Next.js screen
* Create a distributed variant using a consensus algorithm like Paxos or Raft
* Create panick handling
* Create a complete Readme.md