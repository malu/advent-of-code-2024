use std::collections::HashSet;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let contents = std::fs::read_to_string(input).expect("file exists");
    let mut lines = contents.lines();

    let pairs: HashSet<_> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| line.split_once('|'))
        .collect();
    let updates: Vec<Vec<_>> = lines.map(|line| line.split(',').collect()).collect();

    let mut incorrect = Vec::new();
    let mut total1 = 0;
    'updates: for update in updates {
        for (i, page) in update.iter().enumerate() {
            if update[i..]
                .iter()
                .any(|later_page| pairs.contains(&(later_page, page)))
            {
                incorrect.push(update);
                continue 'updates;
            }
        }

        total1 += update[update.len() / 2].parse::<usize>().unwrap();
    }

    println!("Total (part 1): {total1}");

    let mut total2 = 0;
    for update in &mut incorrect {
        sort(&pairs, update);
        total2 += update[update.len() / 2].parse::<usize>().unwrap();
    }

    println!("Total (part 2): {total2}");
}

fn sort(pairs: &HashSet<(&str, &str)>, update: &mut [&str]) {
    for i in 0..update.len() {
        for j in i..update.len() {
            if pairs.contains(&(update[j], update[i])) {
                update.swap(i, j);
            }
        }
    }
}
