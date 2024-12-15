use std::collections::BTreeSet;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");
    let lines = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut split = lines.split(|line| line.is_empty());
    let mut map = split.next().unwrap().to_vec();
    let instructions = split.next().unwrap().join(&[][..]);

    let mut robot = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(_, cell)| **cell == b'@')
                .map(|(x, _)| (x as isize, y as isize))
        })
        .unwrap();

    map[robot.1 as usize][robot.0 as usize] = b'.';

    let mut robot2 = robot;
    robot2.0 *= 2;
    let mut map2 = convert_map_for_part2(&map);

    for instruction in &instructions {
        let dir = match instruction {
            b'>' => (1, 0),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            b'^' => (0, -1),
            _ => unreachable!(),
        };

        let mut look_at = robot;
        while map[(look_at.1 + dir.1) as usize][(look_at.0 + dir.0) as usize] == b'O' {
            look_at.0 += dir.0;
            look_at.1 += dir.1;
        }

        if map[(look_at.1 + dir.1) as usize][(look_at.0 + dir.0) as usize] != b'.' {
            continue;
        }

        while look_at != robot {
            map[(look_at.1 + dir.1) as usize][(look_at.0 + dir.0) as usize] =
                map[look_at.1 as usize][look_at.0 as usize];

            look_at.0 -= dir.0;
            look_at.1 -= dir.1;
        }

        robot.0 += dir.0;
        robot.1 += dir.1;
        map[robot.1 as usize][robot.0 as usize] = b'.';
    }

    let mut total1 = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == b'O' {
                total1 += x + (y * 100);
            }
        }
    }

    println!("Total (part 1): {total1}");

    for instruction in &instructions {
        let dir = match instruction {
            b'>' => (1, 0),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            b'^' => (0, -1),
            _ => unreachable!(),
        };

        if let Some(map) = try_move(&map2, robot2, dir) {
            map2 = map;
            robot2.0 += dir.0;
            robot2.1 += dir.1;
        };
        map2[robot2.1 as usize][robot2.0 as usize] = b'.';
    }

    let mut total2 = 0;
    for (y, row) in map2.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == b'[' {
                total2 += x + (y * 100);
            }
        }
    }

    println!("Total (part 2): {total2}");
}

fn convert_map_for_part2(input: &[Vec<u8>]) -> Vec<Vec<u8>> {
    input
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|cell| match cell {
                    b'O' => [b'[', b']'],
                    b'.' => [b'.', b'.'],
                    b'#' => [b'#', b'#'],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn try_move(input: &[Vec<u8>], robot: (isize, isize), dir: (isize, isize)) -> Option<Vec<Vec<u8>>> {
    let mut pieces_to_move: BTreeSet<(isize, isize)> = BTreeSet::new();
    pieces_to_move.insert(robot);
    loop {
        let mut next_pieces_to_move = pieces_to_move.clone();
        for p in &pieces_to_move {
            let dependencies = match dir {
                (1, 0) => {
                    vec![(p.0 + 1, p.1)]
                }
                (0, 1) => {
                    if input[p.1 as usize + 1][p.0 as usize] == b'[' {
                        vec![(p.0, p.1 + 1), (p.0 + 1, p.1 + 1)]
                    } else if input[p.1 as usize + 1][p.0 as usize] == b']' {
                        vec![(p.0, p.1 + 1), (p.0 - 1, p.1 + 1)]
                    } else {
                        vec![(p.0, p.1 + 1)]
                    }
                }
                (-1, 0) => {
                    vec![(p.0 - 1, p.1)]
                }
                (0, -1) => {
                    if input[p.1 as usize - 1][p.0 as usize] == b'[' {
                        vec![(p.0, p.1 - 1), (p.0 + 1, p.1 - 1)]
                    } else if input[p.1 as usize - 1][p.0 as usize] == b']' {
                        vec![(p.0, p.1 - 1), (p.0 - 1, p.1 - 1)]
                    } else {
                        vec![(p.0, p.1 - 1)]
                    }
                }
                _ => unreachable!(),
            };

            for d in dependencies {
                if input[d.1 as usize][d.0 as usize] == b'#' {
                    return None;
                }

                if input[d.1 as usize][d.0 as usize] == b'.' {
                    continue;
                }

                next_pieces_to_move.insert(d);
            }
        }

        if next_pieces_to_move == pieces_to_move {
            break;
        }

        pieces_to_move = next_pieces_to_move;
    }

    if pieces_to_move
        .iter()
        .any(|(x, y)| input[*y as usize][*x as usize] == b'#')
    {
        return None;
    }

    let mut result = input.to_owned();

    for p in &pieces_to_move {
        result[p.1 as usize][p.0 as usize] = b'.';
    }

    for p in &pieces_to_move {
        result[p.1 as usize + dir.1 as usize][p.0 as usize + dir.0 as usize] =
            input[p.1 as usize][p.0 as usize];
    }

    Some(result)
}
