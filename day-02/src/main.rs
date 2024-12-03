fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let contents = std::fs::read_to_string(input).expect("file exists");

    let count1 = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
        })
        .map(|xs| xs.collect::<Vec<_>>())
        .filter(|xs| is_safe(xs))
        .count();

    println!("Count (part 1): {count1}");

    let count2 = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
        })
        .map(|xs| xs.collect::<Vec<_>>())
        .filter(|xs| {
            for i in 0..xs.len() {
                let mut xs = xs.clone();
                xs.remove(i);
                if is_safe(&xs) {
                    return true;
                }
            }

            false
        })
        .count();

    println!("Count (part 2): {count2}");
}

fn is_safe(xs: &[usize]) -> bool {
    let mut global_ordering = None;

    for (x, y) in xs.iter().zip(xs.iter().skip(1)) {
        if x.abs_diff(*y) > 3 {
            return false;
        }

        let local_ordering = x.cmp(y);
        global_ordering = global_ordering.or(Some(local_ordering));

        match (global_ordering.unwrap(), local_ordering) {
            (ord, local_ord) if ord == local_ord => continue,
            _ => return false,
        }
    }

    true
}
