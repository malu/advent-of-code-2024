fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");
    let lines = input.lines().collect::<Vec<_>>();

    let mut total1 = 0;
    let mut total2 = 0;
    for scenario in lines.split(|line| line.is_empty()) {
        let scenario = parse_scenario(scenario);
        if let Some(cost) = scenario.cost() {
            total1 += cost;
        }

        let scenario = scenario.into_part2();
        if let Some(cost) = scenario.cost() {
            total2 += cost;
        }
    }

    println!("Total (part 1): {total1}");
    println!("Total (part 2): {total2}");
}

struct Scenario {
    a: (i64, i64),
    b: (i64, i64),
    t: (i64, i64),
}

impl Scenario {
    fn cost(&self) -> Option<i64> {
        let det = self.a.0 * self.b.1 - self.b.0 * self.a.1;
        if det != 0 {
            // at most one solution
            let l = self.t.1 * self.a.0 - self.t.0 * self.a.1;
            let r = self.b.1 * self.a.0 - self.b.0 * self.a.1;
            let b = l / r;
            if l != b * r {
                return None;
            }

            let l = self.t.0 - b * self.b.0;
            let r = self.a.0;
            let a = l / r;
            if l != a * r {
                return None;
            }

            Some(3 * a + b)
        } else {
            // potentially multiple, need to find cheapest
            let mut best = None;
            let mut b = 0;
            while b * self.b.0 <= self.t.0 && b * self.b.1 <= self.t.1 {
                let d = (self.t.0 - (self.b.0 * b), self.t.1 - (self.b.1 * b));

                let a = d.0 / self.a.0;
                if self.t == (self.a.0 * a + self.b.0 * b, self.a.1 * a + self.b.1 * b) {
                    let c = 3 * a + b;
                    if best.is_none_or(|cost| cost > c) {
                        best = Some(c);
                    }
                }

                b += 1;
            }

            best
        }
    }

    fn into_part2(self) -> Self {
        Self {
            t: (self.t.0 + 10000000000000, self.t.1 + 10000000000000),
            ..self
        }
    }
}

fn parse_scenario(scenario: &[&str]) -> Scenario {
    let (x, y) = scenario[0]
        .strip_prefix("Button A: X+")
        .unwrap()
        .split_once(", Y+")
        .unwrap();
    let a = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
    let (x, y) = scenario[1]
        .strip_prefix("Button B: X+")
        .unwrap()
        .split_once(", Y+")
        .unwrap();
    let b = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
    let (x, y) = scenario[2]
        .strip_prefix("Prize: X=")
        .unwrap()
        .split_once(", Y=")
        .unwrap();
    let t = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());

    Scenario { a, b, t }
}
