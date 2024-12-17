fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");

    let mut cpu = parse(input.lines());
    let original = cpu.clone();

    while cpu.step() {}

    println!(
        "Part 1: {}",
        cpu.output
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    let start = 0;
    let mut step = 1;
    let mut a = start;
    loop {
        let mut cpu = original.clone();
        cpu.a = a;

        while cpu.step() {}

        if cpu.output.len() > cpu.instructions.len() {
            panic!("Exceeded expected output, a = {a} (0x{a:X})");
        }

        if cpu.output == cpu.instructions {
            println!("Part 2: {a}");
            break;
        } else {
            let common = cpu
                .instructions
                .iter()
                .zip(cpu.output.iter())
                .take_while(|(i, o)| i == o)
                .count();
            let bits = (3 * common).saturating_sub(3 + 8 + 8);
            step = std::cmp::max(step, 1 << bits);
        }

        a += step;
    }
}

#[derive(Clone)]
struct Cpu {
    a: u128,
    b: u128,
    c: u128,
    pc: usize,
    instructions: Vec<u8>,
    output: Vec<u8>,
}

impl Cpu {
    fn step(&mut self) -> bool {
        let Some(instruction) = self.instructions.get(self.pc) else {
            return false;
        };
        let Some(operand) = self.instructions.get(self.pc + 1).copied() else {
            return false;
        };
        self.pc += 2;

        match instruction {
            0 => {
                // adv
                self.a >>= self.combo(operand.into());
            }
            1 => {
                // bxl
                self.b ^= u128::from(operand);
            }
            2 => {
                // bst
                self.b = self.combo(operand.into()) % 8;
            }
            3 => {
                // jnz
                if self.a != 0 {
                    self.pc = operand as usize;
                }
            }
            4 => {
                // bxc
                self.b ^= self.c;
            }
            5 => {
                // out
                self.output.push((self.combo(operand.into()) % 8) as u8);
            }
            6 => {
                // bdv
                self.b = self.a >> self.combo(operand.into());
            }
            7 => {
                // cdv
                self.c = self.a >> self.combo(operand.into());
            }
            _ => unreachable!(),
        }

        true
    }

    fn combo(&self, v: u128) -> u128 {
        match v {
            0..=3 => v,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Cpu {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut instructions = Vec::new();

    for line in lines {
        if let Some(rest) = line.strip_prefix("Register A: ") {
            a = rest.parse().unwrap();
        } else if let Some(rest) = line.strip_prefix("Register B: ") {
            b = rest.parse().unwrap();
        } else if let Some(rest) = line.strip_prefix("Register C: ") {
            c = rest.parse().unwrap();
        } else if let Some(rest) = line.strip_prefix("Program: ") {
            instructions = rest.split(',').map(|i| i.parse::<u8>().unwrap()).collect();
        }
    }

    Cpu {
        a,
        b,
        c,
        pc: 0,
        instructions,
        output: Vec::new(),
    }
}
