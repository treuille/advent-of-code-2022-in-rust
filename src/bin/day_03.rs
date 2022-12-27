use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    // Parse the input
    let input: Vec<&str> = include_str!("../../puzzle_inputs/day_03.txt")
        // let input: Vec<&str> = TEST_INPUT
        .trim()
        .lines()
        .collect();

    println!("day 3a: {} (7763)", solve_a(&input));
    println!("day 3b: {} (2569)", solve_b(&input));
}

fn solve_a(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| {
            let both_sacks: Vec<char> = line.chars().collect();
            let n_items = both_sacks.len();
            let sack_1: HashSet<&char> = both_sacks[0..(n_items / 2)].iter().collect();
            let sack_2: HashSet<&char> = both_sacks[(n_items / 2)..n_items].iter().collect();
            let common_item = **sack_1.intersection(&sack_2).next().unwrap();
            priority(common_item)
        })
        .sum()
}

fn solve_b(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .tuples()
        .map(|(sack_1, sack_2, sack_3)| {
            let sack_12: HashSet<char> = sack_1.intersection(&sack_2).copied().collect();
            let item = sack_12.intersection(&sack_3).next().unwrap();
            priority(*item)
        })
        .sum()
}

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        item => unreachable!("Unexpected item: {}", item),
    }
}
