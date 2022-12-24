use aoc::parse_grid::parse_char_grid;
use ndarray::{Array2, Axis};
use std::iter;

fn main() {
    let input = include_str!("../../puzzle_inputs/day_08.txt");
    let to_u8 = |c: char| c.to_digit(10).unwrap() as u8;
    let trees = parse_char_grid(input, to_u8);

    println!("day 8a: {} (1789)", solve_a(&trees));
    println!("day 8b: {} (314820)", solve_b(&trees));
}

fn solve_a(trees: &Array2<u8>) -> usize {
    // Fill in a visibility array by iterating over rows and colums,
    // forward and backwards.
    let mut visible = Array2::<bool>::default(trees.dim());
    for (axis, reversed) in [Axis(0), Axis(1)].into_iter().zip([false, true]) {
        for (tree_lane, mut vis_lane) in trees.lanes(axis).into_iter().zip(visible.lanes_mut(axis))
        {
            let mut tree_lane: Vec<&u8> = tree_lane.iter().collect();
            let mut vis_lane: Vec<&mut bool> = vis_lane.iter_mut().collect();
            if reversed {
                tree_lane.reverse();
                vis_lane.reverse();
            }
            let mut pair_iter = tree_lane.into_iter().zip(vis_lane.into_iter());
            let (edge_height, edge_vis) = pair_iter.next().unwrap();
            let mut max_height: u8 = *edge_height;
            *edge_vis = true;
            for (height, vis) in pair_iter {
                if *height > max_height {
                    *vis = true;
                    max_height = *height;
                }
            }
        }
    }
    visible.map(|b| *b as usize).sum()
}

fn solve_b(trees: &Array2<u8>) -> usize {
    // A point on the grid.
    type Pt = (usize, usize);

    // For each compass direction, these functions advance one grid cell, preventing
    // usize underflow or overflow.
    type DirFn = Box<dyn Fn(Pt) -> Option<Pt>>;
    let directions: [DirFn; 4] = [
        Box::new(|(x, y)| x.checked_sub(1).map(|x| (x, y))), // up
        Box::new(|(x, y)| y.checked_sub(1).map(|y| (x, y))), // left
        Box::new(|(x, y)| x.checked_add(1).map(|x| (x, y))), // down
        Box::new(|(x, y)| y.checked_add(1).map(|y| (x, y))), // right
    ];

    // Iterate through every grid cell, calculating the scenic score.
    trees
        .indexed_iter()
        .map(|(pos, &height)| {
            directions
                .iter()
                .map(|dir_fn| {
                    // This function advances one grid cell at a time, picking up
                    // tree heights from the grid while preventing going off the grid.
                    let traverse = |&(pos, _): &(Pt, u8)| -> Option<(Pt, u8)> {
                        dir_fn(pos).and_then(|pos2| trees.get(pos2).map(|&height2| (pos2, height2)))
                    };

                    // This iterator traverses the grid so long as trees are
                    // strictly shorter than `height`.
                    iter::successors(traverse(&(pos, height)), |&(pos2, height2)| {
                        (height2 < height)
                            .then_some(&(pos2, height2))
                            .and_then(traverse)
                    })
                    .count()
                })
                .product()
        })
        .max()
        .unwrap()
}
