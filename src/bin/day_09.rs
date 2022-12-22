use aoc::parse_regex::parse_lines;
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
    let input = TEST_INPUT;
    let re = Regex::new(r"(L|R|U|D) (\d+)").unwrap();
    let input = parse_lines(re, input);

    // for (direction, count) in input {
    //     let direction: &str = direction;
    //     let count: usize = count;
    //     println!("{} {}", direction, count);
    // }

    let initial_conditions = (0, 0, 0, 0);
    [initial_conditions]
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
                .scan(
                    initial_conditions,
                    |(head_x, head_y, tail_x, tail_y), (delta_x, delta_y)| {
                        *head_x += delta_x;
                        *head_y += delta_y;
                        *tail_x += match *head_x - *tail_x {
                            2 => 1,
                            -2 => -1,
                            -1..=1 => 0,
                            _ => panic!("xs too distant: {} and {}", *head_x, *tail_x),
                        };
                        *tail_y += match *head_y - *tail_y {
                            2 => 1,
                            -2 => -1,
                            -1..=1 => 0,
                            _ => panic!("ys too distant: {} and {}", *head_x, *tail_x),
                        };
                        Some((*head_x, *head_y, *tail_x, *tail_y))
                    },
                ),
        )
        .for_each(|(head_x, head_y, tail_x, tail_y)| {
            for y in (0..=5).rev() {
                for x in 0..=5 {
                    print!(
                        "{}",
                        match (x, y) {
                            pt if pt == (head_x, head_y) => "H",
                            pt if pt == (tail_x, tail_y) => "T",
                            (0, 0) => "s",
                            _ => ".",
                        }
                    );
                }
                println!();
            }
            println!();
        });
}
