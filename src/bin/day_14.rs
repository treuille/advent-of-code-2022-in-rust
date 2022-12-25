use itertools::Itertools;
use ndarray::{s, Array1, Array2};

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
enum GridCell {
    Air,
    Sand,
    Rock,
}

#[derive(Debug)]
struct GridBounds {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

const TEST_INPUT: &str = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
const SAND_START: (usize, usize) = (500, 0);

fn main() {
    let input = include_str!("../../puzzle_inputs/day_14.txt");
    // let input = TEST_INPUT;
    let (mut grid, bounds) = parse_input(input);
    // print_grid(&grid, &bounds, None);
    // println!();

    let mut n_sand: usize = 0;
    loop {
        print_grid(&grid, &bounds, None);
        println!();

        let mut sand_pos = SAND_START;
        loop {
            // Advance the simulation
            let mut halted = true;

            let next_pos = [
                (sand_pos.0, sand_pos.1 + 1),
                (sand_pos.0 - 1, sand_pos.1 + 1),
                (sand_pos.0 + 1, sand_pos.1 + 1),
            ];

            for pos in next_pos {
                if pos.1 > bounds.max_y {
                    panic!("Reached the bottom with {} grid cells of sand!", n_sand);
                } else if grid[pos] == GridCell::Air {
                    sand_pos = pos;
                    halted = false;
                    break;
                }
            }

            if halted {
                grid[sand_pos] = GridCell::Sand;
                n_sand += 1;
                break;
            }
        }
    }
}

fn print_grid(grid: &Array2<GridCell>, bounds: &GridBounds, sand_pos: Option<(usize, usize)>) {
    let char_grid: Array2<char> = Array1::from_iter(grid.indexed_iter().map(|(pos, cell)| {
        if sand_pos == Some(pos) {
            'O'
        } else if pos == SAND_START {
            '+'
        } else {
            match cell {
                GridCell::Air => '.',
                GridCell::Sand => 'o',
                GridCell::Rock => '#',
            }
        }
    }))
    .into_shape(grid.dim())
    .unwrap();
    let char_sub_grid =
        char_grid.slice(s![bounds.min_x..=bounds.max_x, bounds.min_y..=bounds.max_y]);
    println!(
        "char_grid: {:?} char_sub_grid: {:?}",
        char_grid.dim(),
        char_sub_grid.dim()
    );

    for row in char_sub_grid.columns() {
        println!("{}", row.iter().join(""));
    }
}

fn parse_input(input: &str) -> (Array2<GridCell>, GridBounds) {
    // Figure out the grid
    let input = input.trim();
    let bounds = {
        let grid_points = input
            .lines()
            .flat_map(|line| {
                line.split(" -> ").map(|pt| {
                    pt.split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
            })
            .collect_vec();
        GridBounds {
            min_x: *grid_points.iter().map(|(x, _)| x).min().unwrap().min(&500),
            max_x: *grid_points.iter().map(|(x, _)| x).max().unwrap().max(&500),
            min_y: *grid_points.iter().map(|(_, y)| y).min().unwrap().min(&0),
            max_y: *grid_points.iter().map(|(_, y)| y).max().unwrap().max(&0),
        }
    };
    println!("bounds: {:?}", bounds);

    // Fill in the grid
    let dim = (bounds.max_x + 1, bounds.max_y + 1);
    let mut grid = Array2::from_elem(dim, GridCell::Air);
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
                grid.slice_mut(s![x1.min(x2)..=x1.max(x2), y1.min(y2)..=y1.max(y2)])
                    .fill(GridCell::Rock)
            });
    }
    (grid, bounds)
}
