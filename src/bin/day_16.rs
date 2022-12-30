use aoc::parse_regex::parse_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::successors;

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

#[allow(unreachable_code)]
fn main() {
    let input = include_str!("../../puzzle_inputs/day_16.txt");
    // let input = TEST_INPUT;
    let puzzle = Puzzle::from_str(input);

    // let answer_b = puzzle.solve_b(State::new());
    // println!("answer_b: {}\n{}\n", answer_b.score, answer_b.path_str());

    // let answer_c = puzzle.solve_c(State::new(), &None);
    // println!("answer_c: {}", answer_c.score);
    // answer_c.print_states();

    let answer_d = puzzle.solve_d(State::new(), &None).expect("No solution");
    println!("answer_d: {}\n{}\n", answer_d.score, answer_d.path_str());
    answer_d.print_states();
}

type StaticStr = &'static str;
type Valves = HashSet<StaticStr>;

#[allow(dead_code)]
struct Puzzle {
    flow_rates: HashMap<StaticStr, usize>,
    tunnels: HashMap<StaticStr, Vec<StaticStr>>,
    valves: Valves,
    shortest_paths: HashMap<(StaticStr, StaticStr), usize>,
}

impl Puzzle {
    fn from_str(input: StaticStr) -> Self {
        // Parse the input.
        let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();

        let flow_rates: HashMap<StaticStr, usize>;
        let tunnels: HashMap<StaticStr, Vec<StaticStr>>;
        (flow_rates, tunnels) = parse_lines(re, input.trim())
            .map(|(name, flow_rate, tunnels): (&str, usize, &str)| {
                let xyz = tunnels.split(", ").collect();
                ((name, flow_rate), (name, xyz))
            })
            .unzip();

        let valves: Valves = flow_rates.keys().cloned().collect();
        let shortest_paths = Puzzle::shortest_paths(valves.iter().copied().collect_vec(), &tunnels);
        Puzzle {
            flow_rates,
            tunnels,
            valves,
            shortest_paths,
        }
    }

    fn shortest_paths(
        valves: Vec<StaticStr>,
        tunnels: &HashMap<StaticStr, Vec<StaticStr>>,
    ) -> HashMap<(StaticStr, StaticStr), usize> {
        // Let's put in all single links.
        let mut shortest_paths: HashMap<(StaticStr, StaticStr), usize> = tunnels
            .iter()
            .flat_map(|(valve, tunnels)| {
                tunnels.iter().map(|next_valve| ((*valve, *next_valve), 1))
            })
            .collect();

        for i in 0..valves.len() {
            let valve_1 = valves[i];
            for valve_2 in valves[0..i].iter().copied() {
                // Hook up valve_1 to everything, even indirectly
                if !shortest_paths.contains_key(&(valve_1, valve_2)) {
                    let min_path = valves[0..i]
                        .iter()
                        .copied()
                        .filter_map(|valve_3| {
                            shortest_paths.get(&(valve_1, valve_3)).and_then(|&dist_1| {
                                shortest_paths
                                    .get(&(valve_3, valve_2))
                                    .map(|&dist_2| dist_1 + dist_2)
                            })
                        })
                        .min();
                    if let Some(min_path) = min_path {
                        shortest_paths.insert((valve_1, valve_2), min_path);
                        shortest_paths.insert((valve_2, valve_1), min_path);
                    }
                }

                // Check to see if there are any other shorter paths to be found.
                for valve_3 in valves[0..i].iter().copied() {
                    if let Some(&dist_1) = shortest_paths.get(&(valve_2, valve_1)) {
                        if let Some(&dist_2) = shortest_paths.get(&(valve_1, valve_3)) {
                            let dist_3 = dist_1 + dist_2;
                            if let Some(&dist_4) = shortest_paths.get(&(valve_2, valve_3)) {
                                if dist_3 < dist_4 {
                                    shortest_paths.insert((valve_2, valve_3), dist_3);
                                    shortest_paths.insert((valve_3, valve_2), dist_3);
                                }
                            } else {
                                shortest_paths.insert((valve_2, valve_3), dist_3);
                                shortest_paths.insert((valve_3, valve_2), dist_3);
                            }
                        }
                    }
                }
            }
        }
        shortest_paths
    }

    /// Finds the best final state.
    fn solve_b(&self, state: State) -> State {
        if state.minute > BEST_MOVES.len() {
            return state;
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

    /// Returns the best possible flow achievable from `valve` starting at `minute`,
    /// assuming we've already scored a flow of `score`.
    fn solve_c(&self, state: State, best_state: &Option<State>) -> State {
        assert!(
            state.minute <= 30,
            "Cannot run for to minute {}",
            state.minute
        );
        if state.minute == 30 {
            // println!("Min: {} state.score: {}", state.minute, state.score);
            return state;
        }

        // First, check if it's even possible to beat the best score.
        let closed: Valves = HashSet::from_iter(self.valves.difference(&state.open).copied());
        if let Some(best_state) = best_state {
            let closed_flow: usize = closed.iter().map(|v| self.flow_rates[v]).sum();
            let max_possible_remaining_score = closed_flow * (30 - state.minute);
            let best_potential_score = state.score + max_possible_remaining_score;
            if best_potential_score < best_state.score {
                return best_state.clone();
            }
        }

        let next_states = closed
            .contains(state.valve)
            .then(|| self.open_valve(&state))
            .into_iter()
            .chain(
                self.tunnels[state.valve]
                    .iter()
                    .map(|&next_valve| self.move_to(&state, next_valve)),
            );

        let mut best_state = best_state.clone();
        for next_state in next_states {
            let next_best_state = self.solve_c(next_state, &best_state);
            if let Some(prev_best_state) = best_state.clone() {
                if next_best_state.score > prev_best_state.score {
                    best_state = Some(next_best_state);
                }
            } else {
                best_state = Some(next_best_state);
            }
        }
        best_state.unwrap()
    }

    /// Returns the best possible flow achievable from `valve` starting at `minute`,
    /// assuming we've already scored a flow of `score`.
    fn solve_d(&self, state: State, best_state: &Option<State>) -> Option<State> {
        #[allow(clippy::comparison_chain)]
        if state.minute == 30 {
            // println!("Min: {} state.score: {}", state.minute, state.score);
            return Some(state);
        } else if state.minute > 30 {
            return state.previous_state.map(|s| *s);
        }

        // First, check if it's even possible to beat the best score.
        // let closed = self.valves.difference(&state.open);
        let next_states = self
            .valves
            .iter()
            .filter(|&&v| {
                (v != state.valve) && (!state.open.contains(v)) && (self.flow_rates[v] > 0)
            })
            .map(|&v| self.jump_to_and_open(&state, v));

        // for next_state in next_states {
        //     println!("considering {:?}\n", next_state);
        // }
        // todo!("Stopping here");

        let max_state = |s1: Option<State>, s2: Option<State>| match (s1, s2) {
            (Some(s1), Some(s2)) => {
                if s1.score > s2.score {
                    Some(s1)
                } else {
                    Some(s2)
                }
            }
            (Some(s1), None) => Some(s1),
            (None, Some(s2)) => Some(s2),
            (None, None) => None,
        };

        let mut best_state = max_state(Some(state.clone()), best_state.clone());
        for next_state in next_states {
            let next_best_state = self.solve_d(next_state.clone(), &best_state);
            // println!(
            //     "considering {:?} score: {}->{} (best: {})",
            //     next_state.path_str(),
            //     next_state.score,
            //     next_best_state
            //         .as_ref()
            //         .map(|s| s.score.to_string())
            //         .unwrap_or_else(|| "None".to_owned()),
            //     best_state
            //         .as_ref()
            //         .map(|s| s.score.to_string())
            //         .unwrap_or_else(|| "None".to_owned())
            // );
            best_state = max_state(next_best_state, best_state);
        }
        best_state
    }

    fn open_valve(&self, state: &State) -> State {
        assert!(
            !state.open.contains(state.valve),
            "Cannot open valve \"{}\" twice.",
            state.valve
        );
        State {
            minute: state.minute + 1,
            valve: state.valve,
            score: state.score + self.flow_rates[state.valve] * (30 - state.minute),
            open: state.open.iter().copied().chain([state.valve]).collect(),
            previous_state: Some(Box::new((*state).clone())),
        }
    }

    fn move_to(&self, state: &State, next_valve: StaticStr) -> State {
        State {
            minute: state.minute + 1,
            valve: next_valve,
            score: state.score,
            open: state.open.clone(),
            previous_state: Some(Box::new((*state).clone())),
        }
    }

    fn jump_to_and_open(&self, state: &State, next_valve: StaticStr) -> State {
        assert!(state.valve != next_valve, "Cannot jump to the same valve");
        assert!(!state.open.contains(next_valve), "Cannot open valve twice");
        let arrival_time = state.minute + self.shortest_paths[&(state.valve, next_valve)];
        State {
            minute: arrival_time + 1,
            valve: next_valve,
            score: state.score + self.flow_rates[next_valve] * (30 - arrival_time),
            open: state.open.iter().copied().chain([next_valve]).collect(),
            previous_state: Some(Box::new((*state).clone())),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    minute: usize,
    valve: StaticStr,
    score: usize,
    open: Valves,
    previous_state: Option<Box<State>>,
}

impl State {
    fn new() -> Self {
        Self {
            minute: 1,
            valve: "AA",
            score: 0,
            open: HashSet::new(),
            previous_state: None,
        }
    }

    fn print_states(&self) {
        let previous_states = successors(Some(self), |s| s.previous_state.as_deref()).collect_vec();
        for state in previous_states.iter().rev() {
            println!(
                "Min: {} valve: {} score: {} opened: {:?}",
                state.minute,
                state.valve,
                state.score,
                state.open.len()
            );
        }
    }

    fn path_str(&self) -> String {
        let previous_states = successors(Some(self), |s| s.previous_state.as_deref()).collect_vec();
        previous_states
            .into_iter()
            .rev()
            .map(|s| s.valve)
            .join("->")
    }
}

enum Move {
    MoveTo(StaticStr),
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
//     valve: StaticStr,
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

// let trace: [(Vec<StaticStr>, usize); 30] = [
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
