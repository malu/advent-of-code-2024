use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let lines = std::fs::read_to_string(input)
        .expect("file exists")
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = lines[0].len();
    let height = lines.len();
    let in_bounds =
        |(x, y): (i8, i8)| x >= 0 && (x as usize) < width && y >= 0 && (y as usize) < height;

    let mut types: BTreeMap<u8, Vec<(u8, u8)>> = BTreeMap::new();
    for (y, row) in lines.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == b'.' {
                continue;
            }

            types.entry(*cell).or_default().push((x as u8, y as u8));
        }
    }

    let mut spots1: BTreeSet<(i8, i8)> = BTreeSet::new();
    for antennas in types.values() {
        for (i, (x1, y1)) in antennas.iter().enumerate() {
            for (x2, y2) in antennas.iter().skip(i + 1) {
                let dx = *x1 as i8 - *x2 as i8;
                let dy = *y1 as i8 - *y2 as i8;

                let spot1 = (*x1 as i8 + dx, *y1 as i8 + dy);
                let spot2 = (*x2 as i8 - dx, *y2 as i8 - dy);

                if in_bounds(spot1) {
                    spots1.insert(spot1);
                }
                if in_bounds(spot2) {
                    spots1.insert(spot2);
                }
            }
        }
    }

    println!("Total (part 1): {}", spots1.len());

    let mut spots2: BTreeSet<(i8, i8)> = BTreeSet::new();
    for antennas in types.values() {
        for (i, (x1, y1)) in antennas.iter().enumerate() {
            for (x2, y2) in antennas.iter().skip(i + 1) {
                let dx = *x1 as i8 - *x2 as i8;
                let dy = *y1 as i8 - *y2 as i8;

                let mut spot1 = (*x1 as i8, *y1 as i8);
                while in_bounds(spot1) {
                    spots2.insert(spot1);
                    spot1 = (spot1.0 + dx, spot1.1 + dy);
                }

                let mut spot2 = (*x2 as i8, *y2 as i8);
                while in_bounds(spot2) {
                    spots2.insert(spot2);
                    spot2 = (spot2.0 - dx, spot2.1 - dy);
                }
            }
        }
    }

    println!("Total (part 2): {}", spots2.len());
}
