use std::iter;

fn main() {
    let input = include_str!("../../puzzle_inputs/day_10.txt");
    let register_vals = parse_input(input);

    // Solve a
    println!("10a: {} (16020)", solve_a(&register_vals));

    // Solve b
    println!("10b: (ECZUZALR)");
    solve_b(&register_vals);
}

fn solve_a(register_vals: &[i32]) -> i32 {
    register_vals
        .iter()
        .zip(2..)
        .filter_map(|(sum, i)| match i {
            20 | 60 | 100 | 140 | 180 | 220 => Some(i * sum),
            _ => None,
        })
        .sum()
}

fn solve_b(register_vals: &[i32]) {
    // Creat the CRT display.
    const WIDTH: usize = 40;
    const N_PIXELS: usize = WIDTH * 6;
    let mut crt = [false; N_PIXELS];

    // Run the simulation.
    let register_vals = iter::once(&1).chain(register_vals.iter());
    for (cycle, &sprite_pos) in register_vals.enumerate() {
        if (((cycle % WIDTH) as i32) - sprite_pos).abs() <= 1 {
            crt[cycle % N_PIXELS] = true;
        }
    }

    // Print out the CRT.
    let crt = crt.map(|pixel| if pixel { '#' } else { '.' });
    crt.chunks(WIDTH)
        .for_each(|row| println!("{}", row.iter().collect::<String>()));
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .flat_map(|line| {
            let mut tokens = line.split(' ');
            match tokens.next().unwrap() {
                "noop" => vec![0],
                "addx" => vec![0, tokens.next().unwrap().parse().unwrap()],
                instr => panic!("Unexpected instruction: {}", instr),
            }
        })
        .scan(1, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        .collect()
}
