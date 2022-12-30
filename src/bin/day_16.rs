use aoc::parse_regex::parse_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
// use std::iter::successors;

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

    // let answer_d = puzzle.solve(StateA::new(), &None).expect("No solution");
    // println!("answer_d: {}\n{}\n", answer_d.score, answer_d.path_str());
    // answer_d.print_states();

    let answer_d = puzzle.solve(StateB::new(), &None).expect("No solution");
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

    // /// Finds the best final state.
    // fn solve_b(&self, state: StateA) -> StateA {
    //     if state.minute > BEST_MOVES.len() {
    //         return state;
    //     }
    //     match BEST_MOVES[state.minute - 1] {
    //         Move::Open => {
    //             println!(
    //                 "Min {}: Opening valve {} with flow rate {} for {} minutes",
    //                 state.minute,
    //                 state.valve,
    //                 self.flow_rates[state.valve],
    //                 30 - state.minute
    //             );
    //             self.solve_b(self.open_valve(&state))
    //         }
    //         Move::MoveTo(next_valve) => self.solve_b(self.move_to(&state, next_valve)),
    //     }
    // }

    /// Returns the best possible flow achievable from `valve` starting at `minute`,
    /// assuming we've already scored a flow of `score`.
    fn solve<S: State>(&self, state: S, best_state: &Option<S>) -> Option<S> {
        if let Some(best_state) = best_state {
            if best_state.score() > state.best_potential_score(self) {
                return Some(best_state.clone());
            }
        }

        let mut best_state = State::max(Some(state.clone()), best_state.clone());
        for next_state in state.next_states(self) {
            let next_best_state = self.solve(next_state.clone(), &best_state);
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
            best_state = State::max(next_best_state, best_state);
        }
        best_state
    }

    // fn open_valve(&self, state: &StateA) -> StateA {
    //     assert!(
    //         !state.open.contains(state.valve),
    //         "Cannot open valve \"{}\" twice.",
    //         state.valve
    //     );
    //     StateA {
    //         minute: state.minute + 1,
    //         valve: state.valve,
    //         score: state.score + self.flow_rates[state.valve] * (30 - state.minute),
    //         open: state.open.iter().copied().chain([state.valve]).collect(),
    //         previous_state: Some(Box::new((*state).clone())),
    //     }
    // }

    // fn move_to(&self, state: &StateA, next_valve: StaticStr) -> StateA {
    //     StateA {
    //         minute: state.minute + 1,
    //         valve: next_valve,
    //         score: state.score,
    //         open: state.open.clone(),
    //         previous_state: Some(Box::new((*state).clone())),
    //     }
    // }
}

trait State: Clone {
    /// Returns the state preceding this one in the history.
    fn previous_state(&self) -> Option<&Self>;

    /// Returns vector of all possible next states.
    fn next_states(&self, puzzle: &Puzzle) -> Vec<Self>
    where
        Self: Sized;

    /// What is the current score accrued at this state.
    fn score(&self) -> usize;

    /// An overestimate of the score that can be achieved from this state.
    fn best_potential_score(&self, puzzle: &Puzzle) -> usize;

    /// A longer representation of the state.
    fn to_str(&self) -> String;

    /// A shorter representation of the state.
    fn to_short_str(&self) -> String;

    fn max(s1: Option<Self>, s2: Option<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        match (s1, s2) {
            (Some(s1), Some(s2)) if s1.score() > s2.score() => Some(s1),
            (Some(_s1), Some(s2)) => Some(s2),
            (Some(s1), None) => Some(s1),
            (None, Some(s2)) => Some(s2),
            (None, None) => None,
        }
    }

    /// Print out a history of the state.
    fn print_states(&self) {
        let mut state = self;
        let mut history = vec![self.to_str()];
        while let Some(prev_state) = state.previous_state() {
            history.push(prev_state.to_str());
            state = prev_state;
        }
        history.into_iter().rev().for_each(|s| println!("{}", s));
    }

    /// A history as a short string.
    fn path_str(&self) -> String {
        let mut state = self;
        let mut history = vec![self.to_str()];
        while let Some(prev_state) = state.previous_state() {
            history.push(prev_state.to_short_str());
            state = prev_state;
        }
        history.into_iter().rev().join("->")
    }
}

#[derive(Debug, Clone)]
struct StateA {
    minute: usize,
    valve: StaticStr,
    score: usize,
    open: Valves,
    previous_state: Option<Box<StateA>>,
}

impl StateA {
    fn new() -> Self {
        Self {
            minute: 1,
            valve: "AA",
            score: 0,
            open: HashSet::new(),
            previous_state: None,
        }
    }

    fn jump_to_and_open(&self, next_valve: StaticStr, puzzle: &Puzzle) -> StateA {
        assert!(self.valve != next_valve, "Cannot jump to the same valve");
        assert!(!self.open.contains(next_valve), "Cannot open valve twice");
        let arrival_time = self.minute + puzzle.shortest_paths[&(self.valve, next_valve)];
        StateA {
            minute: arrival_time + 1,
            valve: next_valve,
            score: self.score + puzzle.flow_rates[next_valve] * (30 - arrival_time),
            open: self.open.iter().copied().chain([next_valve]).collect(),
            previous_state: Some(Box::new((*self).clone())),
        }
    }
}

impl State for StateA {
    /// Returns the state preceding this one in the history.
    fn previous_state(&self) -> Option<&Self> {
        self.previous_state.as_deref()
    }

    /// Returns vector of all possible next states.
    fn next_states(&self, puzzle: &Puzzle) -> Vec<Self>
    where
        Self: Sized,
    {
        puzzle
            .valves
            .iter()
            .filter(|&&v| {
                (v != self.valve) && (!self.open.contains(v)) && (puzzle.flow_rates[v] > 0)
            })
            .map(|&v| self.jump_to_and_open(v, puzzle))
            .filter(|s| s.minute <= 30)
            .collect_vec()
    }

    /// What is the current score accrued at this state.
    fn score(&self) -> usize {
        self.score
    }

    /// An overestimate of the score that can be achieved from this state.
    fn best_potential_score(&self, puzzle: &Puzzle) -> usize {
        self.score
            + puzzle
                .valves
                .iter()
                .filter(|&&v| !self.open.contains(v))
                .map(|&v| puzzle.flow_rates[v] * (30 - self.minute))
                .sum::<usize>()
    }

    /// A longer representation of the state.
    fn to_str(&self) -> String {
        format!(
            "Min: {} valve: {} score: {} opened: {:?}",
            self.minute,
            self.valve,
            self.score,
            self.open.len()
        )
    }

    /// A shorter representation of the state.
    fn to_short_str(&self) -> String {
        self.valve.to_string()
    }
}

#[derive(Debug, Clone)]
struct StateB {
    minute: [usize; 2],
    valve: [StaticStr; 2],
    score: usize,
    open: Valves,
    previous_state: Option<Box<StateB>>,
}

impl StateB {
    fn new() -> Self {
        Self {
            minute: [1, 1],
            valve: ["AA", "AA"],
            score: 0,
            open: HashSet::new(),
            previous_state: None,
        }
    }

    fn jump_to_and_open(&self, player: usize, next_valve: StaticStr, puzzle: &Puzzle) -> StateB {
        assert!(
            self.valve[player] != next_valve,
            "Cannot jump to the same valve"
        );
        assert!(!self.open.contains(next_valve), "Cannot open valve twice");
        let arrival_time =
            self.minute[player] + puzzle.shortest_paths[&(self.valve[player], next_valve)];
        let mut minute = self.minute.clone();
        let mut valve = self.valve.clone();
        minute[player] = arrival_time + 1;
        valve[player] = next_valve;
        StateB {
            minute,
            valve,
            score: self.score + puzzle.flow_rates[next_valve] * (26 - arrival_time),
            open: self.open.iter().copied().chain([next_valve]).collect(),
            previous_state: Some(Box::new((*self).clone())),
        }
    }
}

impl State for StateB {
    /// Returns the state preceding this one in the history.
    fn previous_state(&self) -> Option<&Self> {
        self.previous_state.as_deref()
    }

    /// Returns vector of all possible next states.
    fn next_states(&self, puzzle: &Puzzle) -> Vec<Self>
    where
        Self: Sized,
    {
        let player = if self.minute[0] < self.minute[1] {
            0
        } else {
            1
        };
        // TODO: I should take the outer product with both player
        puzzle
            .valves
            .iter()
            .filter(|&&v| {
                (v != self.valve[player]) && (!self.open.contains(v)) && (puzzle.flow_rates[v] > 0)
            })
            .map(|&v| self.jump_to_and_open(player, v, puzzle))
            .filter(|s| s.minute[player] <= 26)
            .collect_vec()
    }

    /// What is the current score accrued at this state.
    fn score(&self) -> usize {
        self.score
    }

    #[allow(unused_variables)]
    /// An overestimate of the score that can be achieved from this state.
    fn best_potential_score(&self, puzzle: &Puzzle) -> usize {
        let min_minute = self.minute[0].min(self.minute[1]);
        self.score
            + puzzle
                .valves
                .iter()
                .filter(|&&v| !self.open.contains(v))
                .map(|&v| puzzle.flow_rates[v] * (26 - min_minute))
                .sum::<usize>()
    }

    /// A longer representation of the state.
    fn to_str(&self) -> String {
        format!(
            "mins: {:?} valves: {:?} score: {} opened: {:?}",
            self.minute,
            self.valve,
            self.score,
            self.open.len()
        )
    }

    /// A shorter representation of the state.
    fn to_short_str(&self) -> String {
        self.valve.iter().join(",")
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
