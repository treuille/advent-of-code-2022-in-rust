// use aoc::parse_regex::parse_lines;
// // use itertools::Itertools;
// use regex::Regex;
use ndarray::{s, Array2};
use std::iter;

fn main() {
    // Parse the input.
    // let input = TEST_INPUT_1;
    // let input = TEST_INPUT_2;
    let input = include_str!("../../puzzle_inputs/day_10.txt");
    let register_vals = parse_input(input);

    println!("10a: {} (16020)", solve_a(&register_vals));
    println!("10b: (ECZUZALR)");
    solve_b(&register_vals);

    // let (w, h) = (40, 6);
    // let compute_crt = |offset: i32, register_offset: i32| {
    //     register_vals
    //         .iter()
    //         .zip(offset..)
    //         .fold(
    //             Array1::<bool>::default(w * h),
    //             |mut crt, (register, cycle)| {
    //                 let x = cycle % (w as i32);
    //                 if (register + register_offset - x).abs() <= 1 {
    //                     crt[[(cycle as usize) % (w * h)]] = true;
    //                     // (() as usize
    //                 }
    //                 crt
    //             },
    //         )
    //         .into_shape((w, h))
    //         .unwrap()
    // };

    // for offset in -3..=3 {
    //     for register_offset in -3..=3 {
    //         println!("\noffset: {} register_offset: {}", offset, register_offset);
    //         print_crt(compute_crt(offset, register_offset));
    //     }
    // }
}

fn solve_a(register_vals: &[i32]) -> i32 {
    register_vals
        .iter()
        .zip(2..)
        .filter_map(|(sum, i)| match i {
            20 | 60 | 100 | 140 | 180 | 220 => Some(i * sum),
            _ => None,
        })
        .sum()
}

fn solve_b(register_vals: &[i32]) {
    let (w, h) = (40, 6);
    let mut crt = Array2::<bool>::default((w, h));
    let register_vals = iter::once(&1).chain(register_vals.iter());
    for (&sprite_pos, cycle) in register_vals.zip(1usize..) {
        // Figure out the pixel position
        let pixel_x = (cycle - 1) % w;
        let pixel_y = ((cycle - 1) / w) % h;

        // Get the current row
        let mut row = crt.slice_mut(s![.., pixel_y]);
        if ((pixel_x as i32) - sprite_pos).abs() <= 1 {
            row[pixel_x] = true;
        }

        // Print the current state.
        println!("Cycle:{:>12}", cycle);
        println!("Pixel:           ({},{})", pixel_x, pixel_y);
        println!("Sprite Position: {:}", sprite_pos);
        // println!("Row dim: {:?}", row.dim());

        // Print the sprite
        print!("Sprite:          ");
        for x in 0..(w as i32) {
            if (x - sprite_pos).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();

        // Print the current row
        print!("Current row:     ");
        for x in 0..=pixel_x {
            if row[x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();

        // Print the crt
        println!();
        print_crt(&crt);

        // Print spacer
        println!();

        // // Break early
        // if cycle == 22 {
        //     break;
        // }
    }
}

fn print_crt(crt: &Array2<bool>) {
    let (w, h) = crt.dim();
    for j in 0..h {
        for i in 0..w {
            print!(
                "{}",
                match crt.get((i, j)).unwrap() {
                    false => ".",
                    true => "#",
                }
            );
        }
        println!();
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .flat_map(|line| {
            let mut tokens = line.split(' ');
            match tokens.next().unwrap() {
                "noop" => vec![0],
                "addx" => vec![0, tokens.next().unwrap().parse().unwrap()],
                instr => panic!("Unexpected instruction: {}", instr),
            }
        })
        .scan(1, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        .collect()
}

const TEST_INPUT_1: &str = "
noop
addx 3
addx -5
";

const TEST_INPUT_2: &str = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
