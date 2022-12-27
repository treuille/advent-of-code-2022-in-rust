use aoc::parse_regex::parse_lines;
use regex::Regex;
use std::iter;

type Stack = Vec<char>;
type Instruction = (usize, usize, usize);

fn main() {
    // Split the input into two pieces
    let input = include_str!("../../puzzle_inputs/day_05.txt");
    let (stack_lines, instructions) = input.split_once("\n\n").unwrap();

    // Parse the stacks
    let mut stack_lines = stack_lines.lines().rev();
    let n_stacks = stack_lines.next().unwrap().split("   ").count();
    let mut stacks: Vec<Stack> = iter::repeat_with(Vec::new).take(n_stacks).collect();
    for line in stack_lines {
        for (item, stack) in line.chars().skip(1).step_by(4).zip(&mut stacks) {
            if item != ' ' {
                stack.push(item);
            }
        }
    }

    // Parse the instructions
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let instructions: Vec<Instruction> = parse_lines(re, instructions).collect();

    // Print the solutions
    println!(
        "day 5a: {} (RNZLFZSJH)",
        sim_crane(&stacks, &instructions, false)
    );
    println!(
        "day 5b: {} (CNSFCGJSM)",
        sim_crane(&stacks, &instructions, true)
    );
}

fn sim_crane(stacks: &[Stack], instructions: &[Instruction], cm_9001: bool) -> String {
    // Clone the stack so we can mutate them.
    let mut stacks = Vec::from(stacks);

    // Run the simulation.
    for (n_items, from_stack, to_stack) in instructions {
        let split_at = stacks[from_stack - 1].len() - n_items;
        let mut items_to_xfer = stacks[from_stack - 1].split_off(split_at);
        if !cm_9001 {
            items_to_xfer.reverse();
        }
        stacks[to_stack - 1].extend(items_to_xfer);
    }

    // Return the top items from each stack as a string.
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}
