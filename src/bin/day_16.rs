use aoc::parse_regex::parse_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

/// Basic idea is to do a depth-first exuastive search through the state space. To make this search
/// feasible, we use the .best_potential_score() method to prune paths which cannot possibly beat
/// the best solution found so far. This lets us solve the problem in <300ms even without
/// memoization. Parts A and B are solved identically but for the state definition: StateA tracks
/// one individual through the tunnels, while StateB tracks two simultaneously.
fn main() {
    let input = include_str!("../../puzzle_inputs/day_16.txt");
    let mut puzzle = Puzzle::from_str(input);

    println!("day 16a: {} (1638)", puzzle.solve(StateA::new(), 0));

    puzzle.total_minutes = 26;
    println!("day 16b: {} (2400)", puzzle.solve(StateB::new(), 0));
}

type Valve = &'static str;

struct Puzzle {
    total_minutes: usize,
    flow_rates: HashMap<Valve, usize>,
    shortest_paths: HashMap<(Valve, Valve), usize>,
}

impl Puzzle {
    fn from_str(input: &'static str) -> Self {
        // Parse out the flow rates and tunnels
        let flow_rates: HashMap<Valve, usize>;
        let tunnels: HashMap<Valve, Vec<Valve>>;
        let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();
        (flow_rates, tunnels) = parse_lines(re, input.trim())
            .map(|(name, flow_rate, tunnels): (&str, usize, &str)| {
                let tunnels = tunnels.split(", ").collect();
                ((name, flow_rate), (name, tunnels))
            })
            .unzip();

        // Calculate shortest paths among all valves with breadth-first search
        let mut shortest_paths: HashMap<(Valve, Valve), usize> = HashMap::new();
        for (&valve_1, adjacent_valves) in tunnels.iter() {
            let mut valves_to_process = adjacent_valves.clone();
            for dist in 1.. {
                if valves_to_process.is_empty() {
                    break;
                }
                for &valve_2 in valves_to_process.iter() {
                    shortest_paths.insert((valve_1, valve_2), dist);
                }
                valves_to_process = valves_to_process
                    .into_iter()
                    .flat_map(|valve_2| tunnels.get(valve_2).unwrap())
                    .filter(|&valve_2| !shortest_paths.contains_key(&(valve_1, valve_2)))
                    .copied()
                    .collect();
            }
        }

        Puzzle {
            flow_rates,
            shortest_paths,
            total_minutes: 30,
        }
    }

    /// Returns either the best possible score starting from `state`, or `best_score` if it's better.
    fn solve<S: State>(&self, state: S, mut best_score: usize) -> usize {
        // This if statement is the key to efficient search, pruning paths which cannot beat
        // `best_score`.
        if best_score < state.best_potential_score(self) {
            best_score = best_score.max(state.score());
            for next_state in state.next_states(self) {
                let potentially_better_score = self.solve(next_state, best_score);
                best_score = best_score.max(potentially_better_score);
            }
        }
        best_score
    }
}

trait State: Sized + Clone {
    /// Returns vector of all possible next states.
    fn next_states(&self, puzzle: &Puzzle) -> Vec<Self>;

    /// What is the current score accrued at this state.
    fn score(&self) -> usize;

    /// An overestimate of the score that can be achieved from this state.
    fn best_potential_score(&self, puzzle: &Puzzle) -> usize;
}

/// State for part A: one individual is moving through the tunnels.
#[derive(Clone)]
struct StateA {
    minute: usize,
    valve: Valve,
    score: usize,
    open: HashSet<Valve>,
}

impl StateA {
    fn new() -> Self {
        Self {
            minute: 1,
            valve: "AA",
            score: 0,
            open: HashSet::new(),
        }
    }

    fn move_to_and_open(&self, next_valve: Valve, puzzle: &Puzzle) -> StateA {
        let arrival_time = self.minute + puzzle.shortest_paths[&(self.valve, next_valve)];
        let mut open_including_next_valve = self.open.clone();
        open_including_next_valve.insert(next_valve);
        StateA {
            minute: arrival_time + 1,
            valve: next_valve,
            score: self.score
                + puzzle.flow_rates[next_valve] * (puzzle.total_minutes - arrival_time),
            open: open_including_next_valve,
        }
    }
}

impl State for StateA {
    /// Returns vector of all possible next states.
    fn next_states(&self, puzzle: &Puzzle) -> Vec<Self> {
        puzzle
            .flow_rates
            .iter()
            .filter(|(&v, &flow_rate)| (flow_rate > 0) && !self.open.contains(v))
            .map(|(&v, _)| self.move_to_and_open(v, puzzle))
            .filter(|s| s.minute <= puzzle.total_minutes)
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
                .flow_rates
                .iter()
                .filter(|(&valve, &flow_rate)| !self.open.contains(valve) && flow_rate > 0)
                .filter_map(|(valve, flow_rate)| {
                    let arrival_time = self.minute + puzzle.shortest_paths[&(self.valve, *valve)];
                    (arrival_time < puzzle.total_minutes)
                        .then(|| flow_rate * (puzzle.total_minutes - arrival_time))
                })
                .sum::<usize>()
    }
}

/// State for part B: tracks two individuals, each with thier own time.
#[derive(Clone)]
struct StateB {
    minute: [usize; 2],
    valve: [Valve; 2],
    score: usize,
    open: HashSet<Valve>,
}

impl StateB {
    fn new() -> Self {
        Self {
            minute: [1, 1],
            valve: ["AA", "AA"],
            score: 0,
            open: HashSet::new(),
        }
    }

    /// Projects a this double-state (StateB) onto the earlier of the two states.
    fn extract_earlier_state(&self) -> StateA {
        #[allow(clippy::obfuscated_if_else)]
        let player = (self.minute[0] < self.minute[1]).then_some(0).unwrap_or(1);
        StateA {
            minute: self.minute[player],
            valve: self.valve[player],
            score: self.score,
            open: self.open.clone(),
        }
    }

    /// Overwrites the earlier of the two states with the given state.
    fn overwrite_earlier_state(&self, state: StateA) -> StateB {
        #[allow(clippy::obfuscated_if_else)]
        let player = (self.minute[0] < self.minute[1]).then_some(0).unwrap_or(1);
        let (mut minute, mut valve) = (self.minute, self.valve);
        minute[player] = state.minute;
        valve[player] = state.valve;
        StateB {
            minute,
            valve,
            score: state.score,
            open: state.open,
        }
    }
}

impl State for StateB {
    /// Returns vector of all possible next states.
    fn next_states(&self, puzzle: &Puzzle) -> Vec<Self> {
        self.extract_earlier_state()
            .next_states(puzzle)
            .into_iter()
            .map(|s| self.overwrite_earlier_state(s))
            .collect()
    }

    /// What is the current score accrued at this state.
    fn score(&self) -> usize {
        self.score
    }

    /// An overestimate of the score that can be achieved from this state.
    fn best_potential_score(&self, puzzle: &Puzzle) -> usize {
        self.extract_earlier_state().best_potential_score(puzzle)
    }
}
