use std::collections::BTreeMap;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");
    let lines = input.lines().collect::<Vec<_>>();

    let split = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
    let mut have = split[0][0].split(", ").collect::<Vec<_>>();
    have.sort();
    let want = split[1];

    let max_len = have.iter().map(|p| p.len()).max().unwrap();
    let mut map = BTreeMap::new();

    let mut total1 = 0;
    let mut total2 = 0;
    for want in want.iter() {
        let c = build(want, &have, max_len, &mut map);
        total1 += std::cmp::min(c, 1);
        total2 += c;
    }

    println!("Total (part 1): {total1}");
    println!("Total (part 2): {total2}");
}

fn build<'a>(
    input: &'a str,
    pieces: &[&str],
    max_prefix_len: usize,
    map: &mut BTreeMap<&'a str, usize>,
) -> usize {
    if input.is_empty() {
        return 1;
    }

    if let Some(res) = map.get(input) {
        return *res;
    }

    let mut res = 0;

    for len in 1..=std::cmp::min(input.len(), max_prefix_len) {
        let prefix = &input[0..len];
        if pieces.binary_search(&prefix).is_ok() {
            res += build(&input[len..], pieces, max_prefix_len, map);
        }
    }

    map.insert(input, res);

    res
}
