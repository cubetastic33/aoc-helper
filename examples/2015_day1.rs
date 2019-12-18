use aoc_helper::Helper;

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
    // Create a new `Helper` instance for day 1 of aoc 2015
    let mut helper = Helper::new(2015, 1);

    // Add a solver function for part 1
    helper.part1(|instructions| {
        instructions.chars().filter(|&x| x == '(').count() as i32
        - instructions.chars().filter(|&x| x == ')').count() as i32
    });

    // Add some example cases for part 2
    helper.part2_examples(&[")", "()())"]);
    // Add a solver function for part 2
    helper.part2(first_instruction_that_sends_to_basement);

    // Run the solver functions on the example cases
    helper.test().unwrap();
    // Run the solver functions on the day's input
    helper.run().unwrap();
}
