use aoc::parse_regex::parse_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

/// Basic idea is to do a depth-first exuastive search through the tunnels. To prune the search, we
/// track the best solution so far, and use .best_potential_score() to prune paths which cannot
/// beat it.
fn main() {
    let input = include_str!("../../puzzle_inputs/day_16.txt");
    let mut puzzle = Puzzle::from_str(input);

    println!(
        "day 16a: {} (1638)",
        puzzle.solve(StateA::new(), &None).unwrap().score
    );

    puzzle.total_minutes = 26;
    println!(
        "day 16b: {} (2400)",
        puzzle.solve(StateB::new(), &None).unwrap().score
    );
}

type Valve = &'static str;

struct Puzzle {
    flow_rates: HashMap<Valve, usize>,
    shortest_paths: HashMap<(Valve, Valve), usize>,
    total_minutes: usize,
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

    /// Returns either the best possible flow starting from `state`, or `best_state` if it's better.
    fn solve<S: State>(&self, state: S, best_state: &Option<S>) -> Option<S> {
        if let Some(best_state) = best_state {
            if best_state.score() > state.best_potential_score(self) {
                return Some(best_state.clone());
            }
        }

        let mut best_state = State::max(Some(state.clone()), best_state.clone());
        for next_state in state.next_states(self) {
            let next_best_state = self.solve(next_state.clone(), &best_state);
            best_state = State::max(next_best_state, best_state);
        }
        best_state
    }
}

trait State: Sized + Clone {
    /// Returns vector of all possible next states.
    fn next_states(&self, puzzle: &Puzzle) -> Vec<Self>;

    /// What is the current score accrued at this state.
    fn score(&self) -> usize;

    /// An overestimate of the score that can be achieved from this state.
    fn best_potential_score(&self, puzzle: &Puzzle) -> usize;

    /// Finds the larger state by score.
    fn max(s1: Option<Self>, s2: Option<Self>) -> Option<Self> {
        match (s1, s2) {
            (Some(s1), Some(s2)) if s1.score() > s2.score() => Some(s1),
            (Some(_), Some(s2)) => Some(s2),
            (Some(s1), None) => Some(s1),
            (None, Some(s2)) => Some(s2),
            (None, None) => None,
        }
    }
}

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
