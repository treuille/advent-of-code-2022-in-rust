use aoc::parse_regex::parse_lines;
use itertools::Itertools;
use regex::Regex;
use std::iter;

const TEST_INPUT: &str = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

fn main() {
    // Parse the input
    // let input = TEST_INPUT;
    let input = include_str!("../../puzzle_inputs/day_09.txt");
    let re = Regex::new(r"(L|R|U|D) (\d+)").unwrap();
    let input = parse_lines(re, input);

    // for (direction, count) in input {
    //     let direction: &str = direction;
    //     let count: usize = count;
    //     println!("{} {}", direction, count);
    // }

    type Pt = (isize, isize);
    let add = |pt1: Pt, pt2: Pt| (pt1.0 + pt2.0, pt1.1 + pt2.1);
    let sub = |pt1: Pt, pt2: Pt| (pt1.0 - pt2.0, pt1.1 - pt2.1);

    let initial_conditions: Vec<Pt> = iter::repeat((0, 0)).take(2).collect();
    let answer = [initial_conditions.clone()]
        .into_iter()
        .chain(
            input
                .flat_map(|(direction, count)| {
                    iter::repeat(match direction {
                        "L" => (-1, 0),
                        "R" => (1, 0),
                        "U" => (0, 1),
                        "D" => (0, -1),
                        _ => unreachable!("Unexpected direction: {}", direction),
                    })
                    .take(count)
                })
                .scan(initial_conditions, |rope, (head_dx, head_dy)| {
                    rope[0] = add(rope[0], (head_dx, head_dy));
                    for (i, j) in (0..).zip(1..rope.len()) {
                        let (dx, dy) = match sub(rope[i], rope[j]) {
                            (-2, -2) => (-1, -1),
                            (-2, -1) => (-1, -1),
                            (-2, 0) => (-1, 0),
                            (-2, 1) => (-1, 1),
                            (-2, 2) => (-1, 1),

                            (2, -2) => (1, -1),
                            (2, -1) => (1, -1),
                            (2, 0) => (1, 0),
                            (2, 1) => (1, 1),
                            (2, 2) => (1, 1),

                            // (-2, -2) => (-1, -1),
                            (-1, -2) => (-1, -1),
                            (0, -2) => (0, -1),
                            (1, -2) => (1, -1),
                            // (2, -2) => (1, -1),

                            // (-2, 2) => (-1, 1),
                            (-1, 2) => (-1, 1),
                            (0, 2) => (0, 1),
                            (1, 2) => (1, 1),
                            // (2, 2) => (1, 1),
                            _ => (0, 0),
                        };
                        rope[j] = add(rope[j], (dx, dy));
                        // + *tail_x += dx;
                        // *tail_y += dy
                    }
                    // *rope[0].1 += head_dy;
                    Some(rope.clone())
                }),
        )
        .map(|rope| {
            // for y in (0..=4).rev() {
            //     for x in 0..=5 {
            //         let mut rope_here = false;
            //         for (i, (rope_x, rope_y)) in rope.iter().enumerate() {
            //             if (x, y) == (*rope_x, *rope_y) {
            //                 if i == 0 {
            //                     print!("H");
            //                 } else {
            //                     print!("{}", i);
            //                 }
            //                 rope_here = true;
            //                 break;
            //             }
            //         }
            //         if !rope_here {
            //             if (x, y) == (0, 0) {
            //                 print!("S");
            //             } else {
            //                 print!(".");
            //             }
            //         }
            //     }
            //     println!();
            // }
            // println!();
            rope.last().unwrap().clone()
        })
        .unique()
        .count();

    println!("answer: {}", answer);
}
