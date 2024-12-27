fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");
    let lines = input.lines().collect::<Vec<_>>();

    let (locks, keys) = lines
        .split(|line| line.is_empty())
        .map(parse)
        .partition::<Vec<_>, _>(|l| l[0] > 0);

    let mut total1 = 0;
    for lock in &locks {
        for key in &keys {
            if compatible(lock, key) {
                total1 += 1;
            }
        }
    }

    println!("Total (part 1): {total1}");
}

fn parse(lines: &[&str]) -> Vec<i8> {
    let mut res = Vec::new();
    for i in 0..lines[0].len() {
        if lines[0].as_bytes()[i] == b'#' {
            res.push(lines.iter().take_while(|l| l.as_bytes()[i] == b'#').count() as i8);
        } else if lines[0].as_bytes()[i] == b'.' {
            res.push(-(lines.iter().take_while(|l| l.as_bytes()[i] == b'.').count() as i8));
        }
    }

    res
}

fn compatible(a: &[i8], b: &[i8]) -> bool {
    a.iter().zip(b.iter()).all(|(a, b)| a + b <= 0)
}
