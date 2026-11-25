# rust-lab

Rust projects, ownership first. Start here: `cli-todo`.

## Projects

| # | Project | What it teaches |
|---|---------|-----------------|
| 1 | `cli-todo` — clap args, serde_json save/load | ownership, structs, enums, Result, cargo |
| 2 | `file-db` — append/search/delete records on disk | error handling, io, testing |
| 3 | `tiny-http` — axum JSON API | async, traits, lifetimes |
| 4 | `grep-clone` — ripgrep-ish search | iterators, benchmarks |

Ship them in order. Do not start track 2 before project 1 is pushed.

## How to run

    cd cli-todo
    cargo run -- add "learn ownership"
    cargo test

## Toolchain (this machine)

    cargo 1.100.0
    rustc 1.100.0

Installed via rustup. Crates used: `clap`, `serde`, `serde_json`, `axum`,
`tokio`. Pin versions in Cargo.toml so builds stay reproducible.

## Rules

- `cargo build` must be clean, no warnings you ignored.
- `cargo clippy` clean before you push.
- Every project has its own README with "how to run".
