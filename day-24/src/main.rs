use rayon::prelude::*;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");
    let mut lines = input.lines();

    let initials = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(parse_initial)
        .collect::<BTreeMap<_, _>>();
    let original_gates = lines.map(parse_gate).collect::<BTreeMap<_, _>>();
    let gates = original_gates
        .iter()
        .map(|(name, g)| (*name, g.expand(&original_gates)))
        .collect::<BTreeMap<_, _>>();

    let z = evaluate_with_initials(&gates, &initials);
    println!("Part 1: {z}");

    let mut tests = BTreeSet::new();
    for bits in 1..46 {
        for x in [0, 1, (1 << bits) - 1, (1 << 45) - 1] {
            for y in [0, 1, (1 << bits) - 1] {
                let initials = initials_for(x, y, &initials);
                tests.insert((x, y, initials));
            }
        }
    }

    let swaps = find_swaps(&tests, &original_gates, Vec::new(), 4).unwrap();
    let mut swaps: Vec<_> = swaps.into_iter().flat_map(|(a, b)| vec![a, b]).collect();
    swaps.sort();
    println!("Part 2: {}", swaps.join(","));
}

fn find_swaps<'a>(
    tests: &'_ BTreeSet<(u64, u64, BTreeMap<&'a str, bool>)>,
    original_gates: &'a BTreeMap<&'a str, Value<'a>>,
    swapped: Vec<(String, String)>,
    available: usize,
) -> Result<BTreeSet<(String, String)>, BTreeSet<usize>> {
    let mut prefix = String::from(" ");
    for i in 0..4 {
        if let Some((l, r)) = swapped.get(i) {
            prefix.push_str(&format!("({l}, {r}) "));
        } else {
            prefix.push_str(&format!("(___, ___) "));
        }
    }
    let expanded_gates = original_gates
        .iter()
        .map(|(name, g)| (*name, g.expand(&original_gates)))
        .collect::<BTreeMap<_, _>>();

    let failing: BTreeSet<_> = tests
        .into_iter()
        .filter(|(a, b, initials)| a + b != evaluate_with_initials(&expanded_gates, &initials))
        .collect();

    if available == 0 {
        if failing.is_empty() {
            println!("{prefix} good");
            return Ok(BTreeSet::new());
        } else {
            return Err(BTreeSet::new());
        }
    }

    let mut diff = 0;
    for (a, b, initials) in tests {
        let res = evaluate_with_initials(&expanded_gates, &initials);
        diff |= (a + b) ^ res;
    }
    let mut suspicious = BTreeSet::new();
    let mut suspicious2 = BTreeSet::new();
    let mut suspicious_outputs = Vec::new();
    while diff > 0 {
        let tz = diff.trailing_zeros();
        let z = format!("z{:0>2}", tz);
        if suspicious.is_empty() {
            original_gates
                .get(z.as_str())
                .unwrap()
                .dependencies_inner(&mut suspicious);
        }
        original_gates
            .get(z.as_str())
            .unwrap()
            .dependencies_inner(&mut suspicious2);
        diff ^= 1 << tz;
        suspicious_outputs.push(z);
    }
    println!("{prefix} wrong outputs: {}", suspicious_outputs.join(", "));

    for l in &suspicious {
        if swapped.iter().any(|(s, t)| s == l || t == l) {
            continue;
        }

        let Some(g) = original_gates.get(l) else {
            continue;
        };

        let swaps = original_gates.into_par_iter().find_map_first(|(r, h)| {
            if swapped.iter().any(|(s, t)| s == r || t == r) {
                return None;
            }
            let mut gates = original_gates.clone();
            gates.insert(l, h.clone());
            gates.insert(r, g.clone());
            if g.depends_on(r, &gates) || h.depends_on(l, &gates) {
                return None;
            }

            let expanded_gates = gates
                .iter()
                .map(|(name, g)| (*name, g.expand(&gates)))
                .collect::<BTreeMap<_, _>>();

            let failing2: BTreeSet<_> = tests
                .into_iter()
                .filter(|(a, b, initials)| {
                    a + b != evaluate_with_initials(&expanded_gates, &initials)
                })
                .collect();
            if failing2.is_subset(&failing) && failing2.len() < failing.len() {
                let mut swapped = swapped.clone();
                swapped.push((l.to_string(), r.to_string()));
                match find_swaps(tests, &gates, swapped, available - 1) {
                    Ok(mut swaps) => {
                        swaps.insert((l.to_string(), r.to_string()));
                        return Some(swaps);
                    }
                    Err(_) => return None,
                }
            }

            None
        });

        if let Some(swaps) = swaps {
            return Ok(swaps);
        }
    }

    Err(BTreeSet::default())
}

fn evaluate_with_initials(
    gates: &BTreeMap<&'_ str, Value<'_>>,
    initials: &BTreeMap<&'_ str, bool>,
) -> u64 {
    let mut z = 0;
    for (name, gate) in gates.iter().rev() {
        if name.starts_with('z') {
            z = z << 1 | gate.evaluate(&initials) as u64;
        }
    }

    z
}

fn initials_for<'a>(
    mut x: u64,
    mut y: u64,
    initials: &BTreeMap<&'a str, bool>,
) -> BTreeMap<&'a str, bool> {
    let mut res = initials.clone();
    for i in 0.. {
        let v = x & 1 == 1;
        x >>= 1;
        if let Some(i) = res.get_mut(format!("x{i:0>2}").as_str()) {
            *i = v;
        } else {
            break;
        }
    }
    for i in 0.. {
        let v = y & 1 == 1;
        y >>= 1;
        if let Some(i) = res.get_mut(format!("y{i:0>2}").as_str()) {
            *i = v;
        } else {
            break;
        }
    }
    res
}

#[derive(Clone, Debug)]
enum Value<'a> {
    Gate(Cow<'a, str>, Gate<'a>),
    Unknown(Cow<'a, str>),
}

impl<'a> From<Gate<'a>> for Value<'a> {
    fn from(gate: Gate<'a>) -> Self {
        Value::Gate("".into(), gate)
    }
}

impl<'a> Value<'a> {
    fn expand(&self, gates: &BTreeMap<&'a str, Value<'a>>) -> Value<'a> {
        match self {
            Value::Gate(name, gate) => match gate {
                Gate::And(lhs, rhs) => Value::Gate(
                    name.clone(),
                    Gate::And(Box::new(lhs.expand(gates)), Box::new(rhs.expand(gates))),
                ),
                Gate::Or(lhs, rhs) => Value::Gate(
                    name.clone(),
                    Gate::Or(Box::new(lhs.expand(gates)), Box::new(rhs.expand(gates))),
                ),
                Gate::Xor(lhs, rhs) => Value::Gate(
                    name.clone(),
                    Gate::Xor(Box::new(lhs.expand(gates)), Box::new(rhs.expand(gates))),
                ),
            },
            Value::Unknown(u) => gates
                .get(u.as_ref())
                .map(|g| g.expand(gates))
                .unwrap_or_else(|| self.clone()),
        }
    }

    fn evaluate(&self, initials: &BTreeMap<&'a str, bool>) -> bool {
        match self {
            Value::Gate(_, gate) => match gate {
                Gate::And(lhs, rhs) => lhs.evaluate(initials) & rhs.evaluate(initials),
                Gate::Or(lhs, rhs) => lhs.evaluate(initials) | rhs.evaluate(initials),
                Gate::Xor(lhs, rhs) => lhs.evaluate(initials) ^ rhs.evaluate(initials),
            },
            Value::Unknown(u) => initials[u.as_ref()].clone(),
        }
    }

    fn depends_on(&self, needle: &str, gates: &BTreeMap<&'a str, Value<'a>>) -> bool {
        match self {
            Value::Gate(_, gate) => match gate {
                Gate::And(lhs, rhs) | Gate::Or(lhs, rhs) | Gate::Xor(lhs, rhs) => {
                    lhs.depends_on(needle, gates) || rhs.depends_on(needle, gates)
                }
            },
            Value::Unknown(u) => {
                if u == needle {
                    return true;
                }

                if let Some(g) = gates.get(u.as_ref()) {
                    return g.depends_on(needle, gates);
                }

                false
            }
        }
    }

    fn dependencies_inner(&'a self, dependencies: &mut BTreeSet<&'a str>) {
        match self {
            Value::Gate(name, gate) => {
                dependencies.insert(name.as_ref());
                gate.dependencies_inner(dependencies);
            }
            Value::Unknown(u) => {
                dependencies.insert(u);
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Gate<'a> {
    And(Box<Value<'a>>, Box<Value<'a>>),
    Or(Box<Value<'a>>, Box<Value<'a>>),
    Xor(Box<Value<'a>>, Box<Value<'a>>),
}

impl<'a> Gate<'a> {
    fn dependencies_inner(&'a self, dependencies: &mut BTreeSet<&'a str>) {
        match self {
            Gate::And(lhs, rhs) | Gate::Or(lhs, rhs) | Gate::Xor(lhs, rhs) => {
                lhs.dependencies_inner(dependencies);
                rhs.dependencies_inner(dependencies);
            }
        }
    }
}

fn parse_initial(input: &str) -> (&str, bool) {
    let (name, value) = input.split_once(": ").unwrap();
    (name, value.parse::<u32>().unwrap() == 1)
}

fn parse_gate<'a>(input: &'a str) -> (&'a str, Value<'a>) {
    if let Some((lhs, rest)) = input.split_once(" AND ") {
        let (rhs, name) = rest.split_once(" -> ").unwrap();
        (
            name,
            Value::Gate(
                name.into(),
                Gate::And(
                    Box::new(Value::Unknown(lhs.into())),
                    Box::new(Value::Unknown(rhs.into())),
                ),
            ),
        )
    } else if let Some((lhs, rest)) = input.split_once(" OR ") {
        let (rhs, name) = rest.split_once(" -> ").unwrap();
        (
            name,
            Value::Gate(
                name.into(),
                Gate::Or(
                    Box::new(Value::Unknown(lhs.into())),
                    Box::new(Value::Unknown(rhs.into())),
                ),
            ),
        )
    } else if let Some((lhs, rest)) = input.split_once(" XOR ") {
        let (rhs, name) = rest.split_once(" -> ").unwrap();
        (
            name,
            Value::Gate(
                name.into(),
                Gate::Xor(
                    Box::new(Value::Unknown(lhs.into())),
                    Box::new(Value::Unknown(rhs.into())),
                ),
            ),
        )
    } else {
        panic!()
    }
}
