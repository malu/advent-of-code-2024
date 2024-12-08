fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let lines = std::fs::read_to_string(input)
        .expect("file exists")
        .lines()
        .map(split_line)
        .collect::<Vec<_>>();

    let total1 = lines
        .iter()
        .filter(|(total, comps)| solvable1(*total, comps))
        .map(|(total, _)| total)
        .sum::<usize>();
    println!("Total (part 1): {total1}");

    let total2 = lines
        .iter()
        .filter(|(total, comps)| solvable2(*total, comps))
        .map(|(total, _)| total)
        .sum::<usize>();
    println!("Total (part 2): {total2}");
}

fn split_line(line: &str) -> (usize, Vec<usize>) {
    let (total, rest) = line.split_once(": ").unwrap();
    let total = total.parse().unwrap();
    let rest = rest
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    (total, rest)
}

fn solvable1(total: usize, components: &[usize]) -> bool {
    for mut v in 0..(1 << (components.len() - 1)) {
        let t = components.iter().skip(1).try_fold(components[0], |acc, c| {
            let acc = if v % 2 > 0 {
                v /= 2;
                acc.checked_add(*c)
            } else {
                v /= 2;
                acc.checked_mul(*c)
            };

            acc.filter(|acc| *acc <= total)
        });

        if t == Some(total) {
            return true;
        }
    }

    false
}

fn solvable2(total: usize, components: &[usize]) -> bool {
    for mut v in 0..(3usize.pow(components.len() as u32 - 1)) {
        let t = components.iter().skip(1).try_fold(components[0], |acc, c| {
            let acc = match v % 3 {
                0 => {
                    v /= 3;
                    acc.checked_add(*c)
                }
                1 => {
                    v /= 3;
                    acc.checked_mul(*c)
                }
                2 => {
                    v /= 3;
                    format!("{acc}{c}").parse::<usize>().ok()
                }
                _ => unreachable!(),
            };

            acc.filter(|acc| *acc <= total)
        });

        if t == Some(total) {
            return true;
        }
    }

    false
}
