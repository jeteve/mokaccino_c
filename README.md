# mokaccino_c

A C library to use [mokaccino](https://crates.io/crates/mokaccino) from C programs.

## Usage

See examples in `tests/c/`.

Those are the programs that are compiled and ran from the test suite.

## Build

```sh
cargo build
# or
cargo build --release
```

You'll then find the binaries (both .so and .a) and the header in `target/include` and in `target/[debug|release]/libmokaccino.*`

## Test

```sh
# Test debug builds:
cargo build && cargo test -- --nocapture

# Test release builds:
cargo build --release && cargo test --release  -- --nocapture
```


> ⚠️ Note `cargo test` will NOT work without the build.