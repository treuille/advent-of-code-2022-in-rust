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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct UVRect {
    min: Pt,
    max: Pt,
}

type UVRectSet = HashSet<UVRect>;

impl UVRect {
    fn from_pt_and_dist((x, y): Pt, dist: i64) -> Self {
        let r = UVRect {
            min: to_uv((x - dist, y)),
            max: to_uv((x + dist + 1, y)),
        };
        assert!(r.min.0 < r.max.0);
        assert!(r.min.1 < r.max.1);
        r
    }

    fn contains_xy(&self, (x, y): Pt) -> bool {
        let (u, v) = to_uv((x, y));
        self.min.0 <= u && u < self.max.0 && self.min.1 <= v && v < self.max.1
    }

    fn contains(&self, other: &Self) -> bool {
        self.min.0 <= other.min.0
            && self.max.0 >= other.max.0
            && self.min.1 <= other.min.1
            && self.max.1 >= other.max.1
    }

    fn subtract(&self, other: &UVRect) -> UVRectSet {
        let us = [self.min.0, self.max.0, other.min.0, other.max.0]
            .into_iter()
            .unique()
            .sorted();
        let vs = [self.min.1, self.max.1, other.min.1, other.max.1]
            .into_iter()
            .unique()
            .sorted();
        iproduct!(us.tuple_windows(), vs.tuple_windows())
            .map(|((u0, u1), (v0, v1))| UVRect {
                min: (u0, v0),
                max: (u1, v1),
            })
            .filter(|r| self.contains(r) && !other.contains(r))
            .collect()
    }
}

fn main() {
    // Test input
    // let input = TEST_INPUT;
    // let y_to_check = 10;
    // let limit_area = 20;

    // Real input
    let y_to_check = 2000000;
    let input = include_str!("../../puzzle_inputs/day_15.txt");
    let limit_area = 4000000;

    let (sensors, beacons, distances) = parse_input(input);
    println!(
        "solve b: {}",
        solve_b_2(&sensors, &beacons, &distances, limit_area),
    );

    // let sensor = (11, 12);
    // let dist = 41;
    // let rect = UVRect::from_pt_and_dist(sensor, dist);
    // let range_x = (sensor.0 - dist - 1)..=(sensor.0 + dist + 1);
    // let range_y = (sensor.1 - dist - 1)..=(sensor.1 + dist + 1);
    // let pts_1 = iproduct!(range_x.clone(), range_y.clone())
    //     .filter(|&pt| manhattan_dist(pt, sensor) <= dist)
    //     .collect::<HashSet<_>>();
    // let pts_2 = iproduct!(range_x, range_y)
    //     .filter(|&pt| rect.contains_xy(pt))
    //     .collect::<HashSet<_>>();
    // println!("pts_1: {:?}", pts_1);
    // println!("pts_2: {:?}", pts_2);
    // assert_eq!(pts_1, pts_2);
    // println!("They are equal.");
}

fn to_uv((x, y): Pt) -> (i64, i64) {
    (x + y, x - y)
}

fn to_xy((u, v): Pt) -> Pt {
    ((u + v) / 2, (u - v) / 2)
}

fn solve_b_2(sensors: &[Pt], _beacons: &[Pt], distances: &[i64], limit_area: i64) -> i64 {
    let limit_center = (limit_area / 2, limit_area / 2);
    let converting_rect = UVRect::from_pt_and_dist(limit_center, limit_area);
    let uv_rects = HashSet::from([converting_rect]);
    let solns = sensors
        .iter()
        .zip(distances.iter())
        .fold(uv_rects, |uv_rects, (sensor, dist)| {
            uv_rects
                .iter()
                .flat_map(|uv_rect| uv_rect.subtract(&UVRect::from_pt_and_dist(*sensor, *dist)))
                .collect()
        })
        .into_iter()
        .filter(|rect| {
            let min_x = to_xy(rect.min).0;
            let max_x = to_xy(rect.max).0;
            let min_y = to_xy((rect.min.0, rect.max.1)).1;
            let max_y = to_xy((rect.max.0, rect.min.1)).1;
            if min_x > limit_area || max_x < 0 || min_y > limit_area || max_y < 0 {
                return false;
            }
            true
        })
        .flat_map(|rect| {
            let min_x = to_xy(rect.min).0;
            let max_x = to_xy(rect.max).0;
            let min_y = to_xy((rect.min.0, rect.max.1)).1;
            let max_y = to_xy((rect.max.0, rect.min.1)).1;
            iproduct!(min_x..=max_x, min_y..=max_y).filter(move |&(x, y)| {
                rect.contains_xy((x, y)) && 0 <= x && x <= limit_area && 0 <= y && y <= limit_area
            })
        })
        .collect_vec();

    let soln = solns.first().unwrap();
    println!("soln: {:?}", soln);
    soln.0 * 4000000 + soln.1
}

/// Returns parallel vectors of sensors, beacons, and the distances between them
fn parse_input(input: &str) -> (Vec<Pt>, Vec<Pt>, Vec<i64>) {
    let pt = r"x=(-?\d+), y=(-?\d+)".to_owned();
    let re = format!("Sensor at {}: closest beacon is at {}", pt, pt);
    let re = Regex::new(&re).unwrap();
    let (sensors, beacons): (Vec<Pt>, Vec<Pt>) = parse_lines(re, input)
        .map(|(sensor_x, sensor_y, beacon_x, beacon_y)| {
            ((sensor_x, sensor_y), (beacon_x, beacon_y))
        })
        .unzip();

    let distances = sensors
        .iter()
        .zip(&beacons)
        .map(|(s, b)| manhattan_dist(*s, *b))
        .collect_vec();

    (sensors, beacons, distances)
    // let (sensors, beacons): (HashSet<Pt>, HashSet<Pt>) = sensor_and_beacons.into_iter().unzip();

    // let sensor_dists: Vec<(Pt, i64)> = sensor_and_beacons
    //     .iter()
    //     .map(|(sensor, beacon)| (*sensor, manhattan_dist(*sensor, *beacon)))
    //     .collect();

    // println!("Part 1: {}", solve_a_1(&beacons, &sensor_dists, y_to_check));
    // println!(
    //     "Part 2: {}",
    //     solve_b_1(&sensors, &beacons, &sensor_dists, limit_area)
    // );
}

// fn solve_a_1(beacons: &HashSet<Pt>, sensor_dists: &[(Pt, i64)], y_to_check: i64) -> usize {
//     let min_x = sensor_dists
//         .iter()
//         .map(|((x, _), dist)| x - dist)
//         .min()
//         .unwrap();
//     let max_x = sensor_dists
//         .iter()
//         .map(|((x, _), dist)| x + dist)
//         .max()
//         .unwrap();
//     println!("min_x: {}, max_x: {}", min_x, max_x);
//     (min_x..=max_x)
//         .filter_map(|x| {
//             let pt = (x, y_to_check);
//             if beacons.contains(&pt) {
//                 return None;
//             }
//             for (sensor, dist) in sensor_dists.iter() {
//                 if manhattan_dist(*sensor, pt) <= *dist {
//                     return Some(());
//                 }
//             }
//             None
//         })
//         .count()
// }

// // println!("Answer: {}", answer);

// fn solve_b_1(
//     sensors: &HashSet<Pt>,
//     beacons: &HashSet<Pt>,
//     sensor_dists: &[(Pt, i64)],
//     limit_area: i64,
// ) -> i64 {
//     let solns = iproduct!(0..=limit_area, 0..=limit_area)
//         .filter_map(|pt| {
//             if sensors.contains(&pt) || beacons.contains(&pt) {
//                 None
//             } else {
//                 sensor_dists
//                     .iter()
//                     .all(|(snsr, dist)| manhattan_dist(*snsr, pt) > *dist)
//                     .then_some(pt)
//             }
//         })
//         .collect_vec();
//     let (x, y) = solns.first().unwrap();
//     x * 4000000 + y
// }

// // println!("Day 14a: {} (578)", solve(grid.clone(), true));
// // println!("Day 14b: {} (24377)", solve(grid, false));
