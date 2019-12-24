# AOC Helper documentation

![rustc 1.40.0](https://img.shields.io/badge/rustc-1.40.0-blue?style=flat-square)
[![crates.io 0.2.0](https://img.shields.io/crates/v/aoc-helper?style=flat-square)](https://crates.io/crates/aoc-helper)
![license](https://img.shields.io/crates/l/aoc-helper?style=flat-square)

AOC Helper is a crate to make solving and sharing aoc problems in rust
easier. It is inspired by [cargo-aoc](https://github.com/gobanos/cargo-aoc).
`cargo-aoc` is a binary that will also take care of compilation, while this
crate is a library you can import and use in your code. This aims to make it
easier to share and debug other people's code, and to perhaps be easier to
set up.

For more examples, check out the examples directory in the repository.

## Usage

To get started, add `aoc-helper` to the `dependencies` section in your
`Cargo.toml`:

```toml
[dependencies]
aoc-helper = "0.2.1"
```

You also need to provide a session ID for `aoc-helper` to be able to
download your input files. The session ID is stored in a cookie called
`session` in your browser on the [aoc website](https://adventofcode.com) if
you're logged in. You can provide the session ID through an
environment variable with the name `AOC_SESSION_ID`, through the
`session_id` functions on `Helper`, or by using an `aoc_helper.toml` file.

If you're using an `aoc_helper.toml` file, you need to specify the `config-file` feature and
specify your session ID in `aoc_helper.toml` like this:

```toml
session-id = "82942d3671962a9f17f8f81723d45b0fcdf8b3b5bf6w3954f02101fa5de1420b6ecd30ed550133f32d6a5c00233076af"
```

Then, create an instance of `AocDay`. Look at its documentation for
information.

## Example

```rust
use aoc_helper::{AocDay, Puzzle};

fn main() {
    // Create a new `AocDay` instance for day 1 of 2015
    // Note: this is not the actual problem from day 1 of 2015
    let mut day_1 = AocDay::new(2015, 1);

    // Create a new `Puzzle` instance for part 2
    let part_2 = Puzzle::new(
        2,
        |x: String| x.lines().filter(|&y| y.contains("foo")).count()
        )
        .with_examples(&["random\nstuff\nfoo\nbaz", "foo\nbar\ntest\ncases"]);

    // Run the solver functions on the example cases
    day_1.test(&part_2);
    // Run the solver functions on the day's input
    day_1.run(&part_2).unwrap();
}
```
