# Rust Exercisms

## Useful Commands
```sh
❯ cargo test
❯ cargo test -- --ignored

❯ exercism submit src/lib.rs Cargo.toml

# override with the nightly for current directory:
❯ rustup override set nightly

# verify the override:
❯ grep '\[overrides\]' -A 10 ~/.rustup/settings.toml
# OR: after `brew install python-yq`
❯ tomlq .overrides ~/.rustup/settings.toml
{
  "/Users/zeph/Exercism/rust/sublist": "nightly-aarch64-apple-darwin"
}

# for sublist (and others with benchmarks)
❯ cargo bench
```

## Notes

* `sublist` uses the benchmarking technique
pointed out by [letsgetrusty](https://www.youtube.com/watch?v=eIB3Pd5LBkc&ab_channel=Let%27sGetRusty). See:
  * [Cargo.toml](./sublist/Cargo.toml)
  * [sublist_benchmark.rs](./sublist/benches/sublist_benchmark.rs)
  * exercism [discussion](https://exercism.org/tracks/rust/exercises/sublist/mentor_discussions/556eedc894c94a40abd06f7fae95ac3d)
  * running in VSCode with nightly toolchain and [these changes](https://github.com/tzaffi/rusticisms/commit/b7e18883ea288351a11ce9644dc74666bfa614f6)