// use aoc::parse_regex::parse_lines;
// // use itertools::Itertools;
// use regex::Regex;
// // use std::iter;

fn main() {
    // Parse the input.
    // let input = TEST_INPUT_1;
    // let input = TEST_INPUT_2;
    let input = include_str!("../../puzzle_inputs/day_10.txt");

    // let re = Regex::new(r"(addx|noop) (\-?\d+)").unwrap();
    // let input = parse_lines(re, input).collect();

    // 20th, 60th, 100th, 140th, 180th, and 220th
    // 19, 59, 99, 139, 179, 219
    let answer: i32 = input
        .trim()
        .lines()
        .flat_map(|line| {
            let mut tokens = line.split(' ');
            match tokens.next().unwrap() {
                "noop" => vec![0],
                "addx" => vec![0, tokens.next().unwrap().parse().unwrap()],
                instr => panic!("Unexpected instruction: {}", instr),
            }
        })
        .scan(1, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        // .enumerate()
        .zip(2..)
        .filter_map(|(sum, i)| match i {
            // 18 | 58 | 98 | 138 | 178 | 218 => Some((i, sum)),
            // 19 | 59 | 99 | 139 | 179 | 219 => Some((i, sum)),
            20 | 60 | 100 | 140 | 180 | 220 => Some(i * sum),
            // 21 | 61 | 101 | 141 | 181 | 221 => Some((i, sum)),
            // 22 | 62 | 102 | 142 | 182 | 222 => Some((i, sum)),
            _ => None,
        })
        // .for_each(|(i, x): (usize, i32)| println!("{} -> {:?}", i, x));
        .sum();
    println!("answer: {}", answer);

    // wu for i in 0..10 {
    //     println!("{}: Hello, world!", i);
    // }
}

const TEST_INPUT_1: &str = "
noop
addx 3
addx -5
";

const TEST_INPUT_2: &str = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
