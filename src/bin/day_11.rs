use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    modulus: u64,
    true_recipient: usize,
    false_recipient: usize,
    inspected: usize,
}

#[derive(Clone)]
enum Operation {
    Squared,
    Times(u64),
    Plus(u64),
}

impl Monkey {
    fn from_str(s: &str) -> Self {
        let skip = |s: &str, prefix: &str| s.split_once(prefix).unwrap().1.to_owned();

        let lines: Vec<&str> = s.lines().collect();

        Self {
            items: skip(lines[1], "Starting items: ")
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect(),
            operation: {
                let operation = skip(lines[2], "new = old ");
                let mut tokens = operation.split(' ');
                match (tokens.next().unwrap(), tokens.next().unwrap()) {
                    ("*", "old") => Operation::Squared,
                    ("*", num) => Operation::Times(num.parse().unwrap()),
                    ("+", num) => Operation::Plus(num.parse().unwrap()),
                    operation => panic!("Unkown operation: {:?}", operation),
                }
            },

            modulus: skip(lines[3], "divisible by ").parse().unwrap(),
            true_recipient: skip(lines[4], "monkey ").parse().unwrap(),
            false_recipient: skip(lines[5], "monkey ").parse().unwrap(),
            inspected: 0,
        }
    }
}

fn main() {
    let input = include_str!("../../puzzle_inputs/day_11.txt");
    let monkeys: Vec<Monkey> = input.trim().split("\n\n").map(Monkey::from_str).collect();

    println!("day 11a: {} (316888)", solve(monkeys.clone(), 20, 3));
    println!("day 11a: {} (35270398814)", solve(monkeys, 10000, 1));
}

fn solve(mut monkeys: Vec<Monkey>, n_rounds: usize, worry_cooldown: u64) -> usize {
    let modulus: u64 = monkeys.iter().map(|m| m.modulus).product::<u64>() * worry_cooldown;

    for _ in 0..n_rounds {
        for i in 0..monkeys.len() {
            monkeys[i].inspected += monkeys[i].items.len();
            while let Some(item) = monkeys[i].items.pop_front() {
                let item = match monkeys[i].operation {
                    Operation::Squared => item * item,
                    Operation::Times(num) => item * num,
                    Operation::Plus(num) => item + num,
                };

                let item = (item / worry_cooldown) % modulus;

                let recipient = match item % monkeys[i].modulus {
                    0 => monkeys[i].true_recipient,
                    _ => monkeys[i].false_recipient,
                };
                monkeys[recipient].items.push_back(item);
            }
        }
    }

    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}
