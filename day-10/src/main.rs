use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");

    let map = Map::from_input(&input);
    let mut total = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x, y) == 0 {
                total += map.reachable(x, y);
            }
        }
    }

    println!("Total (part 1): {total}");

    let mut total = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x, y) == 0 {
                total += map.reachable2(x, y);
            }
        }
    }

    println!("Total (part 2): {total}");
}

struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_input(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = lines[0].len();
        let height = lines.len();
        let data: Vec<_> = lines
            .iter()
            .flatten()
            .copied()
            .filter_map(|c| c.to_digit(10).map(|n| n as u8))
            .collect();

        Self {
            data,
            width,
            height,
        }
    }

    fn at(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[self.at(x, y)]
    }

    fn reachable(&self, x: usize, y: usize) -> usize {
        let mut border = BTreeSet::new();
        border.insert((x, y));
        let mut visited = BTreeSet::new();

        let mut total = 0;
        while let Some((x, y)) = border.pop_first() {
            let level = self.get(x, y);
            visited.insert((x, y));
            if level == 9 {
                total += 1;
                continue;
            }

            if x > 0 && self.get(x - 1, y) == level + 1 && !visited.contains(&(x - 1, y)) {
                border.insert((x - 1, y));
            }
            if x < self.width - 1
                && self.get(x + 1, y) == level + 1
                && !visited.contains(&(x + 1, y))
            {
                border.insert((x + 1, y));
            }
            if y > 0 && self.get(x, y - 1) == level + 1 && !visited.contains(&(x, y - 1)) {
                border.insert((x, y - 1));
            }
            if y < self.height - 1
                && self.get(x, y + 1) == level + 1
                && !visited.contains(&(x, y + 1))
            {
                border.insert((x, y + 1));
            }
        }

        total
    }

    fn reachable2(&self, x: usize, y: usize) -> usize {
        let mut border = BTreeSet::new();
        let mut next = BTreeSet::new();
        let mut paths: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        border.insert((x, y));
        paths.insert((x, y), 1);

        let mut level = self.get(x, y);
        let mut total = 0;
        loop {
            while let Some((x, y)) = border.pop_first() {
                let paths_here = paths.get(&(x, y)).copied().unwrap_or_default();
                if level == 9 {
                    total += paths.get(&(x, y)).copied().unwrap_or_default();
                }

                if x > 0 && self.get(x - 1, y) == level + 1 {
                    next.insert((x - 1, y));
                    *paths.entry((x - 1, y)).or_default() += paths_here;
                }
                if x < self.width - 1 && self.get(x + 1, y) == level + 1 {
                    next.insert((x + 1, y));
                    *paths.entry((x + 1, y)).or_default() += paths_here;
                }
                if y > 0 && self.get(x, y - 1) == level + 1 {
                    next.insert((x, y - 1));
                    *paths.entry((x, y - 1)).or_default() += paths_here;
                }
                if y < self.height - 1 && self.get(x, y + 1) == level + 1 {
                    next.insert((x, y + 1));
                    *paths.entry((x, y + 1)).or_default() += paths_here;
                }
            }

            if next.is_empty() {
                break;
            }

            std::mem::swap(&mut border, &mut next);
            level += 1;
        }

        total
    }
}
