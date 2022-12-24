// use aoc::grid::{neighbors, parse_char_grid};
// use core::cmp::Reverse;
// use ndarray::Array2;
// use std::collections::BinaryHeap;
// use std::convert::identity;

use std::cmp::{Ord, Ordering};
// use std::iter;

#[derive(Debug)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

// impl PartialEq<Self> for Packet {
//     fn eq(&self, _: &Self) -> bool {
//         todo!("Need to implement PartialEq for Packet");
//     }
// }

// impl PartialOrd<Self> for Packet {
//     fn partial_cmp(&self, _: &Self) -> std::option::Option<Ordering> {
//         todo!("Need to implement PartialOrd for Packet");
//     }
// }

// impl Eq<Self> for Packet {

// impl Ord for Packet {
fn cmp(packet_1: &Packet, packet_2: &Packet) -> Ordering {
    match (packet_1, packet_2) {
        (Packet::Num(num_1), Packet::Num(num_2)) => num_1.cmp(num_2),
        (Packet::List(list_1), Packet::List(list_2)) => {
            for (item_1, item_2) in list_1.iter().zip(list_2) {
                match cmp(item_1, item_2) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => (),
                }
            }
            list_1.len().cmp(&list_2.len())
        }
        (Packet::Num(num), list_packet) => cmp(&Packet::List(vec![Packet::Num(*num)]), list_packet),
        (list_packet, Packet::Num(num)) => cmp(list_packet, &Packet::List(vec![Packet::Num(*num)])),
    }
}

// type CharIter = Box<dyn Iterator<Item = char>>;
fn parse(input: &[char]) -> Packet {
    if input.is_empty() {
        Packet::List(vec![])
    } else {
        if input[0] == '[' {
            assert_eq!(input[input.len() - 1], ']');
            let mut depth = 0;
            let mut list = Vec::new();
            let mut start = 1;
            for i in 1..input.len() - 1 {
                if input[i] == '[' {
                    depth += 1;
                } else if input[i] == ']' {
                    depth -= 1;
                } else if depth == 0 && input[i] == ',' {
                    list.push(parse(&input[start..i]));
                    start = i + 1;
                    // ------
                    // let right = parse(&input[i + 1..input.len() - 1]);
                    // return Packet::List(vec![left, right]);
                }
            }
            Packet::List(list)
        } else if input[0] == ']' {
            panic!("Unexpected ']'")
        } else if input[0] == ',' {
            panic!("Unexpected ','")
        } else {
            Packet::Num(input.iter().collect::<String>().parse().unwrap())
        }
    }
}
// fn parse(input: &[char]) -> Packet {
//     let len = input.len();
//     if input[0] == '[' {
//         assert_eq!(
//             input[len - 1],
//             ']',
//             "Error parsing \"{}\"",
//             input.iter().collect::<String>()
//         );
//         Packet::List(
//             input[1..len - 1]
//                 .split(|&c| c == ',')
//                 .map(|slice| {
//                     println!("Recursing: \"{}\"", slice.iter().collect::<String>());
//                     parse(slice)
//                 })
//                 .collect(),
//         )
//     } else {
//         println!("Num: {}", input.iter().collect::<String>());
//         Packet::Num(input.iter().collect::<String>().parse().unwrap())
//     }
// }

fn main() {
    // let input = include_str!("../../puzzle_inputs/day_12.txt");
    // let mut height_map = parse_char_grid(input, identity);

    for line in TEST_INPUT.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        println!("{}", line);
        println!("{:?}\n", parse(line.chars().collect::<Vec<_>>().as_slice()));
    }
}

const TEST_INPUT: &str = "
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
