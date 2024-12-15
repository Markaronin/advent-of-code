use std::{collections::BTreeSet, str::FromStr};

use advent_of_code_util::{base_aoc, icoordinate::ICoordinate, parse::read_parsed_lines};
use itertools::Itertools;

const WIDTH: isize = if cfg!(test) { 11 } else { 101 };
const HEIGHT: isize = if cfg!(test) { 7 } else { 103 };

fn quadrant(coord: ICoordinate) -> Option<usize> {
    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;
    match (coord.x.cmp(&mid_x), coord.y.cmp(&mid_y)) {
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(0),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(1),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(2),
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(3),
        (std::cmp::Ordering::Equal, _) => None,
        (_, std::cmp::Ordering::Equal) => None,
    }
}

#[derive(Debug)]
struct Robot {
    pos: ICoordinate,
    vel: ICoordinate,
}
impl Robot {
    pub fn position_after_n_turns(&self, n: isize) -> ICoordinate {
        // Ensure we don't accidentally underflow by adding a lot of WIDTH and HEIGHT
        ICoordinate {
            x: (self.pos.x + (WIDTH * n) + (self.vel.x * n)) % WIDTH,
            y: (self.pos.y + (HEIGHT * n) + (self.vel.y * n)) % HEIGHT,
        }
    }
}
impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s
            .split_ascii_whitespace()
            .map(|s| {
                s[2..]
                    .split(',')
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect_tuple::<(isize, isize)>()
                    .unwrap()
            })
            .map(|(x, y)| ICoordinate { x, y })
            .collect_tuple()
            .unwrap();
        Ok(Self { pos, vel })
    }
}

fn display(input: &BTreeSet<ICoordinate>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if input.contains(&ICoordinate { x, y }) {
                print!("X")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

fn get_program_output(input_file: &str) -> (isize, isize) {
    let input: Vec<Robot> = read_parsed_lines(input_file);

    let answer_1 = {
        let mut robots_per_quadrant = [0, 0, 0, 0];
        input
            .iter()
            .flat_map(|r| quadrant(r.position_after_n_turns(100)))
            .for_each(|quad| robots_per_quadrant[quad] += 1);
        robots_per_quadrant.into_iter().product()
    };

    for i in 1..10000 {
        let positions = input
            .iter()
            .map(|r: &Robot| r.position_after_n_turns(i))
            .collect::<BTreeSet<ICoordinate>>();
        if positions.iter().any(|p| {
            [
                ICoordinate { x: p.x + 1, y: p.y },
                ICoordinate {
                    x: p.x + 1,
                    y: p.y + 1,
                },
                ICoordinate {
                    x: p.x + 1,
                    y: p.y - 1,
                },
                ICoordinate { x: p.x - 1, y: p.y },
                ICoordinate {
                    x: p.x - 1,
                    y: p.y + 1,
                },
                ICoordinate {
                    x: p.x - 1,
                    y: p.y - 1,
                },
                ICoordinate { x: p.x, y: p.y + 1 },
                ICoordinate { x: p.x, y: p.y - 1 },
            ]
            .iter()
            .all(|surrounding| positions.contains(surrounding))
        }) {
            println!("{i}");
            display(&positions)
        }
    }

    (answer_1, 0)
}

base_aoc!(12, 0);
