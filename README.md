# mokaccino_c

A C library to use [mokaccino](https://crates.io/crates/mokaccino) from C programs.

## Usage

See examples in `tests/c/`.

Those are the programs that are compiled and ran from the test suite. They are also tested for memory leaks and
do not contain any.

When you use mokaccino, it is important that you use the provided de-allocators for the types of objects you get
from the library, including strings (chars*).

## Building

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