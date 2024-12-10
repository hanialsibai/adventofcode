<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2024

Rust fun

## Commands

- Download: `cargo download 1`
- Solve: `cargo solve <day>` ; add `--release` for the release version; add `--dhat` for memory profiling
- Run test `cargo test --bin <day>`; remove bin for all tests to run
- Run all solutions `cargo all`
- Benchmark `cargo time <day> [--all] [--store]`; store flag writes to README
- Read puzzle: `cargo read <day>`
- All in one: `cargo today`

<!--- advent_readme_stars table --->
## 2024 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2024/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2024/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2024/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2024/day/4) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `101.1µs` | `106.8µs` |
| [Day 2](./src/bin/02.rs) | `120.1µs` | `279.2µs` |
| [Day 3](./src/bin/03.rs) | `53.0ms` | `23.9ms` |
| [Day 4](./src/bin/04.rs) | `2.4ms` | `1.3ms` |

**Total: 81.21ms**
<!--- benchmarking table --->

Template from [advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust)
