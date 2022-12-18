use aoc::parse_regex::parse_lines;
use regex::Regex;

static TEST_INPUT: &str = "
A Y
B X
C Z
";

fn main() {
    let input = read_input(TEST_INPUT);
    for (x, y) in input {
        println!("input: {} {}", x, y);
    }
}

fn read_input(input: &str) -> Vec<(char, char)> {
    let re = Regex::new("(A|B|C) (X|Y|Z)").unwrap();
    parse_lines(re, input).collect()
    // let result: Result<u> = input
    //     .trim()
    //     .split("\n\n")
    //     .map(|lines| lines.lines().map(|s| s.parse()).collect())
    //     .collect();
    // result.unwrap()
}
