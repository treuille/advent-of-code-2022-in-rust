use aoc::parse_regex::parse_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../puzzle_inputs/day_16.txt");
    let puzzle = Puzzle::from_str(input);

    println!(
        "day 15a: {} (1638)",
        puzzle.solve(StateA::new(), &None).unwrap().score
    );

    println!(
        "day 15b: {} (2400)",
        puzzle.solve(StateB::new(), &None).unwrap().score
    );
}

type StaticStr = &'static str;

struct Puzzle {
    flow_rates: HashMap<StaticStr, usize>,
    shortest_paths: HashMap<(StaticStr, StaticStr), usize>,
}

impl Puzzle {
    fn from_str(input: StaticStr) -> Self {
        // Parse out the flow rates and tunnels
        let flow_rates: HashMap<StaticStr, usize>;
        let tunnels: HashMap<StaticStr, Vec<StaticStr>>;
        let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();
        (flow_rates, tunnels) = parse_lines(re, input.trim())
            .map(|(name, flow_rate, tunnels): (&str, usize, &str)| {
                let tunnels = tunnels.split(", ").collect();
                ((name, flow_rate), (name, tunnels))
            })
            .unzip();

        Puzzle {
            flow_rates,
            shortest_paths: Puzzle::shortest_paths(&tunnels),
        }
    }

    /// Compute the all-pairs shortest paths among the valves.
    fn shortest_paths(
        tunnels: &HashMap<StaticStr, Vec<StaticStr>>,
    ) -> HashMap<(StaticStr, StaticStr), usize> {
        let valves = tunnels.keys().cloned().collect_vec();

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
            best_state = State::max(next_best_state, best_state);
        }
        best_state
    }
}

trait State: Clone {
    /// Returns vector of all possible next states.
    fn next_states(&self, puzzle: &Puzzle) -> Vec<Self>
    where
        Self: Sized;

    /// What is the current score accrued at this state.
    fn score(&self) -> usize;

    /// An overestimate of the score that can be achieved from this state.
    fn best_potential_score(&self, puzzle: &Puzzle) -> usize;

    /// Finds the larger state by score.
    fn max(s1: Option<Self>, s2: Option<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        match (s1, s2) {
            (Some(s1), Some(s2)) if s1.score() > s2.score() => Some(s1),
            (Some(_), Some(s2)) => Some(s2),
            (Some(s1), None) => Some(s1),
            (None, Some(s2)) => Some(s2),
            (None, None) => None,
        }
    }
}

#[derive(Debug, Clone)]
struct StateA {
    minute: usize,
    valve: StaticStr,
    score: usize,
    open: HashSet<StaticStr>,
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

    fn jump_to_and_open(&self, next_valve: StaticStr, puzzle: &Puzzle) -> StateA {
        assert!(self.valve != next_valve, "Cannot jump to the same valve");
        assert!(!self.open.contains(next_valve), "Cannot open valve twice");
        let arrival_time = self.minute + puzzle.shortest_paths[&(self.valve, next_valve)];
        StateA {
            minute: arrival_time + 1,
            valve: next_valve,
            score: self.score + puzzle.flow_rates[next_valve] * (30 - arrival_time),
            open: self.open.iter().copied().chain([next_valve]).collect(),
        }
    }
}

impl State for StateA {
    /// Returns vector of all possible next states.
    fn next_states(&self, puzzle: &Puzzle) -> Vec<Self>
    where
        Self: Sized,
    {
        puzzle
            .flow_rates
            .keys()
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
                .flow_rates
                .keys()
                .filter(|&&v| !self.open.contains(v))
                .map(|&v| puzzle.flow_rates[v] * (30 - self.minute))
                .sum::<usize>()
    }
}

#[derive(Debug, Clone)]
struct StateB {
    minute: [usize; 2],
    valve: [StaticStr; 2],
    score: usize,
    open: HashSet<StaticStr>,
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

    fn jump_to_and_open(&self, player: usize, next_valve: StaticStr, puzzle: &Puzzle) -> StateB {
        assert!(
            self.valve[player] != next_valve,
            "Cannot jump to the same valve"
        );
        assert!(!self.open.contains(next_valve), "Cannot open valve twice");
        let arrival_time =
            self.minute[player] + puzzle.shortest_paths[&(self.valve[player], next_valve)];
        let (mut minute, mut valve) = (self.minute, self.valve);
        minute[player] = arrival_time + 1;
        valve[player] = next_valve;
        let mut open = self.open.clone();
        open.insert(next_valve);
        StateB {
            minute,
            valve,
            score: self.score + puzzle.flow_rates[next_valve] * (26 - arrival_time),
            open,
        }
    }
}

impl State for StateB {
    /// Returns vector of all possible next states.
    fn next_states(&self, puzzle: &Puzzle) -> Vec<Self>
    where
        Self: Sized,
    {
        // let player = match self.minute[0] < self.minute[1] {
        //     true => 0,
        //     false => 1,
        // };
        let player = (self.minute[0] < self.minute[1]).then_some(0).unwrap_or(1);

        puzzle
            .flow_rates
            .keys()
            .filter(|&&v| {
                (v != self.valve[player])         // Don't move back to your own valve.
                    && (!self.open.contains(v))   // Move only to closed valves.
                    && (puzzle.flow_rates[v] > 0) // Don't bother opening valves with no flow
                    && (v != "KZ" || player == 0) // Distinguish between players 0 and 1
            })
            .map(|&v| self.jump_to_and_open(player, v, puzzle))
            .filter(|s| s.minute[player] <= 26)
            .collect_vec()
    }

    /// What is the current score accrued at this state.
    fn score(&self) -> usize {
        self.score
    }

    /// An overestimate of the score that can be achieved from this state.
    fn best_potential_score(&self, puzzle: &Puzzle) -> usize {
        let min_minute = self.minute[0].min(self.minute[1]);
        self.score
            + puzzle
                .flow_rates
                .keys()
                .filter(|&&v| !self.open.contains(v))
                .map(|&v| puzzle.flow_rates[v] * (26 - min_minute))
                .sum::<usize>()
    }
}
