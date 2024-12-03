fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let contents = std::fs::read_to_string(input).expect("file exists");
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = contents
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| {
            (
                line.next()
                    .expect("first entry")
                    .parse::<usize>()
                    .expect("integer"),
                line.next()
                    .expect("second entry")
                    .parse::<usize>()
                    .expect("integer"),
            )
        })
        .collect();

    left.sort();
    right.sort();

    let total1 = left
        .iter()
        .zip(&right)
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<usize>();

    println!("Total (part 1): {total1}");

    let count_in_right = |n: usize| {
        let a = right.partition_point(|x| *x < n);
        let b = right.partition_point(|x| *x <= n);

        b - a
    };

    let total2 = left.iter().map(|n| *n * count_in_right(*n)).sum::<usize>();
    println!("Total (part 2): {total2}");
}
