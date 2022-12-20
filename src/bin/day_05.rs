// use aoc::parse_regex::parse_lines;
// use regex::Regex;
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
    // let re = Regex::new(r"(\d+)\-(\d+),(\d+)\-(\d+)").unwrap();
    // let input = include_str!("../../puzzle_inputs/day_04.txt");
    // let input: Vec<(usize, usize, usize, usize)> = parse_lines(re, input.trim()).collect();
    let input = TEST_INPUT;
    let (stacks, _instructions) = input.trim().split_once("\n\n").unwrap();
    let mut stacks = stacks.lines().rev();
    let first_line = stacks.next().unwrap();
    println!("- {}", first_line);
    let n_stacks = first_line.split("   ").count();
    println!("n_stacks: {:?}", n_stacks);
    let stacks: Vec<Vec<char>> = iter::repeat_with(Vec::new).take(n_stacks).collect();

    println!("stacks: {:?}", stacks);

    // println!("- {}", line);
    // });
    println!("That's all!");

    //     println!(
    //         "day 4a: {}",
    //         input
    //             .iter()
    //             .filter(|(a1, a2, b1, b2)| (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2))
    //             .count()
    //     );

    //     println!(
    //         "day 4b: {}",
    //         input
    //             .iter()
    //             .filter(|(a1, a2, b1, b2)| a2 >= b1 && b2 >= a1)
    //             .count()
    //     );
}
