# Advent of Code 2023

This repository contains my solutions in Rust to the [Advent of Code 2023](https://adventofcode.com/2023) challenges.

## Usage

To run the solutions, you need to have [Rust](https://www.rust-lang.org/) installed. Then you can run the solutions with:

```bash
cd day1 && cargo run --bin part_1
```

Or to run the tests (sample scenarios):

```bash
cd day1 && cargo test
```

There's a Cargo project created for every day, with a `part_1` and `part_2` binary. The input is read from the `input` file in the project root.

## Development of solutions

When developing the solutions, having the following running is useful to automatically run your tests and if they pass, then run it against the real input after file changes (this requires https://crates.io/crates/cargo-watch):

```bash
cargo watch -x 'test --lib' -x 'run --bin part_1 --release' -x 'run --bin part_2 --release'
```

## Create new day

You can run the following bash script which creates a new cargo project:

```bash
./setup_next_day.sh
```