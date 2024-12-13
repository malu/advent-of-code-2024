use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input)
        .expect("file exists")
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let map = Map::from_input(&input);

    let mut total1 = 0;
    for region in &map.regions {
        let perimeter = perimeter(region);
        let area = region.len();
        total1 += area * perimeter;
    }

    println!("Total (part 1): {total1}");

    let mut total2 = 0;
    for region in &map.regions {
        let sides = sides(region);
        let area = region.len();
        total2 += area * sides;
    }

    println!("Total (part 2): {total2}");
}

struct Map {
    regions: Vec<BTreeSet<(usize, usize)>>,
}

impl Map {
    fn from_input(input: &[Vec<u8>]) -> Self {
        let height = input.len();
        let width = input[0].len();
        let mut data = vec![0; width * height];

        for y in 0..width {
            for x in 0..height {
                data[y * width + x] = input[y][x];
            }
        }

        let at = |x, y| data[y * width + x];

        let mut types = BTreeMap::new();
        let mut regions = BTreeMap::new();
        let mut regions2 = Vec::new();
        for y in 0..width {
            for x in 0..height {
                if regions.contains_key(&(x, y)) {
                    continue;
                }

                let t = at(x, y);
                let mut region = BTreeSet::new();
                let mut border = BTreeSet::new();
                border.insert((x, y));

                while let Some((x, y)) = border.pop_first() {
                    region.insert((x, y));
                    if x > 0 && t == at(x - 1, y) && !region.contains(&(x - 1, y)) {
                        border.insert((x - 1, y));
                    }
                    if x < width - 1 && t == at(x + 1, y) && !region.contains(&(x + 1, y)) {
                        border.insert((x + 1, y));
                    }
                    if y > 0 && t == at(x, y - 1) && !region.contains(&(x, y - 1)) {
                        border.insert((x, y - 1));
                    }
                    if y < height - 1 && t == at(x, y + 1) && !region.contains(&(x, y + 1)) {
                        border.insert((x, y + 1));
                    }
                }

                for (x, y) in &region {
                    regions.insert((*x, *y), region.clone());
                }

                types.insert(t, region.clone());
                regions2.push(region);
            }
        }

        Self { regions: regions2 }
    }
}

fn perimeter(area: &BTreeSet<(usize, usize)>) -> usize {
    let contains = |x: isize, y: isize| {
        if x < 0 || y < 0 {
            return true;
        }

        area.contains(&(x as usize, y as usize))
    };

    let mut result = 0;
    for (x, y) in area {
        let x = *x as isize;
        let y = *y as isize;
        if !contains(x - 1, y) {
            result += 1;
        }
        if !contains(x + 1, y) {
            result += 1;
        }
        if !contains(x, y - 1) {
            result += 1;
        }
        if !contains(x, y + 1) {
            result += 1;
        }
    }

    result
}

#[allow(clippy::nonminimal_bool)]
fn sides(area: &BTreeSet<(usize, usize)>) -> usize {
    let contains = |x: isize, y: isize| {
        if x < 0 || y < 0 {
            return false;
        }

        area.contains(&(x as usize, y as usize))
    };

    let mut result = 0;
    for (x, y) in area {
        let x = *x as isize;
        let y = *y as isize;
        if !contains(x - 1, y) && !(contains(x, y - 1) && !contains(x - 1, y - 1)) {
            result += 1;
        }
        if !contains(x + 1, y) && !(contains(x, y - 1) && !contains(x + 1, y - 1)) {
            result += 1;
        }
        if !contains(x, y - 1) && !(contains(x - 1, y) && !contains(x - 1, y - 1)) {
            result += 1;
        }
        if !contains(x, y + 1) && !(contains(x - 1, y) && !contains(x - 1, y + 1)) {
            result += 1;
        }
    }

    result
}
