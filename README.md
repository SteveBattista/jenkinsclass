# fortune

A small Rust CLI that prints a random developer-themed fortune.

## What It Does

- Formats fortune output with a recipient name.
- Supports `--name` to personalize output.
- Supports `--seed` for deterministic random selection.
- Supports `--list` to print all fortunes.
- Loads fortunes from `data/fortunes.txt` instead of hardcoding them in Rust source.

## CLI Usage

Build and run:

```bash
cargo run -- --name Avery
```

List all fortunes:

```bash
cargo run -- --list
```

Deterministic output with a seed:

```bash
cargo run -- --name Taylor --seed 7
```

Show help:

```bash
cargo run -- --help
```

### Arguments

- `--name <NAME>`: Recipient shown in the formatted message. Defaults to `you`.
- `--seed <N>`: Optional `u64` seed for reproducible fortune selection.
- `--list`: Print every fortune and exit.

## Output Format

The formatter function (`print_fortune`) assembles and returns a string. It does not print directly.

`main` is responsible for printing the returned string.

## Examples

Run with a custom name:

```bash
cargo run -- --name Avery
```

Example output:

```text
🥠 Fortune Cookie 🥠
This fortune is for Avery.

"May the force be with your code."

✨ May your fortune come true! ✨
```

Run with deterministic seed:

```bash
cargo run -- --name Taylor --seed 7
```

Example output:

```text
🥠 Fortune Cookie 🥠
This fortune is for Taylor.

"The best way to predict the future is to invent it."

✨ May your fortune come true! ✨
```

List all fortunes:

```bash
cargo run -- --list
```

Example output (first lines):

```text
Why do programmers prefer dark mode? Because light attracts bugs!
Why did the programmer quit his job? Because he didn't get arrays.
How many programmers does it take to change a light bulb? None, that's a hardware problem.
...
```

## Project Structure

```text
Cargo.toml
README.md
data/
  fortunes.txt
src/
  lib.rs
  main.rs
tests/
  arg_parsing_tests.rs
  cli_output_tests.rs
  print_fortune_tests.rs
```

## Design Notes

### Pure argument parsing

`src/lib.rs` includes a pure helper:

- `parse_name_from_args<I, S>(args: I) -> String`

This allows parser behavior to be unit tested independently from process environment.

A thin wrapper remains for runtime CLI integration:

- `get_name_from_args() -> String`

### Randomness

- Default random mode uses `rand::rng()`.
- Seeded mode uses `StdRng::seed_from_u64(seed)` for reproducibility.

### Fortune data source

`FORTUNES` is loaded from `data/fortunes.txt` using `include_str!` and a lazy static (`LazyLock`).

## Quality Checks

Run tests:

```bash
cargo test
```

Run strict lints:

```bash
cargo clippy --all-targets -- -W clippy::pedantic
```

Build all release targets:

```bash
cargo build --release --all-targets
```

## Tests Included

- `tests/print_fortune_tests.rs`
  - Validates formatting behavior for names, empty text, and UTF-8 content.
- `tests/arg_parsing_tests.rs`
  - Covers no args, missing `--name` value, empty name, repeated `--name`, and unknown flags.
- `tests/cli_output_tests.rs`
  - Validates binary output structure and `--list` behavior.

## Requirements

- Rust toolchain with Cargo (edition `2024`).

Install Rust (if needed):

```bash
curl https://sh.rustup.rs -sSf | sh
```
