use std::collections::BTreeMap;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");
    let lines = input.lines().map(parse).collect::<Vec<_>>();

    let mut total1 = 0;
    for line in &lines {
        let mut cost = 0;
        for (cur, next) in std::iter::once(&Numeric::A)
            .chain(line.iter())
            .zip(line.iter())
        {
            cost += cur
                .routes_to(*next)
                .into_iter()
                .map(|mut route| {
                    route.push(Directional::A);
                    steps(&route, 2, &mut Default::default())
                })
                .min()
                .unwrap();
        }

        let complexity = numeric_code(line) * cost;
        total1 += complexity;
    }
    println!("Total (part 1): {total1}");

    let mut total2 = 0;
    for line in &lines {
        let mut cost = 0;
        for (cur, next) in std::iter::once(&Numeric::A)
            .chain(line.iter())
            .zip(line.iter())
        {
            cost += cur
                .routes_to(*next)
                .into_iter()
                .map(|mut route| {
                    route.push(Directional::A);
                    steps(&route, 25, &mut Default::default())
                })
                .min()
                .unwrap();
        }

        let complexity = numeric_code(line) * cost;
        total2 += complexity;
    }
    println!("Total (part 2): {total2}");
}

fn parse(input: &str) -> Vec<Numeric> {
    input
        .bytes()
        .map(|b| match b {
            b'A' => Numeric::A,
            b'0' => Numeric::Zero,
            b'1' => Numeric::One,
            b'2' => Numeric::Two,
            b'3' => Numeric::Three,
            b'4' => Numeric::Four,
            b'5' => Numeric::Five,
            b'6' => Numeric::Six,
            b'7' => Numeric::Seven,
            b'8' => Numeric::Eight,
            b'9' => Numeric::Nine,
            _ => unreachable!(),
        })
        .collect()
}

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Directional {
    #[default]
    A,
    U,
    R,
    D,
    L,
}

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
#[repr(u8)]
enum Numeric {
    #[default]
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

fn numeric_code(input: &[Numeric]) -> usize {
    let mut total = 0;
    for n in input {
        let v = match n {
            Numeric::A => break,
            Numeric::Zero => 0,
            Numeric::One => 1,
            Numeric::Two => 2,
            Numeric::Three => 3,
            Numeric::Four => 4,
            Numeric::Five => 5,
            Numeric::Six => 6,
            Numeric::Seven => 7,
            Numeric::Eight => 8,
            Numeric::Nine => 9,
        };

        total *= 10;
        total += v;
    }

    total
}

fn steps(
    input: &[Directional],
    layers: usize,
    memo: &mut BTreeMap<(usize, Vec<Directional>), usize>,
) -> usize {
    if layers == 0 {
        return input.len();
    }

    if let Some(memo) = memo.get(&(layers, input.to_vec())) {
        return *memo;
    }

    let mut total = 0;
    for (cur, next) in std::iter::once(&Directional::A)
        .chain(input.iter())
        .zip(input.iter())
    {
        total += cur
            .routes_to(*next)
            .into_iter()
            .map(|mut route| {
                route.push(Directional::A);
                steps(&route, layers - 1, memo)
            })
            .min()
            .unwrap();
    }

    memo.insert((layers, input.to_vec()), total);

    total
}

impl Numeric {
    fn routes_to(self, to: Numeric) -> Vec<Vec<Directional>> {
        match (self, to) {
            (Numeric::A, Numeric::A) => vec![vec![]],
            (Numeric::A, Numeric::Zero) => vec![vec![Directional::L]],
            (Numeric::A, Numeric::One) => {
                vec![vec![Directional::U, Directional::L, Directional::L]]
            }
            (Numeric::A, Numeric::Two) => vec![
                vec![Directional::U, Directional::L],
                vec![Directional::L, Directional::U],
            ],
            (Numeric::A, Numeric::Three) => vec![vec![Directional::U]],
            (Numeric::A, Numeric::Four) => vec![vec![
                Directional::U,
                Directional::U,
                Directional::L,
                Directional::L,
            ]],
            (Numeric::A, Numeric::Five) => vec![
                vec![Directional::U, Directional::U, Directional::L],
                vec![Directional::L, Directional::U, Directional::U],
            ],
            (Numeric::A, Numeric::Six) => vec![vec![Directional::U, Directional::U]],
            (Numeric::A, Numeric::Seven) => vec![vec![
                Directional::U,
                Directional::U,
                Directional::U,
                Directional::L,
                Directional::L,
            ]],
            (Numeric::A, Numeric::Eight) => vec![
                vec![
                    Directional::U,
                    Directional::U,
                    Directional::U,
                    Directional::L,
                ],
                vec![
                    Directional::L,
                    Directional::U,
                    Directional::U,
                    Directional::U,
                ],
            ],
            (Numeric::A, Numeric::Nine) => {
                vec![vec![Directional::U, Directional::U, Directional::U]]
            }
            (Numeric::Zero, Numeric::A) => vec![vec![Directional::R]],
            (Numeric::Zero, Numeric::Zero) => vec![vec![]],
            (Numeric::Zero, Numeric::One) => vec![vec![Directional::U, Directional::L]],
            (Numeric::Zero, Numeric::Two) => vec![vec![Directional::U]],
            (Numeric::Zero, Numeric::Three) => vec![
                vec![Directional::U, Directional::R],
                vec![Directional::R, Directional::U],
            ],
            (Numeric::Zero, Numeric::Four) => {
                vec![vec![Directional::U, Directional::U, Directional::L]]
            }
            (Numeric::Zero, Numeric::Five) => vec![vec![Directional::U, Directional::U]],
            (Numeric::Zero, Numeric::Six) => vec![
                vec![Directional::U, Directional::U, Directional::R],
                vec![Directional::R, Directional::U, Directional::U],
            ],
            (Numeric::Zero, Numeric::Seven) => vec![vec![
                Directional::U,
                Directional::U,
                Directional::U,
                Directional::L,
            ]],
            (Numeric::Zero, Numeric::Eight) => {
                vec![vec![Directional::U, Directional::U, Directional::U]]
            }
            (Numeric::Zero, Numeric::Nine) => vec![vec![
                Directional::U,
                Directional::U,
                Directional::U,
                Directional::R,
            ]],
            (Numeric::One, Numeric::A) => {
                vec![vec![Directional::R, Directional::R, Directional::D]]
            }
            (Numeric::One, Numeric::Zero) => vec![vec![Directional::R, Directional::D]],
            (Numeric::One, Numeric::One) => vec![vec![]],
            (Numeric::One, Numeric::Two) => vec![vec![Directional::R]],
            (Numeric::One, Numeric::Three) => vec![vec![Directional::R, Directional::R]],
            (Numeric::One, Numeric::Four) => vec![vec![Directional::U]],
            (Numeric::One, Numeric::Five) => vec![
                vec![Directional::R, Directional::U],
                vec![Directional::U, Directional::R],
            ],
            (Numeric::One, Numeric::Six) => vec![
                vec![Directional::R, Directional::R, Directional::U],
                vec![Directional::U, Directional::R, Directional::R],
            ],
            (Numeric::One, Numeric::Seven) => vec![vec![Directional::U, Directional::U]],
            (Numeric::One, Numeric::Eight) => vec![
                vec![Directional::R, Directional::U, Directional::U],
                vec![Directional::U, Directional::U, Directional::R],
            ],
            (Numeric::One, Numeric::Nine) => vec![
                vec![
                    Directional::R,
                    Directional::R,
                    Directional::U,
                    Directional::U,
                ],
                vec![
                    Directional::U,
                    Directional::U,
                    Directional::R,
                    Directional::R,
                ],
            ],
            (Numeric::Two, Numeric::A) => vec![
                vec![Directional::R, Directional::D],
                vec![Directional::D, Directional::R],
            ],
            (Numeric::Two, Numeric::Zero) => vec![vec![Directional::D]],
            (Numeric::Two, Numeric::One) => vec![vec![Directional::L]],
            (Numeric::Two, Numeric::Two) => vec![vec![]],
            (Numeric::Two, Numeric::Three) => vec![vec![Directional::R]],
            (Numeric::Two, Numeric::Four) => vec![
                vec![Directional::L, Directional::U],
                vec![Directional::U, Directional::L],
            ],
            (Numeric::Two, Numeric::Five) => vec![vec![Directional::U]],
            (Numeric::Two, Numeric::Six) => vec![
                vec![Directional::R, Directional::U],
                vec![Directional::U, Directional::R],
            ],
            (Numeric::Two, Numeric::Seven) => vec![
                vec![Directional::L, Directional::U, Directional::U],
                vec![Directional::U, Directional::U, Directional::L],
            ],
            (Numeric::Two, Numeric::Eight) => vec![vec![Directional::U, Directional::U]],
            (Numeric::Two, Numeric::Nine) => vec![
                vec![Directional::R, Directional::U, Directional::U],
                vec![Directional::U, Directional::U, Directional::R],
            ],
            (Numeric::Three, Numeric::A) => vec![vec![Directional::D]],
            (Numeric::Three, Numeric::Zero) => vec![
                vec![Directional::L, Directional::D],
                vec![Directional::D, Directional::L],
            ],
            (Numeric::Three, Numeric::One) => vec![vec![Directional::L, Directional::L]],
            (Numeric::Three, Numeric::Two) => vec![vec![Directional::L]],
            (Numeric::Three, Numeric::Three) => vec![vec![]],
            (Numeric::Three, Numeric::Four) => vec![
                vec![Directional::L, Directional::L, Directional::U],
                vec![Directional::U, Directional::L, Directional::L],
            ],
            (Numeric::Three, Numeric::Five) => vec![
                vec![Directional::L, Directional::U],
                vec![Directional::U, Directional::L],
            ],
            (Numeric::Three, Numeric::Six) => vec![vec![Directional::U]],
            (Numeric::Three, Numeric::Seven) => vec![
                vec![
                    Directional::L,
                    Directional::L,
                    Directional::U,
                    Directional::U,
                ],
                vec![
                    Directional::U,
                    Directional::U,
                    Directional::L,
                    Directional::L,
                ],
            ],
            (Numeric::Three, Numeric::Eight) => vec![
                vec![Directional::L, Directional::U, Directional::U],
                vec![Directional::U, Directional::U, Directional::L],
            ],
            (Numeric::Three, Numeric::Nine) => vec![vec![Directional::U, Directional::U]],
            (Numeric::Four, Numeric::A) => vec![vec![
                Directional::R,
                Directional::R,
                Directional::D,
                Directional::D,
            ]],
            (Numeric::Four, Numeric::Zero) => {
                vec![vec![Directional::R, Directional::D, Directional::D]]
            }
            (Numeric::Four, Numeric::One) => vec![vec![Directional::D]],
            (Numeric::Four, Numeric::Two) => vec![
                vec![Directional::R, Directional::D],
                vec![Directional::D, Directional::R],
            ],
            (Numeric::Four, Numeric::Three) => vec![
                vec![Directional::R, Directional::R, Directional::D],
                vec![Directional::D, Directional::R, Directional::R],
            ],
            (Numeric::Four, Numeric::Four) => vec![vec![]],
            (Numeric::Four, Numeric::Five) => vec![vec![Directional::R]],
            (Numeric::Four, Numeric::Six) => vec![vec![Directional::R, Directional::R]],
            (Numeric::Four, Numeric::Seven) => vec![vec![Directional::U]],
            (Numeric::Four, Numeric::Eight) => vec![
                vec![Directional::R, Directional::U],
                vec![Directional::U, Directional::R],
            ],
            (Numeric::Four, Numeric::Nine) => vec![
                vec![Directional::R, Directional::R, Directional::U],
                vec![Directional::U, Directional::R, Directional::R],
            ],
            (Numeric::Five, Numeric::A) => vec![
                vec![Directional::R, Directional::D, Directional::D],
                vec![Directional::D, Directional::D, Directional::R],
            ],
            (Numeric::Five, Numeric::Zero) => vec![vec![Directional::D, Directional::D]],
            (Numeric::Five, Numeric::One) => vec![
                vec![Directional::L, Directional::D],
                vec![Directional::D, Directional::L],
            ],
            (Numeric::Five, Numeric::Two) => vec![vec![Directional::D]],
            (Numeric::Five, Numeric::Three) => vec![
                vec![Directional::R, Directional::D],
                vec![Directional::D, Directional::R],
            ],
            (Numeric::Five, Numeric::Four) => vec![vec![Directional::L]],
            (Numeric::Five, Numeric::Five) => vec![vec![]],
            (Numeric::Five, Numeric::Six) => vec![vec![Directional::R]],
            (Numeric::Five, Numeric::Seven) => vec![
                vec![Directional::L, Directional::U],
                vec![Directional::U, Directional::L],
            ],
            (Numeric::Five, Numeric::Eight) => vec![vec![Directional::U]],
            (Numeric::Five, Numeric::Nine) => vec![
                vec![Directional::R, Directional::U],
                vec![Directional::U, Directional::R],
            ],
            (Numeric::Six, Numeric::A) => vec![vec![Directional::D, Directional::D]],
            (Numeric::Six, Numeric::Zero) => vec![
                vec![Directional::L, Directional::D, Directional::D],
                vec![Directional::D, Directional::D, Directional::L],
            ],
            (Numeric::Six, Numeric::One) => vec![
                vec![Directional::L, Directional::L, Directional::D],
                vec![Directional::D, Directional::L, Directional::L],
            ],
            (Numeric::Six, Numeric::Two) => vec![
                vec![Directional::L, Directional::D],
                vec![Directional::D, Directional::L],
            ],
            (Numeric::Six, Numeric::Three) => vec![vec![Directional::D]],
            (Numeric::Six, Numeric::Four) => vec![vec![Directional::L, Directional::L]],
            (Numeric::Six, Numeric::Five) => vec![vec![Directional::L]],
            (Numeric::Six, Numeric::Six) => vec![vec![]],
            (Numeric::Six, Numeric::Seven) => vec![
                vec![Directional::L, Directional::L, Directional::U],
                vec![Directional::U, Directional::L, Directional::L],
            ],
            (Numeric::Six, Numeric::Eight) => vec![
                vec![Directional::L, Directional::U],
                vec![Directional::U, Directional::L],
            ],
            (Numeric::Six, Numeric::Nine) => vec![vec![Directional::U]],
            (Numeric::Seven, Numeric::A) => vec![vec![
                Directional::R,
                Directional::R,
                Directional::D,
                Directional::D,
                Directional::D,
            ]],
            (Numeric::Seven, Numeric::Zero) => vec![vec![
                Directional::R,
                Directional::D,
                Directional::D,
                Directional::D,
            ]],
            (Numeric::Seven, Numeric::One) => vec![vec![Directional::D, Directional::D]],
            (Numeric::Seven, Numeric::Two) => vec![
                vec![Directional::R, Directional::D, Directional::D],
                vec![Directional::D, Directional::D, Directional::R],
            ],
            (Numeric::Seven, Numeric::Three) => vec![
                vec![
                    Directional::R,
                    Directional::R,
                    Directional::D,
                    Directional::D,
                ],
                vec![
                    Directional::D,
                    Directional::D,
                    Directional::R,
                    Directional::R,
                ],
            ],
            (Numeric::Seven, Numeric::Four) => vec![vec![Directional::D]],
            (Numeric::Seven, Numeric::Five) => vec![
                vec![Directional::R, Directional::D],
                vec![Directional::D, Directional::R],
            ],
            (Numeric::Seven, Numeric::Six) => vec![
                vec![Directional::R, Directional::R, Directional::D],
                vec![Directional::D, Directional::R, Directional::R],
            ],
            (Numeric::Seven, Numeric::Seven) => vec![vec![]],
            (Numeric::Seven, Numeric::Eight) => vec![vec![Directional::R]],
            (Numeric::Seven, Numeric::Nine) => vec![vec![Directional::R, Directional::R]],
            (Numeric::Eight, Numeric::A) => vec![
                vec![
                    Directional::R,
                    Directional::D,
                    Directional::D,
                    Directional::D,
                ],
                vec![
                    Directional::D,
                    Directional::D,
                    Directional::D,
                    Directional::R,
                ],
            ],
            (Numeric::Eight, Numeric::Zero) => vec![vec![
                Directional::D,
                Directional::D,
                Directional::D,
                Directional::D,
            ]],
            (Numeric::Eight, Numeric::One) => vec![
                vec![Directional::L, Directional::D, Directional::D],
                vec![Directional::D, Directional::D, Directional::L],
            ],
            (Numeric::Eight, Numeric::Two) => vec![vec![Directional::D, Directional::D]],
            (Numeric::Eight, Numeric::Three) => vec![
                vec![Directional::R, Directional::D, Directional::D],
                vec![Directional::D, Directional::D, Directional::R],
            ],
            (Numeric::Eight, Numeric::Four) => vec![
                vec![Directional::L, Directional::D],
                vec![Directional::D, Directional::L],
            ],
            (Numeric::Eight, Numeric::Five) => vec![vec![Directional::D]],
            (Numeric::Eight, Numeric::Six) => vec![
                vec![Directional::R, Directional::D],
                vec![Directional::D, Directional::R],
            ],
            (Numeric::Eight, Numeric::Seven) => vec![vec![Directional::L]],
            (Numeric::Eight, Numeric::Eight) => vec![vec![]],
            (Numeric::Eight, Numeric::Nine) => vec![vec![Directional::R]],
            (Numeric::Nine, Numeric::A) => {
                vec![vec![Directional::D, Directional::D, Directional::D]]
            }
            (Numeric::Nine, Numeric::Zero) => vec![
                vec![
                    Directional::D,
                    Directional::D,
                    Directional::D,
                    Directional::L,
                ],
                vec![
                    Directional::L,
                    Directional::D,
                    Directional::D,
                    Directional::D,
                ],
            ],
            (Numeric::Nine, Numeric::One) => vec![
                vec![
                    Directional::D,
                    Directional::D,
                    Directional::L,
                    Directional::L,
                ],
                vec![
                    Directional::L,
                    Directional::L,
                    Directional::D,
                    Directional::D,
                ],
            ],
            (Numeric::Nine, Numeric::Two) => vec![
                vec![Directional::D, Directional::D, Directional::L],
                vec![Directional::L, Directional::D, Directional::D],
            ],
            (Numeric::Nine, Numeric::Three) => vec![vec![Directional::D, Directional::D]],
            (Numeric::Nine, Numeric::Four) => vec![
                vec![Directional::D, Directional::L, Directional::L],
                vec![Directional::L, Directional::L, Directional::D],
            ],
            (Numeric::Nine, Numeric::Five) => vec![
                vec![Directional::D, Directional::L],
                vec![Directional::L, Directional::D],
            ],
            (Numeric::Nine, Numeric::Six) => vec![vec![Directional::D]],
            (Numeric::Nine, Numeric::Seven) => vec![vec![Directional::L, Directional::L]],
            (Numeric::Nine, Numeric::Eight) => vec![vec![Directional::L]],
            (Numeric::Nine, Numeric::Nine) => vec![vec![]],
        }
    }
}

impl Directional {
    fn routes_to(self, to: Directional) -> Vec<Vec<Directional>> {
        match (self, to) {
            (Directional::A, Directional::A) => vec![vec![]],
            (Directional::A, Directional::U) => vec![vec![Directional::L]],
            (Directional::A, Directional::R) => vec![vec![Directional::D]],
            (Directional::A, Directional::D) => vec![
                vec![Directional::L, Directional::D],
                vec![Directional::D, Directional::L],
            ],
            (Directional::A, Directional::L) => {
                vec![vec![Directional::D, Directional::D, Directional::L]]
            }
            (Directional::U, Directional::A) => vec![vec![Directional::R]],
            (Directional::U, Directional::U) => vec![vec![]],
            (Directional::U, Directional::R) => vec![
                vec![Directional::R, Directional::D],
                vec![Directional::D, Directional::R],
            ],
            (Directional::U, Directional::D) => vec![vec![Directional::D]],
            (Directional::U, Directional::L) => vec![vec![Directional::D, Directional::L]],
            (Directional::R, Directional::A) => vec![vec![Directional::U]],
            (Directional::R, Directional::U) => vec![
                vec![Directional::L, Directional::U],
                vec![Directional::U, Directional::L],
            ],
            (Directional::R, Directional::R) => vec![vec![]],
            (Directional::R, Directional::D) => vec![vec![Directional::L]],
            (Directional::R, Directional::L) => vec![vec![Directional::L, Directional::L]],
            (Directional::D, Directional::A) => vec![
                vec![Directional::R, Directional::U],
                vec![Directional::U, Directional::R],
            ],
            (Directional::D, Directional::U) => vec![vec![Directional::U]],
            (Directional::D, Directional::R) => vec![vec![Directional::R]],
            (Directional::D, Directional::D) => vec![vec![]],
            (Directional::D, Directional::L) => vec![vec![Directional::L]],
            (Directional::L, Directional::A) => {
                vec![vec![Directional::R, Directional::R, Directional::U]]
            }
            (Directional::L, Directional::U) => vec![vec![Directional::R, Directional::U]],
            (Directional::L, Directional::R) => vec![vec![Directional::R, Directional::R]],
            (Directional::L, Directional::D) => vec![vec![Directional::R]],
            (Directional::L, Directional::L) => vec![vec![]],
        }
    }
}
