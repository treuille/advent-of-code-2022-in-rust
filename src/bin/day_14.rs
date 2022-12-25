use itertools::Itertools;
use std::collections::HashMap;

const TEST_INPUT: &str = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

#[derive(Clone, PartialEq)]
enum GridCell {
    Sand,
    Rock,
}

type Pt = (usize, usize);
type Grid = HashMap<Pt, GridCell>;
const SAND_START: Pt = (500, 0);

fn main() {
    let input = include_str!("../../puzzle_inputs/day_14.txt");
    // let input = TEST_INPUT;
    let grid = parse_input(input);

    println!("Day 14a: {} (578)", solve(grid.clone(), true));
    println!("Day 14b: {} (??)", solve(grid, false));
}

fn solve(mut grid: Grid, part_a: bool) -> usize {
    let max_y: usize = *grid.keys().map(|(_, y)| y).max().unwrap();

    let mut n_sand: usize = 0;
    loop {
        let mut sand_pos = SAND_START;
        loop {
            // print_grid(&grid, Some(sand_pos));
            // println!();

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
                        return n_sand;
                    } else if !grid.contains_key(&pos) {
                        sand_pos = pos;
                        halted = false;
                        break;
                    }
                }
            }

            if halted {
                grid.insert(sand_pos, GridCell::Sand);
                n_sand += 1;
                if !part_a && sand_pos == SAND_START {
                    return n_sand;
                }
                break;
            }
        }
    }
}

fn print_grid(grid: &Grid, sand_pos: Option<(usize, usize)>) {
    let grid_pts = grid.keys().chain([&SAND_START]).collect_vec();

    let min_x = grid_pts.iter().map(|pt| pt.0).min().unwrap();
    let max_x = grid_pts.iter().map(|pt| pt.0).max().unwrap();
    let min_y = grid_pts.iter().map(|pt| pt.1).min().unwrap();
    let max_y = grid_pts.iter().map(|pt| pt.1).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                if sand_pos == Some((x, y)) {
                    'O'
                } else if (x, y) == SAND_START {
                    '+'
                } else {
                    match grid.get(&(x, y)) {
                        Some(GridCell::Sand) => 'o',
                        Some(GridCell::Rock) => '#',
                        None => '.',
                    }
                }
            );
        }
        println!();
    }
}

fn parse_input(input: &str) -> Grid {
    // Figure out the grid
    let input = input.trim();

    // Fill in the grid
    let mut grid = HashMap::new();
    for line in input.lines() {
        line.split(" -> ")
            .map(|pt| {
                pt.split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .tuple_windows()
            .for_each(|((x1, y1), (x2, y2))| {
                for x in x1.min(x2)..=x1.max(x2) {
                    for y in y1.min(y2)..=y1.max(y2) {
                        grid.insert((x, y), GridCell::Rock);
                    }
                }
            });
    }
    grid
}
