use aoc::parse_grid::parse_char_grid;
use std::convert::identity;

const TEST_INPUT: &str = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

fn main() {
    println!("day 12");

    // let input = include_str!("../../puzzle_inputs/day_08.txt");
    let input = TEST_INPUT;
    let height_map = parse_char_grid(input, identity);

    println!("height_map:\n{:?}", height_map);

    // let input = include_str!("../../puzzle_inputs/day_11.txt");
    // o
    // let monkeys: Vec<Monkey> = input.trim().split("\n\n").map(Monkey::from_str).collect();
}
