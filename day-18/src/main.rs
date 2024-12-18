use std::collections::{BTreeMap, BTreeSet};

const WIDTH: usize = 71;
const HEIGHT: usize = 71;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");

    let mut map = [[0u32; WIDTH]; HEIGHT];

    for (i, pos) in input.lines().map(parse).enumerate() {
        map[pos.1][pos.0] = i as u32 + 1;
    }

    let part1 =
        shortest(&map, (0, 0), (WIDTH - 1, HEIGHT - 1), 1024).expect("part 1 has reachable end");
    println!("Part 1: {part1}");

    for n in 1024.. {
        if shortest(&map, (0, 0), (WIDTH - 1, HEIGHT - 1), n).is_none() {
            println!("Part 2: {}", input.lines().nth(n as usize - 1).unwrap());
            break;
        }
    }
}

fn shortest(
    map: &[[u32; WIDTH]; HEIGHT],
    start: (usize, usize),
    end: (usize, usize),
    n: u32,
) -> Option<usize> {
    let mut shortest = BTreeMap::new();
    let mut border = BTreeSet::new();
    shortest.insert(start, 0);
    if map[0][1] == 0 {
        border.insert((1, 0));
        shortest.insert((1, 0), 1);
    }
    if map[1][0] == 0 {
        border.insert((0, 1));
        shortest.insert((0, 1), 1);
    }

    while let Some((x, y)) = border.pop_first() {
        let cur = shortest.get(&(x, y)).copied().unwrap();
        //println!("({x}, {y}): {cur}");
        if x > 0
            && shortest.get(&(x - 1, y)).is_none_or(|left| *left > cur + 1)
            && !(1..=n).contains(&map[y][x - 1])
        {
            border.insert((x - 1, y));
            shortest.insert((x - 1, y), cur + 1);
        }
        if y > 0
            && shortest.get(&(x, y - 1)).is_none_or(|up| *up > cur + 1)
            && !(1..=n).contains(&map[y - 1][x])
        {
            border.insert((x, y - 1));
            shortest.insert((x, y - 1), cur + 1);
        }
        if x < WIDTH - 1
            && shortest
                .get(&(x + 1, y))
                .is_none_or(|right| *right > cur + 1)
            && !(1..=n).contains(&map[y][x + 1])
        {
            border.insert((x + 1, y));
            shortest.insert((x + 1, y), cur + 1);
        }
        if y < HEIGHT - 1
            && shortest.get(&(x, y + 1)).is_none_or(|down| *down > cur + 1)
            && !(1..=n).contains(&map[y + 1][x])
        {
            border.insert((x, y + 1));
            shortest.insert((x, y + 1), cur + 1);
        }
    }

    shortest.get(&end).copied()
}

fn parse(line: &str) -> (usize, usize) {
    let (a, b) = line.split_once(',').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}
