use itertools::{iproduct, Itertools};
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../puzzle_inputs/day_14.txt");
    let grid = parse_input(input);

    println!("day 14a: {} (578)", solve(grid.clone(), true));
    println!("day 14b: {} (24377)", solve(grid, false));
}

fn solve(mut grid: HashSet<(usize, usize)>, part_a: bool) -> usize {
    let max_y: usize = *grid.iter().map(|(_, y)| y).max().unwrap();
    let n_rocks = grid.len();
    let sand_start: (usize, usize) = (500, 0);

    loop {
        let mut sand_pos = sand_start;
        loop {
            // Advance the simulation
            let mut halted = true;

            if part_a || sand_pos.1 < max_y + 1 {
                let next_pos = [
                    (sand_pos.0, sand_pos.1 + 1),
                    (sand_pos.0 - 1, sand_pos.1 + 1),
                    (sand_pos.0 + 1, sand_pos.1 + 1),
                ];
                for pos in next_pos {
                    if part_a && pos.1 > max_y {
                        return grid.len() - n_rocks;
                    } else if !grid.contains(&pos) {
                        sand_pos = pos;
                        halted = false;
                        break;
                    }
                }
            }

            if halted {
                grid.insert(sand_pos);
                if !part_a && sand_pos == sand_start {
                    return grid.len() - n_rocks;
                }
                break;
            }
        }
    }
}

fn parse_input(input: &str) -> HashSet<(usize, usize)> {
    input
        .trim()
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|pt| {
                    pt.split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .tuple_windows()
                .flat_map(|((x1, y1), (x2, y2))| {
                    iproduct!(x1.min(x2)..=x1.max(x2), y1.min(y2)..=y1.max(y2))
                })
        })
        .collect()
}
