use std::collections::{BTreeMap, BTreeSet};
use std::iter::successors;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");
    let lines = input
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut total1 = 0;
    for line in &lines {
        let mut successors = successors(Some(*line), |n| Some(next(*n)));
        let x = successors.nth(2000).unwrap();
        total1 += x;
    }

    println!("Total (part 1): {total1}");

    let mut by_changes = BTreeMap::<_, u64>::new();
    for line in &lines {
        let mut seen_changes = BTreeSet::new();

        let successors = successors(Some(*line), |n| Some(next(*n)))
            .take(2001)
            .collect::<Vec<_>>();
        let prices = successors.into_iter().map(|n| n % 10).collect::<Vec<_>>();
        let differences = prices
            .iter()
            .zip(prices.iter().skip(1))
            .map(|(prev, cur)| *cur as i8 - *prev as i8)
            .collect::<Vec<_>>();
        for (price, changes) in prices.iter().skip(4).zip(differences.windows(4)) {
            let [a, b, c, d] = changes else {
                unreachable!()
            };
            let a = *a;
            let b = *b;
            let c = *c;
            let d = *d;

            if seen_changes.contains(&(a, b, c, d)) {
                continue;
            }

            *by_changes.entry((a, b, c, d)).or_default() += *price;
            seen_changes.insert((a, b, c, d));
        }
    }

    let total2 = by_changes
        .into_iter()
        .max_by_key(|(_changes, price)| *price)
        .unwrap();
    println!("Total (part 2): {}", total2.1);
}

fn next(mut secret: u64) -> u64 {
    secret = prune(mix(secret, secret * 64));
    secret = prune(mix(secret, secret / 32));
    secret = prune(mix(secret, secret * 2048));
    secret
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(a: u64) -> u64 {
    a % 16777216
}
