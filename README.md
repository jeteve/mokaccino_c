# mokaccino_c

A C library based on (mokaccino)[https://crates.io/crates/mokaccino]

This will expose the same interface as the (Python binding)[https://github.com/jeteve/mokaccino_py]

## Build

```sh
cargo build
```

## Test

```sh
# Test debug builds:
cargo build && cargo test -- --nocapture

# Test release builds:
cargo build --release && cargo test --release  -- --nocapture
```


> ⚠️ Note `cargo test` will NOT work without the build.