# mokaccino_c

A C library to use [mokaccino](https://crates.io/crates/mokaccino) from C programs.

## About

Mokaccino is a percolator library, allowing you to match streaming transient events or documents
against a set of fairly static queries identified by your application ID.

Mokkacino C binding is developed at https://github.com/jeteve/mokaccino_c by (Jerome Eteve)[https://github.com/jeteve].

## Usage

See examples in `tests/c/`.

Those are the programs that are compiled and ran from the test suite. They are also tested for memory leaks.

When you use mokaccino, it is important that you use the provided de-allocators for the types of objects you get from the library, _including_ strings (chars*).

You are encourage to look at all the examples, but if you want a quick start,
here's a [complete percolation example here](https://github.com/jeteve/mokaccino_c/blob/main/tests/c/percolator.c).


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