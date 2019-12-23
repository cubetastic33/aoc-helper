//! # AOC Helper documentation
//!
//! AOC Helper is a crate to make solving and sharing aoc problems in rust
//! easier. It is inspired by [cargo-aoc](https://github.com/gobanos/cargo-aoc).
//! `cargo-aoc` is a binary that will also take care of compilation, while this
//! crate is a library you can import and use in your code. This aims to make it
//! easier to share and debug other people's code, and to perhaps be easier to
//! set up.
//!
//! ## Usage
//!
//! To get started, add `aoc-helper` to the `dependencies` section in your
//! `Cargo.toml`:
//!
//! ~~~~
//! [dependencies]
//! aoc-helper = "0.2.0"
//! ~~~~
//!
//! You also need to provide a session ID for `aoc-helper` to be able to
//! download your input files. The session ID is stored in a cookie called
//! `session` in your browser on the [aoc website](https://adventofcode.com) if
//! you're logged in. You can provide the session ID through an
//! environment variable with the name `AOC_SESSION_ID`, through the
//! `session_id` functions on `Helper`, or by using an `aoc_helper.toml` file.
//!
//! If you're using an `aoc_helper.toml` file, you need to specify the `config-file` feature and
//! specify your session ID in `aoc_helper.toml` like this:
//!
//! ~~~~
//! session-id = "82942d3671962a9f17f8f81723d45b0fcdf8b3b5bf6w3954f02101fa5de1420b6ecd30ed550133f32d6a5c00233076af"
//! ~~~~
//!
//! Then, create an instance of [`AocDay`](./struct.AocDay.html). Look at its documentation for
//! information.

use std::fmt::Display;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Read, Write};
use std::env;
use std::error::Error;

use time::{Date, Instant};
use colored::*;
#[cfg(feature = "config-file")]
use toml::Value;

#[derive(Debug, Copy, Clone)]
pub enum AocError {
    MissingSessionId,
    SpecifiedDateInFuture,
    NoPuzzleOnDate,
}

impl Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            AocError::MissingSessionId => "No session ID specified",
            AocError::SpecifiedDateInFuture => "The specified puzzle date is in the future",
            AocError::NoPuzzleOnDate => "There was no puzzle on the specified date",
        };
        write!(f, "Error: {}", msg)
    }
}

impl Error for AocError {}

fn aoc_err(err: AocError) -> Result<(), Box<dyn Error>> {
    Err(Box::new(err))
}

/// The `AocDay` struct stores information for an aoc day.
///
/// You can provide an optional serializer function to serialize the input data
/// into a custom type. The serializer, and the solver functions if you're not
/// using a custom serializer function, take `String`s as input.
pub struct AocDay<T> {
    year: i32,
    day: u8,
    session_id: Option<String>,
    input_path: String,
    serializer: fn(String) -> T,
}

/// The `Puzzle` struct stores information for an aoc puzzle. Two puzzles
/// release each aoc day.
pub struct Puzzle<T, D> {
    part: u8,
    examples: Vec<String>,
    solver: fn(T) -> D,
}

impl AocDay<String> {
    /// Create a new `AocDay` instance for the provided year and day.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::AocDay;
    ///
    /// // Create a new AocDay instance for day 12 of aoc 2015
    /// let day_12 = AocDay::new(2015, 12);
    /// ~~~~
    pub fn new(year: i32, day: u8) -> Self {
        AocDay {
            year,
            day,
            session_id: env::var("AOC_SESSION_ID").ok(),
            input_path: format!("inputs/{}/day{}.txt", year, day),
            serializer: |x| x,
        }
    }
}

impl<T> AocDay<T> {
    /// Create a new `AocDay` instance for the provided year and day, with a
    /// custom serializer function.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::AocDay;
    ///
    /// let day_2 = AocDay::new_with_serializer(2017, 2, |input| input.split_whitespace().collect::<Vec<_>>());
    /// ~~~~
    pub fn new_with_serializer(year: i32, day: u8, serializer: fn(String) -> T) -> Self {
        AocDay {
            year,
            day,
            session_id: env::var("AOC_SESSION_ID").ok(),
            input_path: format!("inputs/{}/day{}.txt", year, day),
            serializer,
        }
    }

    /// Provide a custom input file
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::AocDay;
    ///
    /// let mut day_2 = AocDay::new(2017, 2);
    /// day_2.input("path/to/my/input/file.txt");
    /// ~~~~
    pub fn input(&mut self, input_path: &str) {
        self.input_path = input_path.to_string();
    }

    /// Chainable version of [`AocDay::input()`](./struct.AocDay.html#method.input)
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::AocDay;
    ///
    /// let day_2 = AocDay::new(2017, 2)
    ///     .with_input("path/to/my/input/file.txt");
    /// ~~~~
    pub fn with_input(mut self, input_path: &str) -> Self {
        self.input(input_path);
        self
    }

    /// Provide the session ID
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::AocDay;
    ///
    /// let mut day_8 = AocDay::new(2015, 8);
    /// day_8.session_id("82942d3671962a9f17f8f81723d45b0fcdf8b3b5bf6w3954f02101fa5de1420b6ecd30ed550133f32d6a5c00233076af");
    /// ~~~~
    pub fn session_id(&mut self, session_id: &str) {
        self.session_id = Some(session_id.to_string());
    }

    /// Chainable version of [`AocDay::session_id()`](./struct.AocDay.html#method.session_id)
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::AocDay;
    ///
    /// let date_8 = AocDay::new(2015, 8)
    ///     .with_session_id("82942d3671962a9f17f8f81723d45b0fcdf8b3b5bf6w3954f02101fa5de1420b6ecd30ed550133f32d6a5c00233076af");
    /// ~~~~
    pub fn with_session_id(mut self, session_id: &str) -> Self {
        self.session_id(session_id);
        self
    }

    /// Run a solver function on some example inputs. The function and the
    /// inputs should be provided using a
    /// [`Puzzle`](.//struct.Puzzle.html)
    /// instance.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::{AocDay, Puzzle};
    ///
    /// let day_5 = AocDay::new(2019, 5);
    /// day_5.test(
    ///     &Puzzle::new(1, |x: String| x.chars().filter(|&y| y == 'z').count())
    ///         .with_examples(&["test", "cases"])
    /// );
    /// ~~~~
    pub fn test(&self, puzzle: &Puzzle<T, impl Display>) {
        println!("Testing day {} of AOC {}", self.day, self.year);
        for (i, example) in puzzle.examples.iter().enumerate() {
            println!("Part {}, Example {}: {}", puzzle.part, i + 1, (puzzle.solver)((self.serializer)(example.to_string())));
        }
    }

    /// Run a solver function on the day's input. The function should be
    /// provided using a
    /// [`Puzzle`](./struct.Puzzle.html)
    /// instance.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::{AocDay, Puzzle};
    ///
    /// let mut day_5 = AocDay::new(2019, 5);
    /// let part_1 = Puzzle::new(
    ///         1,
    ///         |x: String| x.chars().filter(|&y| y == 'z').count()
    ///     )
    ///     .with_examples(&["foo", "bar", "baz"]);
    /// let part_2 = Puzzle::new(
    ///         2,
    ///         |x: String| x.chars().filter(|&y| y != 'z').count()
    ///     )
    ///     .with_examples(&["fubar", "bazz", "fubaz"]);
    /// day_5.test(&part_1);
    /// day_5.test(&part_2);
    /// day_5.run(&part_1);
    /// day_5.run(&part_2);
    /// ~~~~
    pub fn run(&mut self, puzzle: &Puzzle<T, impl Display>) -> Result<(), Box<dyn Error>> {
        #[cfg(feature = "config-file")]
        {
            if self.session_id == None {
                // Try to get session ID from config file
                if let Ok(mut config_file) = File::open("aoc_helper.toml") {
                    let mut contents = String::new();
                    config_file.read_to_string(&mut contents).unwrap();
                    let value: Value = contents.parse().unwrap();
                    if let Some(session_id) = value.get("session-id") {
                        self.session_id = Some(session_id.as_str().unwrap().to_owned());
                    }
                }
            }
        }
        if self.session_id == None {
            return aoc_err(AocError::MissingSessionId);
        }

        let running_date = Date::try_from_ymd(self.year, 12, self.day).unwrap();
        // Due to timezone differences, this will be a little more lenient than the aoc website in accepting dates
        let today = Date::today();
        let max_year = if today.month() < 12 { today.year() - 1 } else { today.year() };
        if running_date > Date::try_from_ymd(max_year, 12, 25).unwrap() {
            return aoc_err(AocError::SpecifiedDateInFuture);
        } else if self.day > 25 || running_date < Date::try_from_ymd(2015, 12, 1).unwrap() {
            return aoc_err(AocError::NoPuzzleOnDate);
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
                    return Err(Box::new(e));
                }
            },
        };
        
        let mut contents = String::new();
        println!("{}", contents);
        input_file.read_to_string(&mut contents)?;
        if contents.len() == 0 {
            // Get the input from the website
            let response = ureq::get(&format!("https://adventofcode.com/{}/day/{}/input", self.year, self.day))
                .set("Cookie", &(String::from("session=") + self.session_id.as_ref().unwrap())).call();
            std::io::copy(&mut response.into_reader(), &mut input_file)?;
            let mut input_file = File::open(&self.input_path)?;
            input_file.read_to_string(&mut contents)?;
        }
        
        print!("[{} {}, {} {}, {} {}]: ",
            "AoC".yellow(), self.year,
            "day".bright_cyan(), self.day,
            "part".bright_cyan(), puzzle.part);
        std::io::stdout().flush()?;

        let input = (self.serializer)(contents.trim().to_string());
        let start_time = Instant::now();
        let output = (puzzle.solver)(input);
        let elapsed = start_time.elapsed();
        println!("{}", output.to_string().bright_white());

        let time_taken = {
            let mut msg_str = String::new();
            let (d, h, m, s, ms, us, ns) = (
                elapsed.whole_days(),
                elapsed.whole_hours() % 24,
                elapsed.whole_minutes() % 60,
                elapsed.whole_seconds() % 60,
                elapsed.whole_milliseconds() % 1000,
                elapsed.whole_microseconds() % 1000,
                elapsed.whole_nanoseconds() % 1000,
            );
            if d > 0 {
                msg_str.push_str(&format!("{}d ", d));
            }
            if h > 0 {
                msg_str.push_str(&format!("{}h ", h));
            }
            if m > 0 {
                msg_str.push_str(&format!("{}m ", m));
            }
            if s > 0 {
                msg_str.push_str(&format!("{}s ", s));
            }
            if ms > 0 {
                msg_str.push_str(&format!("{}ms ", ms));
            }
            if us > 0 {
                msg_str.push_str(&format!("{}us ", us));
            }
            if ns > 0 {
                msg_str.push_str(&format!("{}ns ", ns));
            }
            msg_str
        };
        println!("{} {}", "Finished in".bright_green(), time_taken);
        
        Ok(())
    }
}

impl<T, D: Display> Puzzle<T, D> {
    /// Create a new `Puzzle` instance for the provided part number and solver
    /// function.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Puzzle;
    ///
    /// let part_1 = Puzzle::new(1, |x| x.lines().count());
    /// ~~~~
    pub fn new(part: u8, solver: fn(T) -> D) -> Self {
        Puzzle {
            part,
            examples: Vec::new(),
            solver,
        }
    }

    /// Provide some example inputs for a `Puzzle` instance that the solver
    /// function will be given when
    /// [`AocDay::test()`](./struct.AocDay.html#method.test)
    /// is called.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Puzzle;
    ///
    /// let mut part_2 = Puzzle::new(2, |x| x.lines().nth(0).unwrap());
    /// part_2.examples(&["example\ninput", "foo\nbar"]);
    /// ~~~~
    pub fn examples<S: ToString>(&mut self, examples: &[S]) {
        self.examples = examples.iter().map(|example| example.to_string()).collect();
    }

    /// Chainable version of [`Puzzle::examples()`](./struct.Puzzle.html#method.examples)
    ///
    /// # Example
    ///
    /// ~~~~
    /// use aoc_helper::Puzzle;
    ///
    /// let part_2 = Puzzle::new(2, |x| lines().nth(0).unwrap())
    ///     .with_examples(&["example\ninput", "foo\nbar"]);
    /// ~~~~
    pub fn with_examples<S: ToString>(mut self, examples: &[S]) -> Self {
        self.examples(examples);
        self
    }
}
