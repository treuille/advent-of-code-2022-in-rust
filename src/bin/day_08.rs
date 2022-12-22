// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::ops::Deref;
// use std::rc::Rc;
use ndarray::{Array1, Array2, Axis};

const TEST_INPUT: &str = "
30373
25512
65332
33549
35390
";

fn main() {
    let input = include_str!("../../puzzle_inputs/day_08.txt");
    // println!("{}", TEST_INPUT.trim());
    let input_lines: Vec<&str> = input.trim().lines().collect();
    let rows = input_lines.len();
    let input = input_lines
        .iter()
        .flat_map(|line| line.chars().filter_map(|c| String::from(c).parse().ok()))
        .collect::<Array1<u8>>();
    let cols = input.len() / rows;
    let trees = input.into_shape((rows, cols)).unwrap();
    println!("trees: {:?}", trees);

    // Create a visibility array
    let mut visible = Array2::<bool>::default((rows, cols));
    println!("visible: {:?}", visible);

    for axis in [Axis(0), Axis(1)] {
        println!("axis: {:?}", axis);
        for (tree_lane, mut vis_lane) in trees.lanes(axis).into_iter().zip(visible.lanes_mut(axis))
        {
            // Iterate forward
            let mut pair_iter = tree_lane.iter().zip(vis_lane.iter_mut());
            let (mut max_height, edge_vis) = pair_iter.next().unwrap();
            *edge_vis = true;
            for (tree, vis) in pair_iter {
                if tree > max_height {
                    *vis = true;
                    max_height = tree;
                }
            }

            // Iterate backwards
            let mut pair_iter = tree_lane.iter().rev().zip(vis_lane.iter_mut().rev());
            let (mut max_height, edge_vis) = pair_iter.next().unwrap();
            *edge_vis = true;
            for (tree, vis) in pair_iter {
                if tree > max_height {
                    *vis = true;
                    max_height = tree;
                }
            }
        }
    }

    println!("trees: {:?}", trees);
    println!("visible: {:?}", visible);
    println!(
        "visible sum: {:?}",
        visible.iter().filter_map(|b| b.then_some(())).count()
    );
    // let traversals = [
    //     Box::new(trees.rows().zip(visible.rows_mut()))
    // let input_lines = input.lines();
    // let root = Path::parse_input(input_lines);

    // // Solve a
    // let (total_size, total_size_list) = root.deref().borrow_mut().total_sizes();
    // let answer_a: usize = total_size_list.iter().filter(|x| **x <= 100000).sum();
    // println!("day 7a: {} (1350966)", answer_a);

    // // Solve b
    // let must_free = total_size - 40000000;
    // let answer_b = total_size_list.iter().filter(|x| **x >= must_free).min();
    // println!("day 7b: {} (6296435)", answer_b.unwrap());
}
