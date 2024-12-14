//const WIDTH: isize = 11;
//const HEIGHT: isize = 7;
const WIDTH: isize = 101;
const HEIGHT: isize = 103;

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let input = std::fs::read_to_string(input).expect("file exists");
    let lines = input.lines().collect::<Vec<_>>();

    let mut quadrants = [0; 4];
    let mut robots = Vec::new();
    for line in lines {
        let robot = Robot::parse(line);
        robots.push(robot);
        let robot = robot.advance(100);

        if robot.pos.0 == WIDTH / 2 || robot.pos.1 == HEIGHT / 2 {
            continue;
        }
        let qx = (robot.pos.0 < WIDTH / 2) as usize;
        let qy = (robot.pos.1 < HEIGHT / 2) as usize;
        quadrants[2 * qy + qx] += 1;
    }

    println!(
        "Total (part 1): {}",
        quadrants.into_iter().product::<usize>()
    );

    let (part2, _) = (0..10000)
        .map(|steps| {
            let robots = robots
                .iter()
                .map(|robot| robot.advance(steps))
                .collect::<Vec<_>>();

            let mut sum = (0.0, 0.0);
            for robot in &robots {
                sum.0 += robot.pos.0 as f32;
                sum.1 += robot.pos.1 as f32;
            }
            let mean = (sum.0 / robots.len() as f32, sum.1 / robots.len() as f32);

            let var = robots
                .iter()
                .map(|robot| {
                    (robot.pos.0 as f32 - mean.0).powi(2) + (robot.pos.1 as f32 - mean.1).powi(2)
                })
                .sum::<f32>();

            (steps, var)
        })
        .min_by(|(_, var1), (_, var2)| var1.total_cmp(var2))
        .unwrap();
    println!("Step (part 2): {part2}");
}

#[derive(Copy, Clone)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Robot {
    fn parse(line: &str) -> Self {
        let (x, rest) = line.strip_prefix("p=").unwrap().split_once(',').unwrap();
        let (y, rest) = rest.split_once(" ").unwrap();
        let pos = (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap());

        let (x, y) = rest.strip_prefix("v=").unwrap().split_once(',').unwrap();
        let vel = (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap());

        Self { pos, vel }
    }

    fn advance(self, steps: usize) -> Self {
        let pos = (
            (self.pos.0 + self.vel.0 * (steps as isize)).rem_euclid(WIDTH),
            (self.pos.1 + self.vel.1 * (steps as isize)).rem_euclid(HEIGHT),
        );

        Self { pos, ..self }
    }
}
