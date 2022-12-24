use aoc::grid::parse_char_grid;
use core::cmp::Reverse;
use ndarray::Array2;
use std::collections::BinaryHeap;
use std::convert::identity;
use std::iter;

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
    let mut height_map = parse_char_grid(input, identity);

    println!("height_map:\n{:?}", height_map);

    // Figure out the starting and ending points
    let (mut start, mut end) = (None, None);
    for (pos, val) in height_map.indexed_iter_mut() {
        if *val == 'S' {
            start = Some(pos);
            *val = 'a';
        } else if *val == 'E' {
            end = Some(pos);
            *val = 'z';
        }
        if start.is_some() && end.is_some() {
            break;
        }
    }
    let (start, end) = (start.unwrap(), end.unwrap());
    println!("height_map:\n{:?}", height_map);
    println!("start: {:?}, end: {:?}", start, end);

    // Find the shortest path
    let steps = Array2::<Option<usize>>::default(height_map.dim());
    let mut heap: BinaryHeap<_> = iter::once(Reverse((0, start))).collect();
    while let Some(Reverse((step, pos))) = heap.pop() {
        if pos == end {
            panic!("Found the end in {} steps", step);
        }

        if steps[pos] == Some(step) {
            continue;
        }

        panic!("Somethign");
        // steps[pos] = Some(
        // for &dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        //     let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        //     if height_map[new_pos] != '#' && steps[new_pos] != Some(step + 1) {
        //         heap.push(Reverse((step + 1, new_pos)));
        //     }
        // }
    }
    println!("heap: {:?}", heap);
    // while let Some(item) = heap.pop() {
    //     println!("item: {:?}", item);
    // }
    // let input = include_str!("../../puzzle_inputs/day_11.txt");
    // o
    // let monkeys: Vec<Monkey> = input.trim().split("\n\n").map(Monkey::from_str).collect();
}
