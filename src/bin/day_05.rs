use aoc::parse_regex::parse_lines;
use regex::Regex;
use std::iter;

static TEST_INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

fn main() {
    let input = include_str!("../../puzzle_inputs/day_05.txt");
    // let input = TEST_INPUT;
    let (stack_lines, instructions) = input.split_once("\n\n").unwrap();
    let mut stack_lines = stack_lines.lines().rev();
    let n_stacks = stack_lines.next().unwrap().split("   ").count();
    println!("n_stacks: {:?}", n_stacks);
    let mut stacks: Vec<Vec<char>> = iter::repeat_with(Vec::new).take(n_stacks).collect();

    println!("stacks: {:?}", stacks);

    for line in stack_lines {
        println!("- \"{}\"", line);
        for (c, stack) in line.chars().skip(1).step_by(4).zip(&mut stacks) {
            if c != ' ' {
                stack.push(c);
            }
        }
    }
    println!("That's all! {:?}", stacks);

    println!("instructions:\n{}", instructions);
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let instructions: Vec<(usize, usize, usize)> = parse_lines(re, instructions).collect();

    println!("day 5a: {}", solve_a(&stacks, &instructions));
}

fn solve_a(stacks: &[Vec<char>], instructions: &Vec<(usize, usize, usize)>) -> String {
    let mut stacks = Vec::from(stacks);
    for (n_items, from_stack, to_stack) in instructions {
        println!("{} items : {} -> {}", n_items, from_stack, to_stack);
        for _ in 0..*n_items {
            let item = stacks[from_stack - 1].pop().unwrap();
            stacks[to_stack - 1].push(item);
            println!("Now: {:?}", stacks);
        }
    }

    // The final solution
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}
