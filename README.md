# Martian Robots

I kept things as simple as possible with a system that does what we need and does it well. This is backed by a significant test suite.

## Requirements

- Rust toolchain which you can install from [rustup](https://rustup.rs).
- No third-party dependencies.

## Build, run and test

From the project's root directory:

```bash
cargo build --release                      # build
cargo run --release -- path/to/input.txt   # run on an input file
cargo test                                 # run the tests
```

## Assumptions and decisions

**Robots and scents**

- Robots do not co-exist. Each robot runs to completion before the next one starts, and a surviving robot never blocks a later one.
- A scent belongs to a coordinate, not to a direction. If a robot is lost going north from `(5, 3)`, a later robot at `(5, 3)` is also protected when going east.

**Input handling**

- Strict input format.
- A start position outside the grid is an error.
- Processing stops at the first invalid record.
- Letters are upper case only.

## Design

A library crate does the work, and a thin binary reads the file and calls it.

| File | Responsibility |
|---|---|
| `src/main.rs` | Reads the file named on the command line and calls the library. |
| `src/mars.rs` | `Mars` holds the grid bounds and the set of scents. `parse_and_execute` runs a whole input, and `execute` runs one robot. Also defines `Instruction`. |
| `src/parser.rs` | Turns lines of text into typed values: world size, start position, instructions. It checks syntax and limits, but knows nothing about the grid. |
| `src/spatial.rs` | `Orientation`, `Coordinate`, `Position` and `Grid`. These are pure and contain no robot or scent state. |
| `src/robot.rs` | `Robot`: its position and whether it is lost, and how it is printed. |
| `src/error.rs` | `MarsError`, one error type for invalid input and output failures. |

### Adding a new command

1. Add a variant to `Instruction` in `src/mars.rs`.
2. Map its letter in `parse_instructions` in `src/parser.rs`.
3. Handle it in the `match` in `Mars::execute`.

With the `enum Instruction` in `src/mars.rs`, we'll have the compiler guide us when adding new instructions.

## Tests

- **`spatial.rs`:** turning and movement for all four orientations, coordinate arithmetic including going below zero, and the grid bounds (corners inclusive, moves off every edge).
- **`robot.rs`:** the robot state and how it is printed.
- **`parser.rs`:** valid input, extra whitespace, and each kind of invalid input for the world size, start position and instructions.
- **`mars.rs`:**
  - The sample from the brief, end to end through the text input.
  - Every error the program can report for invalid input.
  - The limits: 99 vs 100 instructions, and world sizes of 50 vs 51.
  - Output already written is kept when a later record is invalid.
  - Scent behaviour: a move at a scented point is ignored while later instructions still run, a scent is per coordinate, it does not protect neighbours, and one robot can be protected by two scents.
  - Loss at every edge, a lost robot ignoring its remaining instructions, a `0 0` world, and non-square worlds.

## What I would do with more time

- Integration tests that run the compiled binary on fixture files and check stdout, stderr and the exit status.
- Better command-line interface (`clap`) for usage and `--help`. Having an interactive session that allows user input through stdin.
- Better errors that implement `Display` and `std::error::Error`, include the line number and the offending text, instead of just printing the name of the error.
- Streaming input line by line rather than loading the whole file.
- Choices on invalid input (i.e. skipping blank lines, or reporting a bad record and continuing with the next one).
- Make the input format more flexible, allow multiple formats, allow blank lines, ...
- Collisions between robots, if the problem needed robots to share the world. The structure supports it: `Mars` already owns the world state, and robots run one at a time.
- Property-based tests (i.e. four turns in the same direction return to the start or a robot is never outside the grid).
