//! # AOC Helper documentation
//!
//! AOC Helper is a crate to make solving and sharing aoc problems in rust
//! easier. It is inspired by [cargo-aoc](https://github.com/gobanos/cargo-aoc).
//! `cargo-aoc` is a binary that will also take care of compilation, while this
//! crate is a library you can import and use in your code. This aims to make it
//! easier to share and debug other people's code, and to perhaps be easier to
//! set up.
//!
//! `aoc-helper` is a very simple crate right now, and all the functionality is
//! in the `Helper` struct.
//!
//! ## Usage
//!
//! To get started, add `aoc-helper` to the `dependencies` section in your
//! `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! cargo-aoc = "0.1.2"
//! ```
//!
//! You also need to provide a session ID for `aoc-helper` to be able to
//! download your input files. The session ID is stored in a cookie called
//! `session` in your browser on the [aoc website](https://adventofcode.com) if
//! you're logged in. You can provide the session ID either through an
//! environment variable with the name `AOC_SESSION_ID` or through the
//! `session_id` function on `Helper`.

use std::fmt::Display;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::Read;
use std::env;

use chrono::prelude::*;
use failure::{Error, format_err};

/// The `Helper` struct stores all necessary information for an aoc day.
///
/// One instance stores the information for one aoc day. You can provide an
/// optional serializer function to serialize the input data into a custom type.
/// The serializer, and the solver functions if you're not using a custom
/// serializer function, take `&str`s as input.
pub struct Helper<T, D: Display> {
    year: i32,
    day: u32,
    session_id: Option<String>,
    input_path: String,
    part1_examples: Vec<String>,
    part2_examples: Vec<String>,
    serializer: fn(&str) -> T,
    part1: Option<fn(T) -> D>,
    part2: Option<fn(T) -> D>,
}

impl<D: Display> Helper<String, D> {
    /// Creates a new `Helper` instance for the provided year and day.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Helper;
    ///
    /// // Create a new `Helper` instance for day 12 of aoc 2015
    /// let helper = Helper::new(2015, 12);
    /// ~~~~
    pub fn new(year: i32, day: u32) -> Self {
        Helper {
            year,
            day,
            session_id: env::var("AOC_SESSION_ID").ok(),
            input_path: format!("inputs/{}/day{}.txt", year, day),
            part1_examples: Vec::new(),
            part2_examples: Vec::new(),
            serializer: str::to_string,
            part1: None,
            part2: None,
        }
    }

    /// Creates a new `Helper` instance for the provided year and day, with a
    /// custom input file.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Helper;
    ///
    /// // Create a new `Helper` instance for day 13 of aoc 2016 with a custom input file at `in.txt`
    /// let helper = Helper::new_with_input_file(2016, 13, "in.txt");
    /// ~~~~
    pub fn new_with_input_file(year: i32, day: u32, input_path: &str) -> Self {
        Helper {
            year,
            day,
            session_id: env::var("AOC_SESSION_ID").ok(),
            input_path: input_path.to_string(),
            part1_examples: Vec::new(),
            part2_examples: Vec::new(),
            serializer: str::to_string,
            part1: None,
            part2: None,
        }
    }
}

impl<T: Clone, D: Display> Helper<T, D> {
    /// Creates a new `Helper` instance for the provided year and day, with a
    /// custom serializer function.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Helper;
    ///
    /// let helper = Helper::new_with_input_file(2017, 2, |input| input.split_whitespace().collect::<Vec<_>>());
    /// ~~~~
    pub fn new_with_serializer(year: i32, day: u32, serializer: fn(&str) -> T) -> Self {
        Helper {
            year,
            day,
            session_id: env::var("AOC_SESSION_ID").ok(),
            input_path: format!("inputs/{}/day{}.txt", year, day),
            part1_examples: Vec::new(),
            part2_examples: Vec::new(),
            serializer,
            part1: None,
            part2: None,
        }
    }

    /// Creates a new `Helper` instance for the provided year and day, with a
    /// custom serializer function and input file.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Helper;
    ///
    /// let helper = Helper::new_with_serializer_and_input_file(2017, 2, |input| input.lines().collect::<Vec<_>>(), "in.txt");
    /// ~~~~
    pub fn new_with_serializer_and_input_file(year: i32, day: u32, serializer: fn(&str) -> T, input_path: &str) -> Self {
        Helper {
            year,
            day,
            session_id: env::var("AOC_SESSION_ID").ok(),
            input_path: input_path.to_string(),
            part1_examples: Vec::new(),
            part2_examples: Vec::new(),
            serializer,
            part1: None,
            part2: None,
        }
    }

    /// Set the session ID.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Helper;
    ///
    /// let helper = Helper::new(2015, 8);
    /// helper.session_id("82942d3671962a9f17f8f81723d45b0fcdf8b3b5bf6w3954f02101fa5de1420b6ecd30ed550133f32d6a5c00233076af");
    /// ~~~~
    pub fn session_id(&mut self, session_id: &str) {
        self.session_id = Some(session_id.to_string());
    }

    /// Add example inputs for the part 1 problem that will be run when you call
    /// `test`.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Helper;
    ///
    /// let helper = Helper::new(2018, 4);
    /// helper.part1_examples(&["part 1", "example", "cases"]);
    /// ~~~~
    pub fn part1_examples<S: ToString>(&mut self, examples: &[S]) {
        self.part1_examples = examples.iter().map(|x| x.to_string()).collect();
    }

    /// Add example inputs for the part 2 problem that will be run when you call
    /// `test`.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Helper;
    ///
    /// let helper = Helper::new(2018, 4);
    /// helper.part2_examples(&["part 2", "example", "cases"]);
    /// ~~~~
    pub fn part2_examples<S: ToString>(&mut self, examples: &[S]) {
        self.part2_examples = examples.iter().map(|x| x.to_string()).collect();
    }

    /// Provide the solver function for the part 1 problem. The return type
    /// should implement [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html).
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Helper;
    ///
    /// let helper = Helper::new(2018, 7);
    /// helper.part1(|x| x.chars().filter(|&y| y == 'z').count());
    /// ~~~~
    pub fn part1(&mut self, solver: fn(T) -> D) {
        self.part1 = Some(solver);
    }

    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Helper;
    ///
    /// let helper = Helper::new(2018, 7);
    /// helper.part2(|x| x.chars().filter(|&y| y != 'z').count());
    /// ~~~~
    /// Provide the solver function for the part 2 problem. The return type
    /// should implement [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html).
    pub fn part2(&mut self, solver: fn(T) -> D) {
        self.part2 = Some(solver);
    }

    /// Run the solver functions on the example inputs provided using
    /// `part1_examples` and `part2_examples`.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Helper;
    ///
    /// let helper = Helper::new(2018, 7);
    /// helper.part1_examples(vec!["test case"]);
    /// helper.part1(|x| x.chars.filter(|&y| y == 'z').count());
    /// helper.test().unwrap();
    /// ~~~~
    pub fn test(&self) -> Result<(), Error> {
        println!("AOC {}, day {}", self.year, self.day);
        for (i, example) in self.part1_examples.iter().enumerate() {
            if let Some(part1) = self.part1 {
                println!("Part 1, Example {}: {}", i + 1, part1((self.serializer)(&example)));
            } else {
                return Err(format_err!("Error: No solver function given for part 1"));
            }
        }
        for (i, example) in self.part2_examples.iter().enumerate() {
            if let Some(part2) = self.part2 {
                println!("Part 2, Example {}: {}", i + 1, part2((self.serializer)(&example)));
            } else {
                return Err(format_err!("Error: No solver function given for part 2"));
            }
        }
        Ok(())
    }

    /// Run the solver functions on the input for the day and print the outputs.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Helper;
    ///
    /// let helper = Helper::new(2019, 5);
    /// helper.part2(|x| x.lines().filter(|&y| y.contains("foo")).count());
    /// helper.run().unwrap();
    /// ~~~~
    pub fn run(&self) -> Result<(), Error> {
        if self.session_id == None {
            return Err(format_err!("Error: No session ID specified"));
        }
        let running_date = NaiveDate::from_ymd(self.year, 12, self.day);
        // Due to timezone differences, this will be a little more lenient that the aoc website in accepting dates
        let today = Utc::today();
        let max_year = if today.month() < 12 { today.year() - 1 } else { today.year() };
        if running_date > NaiveDate::from_ymd(max_year, 12, 25) {
            return Err(format_err!("Error: The specified puzzle date is in the future"));
        } else if running_date < NaiveDate::from_ymd(2015, 12, 1) || self.day > 25 {
            return Err(format_err!("Error: There was no puzzle on the specified date"));
        }
        let mut input_file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.input_path) {
            Ok(file) => file,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    create_dir_all(&self.input_path.split('/').take(2).collect::<Vec<_>>().join("/"))?;
                    OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(&self.input_path)?
                } else {
                    return Err(format_err!("{}", e.to_string()));
                }
            },
        };
        let mut contents = String::new();
        input_file.read_to_string(&mut contents)?;
        if contents.len() == 0 {
            // Get the input from the website
            let response = ureq::get(&format!("https://adventofcode.com/{}/day/{}/input", self.year, self.day))
                .set("Cookie", &(String::from("session=") + self.session_id.as_ref().unwrap())).call();
            std::io::copy(&mut response.into_reader(), &mut input_file)?;
            let mut input_file = File::open(&self.input_path)?;
            input_file.read_to_string(&mut contents)?;
        }
        let input = (self.serializer)(contents.trim());
        let mut executed = false;
        if let Some(part1) = self.part1 {
            println!("Part 1: {}", (part1)(input.clone()));
            executed = true;
        }
        if let Some(part2) = self.part2 {
            println!("Part 2: {}", (part2)(input));
            executed = true;
        }
        if !executed {
            return Err(format_err!("Error: no solver functions specified"));
        }
        Ok(())
    }
}
