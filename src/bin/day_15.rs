use aoc::parse_regex::parse_lines;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::collections::HashSet;

const TEST_INPUT: &str = "
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

type Pt = (i64, i64);
type Sensor = (Pt, Pt);

fn manhattan_dist(a: Pt, b: Pt) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn main() {
    println!("Day 15: Sensor Boost");

    // let input = TEST_INPUT;
    let input = include_str!("../../puzzle_inputs/day_15.txt");
    let pt_re = r"x=(-?\d+), y=(-?\d+)".to_owned();
    let re = Regex::new(&format!(
        "Sensor at {}: closest beacon is at {}",
        pt_re, pt_re
    ))
    .unwrap();
    let sensor_and_beacons: Vec<Sensor> = parse_lines(re, input)
        .map(|(sensor_x, sensor_y, beacon_x, beacon_y)| {
            ((sensor_x, sensor_y), (beacon_x, beacon_y))
        })
        .collect();

    let sensor_dists: Vec<(Pt, i64)> = sensor_and_beacons
        .iter()
        .map(|(sensor, beacon)| (*sensor, manhattan_dist(*sensor, *beacon)))
        .collect();

    let (sensors, beacons): (HashSet<Pt>, HashSet<Pt>) = sensor_and_beacons.into_iter().unzip();
    println!("Sensors: {:?}", sensors);
    println!("Beacons: {:?}", beacons);

    for row in sensor_dists.iter() {
        println!("{:?}", row);
    }

    let min_x = sensor_dists
        .iter()
        .map(|((x, _), dist)| x - dist)
        .min()
        .unwrap();
    let max_x = sensor_dists
        .iter()
        .map(|((x, _), dist)| x + dist)
        .max()
        .unwrap();
    println!("min_x: {}, max_x: {}", min_x, max_x);

    // // let y_to_check = 10;
    // let y_to_check = 2000000;
    // let answer = (min_x..=max_x)
    //     .filter_map(|x| {
    //         let pt = (x, y_to_check);
    //         if beacons.contains(&pt) {
    //             return None;
    //         }
    //         for (sensor, dist) in sensor_dists.iter() {
    //             if manhattan_dist(*sensor, pt) <= *dist {
    //                 return Some(());
    //             }
    //         }
    //         None
    //     })
    //     .count();

    // println!("Answer: {}", answer);

    let limit = 4000000;
    // let limit = 20;
    let solns = iproduct!(0..=limit, 0..=limit)
        .filter_map(|pt| {
            if sensors.contains(&pt) || beacons.contains(&pt) {
                None
            } else {
                sensor_dists
                    .iter()
                    .all(|(snsr, dist)| manhattan_dist(*snsr, pt) > *dist)
                    .then_some(pt)
            }
        })
        .collect_vec();
    let (x, y) = solns.first().unwrap();
    let frequency = x * 4000000 + y;
    println!("Solns: {:?}", solns);
    println!("frequency: {:?}", frequency);
}

// println!("Day 14a: {} (578)", solve(grid.clone(), true));
// println!("Day 14b: {} (24377)", solve(grid, false));
