use std::collections::HashSet;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let contents = std::fs::read_to_string(input)
        .expect("file exists")
        .lines()
        .map(|line| line.bytes().into_iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_pos = contents
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, col)| (*col == b'^').then_some((x as isize, y as isize)))
        })
        .expect("starting position");

    let visited = visited(&contents, start_pos);
    println!("Total (part 1): {}", visited.len());

    let base_map = Map::from_input(&contents);
    let mut total2 = 0;
    for pos in visited {
        if pos == start_pos {
            continue;
        }

        let mut map = base_map.clone();
        map.insert_obstacle(pos.0 as usize, pos.1 as usize);
        let looping = map.loops_from(start_pos.0 as usize, start_pos.1 as usize, Direction::U);
        if looping {
            total2 += 1;
        }
    }

    println!("Total (part 2): {total2}");
}

fn visited(field: &[Vec<u8>], mut pos: (isize, isize)) -> HashSet<(isize, isize)> {
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

        if field[next_y as usize][next_x as usize] == b'#' {
            dir = (-dir.1, dir.0);
        } else {
            pos = (next_x, next_y);
            visited.insert(pos);
        }
    }

    visited
}

fn is_looping(field: &[Vec<u8>], pos: (isize, isize)) -> bool {
    let map = Map::from_input(field);
    map.loops_from(pos.0 as usize, pos.1 as usize, Direction::U)
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    U,
    R,
    D,
    L,
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct Spans {
    u: u16,
    r: u16,
    d: u16,
    l: u16,
}

#[derive(Clone, Default, PartialEq, Eq)]
struct Map {
    width: usize,
    height: usize,
    data: Box<[Spans]>,
}

impl Map {
    fn from_input(input: &[Vec<u8>]) -> Self {
        let height = input.len();
        let width = input[0].len();
        let data = vec![Spans::default(); width * height];

        let mut res = Self {
            width,
            height,
            data: data.into_boxed_slice(),
        };

        for y in 0..height {
            for x in 0..width {
                if x == 0 {
                    res.data[res.at(x, y)].l = 1;
                    continue;
                }

                if input[y][x - 1] == b'#' {
                    res.data[res.at(x, y)].l = 0;
                } else {
                    res.data[res.at(x, y)].l = res.data[res.at(x - 1, y)].l + 1;
                }
            }

            for x in (0..width).rev() {
                if x == width - 1 {
                    res.data[res.at(x, y)].r = 1;
                    continue;
                }

                if input[y][x + 1] == b'#' {
                    res.data[res.at(x, y)].r = 0;
                } else {
                    res.data[res.at(x, y)].r = res.data[res.at(x + 1, y)].r + 1;
                }
            }
        }

        for x in 0..width {
            for y in 0..height {
                if y == 0 {
                    res.data[res.at(x, y)].u = 1;
                    continue;
                }

                if input[y - 1][x] == b'#' {
                    res.data[res.at(x, y)].u = 0;
                } else {
                    res.data[res.at(x, y)].u = res.data[res.at(x, y - 1)].u + 1;
                }
            }

            for y in (0..height).rev() {
                if y == height - 1 {
                    res.data[res.at(x, y)].d = 1;
                    continue;
                }

                if input[y + 1][x] == b'#' {
                    res.data[res.at(x, y)].d = 0;
                } else {
                    res.data[res.at(x, y)].d = res.data[res.at(x, y + 1)].d + 1;
                }
            }
        }

        res
    }

    fn insert_obstacle(&mut self, ox: usize, oy: usize) {
        for x in (0..=(ox-1)).rev() {
            if self.data[self.at(x, oy)].r == 0 {
                break;
            }

            self.data[self.at(x, oy)].r = (ox - x - 1) as u16;
        }

        for x in (ox+1)..self.width {
            if self.data[self.at(x, oy)].l == 0 {
                break;
            }

            self.data[self.at(x, oy)].l = (x - ox - 1) as u16;
        }

        for y in (0..=(oy-1)).rev() {
            if self.data[self.at(ox, y)].d == 0 {
                break;
            }

            self.data[self.at(ox, y)].d = (oy - y - 1) as u16;
        }

        for y in (oy+1)..self.height {
            if self.data[self.at(ox, y)].u == 0 {
                break;
            }

            self.data[self.at(ox, y)].u = (y - oy - 1) as u16;
        }
    }

    fn at(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn loops_from(&self, mut x: usize, mut y: usize, mut direction: Direction) -> bool {
        let mut visited = HashSet::new();

        loop {
            if visited.contains(&(x, y, direction)) {
                return true;
            }

            visited.insert((x, y, direction));
            match direction {
                Direction::U => {
                    if self.data[self.at(x, y)].u as usize > y {
                        return false;
                    }
                    y -= self.data[self.at(x, y)].u as usize;
                    direction = Direction::R;
                }
                Direction::R => {
                    if self.data[self.at(x, y)].r as usize + x >= self.width {
                        return false;
                    }
                    x += self.data[self.at(x, y)].r as usize;
                    direction = Direction::D;
                }
                Direction::D => {
                    if self.data[self.at(x, y)].d as usize + y >= self.height {
                        return false;
                    }
                    y += self.data[self.at(x, y)].d as usize;
                    direction = Direction::L;
                }
                Direction::L => {
                    if self.data[self.at(x, y)].l as usize > x {
                        return false;
                    }
                    x -= self.data[self.at(x, y)].l as usize;
                    direction = Direction::U;
                }
            }
        }
    }
}
