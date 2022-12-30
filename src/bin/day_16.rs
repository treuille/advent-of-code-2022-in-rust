use aoc::parse_regex::parse_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

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

fn main() {
    // let input = include_str!("../../puzzle_inputs/day_15.txt");
    let input = TEST_INPUT.trim();
    let puzzle = Puzzle::from_str(input);

    // println!("flow_rate: {:?}", flow_rates);
    // println!("tunnels: {:?}", tunnels);

    // let answer = solve("AA", 1, &HashSet::new(), 0, 0, &flow_rates, &tunnels);
    let answer = puzzle.solve_b(State::new());
    println!("answer: {}", answer);
}
type Valves = HashSet<&'static str>;

#[allow(dead_code)]
struct Puzzle {
    flow_rates: HashMap<&'static str, usize>,
    tunnels: HashMap<&'static str, Vec<&'static str>>,
    valves: Valves,
}

impl Puzzle {
    fn from_str(input: &'static str) -> Self {
        // Parse the input.
        let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();

        let flow_rates: HashMap<&'static str, usize>;
        let tunnels: HashMap<&'static str, Vec<&'static str>>;
        (flow_rates, tunnels) = parse_lines(re, input.trim())
            .map(|(name, flow_rate, tunnels): (&str, usize, &str)| {
                let xyz = tunnels.split(", ").collect();
                ((name, flow_rate), (name, xyz))
            })
            .unzip();

        let valves = flow_rates.keys().cloned().collect();
        Puzzle {
            flow_rates,
            tunnels,
            valves,
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    /// Returns the best possible flow achievable from `valve` starting at `minute`,
    /// assuming we've already scored a flow of `score`.
    fn solve_b(&self, state: State) -> usize {
        if state.minute > BEST_MOVES.len() {
            return state.score;
        }
        match BEST_MOVES[state.minute - 1] {
            Move::Open => {
                println!(
                    "Min {}: Opening valve {} with flow rate {} for {} minutes",
                    state.minute,
                    state.valve,
                    self.flow_rates[state.valve],
                    30 - state.minute
                );
                self.solve_b(self.open_valve(&state))
            }
            Move::MoveTo(next_valve) => self.solve_b(self.move_to(&state, next_valve)),
        }
    }

    fn open_valve(&self, state: &State) -> State {
        State {
            minute: state.minute + 1,
            valve: state.valve,
            score: state.score + self.flow_rates[state.valve] * (30 - state.minute),
            open: state.open.iter().copied().chain([state.valve]).collect(),
        }
    }

    fn move_to(&self, state: &State, next_valve: &'static str) -> State {
        State {
            minute: state.minute + 1,
            valve: next_valve,
            score: state.score,
            open: state.open.clone(),
        }
    }
}

#[derive(Debug)]
struct State {
    minute: usize,
    valve: &'static str,
    score: usize,
    open: Valves,
}

impl State {
    fn new() -> Self {
        Self {
            minute: 1,
            valve: "AA",
            score: 0,
            open: HashSet::new(),
        }
    }
}

enum Move {
    MoveTo(&'static str),
    Open,
}

const BEST_MOVES: [Move; 24] = [
    Move::MoveTo("DD"), // min 1
    Move::Open,         // min 2
    Move::MoveTo("CC"), // min 3
    Move::MoveTo("BB"), // min 4
    Move::Open,         // min 5
    Move::MoveTo("AA"), // min 6
    Move::MoveTo("II"), // min 7
    Move::MoveTo("JJ"), // min 8
    Move::Open,         // min 9
    Move::MoveTo("II"), // min 10
    Move::MoveTo("AA"), // min 11
    Move::MoveTo("DD"), // min 12
    Move::MoveTo("EE"), // min 13
    Move::MoveTo("FF"), // min 14
    Move::MoveTo("GG"), // min 15
    Move::MoveTo("HH"), // min 16
    Move::Open,         // min 17
    Move::MoveTo("GG"), // min 18
    Move::MoveTo("FF"), // min 19
    Move::MoveTo("EE"), // min 20
    Move::Open,         // min 21
    Move::MoveTo("DD"), // min 22
    Move::MoveTo("CC"), // min 23
    Move::Open,         // min 24
];

// fn solve(
//     valve: &'static str,
//     minute: usize,
//     open: &Valves,
//     score_so_far: usize,
//     mut best_score: usize,
//     tunnels: &Tunnels,
// ) -> usize {
//     println!("{}: {} {:?}",minute, valve, open);

//     // If it's minute 30, then we're out of time.
//     if minute == 30 {
//         return best_score;
//     }

//     // If all valves are open, then we're done.
//     let all_valves: Valves = flow_rates.keys().copied().collect();
//     if all_valves == *open {
//         return best_score;
//     }

//     let closed_valves: Valves = all_valves.difference(open).copied().collect();
//     let closed_flow: usize = closed_valves.iter().map(|v| flow_rates[v]).sum();
//     let best_potential = score_so_far + closed_flow * (30 - minute);
//     if best_potential < best_score {
//         return best_score;
//     }

//     if closed_valves.contains(valve) {
//         let mut next_open = open.clone();
//         next_open.insert(valve);
//         let score = flow_rates[valve] * (30 - minute);
//         let score = score
//             + solve(
//                 valve,
//                 minute + 1,
//                 &next_open,
//                 score_so_far + score,
//                 best_score,
//                 flow_rates,
//                 tunnels,
//             );
//         if score > best_score {
//             best_score = score;
//         }
//     }
//     for &next_valve in tunnels.get(valve).unwrap().iter() {
//         let score = solve(
//             next_valve,
//             minute + 1,
//             open,
//             score_so_far,
//             best_score,
//             flow_rates,
//             tunnels,
//         );
//         if score > best_score {
//             best_score = score;
//         }
//     }
//     best_score
// }

// let trace: [(Vec<&'static str>, usize); 30] = [
//     (vec![], 0),                                    // min 1
//     (vec![], 0),                                    // min 2
//     (vec!["DD"], 20),                               // min 3
//     (vec!["DD"], 20),                               // min 4
//     (vec!["DD"], 20),                               // min 5
//     (vec!["BB", "DD"], 33),                         // min 6
//     (vec!["BB", "DD"], 33),                         // min 7
//     (vec!["BB", "DD"], 33),                         // min 8
//     (vec!["BB", "DD"], 33),                         // min 9
//     (vec!["BB", "DD", "JJ"], 54),                   // min 10
//     (vec!["BB", "DD", "JJ"], 54),                   // min 11
//     (vec!["BB", "DD", "JJ"], 54),                   // min 12
//     (vec!["BB", "DD", "JJ"], 54),                   // min 13
//     (vec!["BB", "DD", "JJ"], 54),                   // min 14
//     (vec!["BB", "DD", "JJ"], 54),                   // min 15
//     (vec!["BB", "DD", "JJ"], 54),                   // min 16
//     (vec!["BB", "DD", "JJ"], 54),                   // min 17
//     (vec!["BB", "DD", "HH", "JJ"], 76),             // min 18
//     (vec!["BB", "DD", "HH", "JJ"], 76),             // min 19
//     (vec!["BB", "DD", "HH", "JJ"], 76),             // min 20
//     (vec!["BB", "DD", "HH", "JJ"], 76),             // min 21
//     (vec!["BB", "DD", "EE", "HH", "JJ"], 79),       // min 22
//     (vec!["BB", "DD", "EE", "HH", "JJ"], 79),       // min 23
//     (vec!["BB", "DD", "EE", "HH", "JJ"], 79),       // min 24
//     (vec!["BB", "CC", "DD", "EE", "HH", "JJ"], 81), // min 25
//     (vec!["BB", "CC", "DD", "EE", "HH", "JJ"], 81), // min 26
//     (vec!["BB", "CC", "DD", "EE", "HH", "JJ"], 81), // min 27
//     (vec!["BB", "CC", "DD", "EE", "HH", "JJ"], 81), // min 28
//     (vec!["BB", "CC", "DD", "EE", "HH", "JJ"], 81), // min 29
//     (vec!["BB", "CC", "DD", "EE", "HH", "JJ"], 81), // min 30
// ];
// println!("Trace: {:?}", trace);
// println!(
//     "Flow sum: {}",
//     trace.iter().map(|(_, flow)| flow).sum::<usize>()
// );
// for (open_valves, flow) in trace.iter() {
//     let flow_2: usize = open_valves
//         .iter()
//         .map(|valve| flow_rates.get(valve).unwrap())
//         .sum();
//     println!("flow: {} flow_2: {}", flow, flow_2);
//     assert_eq!(*flow, flow_2);
//     // println!("{}: {}", open_valves.join(" -> "), flow);
// }

// // Count all the letters
// let mut counts = HashMap::new();
// for (open_valves, _) in trace.iter() {
//     for valve in open_valves.iter() {
//         *counts.entry(valve).or_insert(0) += 1;
//     }
// }
// println!("counts: {:?}", counts);
