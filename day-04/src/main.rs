fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let contents = std::fs::read_to_string(input).expect("file exists");
    let lines = contents.lines().collect::<Vec<_>>();
    let expanded = expand(&lines);

    let mut total1 = 0;
    for line in expanded {
        total1 += line.matches("XMAS").count();
        total1 += line.matches("SAMX").count();
    }

    println!("Total (part 1): {total1}");

    let mut total2 = 0;
    for lines in lines.windows(3) {
        let [a, b, c] = lines else {
            unreachable!();
        };

        let a = a.as_bytes();
        let b = b.as_bytes();
        let c = c.as_bytes();

        for x in 0..(a.len() - 2) {
            if b[x + 1] != b'A' {
                continue;
            }

            let i = x;
            let j = x + 2;
            let asc = (c[i] == b'M' && a[j] == b'S') || (c[i] == b'S' && a[j] == b'M');
            let desc = (a[i] == b'M' && c[j] == b'S') || (a[i] == b'S' && c[j] == b'M');

            if asc && desc {
                total2 += 1;
            }
        }
    }

    println!("Total (part 2): {total2}");
}

fn expand(lines: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    // horizontals
    for &line in lines {
        result.push(line.to_owned());
    }

    // verticals
    for x in 0..lines[0].len() {
        let mut vertical = Vec::new();
        for line in lines {
            vertical.push(line.as_bytes()[x]);
        }
        result.push(String::from_utf8(vertical).expect("valid utf8"));
    }

    // descending diagonals
    for x in 0..(lines[0].len() + lines.len()) {
        let mut diag = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            let Some(i) = (x + y).checked_sub(lines.len()) else {
                continue;
            };

            let Some(b) = line.as_bytes().get(i) else {
                continue;
            };
            diag.push(*b);
        }

        result.push(String::from_utf8(diag).expect("valid utf8"));
    }

    // ascending diagonals
    for x in 0..(lines[0].len() + lines.len()) {
        let mut diag = Vec::new();
        for (y, line) in lines.iter().rev().enumerate() {
            let Some(i) = (x + y).checked_sub(lines.len()) else {
                continue;
            };

            let Some(b) = line.as_bytes().get(i) else {
                continue;
            };
            diag.push(*b);
        }

        result.push(String::from_utf8(diag).expect("valid utf8"));
    }

    result
}
