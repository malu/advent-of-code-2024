use std::collections::BTreeMap;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");
    let map = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut start = None;
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == b'S' {
                start = Some((x as isize, y as isize));
            }
        }
    }

    let pos = start.unwrap();

    let mut cheapest = BTreeMap::new();
    let mut border = BTreeMap::new();
    cheapest.insert(pos, 0);
    border.insert(pos, 0);

    while let Some((pos, cost)) = border.pop_first() {
        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next = (pos.0 + dir.0, pos.1 + dir.1);
            if map[next.1 as usize][next.0 as usize] != b'#' {
                let cheapest = cheapest.entry(next).or_insert(isize::MAX);
                if cost + 1 < *cheapest {
                    *cheapest = cost + 1;
                    border.insert(next, cost + 1);
                }
            }
        }
    }

    let mut shortcuts = BTreeMap::new();
    for (from, from_cost) in &cheapest {
        for dir in [(2, 0), (-2, 0), (0, 2), (0, -2)] {
            let to = (from.0 + dir.0, from.1 + dir.1);
            if let Some(to_cost) = cheapest.get(&to) {
                let diff = to_cost - *from_cost - 2;
                shortcuts.insert((from, to), diff);
            }
        }
    }

    let part1 = shortcuts.values().filter(|s| **s >= 100).count();
    println!("Total (part 1): {part1}");

    let mut shortcuts2 = BTreeMap::new();
    for (from, from_cost) in &cheapest {
        for dist in 1..=20isize {
            for x in -dist..=dist {
                for y in -(dist - x.abs())..=(dist - x.abs()) {
                    let to = (from.0 + x, from.1 + y);
                    if let Some(to_cost) = cheapest.get(&to) {
                        let diff = to_cost - *from_cost - (x.abs() + y.abs());
                        shortcuts2.insert((from, to), diff);
                    }
                }
            }
        }
    }

    /*
    {
        let mut grouped = BTreeMap::<isize, usize>::new();
        for diff in shortcuts2.values() {
            *grouped.entry(*diff).or_default() += 1;
        }

        for (diff, c) in grouped {
            println!("{diff:>4}: {c:>6}");
        }
    }
    */

    let part2 = shortcuts2.values().filter(|s| **s >= 100).count();
    println!("Total (part 2): {part2}");
}
