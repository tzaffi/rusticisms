# Rust Exercisms

## Useful Commands
```sh
❯ cargo test
❯ cargo test -- --ignore

❯ exercism submit src/lib.rs Cargo.toml

# for sublist (and others with benchmarks)
❯ cargo bench
```

## Notes

* `sublist` uses the benchmarking technique
pointed out by [letsgetrusty](https://www.youtube.com/watch?v=eIB3Pd5LBkc&ab_channel=Let%27sGetRusty). See:
  * [Cargo.toml](./sublist/Cargo.toml)
  * [sublist_benchmark.rs](./sublist/benches/sublist_benchmark.rs)
  * exercism [discussion](https://exercism.org/tracks/rust/exercises/sublist/mentor_discussions/556eedc894c94a40abd06f7fae95ac3d)