# AOC Helper documentation

AOC Helper is a crate to make solving and sharing aoc problems in rust
easier. It is inspired by [cargo-aoc](https://github.com/gobanos/cargo-aoc).
`cargo-aoc` is a binary that will also take care of compilation, while this
crate is a library you can import and use in your code. This aims to make it
easier to share and debug other people's code, and to perhaps be easier to
set up.

`aoc-helper` is a very simple crate right now, and all the functionality is
in the `Helper` struct.

## Usage

To get started, add `aoc-helper` to the `dependencies` section in your
`Cargo.toml`:

```toml
[dependencies]
cargo-aoc = "0.1.0"
```

You also need to provide a session ID for `aoc-helper` to be able to
download your input files. The session ID is stored in a cookie called
`session` in your browser on the [aoc website](https://adventofcode.com) if
you're logged in. You can provide the session ID either through an
environment variable with the name `AOC_SESSION_ID` or through the
`session_id` function on `Helper`.
The `Helper` struct stores all necessary information for an aoc day.

One instance stores the information for one aoc day. You can provide an
optional serializer function to serialize the input data into a custom type.
The serializer, and the solver functions if you're not using a custom
serializer function, take `&str`s as input.

## Example

```rust
use aoc_helper::Helper;

// Create a new `Helper` instance for day 1 of 2015
// Note: this is not the actual problem from day 1 of 2015
let helper = Helper::new(2015, 1);
// Add some example cases for part 2
helper.part2_examples(vec!["random\nstuff\nfoo\nbaz", "foo\nbar\ntest\cases"]);
// Add a solver function for part 2
helper.part2(|x| x.lines().filter(|&y| y.contains("foo")).count());

// Run the solver functions on the example cases
helper.test().unwrap();
// Run the solver functions on the day's input
helper.run().unwrap();
```
