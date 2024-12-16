use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");
    let map = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut start = None;
    let mut end = None;
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == b'S' {
                start = Some((x as isize, y as isize));
            } else if *cell == b'E' {
                end = Some((x as isize, y as isize));
            }
        }
    }

    let pos = (start.unwrap(), (1, 0));
    let end = end.unwrap();

    let mut cheapest = BTreeMap::new();
    let mut border = BTreeMap::new();
    border.insert(pos, 0);

    while let Some(((pos, dir), cost)) = border.pop_first() {
        {
            let next = (pos.0 + dir.0, pos.1 + dir.1);
            if map[next.1 as usize][next.0 as usize] != b'#' {
                let cheapest = cheapest.entry((next, dir)).or_insert(isize::MAX);
                if cost + 1 < *cheapest {
                    *cheapest = cost + 1;
                    border.insert((next, dir), cost + 1);
                }
            }
        }

        {
            let next = (-dir.1, dir.0);
            let cheapest = cheapest.entry((pos, next)).or_insert(isize::MAX);
            if cost + 1000 < *cheapest {
                *cheapest = cost + 1000;
                border.insert((pos, next), cost + 1000);
            }
        }

        {
            let next = (dir.1, -dir.0);
            let cheapest = cheapest.entry((pos, next)).or_insert(isize::MAX);
            if cost + 1000 < *cheapest {
                *cheapest = cost + 1000;
                border.insert((pos, next), cost + 1000);
            }
        }
    }

    let part1 = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .into_iter()
        .filter_map(|dir| cheapest.get(&(end, dir)))
        .min()
        .unwrap();
    println!("Total (part 1): {part1}");

    let mut on_best_path = BTreeSet::new();
    on_best_path.insert(end);
    on_best_path.insert(start.unwrap());
    let mut border = BTreeSet::new();
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        if cheapest.get(&(end, dir)) == Some(part1) {
            border.insert((end, dir));
        }
    }

    while let Some((pos, dir)) = border.pop_first() {
        let cur = cheapest.get(&(pos, dir)).unwrap();

        {
            let next = (pos.0 - dir.0, pos.1 - dir.1);
            if cheapest
                .get(&(next, dir))
                .is_some_and(|cheapest| *cheapest == cur - 1)
            {
                border.insert((next, dir));
                on_best_path.insert(next);
            }
        }

        {
            let next = (-dir.1, dir.0);
            if cheapest
                .get(&(pos, next))
                .is_some_and(|cheapest| *cheapest == cur - 1000)
            {
                border.insert((pos, next));
                on_best_path.insert(pos);
            }
        }

        {
            let next = (dir.1, -dir.0);
            if cheapest
                .get(&(pos, next))
                .is_some_and(|cheapest| *cheapest == cur - 1000)
            {
                border.insert((pos, next));
                on_best_path.insert(pos);
            }
        }
    }

    println!("Total (part 2): {}", on_best_path.len());
}
