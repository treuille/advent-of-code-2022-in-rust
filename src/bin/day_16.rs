use aoc::parse_regex::parse_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

const TEST_INPUT: &str = "
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

type FlowRates = HashMap<&'static str, usize>;
type Tunnels = HashMap<&'static str, Vec<&'static str>>;

fn main() {
    // let input = include_str!("../../puzzle_inputs/day_15.txt");
    let input = TEST_INPUT.trim();

    // Parse the input.
    let re =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();

    let (flow_rates, tunnels): (FlowRates, Tunnels) = parse_lines(re, input)
        .map(|(name, flow_rate, tunnels): (&str, usize, &str)| {
            let xyz = tunnels.split(", ").collect();
            ((name, flow_rate), (name, xyz))
        })
        .unzip();

    println!("flow_rate: {:?}", flow_rates);
    println!("tunnels: {:?}", tunnels);
}

fn solve(
    valve: &str,
    minute: usize,
    best_score: usize,
    flow_rates: &FlowRates,
    tunnels: &Tunnels,
) -> usize {
    todo!();
}
