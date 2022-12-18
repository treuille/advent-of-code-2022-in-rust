fn main() {
    let input = read_input(include_str!("../../puzzle_inputs/day_01.txt"));
    let mut sums: Vec<u64> = input.iter().map(|nums| nums.iter().sum()).collect();

    // Solve problem a.
    println!("day 1a: {}", sums.iter().max().unwrap());

    // Solve problem b.
    sums.sort();
    println!("day 1b: {:?}", sums.iter().rev().take(3).sum::<u64>());
}

fn read_input(input: &str) -> Vec<Vec<u64>> {
    let result: Result<Vec<Vec<u64>>, _> = input
        .trim()
        .split("\n\n")
        .map(|lines| lines.lines().map(|s| s.parse()).collect())
        .collect();
    result.unwrap()
}
