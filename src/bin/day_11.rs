use std::fmt::Debug;

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test_modulus: u32,
    true_monkey: u32,
    false_monkey: u32,
}

#[derive(Debug)]
enum Operation {
    Squared,
    Times(u32),
    Plus(u32),
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
            test_modulus: skip(lines[3], "divisible by ").parse().unwrap(),
            true_monkey: skip(lines[4], "monkey ").parse().unwrap(),
            false_monkey: skip(lines[5], "monkey ").parse().unwrap(),
        }
    }
}

fn main() {
    // let input = include_str!("../../puzzle_inputs/day_10.txt");
    let monkeys: Vec<Monkey> = TEST_INPUT
        .trim()
        .split("\n\n")
        .map(Monkey::from_str)
        .collect();
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {}: {:?}\n", i, monkey);
    }
    // println!("Hello day 11");
}

const TEST_INPUT: &str = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
