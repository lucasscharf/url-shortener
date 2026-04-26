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
mode="Batch" #options Batch and Interactive
batch_file_path="./operations.csv"
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

During the process of writing the capabillity to store/retrieve from database, I've found myself in an interesting position.

Should I create variants of my "objects" that operate also in database having a database_counter?
Should I create a stored type in the structs so each variant of my shorteners would be able to deal with the database?
Should I make the database be something transparent to the shortener only initializing it before hand and persist data after each operation?
Also, I'm starting to have the feeling that the code is having some duplications. And any decision I made on this, I will have to repeat for both Batch and Interactive mode.

So, I will do what any experient dev would do in this kind of situation. I will make me a sandwitch.

The life is way better after a sandwitch. After some tought, since this is a pet project to learn Rust, I've decided to try using closures to implement the database approach. If it works well, then I will extend to other approaches.

## Dependencies

- [`inquire`](https://crates.io/crates/inquire) — interactive CLI prompts.
- [`sha2`](https://crates.io/crates/sha2) — SHA-256 implementation for the hash strategy.
- [`hex`](https://crates.io/crates/hex) — hex encoding of the digest.
- [`config-file`](https://crates.io/crates/config-file) — load `config.toml` into a typed struct at startup.
- [`serde`](https://crates.io/crates/serde) — derive-based deserialization used by the config and CSV records.
- [`csv`](https://crates.io/crates/csv) — read batch operations from a CSV file.
- [`strum`](https://crates.io/crates/strum) / [`strum_macros`](https://crates.io/crates/strum_macros) — derives for iterating and displaying the `Operations` enum used in the menu.

## Next steps

* **Create a simple shortener without validations and using counter as keys**
* **Create a variant using hash for keys**
* **Add a simple CLI menu**
* **Create configurations to allow different strategies**
* **Do batch operations with files**
* **Add capability to store and retrieve from SQLite**
* Add continuous backups with threads into a SQLite
* Add capability to remove a key
* Add capability to do operations over REST
* Use rusqlite_serde
* Clean up everything removing tons of ifs and make the code more rust-like
* Create a Next.js screen
* Create a distributed variant using a consensus algorithm like Paxos or Raft
* Create panick handling
* Create a complete Readme.md