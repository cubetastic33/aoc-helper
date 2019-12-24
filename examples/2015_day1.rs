use aoc_helper::{AocDay, Puzzle};

// The solver function for part 1
fn first_instruction_that_sends_to_basement(instructions: String) -> usize {
    let mut floor = 0;
    for (i, instruction) in instructions.chars().enumerate() {
        match instruction {
            '(' => floor += 1,
            _ => floor -= 1,
        }
        if floor < 0 {
            return i + 1;
        }
    }
    unreachable!();
}

fn main() {
    // NOTE: You need to specify a session ID for this to work

    // Create a new `AocDay` instance for day 1 of aoc 2015
    let mut day_1 = AocDay::new(2015, 1);

    // Create a new `Puzzle` instance for part 1
    let part_1 = Puzzle::new(1, |instructions: String| {
            let mut floor = 0;
            instructions.chars().for_each(|x| if x == '(' { floor += 1 } else { floor -= 1 });
            floor
        })
        .with_examples(&["(())", "()()", ")))", ")())())"]);

    // Create a new `Puzzle` instance for part 2
    let part_2 = Puzzle::new(2, first_instruction_that_sends_to_basement)
        .with_examples(&[")", "()())"]);

    // Test the example cases
    day_1.test(&part_1);
    day_1.test(&part_2);
    // Run the day's input
    day_1.run(&part_1).unwrap();
    day_1.run(&part_2).unwrap();
}
