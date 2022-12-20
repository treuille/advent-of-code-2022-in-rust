use aoc::parse_regex::parse_lines;
use regex::Regex;

static TEST_INPUT: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

fn main() {
    let re = Regex::new(r"(\d+)\-(\d+),(\d+)\-(\d+)").unwrap();
    let input = include_str!("../../puzzle_inputs/day_04.txt");
    let input: Vec<(usize, usize, usize, usize)> = parse_lines(re, input.trim()).collect();

    println!(
        "day 4a: {}",
        input
            .iter()
            .filter(|(a1, a2, b1, b2)| (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2))
            .count()
    );

    println!(
        "day 4b: {}",
        input
            .iter()
            .filter(|(a1, a2, b1, b2)| a2 >= b1 && b2 >= a1)
            .count()
    );
}
