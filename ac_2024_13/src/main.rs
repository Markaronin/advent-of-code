use advent_of_code_util::{base_aoc, parse::read_blocks, Coordinate};
use itertools::Itertools;

#[derive(Debug)]
struct ClawMachine {
    a: Coordinate,
    b: Coordinate,
    end: Coordinate,
}
impl ClawMachine {
    fn solve_for_b(&self) -> Option<usize> {
        let a1 = self.a.x as isize;
        let a2 = self.a.y as isize;
        let b1 = self.b.x as isize;
        let b2 = self.b.y as isize;
        let c1 = self.end.x as isize;
        let c2 = self.end.y as isize;

        let top = (a1 * c2) - (a2 * c1);
        let bottom = (a1 * b2) - (a2 * b1);
        if top % bottom == 0 {
            (top / bottom).try_into().ok()
        } else {
            None
        }
    }

    fn solve_for_a_given_b(&self, b: usize) -> Option<usize> {
        let a1 = self.a.x as isize;
        let b1 = self.b.x as isize;
        let c1 = self.end.x as isize;

        let top = c1 - (b1 * b as isize);
        let bottom = a1;
        if top % bottom == 0 {
            (top / bottom).try_into().ok()
        } else {
            None
        }
    }

    pub fn min_tokens(&self, max_presses: usize) -> Option<usize> {
        self.solve_for_b().and_then(|b_presses| {
            self.solve_for_a_given_b(b_presses).and_then(|a_presses| {
                if a_presses > max_presses || b_presses > max_presses {
                    None
                } else {
                    Some((3 * a_presses) + b_presses)
                }
            })
        })
    }
}

impl From<Vec<String>> for ClawMachine {
    fn from(value: Vec<String>) -> Self {
        let line_a = value[0].split_ascii_whitespace().collect_vec();
        let a_x = line_a[2][2..]
            .trim_end_matches(',')
            .parse::<usize>()
            .unwrap();
        let a_y = line_a[3][2..].parse::<usize>().unwrap();
        let line_b = value[1].split_ascii_whitespace().collect_vec();
        let b_x = line_b[2][2..]
            .trim_end_matches(',')
            .parse::<usize>()
            .unwrap();
        let b_y = line_b[3][2..].parse::<usize>().unwrap();

        let line_prize = value[2].split_ascii_whitespace().collect_vec();
        let prize_x = line_prize[1][2..]
            .trim_end_matches(',')
            .parse::<usize>()
            .unwrap();
        let prize_y = line_prize[2][2..].parse::<usize>().unwrap();

        Self {
            a: Coordinate { x: a_x, y: a_y },
            b: Coordinate { x: b_x, y: b_y },
            end: Coordinate {
                x: prize_x,
                y: prize_y,
            },
        }
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input: Vec<ClawMachine> = read_blocks(input_file)
        .into_iter()
        .map(|block| block.into())
        .collect_vec();

    let answer_1 = input.iter().map(|c| c.min_tokens(100).unwrap_or(0)).sum();

    let answer_2 = input
        .iter()
        .map(|c| ClawMachine {
            a: c.a,
            b: c.b,
            end: Coordinate {
                x: c.end.x + 10000000000000,
                y: c.end.y + 10000000000000,
            },
        })
        .map(|c| c.min_tokens(usize::MAX).unwrap_or(0))
        .sum();

    (answer_1, answer_2)
}

base_aoc!(480, 1545093008511);
