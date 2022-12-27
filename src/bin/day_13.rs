use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

fn main() {
    let input = include_str!("../../puzzle_inputs/day_13.txt");

    println!("day 13a: {} (5852)", solve_a(input));
    println!("day 13b: {} (24190)", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .zip(1..)
        .filter_map(|(pair, i)| {
            let (line_1, line_2) = pair.split_once('\n').unwrap();
            let packet_1 = parse(&mut line_1.chars());
            let packet_2 = parse(&mut line_2.chars());
            (cmp(&packet_1, &packet_2) == Ordering::Less).then_some(i)
        })
        .sum()
}

fn solve_b(input: &str) -> usize {
    let divider_1 = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);
    let eq = |p1: &Packet, p2: &Packet| cmp(p1, p2) == Ordering::Equal;
    let mut packets: Vec<Packet> = input
        .trim()
        .split('\n')
        .filter_map(|line| (!line.is_empty()).then(|| parse(&mut line.chars())))
        .chain([divider_1.clone(), divider_2.clone()])
        .collect();
    packets.sort_by(cmp);
    packets
        .into_iter()
        .zip(1..)
        .filter_map(|(packet, i)| (eq(&packet, &divider_1) || eq(&packet, &divider_2)).then_some(i))
        .product()
}

fn cmp(packet_1: &Packet, packet_2: &Packet) -> Ordering {
    match (packet_1, packet_2) {
        (Packet::Num(num_1), Packet::Num(num_2)) => num_1.cmp(num_2),
        (Packet::List(list_1), Packet::List(list_2)) => {
            for (item_1, item_2) in list_1.iter().zip(list_2) {
                match cmp(item_1, item_2) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => (),
                }
            }
            list_1.len().cmp(&list_2.len())
        }
        (Packet::Num(num), list_packet) => cmp(&Packet::List(vec![Packet::Num(*num)]), list_packet),
        (list_packet, Packet::Num(num)) => cmp(list_packet, &Packet::List(vec![Packet::Num(*num)])),
    }
}

fn parse(input: &mut impl Iterator<Item = char>) -> Packet {
    let mut buffer = String::new();
    let mut list = Vec::new();
    while let Some(c) = input.next() {
        match c {
            '[' => list.push(parse(input)),

            ']' => {
                if let Ok(num) = buffer.drain(..).collect::<String>().parse() {
                    list.push(Packet::Num(num));
                }
                return Packet::List(list);
            }
            ',' => {
                if let Ok(num) = buffer.drain(..).collect::<String>().parse() {
                    list.push(Packet::Num(num));
                }
            }
            _ => buffer.push(c),
        }
    }
    list.pop().unwrap()
}
