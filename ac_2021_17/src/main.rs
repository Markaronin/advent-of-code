use advent_of_code_util::*;
use itertools::Itertools;

#[derive(Debug)]
struct Vector {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct TargetArea {
    start: Vector,
    end: Vector,
}
impl TargetArea {
    fn from_str(line: &str) -> Self {
        let (start, end) = line
            .split_whitespace()
            .collect_tuple::<(&str, &str, &str, &str)>()
            .map(|(_, _, x_str, y_str)| {
                (
                    {
                        let mut chars = x_str.chars();
                        chars.next();
                        chars.next();
                        chars.next_back();
                        chars
                            .collect::<String>()
                            .split("..")
                            .map(|num| num.parse::<isize>().unwrap())
                            .collect_tuple::<(isize, isize)>()
                            .map(|(x, y)| Vector { x, y })
                            .unwrap()
                    },
                    {
                        let mut chars = y_str.chars();
                        chars.next();
                        chars.next();
                        chars
                            .collect::<String>()
                            .split("..")
                            .map(|num| num.parse::<isize>().unwrap())
                            .collect_tuple::<(isize, isize)>()
                            .map(|(x, y)| Vector { x, y })
                            .unwrap()
                    },
                )
            })
            .unwrap();
        TargetArea { start, end }
    }

    fn contains(&self, coord: &Vector) -> bool {
        (self.start.x..=self.start.y).contains(&coord.x)
            && (self.end.x..=self.end.y).contains(&coord.y)
    }
}

struct MovingCoordinate {
    position: Vector,
    velocity: Vector,
}
impl MovingCoordinate {
    fn step(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        if self.velocity.x > 0 {
            self.velocity.x -= 1;
        } else if self.velocity.x < 0 {
            self.velocity.x += 1;
        }
        self.velocity.y -= 1;
    }

    fn is_within(&self, target_area: &TargetArea) -> bool {
        target_area.contains(&self.position)
    }

    fn can_intercept(&self, target_area: &TargetArea) -> bool {
        !((self.position.x > target_area.start.y && self.velocity.x >= 0)
            || (self.position.x < target_area.start.x && self.velocity.x <= 0)
            || (self.position.y < target_area.end.x && self.velocity.y <= 0))
    }

    fn highest_point(&mut self, target_area: &TargetArea) -> Result<isize, isize> {
        let mut highest_point = self.position.y.clone();
        let mut did_intersect = false;
        if self.is_within(target_area) {
            did_intersect = true;
        }
        while self.can_intercept(target_area) || self.velocity.y > 0 {
            self.step();
            highest_point = std::cmp::max(highest_point, self.position.y);
            if self.is_within(target_area) {
                did_intersect = true;
            }
        }
        if did_intersect {
            Ok(highest_point)
        } else {
            Err(highest_point)
        }
    }
}

fn get_program_output(input_file: &str) -> (isize, isize) {
    let input = read_lines(input_file);

    let target_area = TargetArea::from_str(&input[0]);

    let mut highest_so_far = isize::MIN;
    let mut num_intercepts = 0;
    for starting_x_velocity in -300..=300 {
        for starting_y_velocity in -300..=300 {
            let mut mc = MovingCoordinate {
                position: Vector { x: 0, y: 0 },
                velocity: Vector {
                    x: starting_x_velocity,
                    y: starting_y_velocity,
                },
            };
            match mc.highest_point(&target_area) {
                Ok(val) => {
                    num_intercepts += 1;
                    highest_so_far = std::cmp::max(highest_so_far, val);
                }
                Err(_) => {}
            }
        }
    }

    (highest_so_far, num_intercepts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 45);
        assert_eq!(part_2_output, 0);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
