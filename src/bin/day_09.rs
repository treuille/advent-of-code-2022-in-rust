use aoc::parse_regex::parse_lines;
use itertools::Itertools;
use regex::Regex;
use std::iter;

fn main() {
    // Parse the input.
    let input = include_str!("../../puzzle_inputs/day_09.txt");
    let re = Regex::new(r"(L|R|U|D) (\d+)").unwrap();
    let input: Vec<(&str, usize)> = parse_lines(re, input).collect();

    println!("day 9a: {} (6503)", solve(&input, 2));
    println!("day 9b: {} (2724)", solve(&input, 10));
}

fn solve(input: &[(&str, usize)], rope_len: usize) -> usize {
    // A point in 2D space.
    type Pt = (isize, isize);
    let add = |pt1: Pt, pt2: Pt| (pt1.0 + pt2.0, pt1.1 + pt2.1);
    let sub = |pt1: Pt, pt2: Pt| (pt1.0 - pt2.0, pt1.1 - pt2.1);

    let initial_rope: Vec<Pt> = iter::repeat((0, 0)).take(rope_len).collect();
    input
        .iter()
        .flat_map(|(direction, count)| {
            iter::repeat(match *direction {
                "L" => (-1, 0),
                "R" => (1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => unreachable!("Unexpected direction: {}", direction),
            })
            .take(*count)
        })
        .scan(initial_rope, |rope, d_head| {
            rope[0] = add(rope[0], d_head);
            for (i, j) in (0..).zip(1..rope.len()) {
                rope[j] = add(
                    rope[j],
                    match sub(rope[i], rope[j]) {
                        (-2, -2) | (-2, -1) | (-1, -2) => (-1, -1),
                        (-2, 0) => (-1, 0),
                        (-2, 1) | (-2, 2) | (-1, 2) => (-1, 1),
                        (0, -2) => (0, -1),
                        (0, 2) => (0, 1),
                        (2, -2) | (2, -1) | (1, -2) => (1, -1),
                        (2, 0) => (1, 0),
                        (2, 1) | (2, 2) | (1, 2) => (1, 1),
                        _ => (0, 0),
                    },
                );
            }
            Some(*rope.last().unwrap())
        })
        .chain([(0, 0)])
        .unique()
        .count()
}
