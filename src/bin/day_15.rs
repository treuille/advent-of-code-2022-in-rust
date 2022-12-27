use aoc::parse_regex::parse_lines;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::collections::HashSet;

fn main() {
    // Real input
    let input = include_str!("../../puzzle_inputs/day_15.txt");
    let (sensors, beacons, distances) = parse_input(input);

    println!(
        "solve a: {} (5878678)",
        solve_a(&sensors, &beacons, &distances, 2000000)
    );
    println!(
        "solve b: {} (11796491041245)",
        solve_b(&sensors, &distances, 4000000)
    );
}

fn solve_a(sensors: &[Pt], beacons: &[Pt], distances: &[i64], y_to_check: i64) -> usize {
    let min_x = sensors
        .iter()
        .zip(distances)
        .map(|((x, _), dist)| x - dist)
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .zip(distances)
        .map(|((x, _), dist)| x + dist)
        .max()
        .unwrap();
    (min_x..=max_x)
        .filter_map(|x| {
            let pt = (x, y_to_check);
            if beacons.contains(&pt) {
                return None;
            }
            for (sensor, dist) in sensors.iter().zip(distances) {
                if manhattan_dist(*sensor, pt) <= *dist {
                    return Some(());
                }
            }
            None
        })
        .count()
}

fn solve_b(sensors: &[Pt], distances: &[i64], limit_area: i64) -> i64 {
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
        .filter_map(|rect| {
            let min_x = to_xy(rect.min).0;
            let max_x = to_xy(rect.max).0;
            let min_y = to_xy((rect.min.0, rect.max.1)).1;
            let max_y = to_xy((rect.max.0, rect.min.1)).1;
            (min_x <= limit_area && max_x >= 0 && min_y <= limit_area && max_y >= 0)
                .then_some((rect, min_x, max_x, min_y, max_y))
        })
        .flat_map(|(rect, min_x, max_x, min_y, max_y)| {
            iproduct!(min_x..=max_x, min_y..=max_y).filter(move |&(x, y)| {
                rect.contains_xy((x, y)) && 0 <= x && x <= limit_area && 0 <= y && y <= limit_area
            })
        })
        .collect_vec();

    let (soln_x, soln_y) = solns.first().unwrap();
    soln_x * 4000000 + soln_y
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
}

/// A point in 2D space.
type Pt = (i64, i64);

/// Converts into a coordinate system where manhattan distances are squares.
fn to_uv((x, y): Pt) -> (i64, i64) {
    (x + y, x - y)
}

/// Converts back into the original coordinate system.
fn to_xy((u, v): Pt) -> Pt {
    ((u + v) / 2, (u - v) / 2)
}

/// Compute the L1 norm between two points.
fn manhattan_dist(a: Pt, b: Pt) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

/// A rectungle in UV-space (representing an L1 ball in XY-space).
#[derive(PartialEq, Eq, Hash)]
struct UVRect {
    min: Pt,
    max: Pt,
}

/// A set of (disjoint) UVRects.
type UVRectSet = HashSet<UVRect>;

impl UVRect {
    /// A rectungle representing an L1 ball of radius `dist` centered at `(x,y)`.
    fn from_pt_and_dist((x, y): Pt, dist: i64) -> Self {
        UVRect {
            min: to_uv((x - dist, y)),
            max: to_uv((x + dist + 1, y)),
        }
    }

    /// Does this rectangle containt this point in XY-space.
    fn contains_xy(&self, (x, y): Pt) -> bool {
        let (u, v) = to_uv((x, y));
        self.min.0 <= u && u < self.max.0 && self.min.1 <= v && v < self.max.1
    }

    /// Does this rectangle contain another rectangle.
    fn contains(&self, other: &Self) -> bool {
        self.min.0 <= other.min.0
            && self.max.0 >= other.max.0
            && self.min.1 <= other.min.1
            && self.max.1 >= other.max.1
    }

    /// Subtract another rectangle from this one, returning the disjoint rectangle fragments.
    fn subtract(&self, other: &UVRect) -> UVRectSet {
        let us = [self.min.0, self.max.0, other.min.0, other.max.0];
        let sorted_us = us.into_iter().unique().sorted();
        let vs = [self.min.1, self.max.1, other.min.1, other.max.1];
        let sorted_vs = vs.into_iter().unique().sorted();
        iproduct!(sorted_us.tuple_windows(), sorted_vs.tuple_windows())
            .map(|((u0, u1), (v0, v1))| UVRect {
                min: (u0, v0),
                max: (u1, v1),
            })
            .filter(|r| self.contains(r) && !other.contains(r))
            .collect()
    }
}
