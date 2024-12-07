use std::collections::BTreeSet;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let contents = std::fs::read_to_string(input)
        .expect("file exists")
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
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

        let overlay = base_map.overlay_obstacle(pos.0 as usize, pos.1 as usize);
        let looping = overlay.loops_from(start_pos.0 as usize, start_pos.1 as usize, Direction::U);
        if looping {
            total2 += 1;
        }
    }

    println!("Total (part 2): {total2}");
}

fn visited(field: &[Vec<u8>], mut pos: (isize, isize)) -> BTreeSet<(isize, isize)> {
    let mut visited = BTreeSet::new();
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

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

    fn overlay_obstacle(&self, ox: usize, oy: usize) -> Overlay {
        Overlay {
            map: self,
            obstacle_x: ox,
            obstacle_y: oy,
        }
    }

    #[inline]
    fn at(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    fn u(&self, x: usize, y: usize) -> u16 {
        self.data[self.at(x, y)].u
    }

    #[inline]
    fn r(&self, x: usize, y: usize) -> u16 {
        self.data[self.at(x, y)].r
    }

    #[inline]
    fn d(&self, x: usize, y: usize) -> u16 {
        self.data[self.at(x, y)].d
    }

    #[inline]
    fn l(&self, x: usize, y: usize) -> u16 {
        self.data[self.at(x, y)].l
    }
}

#[derive(Clone)]
struct Overlay<'a> {
    map: &'a Map,
    obstacle_x: usize,
    obstacle_y: usize,
}

impl Overlay<'_> {
    #[inline]
    fn u(&self, x: usize, y: usize) -> usize {
        let u = self.map.u(x, y) as usize;
        if x == self.obstacle_x && y > self.obstacle_y && y - self.obstacle_y <= u {
            y - self.obstacle_y - 1
        } else {
            u
        }
    }

    #[inline]
    fn r(&self, x: usize, y: usize) -> usize {
        let r = self.map.r(x, y) as usize;
        if y == self.obstacle_y && x < self.obstacle_x && self.obstacle_x - x <= r {
            self.obstacle_x - x - 1
        } else {
            r
        }
    }

    #[inline]
    fn d(&self, x: usize, y: usize) -> usize {
        let d = self.map.d(x, y) as usize;
        if x == self.obstacle_x && y < self.obstacle_y && self.obstacle_y - y <= d {
            self.obstacle_y - y - 1
        } else {
            d
        }
    }

    #[inline]
    fn l(&self, x: usize, y: usize) -> usize {
        let l = self.map.l(x, y) as usize;
        if y == self.obstacle_y && x > self.obstacle_x && x - self.obstacle_x <= l {
            x - self.obstacle_x - 1
        } else {
            l
        }
    }

    fn loops_from(&self, mut x: usize, mut y: usize, mut direction: Direction) -> bool {
        let mut visited = vec![0u8; self.map.width * self.map.height];
        let dir_to_bit = |dir: Direction| match dir {
            Direction::U => 1 << 0,
            Direction::R => 1 << 1,
            Direction::D => 1 << 2,
            Direction::L => 1 << 3,
        };

        loop {
            if visited[self.map.at(x, y)] & dir_to_bit(direction) > 0 {
                return true;
            }
            visited[self.map.at(x, y)] |= dir_to_bit(direction);

            match direction {
                Direction::U => {
                    let u = self.u(x, y);
                    if u > y {
                        return false;
                    }
                    y -= u;
                    direction = Direction::R;
                }
                Direction::R => {
                    let r = self.r(x, y);
                    if r + x >= self.map.width {
                        return false;
                    }
                    x += r;
                    direction = Direction::D;
                }
                Direction::D => {
                    let d = self.d(x, y);
                    if d + y >= self.map.height {
                        return false;
                    }
                    y += d;
                    direction = Direction::L;
                }
                Direction::L => {
                    let l = self.l(x, y);
                    if l > x {
                        return false;
                    }
                    x -= l;
                    direction = Direction::U;
                }
            }
        }
    }
}
