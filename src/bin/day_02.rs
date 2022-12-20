use aoc::parse_regex::parse_lines;
use regex::Regex;

fn main() {
    let input = parse_input(include_str!("../../puzzle_inputs/day_02.txt"));

    println!("day 2a: {}", input.iter().map(compute_score_a).sum::<u64>());

    println!("day 2b: {}", input.iter().map(compute_score_b).sum::<u64>());
}

fn compute_score_a(round: &(char, char)) -> u64 {
    let winner_score = match round {
        &('A', 'Y') | &('B', 'Z') | &('C', 'X') => 6,
        &('A', 'X') | &('B', 'Y') | &('C', 'Z') => 3,
        _ => 0,
    };

    let choice_score = match round.1 {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        c => unreachable!("Unexpected play: {}", c),
    };

    winner_score + choice_score
}

fn compute_score_b(round: &(char, char)) -> u64 {
    let winner_score = match round.1 {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        c => unreachable!("Unexpected play: {}", c),
    };

    let choice_score = match round {
        &('A', 'Y') | &('B', 'X') | &('C', 'Z') => 1,
        &('B', 'Y') | &('C', 'X') | &('A', 'Z') => 2,
        &('C', 'Y') | &('A', 'X') | &('B', 'Z') => 3,
        &(c1, c2) => unimplemented!("Unexpected play: {} {}", c1, c2),
    };

    winner_score + choice_score
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    let re = Regex::new("(A|B|C) (X|Y|Z)").unwrap();
    parse_lines(re, input).collect()
}
