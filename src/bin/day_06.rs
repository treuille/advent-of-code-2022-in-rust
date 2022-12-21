use itertools::Itertools;

fn main() {
    let input = include_str!("../../puzzle_inputs/day_06.txt");
    let input: Vec<char> = input.chars().collect();
    println!("day 6a: {} (1235)", solve(&input, 4));
    println!("day 6b: {} (3501)", solve(&input, 14));
}

fn solve(input: &[char], window_len: usize) -> usize {
    let equal = |items: &Vec<&char>| items[0] == items[1];
    for (i, window) in input.windows(window_len).enumerate() {
        if window.iter().combinations(2).filter(equal).count() == 0 {
            return i + window_len;
        }
    }
    unreachable!(
        "Sequence \"{:?}\" has no unique length-{} subsequence.",
        input, window_len
    );
}
