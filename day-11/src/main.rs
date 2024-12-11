use std::collections::BTreeMap;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input)
        .expect("file exists")
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let part1 = iterate(&input, 25);
    println!("Total (part 1): {part1}");

    let part2 = iterate(&input, 75);
    println!("Total (part 2): {part2}");
}

enum NewStones {
    One(usize),
    Two(usize, usize),
}

fn iterate(initial: &[usize], iterations: usize) -> usize {
    let mut counts: BTreeMap<usize, usize> = initial.iter().map(|i| (*i, 1)).collect();
    let mut next_counts: BTreeMap<usize, usize> = BTreeMap::default();

    for _ in 0..iterations {
        for (s, count) in &counts {
            match blink(*s) {
                NewStones::One(s) => *next_counts.entry(s).or_default() += count,
                NewStones::Two(s, t) => {
                    *next_counts.entry(s).or_default() += count;
                    *next_counts.entry(t).or_default() += count;
                }
            }
        }

        std::mem::swap(&mut counts, &mut next_counts);
        next_counts.clear();
    }

    counts.values().sum()
}

fn blink(stone: usize) -> NewStones {
    let s = stone.to_string();
    match stone {
        0 => NewStones::One(1),
        _ if s.len() % 2 == 0 => {
            let (a, b) = s.split_at(s.len() / 2);
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            NewStones::Two(a, b)
        }
        n => NewStones::One(n * 2024),
    }
}
