use std::collections::HashSet;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let contents = std::fs::read_to_string(input)
        .expect("file exists")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_pos = contents
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, col)| (*col == '^').then_some((x as isize, y as isize)))
        })
        .expect("starting position");

    let visited = visited(&contents, start_pos);
    println!("Total (part 1): {}", visited.len());

    let mut total2 = 0;
    for pos in visited {
        if pos == start_pos {
            continue;
        }

        let mut contents = contents.clone();
        contents[pos.1 as usize][pos.0 as usize] = '#';
        if is_looping(&contents, start_pos) {
            total2 += 1;
        }
    }

    println!("Total (part 2): {total2}");
}

fn visited(field: &[Vec<char>], mut pos: (isize, isize)) -> HashSet<(isize, isize)> {
    let mut visited = HashSet::new();
    let mut dir = (0, -1);

    visited.insert(pos);

    loop {
        let next_x = pos.0 + dir.0;
        let next_y = pos.1 + dir.1;

        if next_x < 0
            || next_x as usize >= field[0].len()
            || next_y < 0
            || next_y as usize >= field.len()
        {
            break;
        }

        if field[next_y as usize][next_x as usize] == '#' {
            dir = (-dir.1, dir.0);
        } else {
            pos = (next_x, next_y);
            visited.insert(pos);
        }
    }

    visited
}

fn is_looping(field: &[Vec<char>], mut pos: (isize, isize)) -> bool {
    let mut visited = HashSet::new();
    let mut dir = (0, -1);

    visited.insert((pos, dir));

    loop {
        let next_x = pos.0 + dir.0;
        let next_y = pos.1 + dir.1;

        if visited.contains(&((next_x, next_y), dir)) {
            return true;
        }

        if next_x < 0
            || next_x as usize >= field[0].len()
            || next_y < 0
            || next_y as usize >= field.len()
        {
            break;
        }

        if field[next_y as usize][next_x as usize] == '#' {
            dir = (-dir.1, dir.0);
        } else {
            pos = (next_x, next_y);
            visited.insert((pos, dir));
        }
    }

    false
}
