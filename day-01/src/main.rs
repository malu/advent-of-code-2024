fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let contents = std::fs::read_to_string(input).expect("file exists");
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = contents
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| {
            (
                line.next().expect("first entry").parse::<usize>().expect("integer"),
                line.next().expect("second entry").parse::<usize>().expect("integer"),
            )
        })
        .collect()
    ;

    left.sort();
    right.sort();

    let total = left.iter().zip(&right).map(|(left, right)| left.abs_diff(*right)).sum::<usize>();

    println!("Total: {total}");
}
