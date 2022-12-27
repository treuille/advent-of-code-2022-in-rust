use aoc::parse_regex::parse_lines;
use itertools::{iproduct, Itertools};
use regex::Regex;

fn main() {
    // Real input
    let input = include_str!("../../puzzle_inputs/day_15.txt");
    let (sensor_balls, beacons) = parse_input(input);

    println!(
        "solve a: {} (5878678)",
        solve_a(&sensor_balls, &beacons, 2000000)
    );
    println!(
        "solve b: {} (11796491041245)",
        solve_b(&sensor_balls, 4000000)
    );
}

/// Returns two parallel vectors sensor balls and beacons.
fn parse_input(input: &str) -> (UVRects, Vec<Pt>) {
    let pt = r"x=(-?\d+), y=(-?\d+)".to_owned();
    let re = format!("Sensor at {}: closest beacon is at {}", pt, pt);
    let re = Regex::new(&re).unwrap();
    parse_lines(re, input)
        .map(|(sensor_x, sensor_y, beacon_x, beacon_y)| {
            let (sensor, beacon) = ((sensor_x, sensor_y), (beacon_x, beacon_y));
            let dist = manhattan_dist(beacon, sensor);
            let sensor_ball = UVRect::from_pt_and_dist(sensor, dist);
            (sensor_ball, beacon)
        })
        .unzip()
}

fn solve_a(sensor_balls: &UVRects, beacons: &[Pt], y_to_check: i64) -> usize {
    let xs = sensor_balls
        .iter()
        .flat_map(|rect| [to_xy(rect.min).0, to_xy(rect.max).0])
        .collect_vec();
    let min_x = xs.iter().min().unwrap();
    let max_x = xs.iter().max().unwrap();
    (*min_x..=*max_x)
        .filter(|x| {
            let pt = (*x, y_to_check);
            if beacons.contains(&pt) {
                return false;
            }
            sensor_balls.iter().any(|ball| ball.contains_xy(pt))
        })
        .count()
}

fn solve_b(sensor_balls: &UVRects, limit_area: i64) -> i64 {
    let limit_center = (limit_area / 2, limit_area / 2);
    let potential_solns = UVRect::from_pt_and_dist(limit_center, limit_area);
    let (soln_x, soln_y) = sensor_balls
        .iter()
        .fold(vec![potential_solns], |uv_rects, sensor_ball| {
            uv_rects
                .iter()
                .flat_map(|uv_rect| uv_rect.subtract(sensor_ball))
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
        .next()
        .unwrap();

    soln_x * 4000000 + soln_y
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
type UVRects = Vec<UVRect>;

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
    fn subtract(&self, other: &UVRect) -> UVRects {
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
