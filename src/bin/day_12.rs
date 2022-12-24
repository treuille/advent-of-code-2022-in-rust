use aoc::grid::{neighbors, parse_char_grid};
use core::cmp::Reverse;
use ndarray::Array2;
use std::collections::BinaryHeap;
use std::convert::identity;
use std::iter;

fn main() {
    let input = include_str!("../../puzzle_inputs/day_12.txt");
    let mut height_map = parse_char_grid(input, identity);

    // Figure out the starting and ending points
    let (mut start, mut end) = (None, None);
    for (pos, height) in height_map.indexed_iter_mut() {
        if *height == 'S' {
            start = Some(pos);
            *height = 'a';
        } else if *height == 'E' {
            end = Some(pos);
            *height = 'z';
        }
        if start.is_some() && end.is_some() {
            break;
        }
    }

    // Solve part a
    let (start, end) = (start.unwrap(), end.unwrap());
    println!("day 12a: {} (447)", solve(&height_map, start, end).unwrap());

    // Solve part b
    println!(
        "day 12b: {} (446)",
        height_map
            .indexed_iter()
            .filter_map(|(pos, height)| (*height == 'a').then_some(pos))
            .filter_map(|start| solve(&height_map, start, end))
            .min()
            .unwrap()
    );
}

fn solve(height_map: &Array2<char>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let can_step = |c1: char, c2: char| -> bool {
        (c2 as u32)
            .checked_sub(c1 as u32)
            .map(|diff| diff <= 1)
            .unwrap_or(true)
    };

    // Find the shortest path
    let dim = height_map.dim();
    let mut steps = Array2::<Option<usize>>::default(dim);
    let mut heap: BinaryHeap<_> = iter::once(Reverse((0, start))).collect();
    while let Some(Reverse((step, pos))) = heap.pop() {
        if pos == end {
            return Some(step);
        } else if steps[pos] == Some(step) {
            continue;
        }

        steps[pos] = Some(step);
        for neighbor in neighbors(pos, dim) {
            if steps[neighbor].is_some() {
                continue;
            } else if can_step(height_map[pos], height_map[neighbor]) {
                heap.push(Reverse((step + 1, neighbor)));
            }
        }
    }
    None
}
