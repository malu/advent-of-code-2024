use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");
    let lines = input.lines().map(|l| l.split_once('-').unwrap());

    let mut connections = BTreeMap::<&str, BTreeSet<&str>>::new();
    for (a, b) in lines {
        connections.entry(a).or_default().insert(b);
        connections.entry(b).or_default().insert(a);
    }

    let mut triplets = BTreeSet::<Vec<&str>>::new();
    for (a, conns) in &connections {
        for b in conns {
            if b < a {
                continue;
            }

            for c in connections.get(b).unwrap() {
                if c < b {
                    continue;
                }

                if conns.contains(c) {
                    triplets.insert(vec![a, b, c]);
                }
            }
        }
    }

    let total1 = triplets
        .iter()
        .filter(|vec| {
            let [a, b, c] = vec.as_slice() else {
                unreachable!();
            };
            a.starts_with('t') || b.starts_with('t') || c.starts_with('t')
        })
        .count();

    println!("Total (part 1): {total1}");

    let mut clusters: Vec<BTreeSet<Vec<&str>>> = Vec::new();
    clusters.push(triplets);
    while !clusters.last().unwrap().is_empty() {
        let mut c = BTreeSet::new();

        for cluster in clusters.last().as_ref().unwrap().iter() {
            let root = &cluster[0];
            for candidate in connections.get(root).unwrap() {
                if candidate < cluster.last().unwrap() {
                    continue;
                }
                if cluster
                    .iter()
                    .all(|m| connections.get(m).unwrap().contains(candidate))
                {
                    let mut new = cluster.clone();
                    new.push(candidate);
                    c.insert(new);
                }
            }
        }

        clusters.push(c);
    }

    // Remove last empty entry
    let _ = clusters.pop();

    let largest = clusters.last_mut().unwrap().pop_first().unwrap();
    println!("Part 2: {}", largest.join(","));
}
