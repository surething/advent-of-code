# 🎄 Advent of Code

<!-- trunk-ignore-all(markdownlint/MD033) -->

<center>
    <a href="https://adventofcode.com/">
        <img src="assets/aoc.png" alt="Advent of Code Logo" width="400"/>
    </a>
</center>

> Multi-year Rust solutions, reusable tooling, and reproducible inputs for every Advent of Code season.

[![Rust 2021](https://img.shields.io/badge/Rust-2024%20Edition-b7410e?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-cargo%20test-blue?logo=github)](#running-solutions)
[![Keep Coding Cozy](https://img.shields.io/badge/Advent%20of%20Code-%F0%9F%8E%84-green)](https://adventofcode.com/)

## Table of Contents

- [Overview](#overview)
- [Highlights](#highlights)
- [Repository Layout](#repository-layout)
- [Getting Started](#getting-started)
- [Running Solutions](#running-solutions)
- [Managing Puzzle Inputs](#managing-puzzle-inputs)
- [Development Workflow](#development-workflow)
- [Acknowledgements](#acknowledgements)

## Overview

This workspace collects my Advent of Code journey from 2015 onward. Each year
gets its own crate, every day ships with unit tests and canonical inputs, and
shared crates keep the ergonomics consistent. The goal: fast iteration,
confident refactors, and a single command away from reliving any seasonal
puzzle.

### Quick Stats

- ✅ Years scaffolded: **2015 → 2025** (that is 11 seasons and counting)
- 🧩 Daily coverage: **25 days × 2 parts** per year when complete for 2015–2024
- 🦀 Implementation language: **100% Rust** (2024 edition)
- 🧪 Testing: `rstest`-powered examples + golden outputs for both parts
- 📦 Resources: inputs live in-version under `crates/aoc-data/resources`

## Highlights

- **Composable architecture** – every solver implements the shared `Task` trait,
  making orchestration and future automation straightforward.
- **Deterministic I/O** – `aoc-data` bundles puzzle inputs so nothing relies on
  ad-hoc downloads.
- **Quality-first workflow** – day modules ship with regression tests plus
  fixture-based sample cases straight from the Advent of Code website.
- **Workspace aware** – multiple crates (`aoc-common`, `aoc-cli`, yearly crates)
  build together, so `cargo test --workspace` keeps everything healthy.

## Repository Layout

| Path                 | What lives here                                                                                                          |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `crates/aoc-20XX/`   | Year-specific solver crates (`day1.rs` … `day25.rs`) implementing `Task` + `ResourceReader`.                             |
| `crates/aoc-common/` | Shared domain types, error handling, and the `Event`, `Day`, `Task`, and `Input` enums/traits re-exported via `prelude`. |
| `crates/aoc-data/`   | Resource loader plus versioned puzzle inputs under `resources/<year>/dayXX`.                                             |
| `crates/aoc-cli/`    | Command-line entry point (currently a scaffold) for eventually running puzzles outside tests.                            |
| `Justfile`           | Handy one-liners (`just build`, `just test`, `just clean`) to standardize local workflows.                               |

> Tip: the repository follows a consistent naming convention, so jumping between
> years in your editor is as simple as switching the `aoc-20XX` crate.

## Getting Started

### Prerequisites

- Rust toolchain (`rustup` with at least Rust 1.75+ works great)
- `cargo` (bundled with Rust) for building and testing
- [`z3`][z3] installed and available working on
  constraint-solving days (e.g., 2024 Day 24)
- Optional: [`just`][just] for the convenience recipes
  defined in the `Justfile`

[z3]: https://github.com/Z3Prover/z3
[just]: https://github.com/casey/just

### Clone & build

```bash
git clone https://github.com/surething/advent-of-code.git
cd advent-of-code
cargo build --workspace
# or, if you have just installed:
just build
```

## Running Solutions

Every solver exposes `solve_part1` and `solve_part2` via the shared `Task` trait:

```rust
impl Task for Solver {
    fn event(&self) -> Event { Event::Event2015 }
    fn day(&self) -> Day { Day::Day1 }
    fn solve_part1(&self, input: &str) -> Result<String> { /* … */ }
    fn solve_part2(&self, input: &str) -> Result<String> { /* … */ }
}
```

Because each day comes with thorough tests, the easiest way to rerun solutions
is via `cargo test`:

```bash
# Run the entire 2015 set (all days, both parts)
cargo test -p aoc-2015

# Focus on a single day's regression tests
cargo test -p aoc-2015 day1::test -- --nocapture

# Run the canonical answer assertions for part 1 only
cargo test -p aoc-2015 day1::test::part1 -- --nocapture

# Exercise everything across the workspace
cargo test --workspace
```

Prefer raw binaries later on? The `aoc-cli` crate is the staging ground for a
dedicated runner—feel free to extend it to parse year/day flags and print
answers in the terminal.

## Managing Puzzle Inputs

The `aoc-data` crate keeps every puzzle input under version control. Files
follow this layout:

```plain
crates/aoc-data/resources/
  └── 2015/
      └── day1/
          ├── example1.txt
          ├── example2.txt
          ├── part1.txt
          └── part2.txt
```

Use the provided `ResourceReader` helper inside your solvers to load whichever
file you need:

```rust
let input = solver.read_resource(Input::Part1)?;
```

Keeping inputs committed makes historical runs reproducible and removes the need
for runtime downloads.

## Development Workflow

- `just test` – run `cargo test --workspace` with a friendly banner
- `just build` – build every crate to ensure nothing regressed
- `just clean` – wipe `target/` artifacts when you need a fresh slate

When editing or adding solutions, lean on `rstest` fixtures for clarity and keep
both parts validated before moving to the next day.

## Acknowledgements

- 🎁 [Advent of Code][aoc] by Eric Wastl for the annual puzzle tradition
- 🦀 The Rust community for the libraries and inspiration that keep these solutions sharp

Happy hacking, and may all your puzzle inputs run in within defined behavior!

[aoc]: https://adventofcode.com/
